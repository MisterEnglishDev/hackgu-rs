"""Run functions of hackGU_vol1.dll in a CPU emulator (unicorn) to get exact results of the game's own code.

Used for data the DLL computes rather than stores (e.g. dungeon piece grids built from BIN_M103). The image is mapped
at its preferred base (0x180000000, so no relocation is needed), the IAT is pointed at stubs, and the DLL's allocator
(0x18067F1B0) is replaced by a bump allocator. Any other import call stops the run with its name, so nothing
unexpected is silently faked.

    emu = Emu()
    p = emu.alloc(size)
    emu.call(0x1804679F0, p)
"""
import struct
import sys
import os

from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE, UC_HOOK_BLOCK, UcError
import collections
from unicorn.x86_const import (UC_X86_REG_RAX, UC_X86_REG_RCX, UC_X86_REG_RDX, UC_X86_REG_R8, UC_X86_REG_R9,
                               UC_X86_REG_RSP, UC_X86_REG_RIP, UC_X86_REG_XMM0, UC_X86_REG_XMM1)
import math

# C runtime maths, evaluated in Python: name -> (arity, function). 'f' suffix = float32 in/out, else float64.
MATH = {'sin': (1, math.sin), 'cos': (1, math.cos), 'tan': (1, math.tan), 'sqrt': (1, math.sqrt),
        'asin': (1, math.asin), 'acos': (1, math.acos), 'atan': (1, math.atan), 'atan2': (2, math.atan2),
        'fabs': (1, abs), 'floor': (1, math.floor), 'ceil': (1, math.ceil), 'pow': (2, math.pow),
        'fmod': (2, math.fmod), 'exp': (1, math.exp), 'log': (1, math.log)}

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from pe_util import PE  # noqa: E402

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from gamedir import dll as _dll  # noqa: E402
DLL = _dll()
BASE = 0x180000000
STUBS = 0x100000000
HEAP = 0x200000000
STACK = 0x300000000
RET = 0x400000000
ALLOC_FN = 0x18067F1B0
FREE_FN = 0x18067F720


class Emu:
    def __init__(self, dll=DLL):
        self.pe = PE(dll)
        d = self.pe.data
        u = self.u = Uc(UC_ARCH_X86, UC_MODE_64)
        pe_off = struct.unpack_from('<I', d, 0x3C)[0]
        size_of_image = struct.unpack_from('<I', d, pe_off + 24 + 56)[0]
        size_of_headers = struct.unpack_from('<I', d, pe_off + 24 + 60)[0]
        u.mem_map(BASE, (size_of_image + 0xFFFF) & ~0xFFFF)
        u.mem_write(BASE, d[:size_of_headers])
        for vaddr, vsize, raddr, rsize, name in self.pe.sections:
            u.mem_write(BASE + vaddr, d[raddr:raddr + min(rsize, max(vsize, rsize))])
        u.mem_map(STUBS, 0x100000)
        u.mem_write(STUBS, b'\xC3' * 0x100000)
        u.mem_map(HEAP, 0x10000000)
        u.mem_map(STACK, 0x200000)
        u.mem_map(RET, 0x1000)
        u.mem_write(RET, b'\xF4')  # hlt
        self.heap = HEAP + 0x1000
        self.imports = {}
        self._patch_iat(pe_off)
        u.hook_add(UC_HOOK_CODE, self._stub_hook, begin=STUBS, end=STUBS + 0x100000)
        u.hook_add(UC_HOOK_CODE, self._fn_hook, begin=ALLOC_FN, end=ALLOC_FN)
        u.hook_add(UC_HOOK_CODE, self._fn_hook, begin=FREE_FN, end=FREE_FN)
        self.trail = collections.deque(maxlen=40)
        u.hook_add(UC_HOOK_BLOCK, lambda uc, a, s, _: self.trail.append(a))

    def _patch_iat(self, pe_off):
        d = self.pe.data
        opt = pe_off + 24
        imp_rva = struct.unpack_from('<I', d, opt + 112 + 8)[0]
        k = 0
        o = self.pe.rva_to_offset(imp_rva)
        while True:
            oft, _, _, name_rva, ft = struct.unpack_from('<5I', d, o)
            if not name_rva:
                break
            dll = self.pe.read_string(name_rva)
            i = 0
            while True:
                th = struct.unpack_from('<Q', d, self.pe.rva_to_offset((oft or ft) + i * 8))[0]
                if not th:
                    break
                fn = ('#%d' % (th & 0xFFFF)) if th >> 63 else self.pe.read_string((th & 0x7FFFFFFF) + 2)
                stub = STUBS + k * 16
                self.imports[stub] = dll + '!' + fn
                self.u.mem_write(BASE + ft + i * 8, struct.pack('<Q', stub))
                k += 1
                i += 1
            o += 20

    def alloc(self, n, zero=True):
        p = self.heap
        self.heap = (self.heap + n + 0xF) & ~0xF
        if zero:
            self.u.mem_write(p, b'\0' * n)
        return p

    def _ret(self, value):
        u = self.u
        rsp = u.reg_read(UC_X86_REG_RSP)
        ret = struct.unpack('<Q', u.mem_read(rsp, 8))[0]
        u.reg_write(UC_X86_REG_RSP, rsp + 8)
        u.reg_write(UC_X86_REG_RAX, value)
        u.reg_write(UC_X86_REG_RIP, ret)

    def _fn_hook(self, u, addr, size, _):
        if addr == ALLOC_FN:
            self._ret(self.alloc(u.reg_read(UC_X86_REG_RCX)))
        elif addr == FREE_FN:
            self._ret(0)

    def _stub_hook(self, u, addr, size, _):
        name = self.imports.get(addr, hex(addr))
        if name.endswith(('!memset',)):
            dst, c, n = u.reg_read(UC_X86_REG_RCX), u.reg_read(UC_X86_REG_RDX) & 0xFF, u.reg_read(UC_X86_REG_R8)
            u.mem_write(dst, bytes([c]) * n)
            self._ret(dst)
        elif name.endswith(('!memcpy', '!memmove')):
            dst, src, n = u.reg_read(UC_X86_REG_RCX), u.reg_read(UC_X86_REG_RDX), u.reg_read(UC_X86_REG_R8)
            u.mem_write(dst, bytes(u.mem_read(src, n)))
            self._ret(dst)
        elif name.split('!')[1].rstrip('f') in MATH:
            fn = name.split('!')[1]
            single = fn.endswith('f') and fn.rstrip('f') + 'f' == fn and fn != 'fmod' or fn == 'fmodf'
            arity, op = MATH[fn[:-1] if single else fn]
            fmt = '<f' if single else '<d'
            n = 4 if single else 8
            args = [struct.unpack(fmt, (u.reg_read(r) & ((1 << (8 * n)) - 1)).to_bytes(n, 'little'))[0]
                    for r in (UC_X86_REG_XMM0, UC_X86_REG_XMM1)[:arity]]
            v = struct.pack(fmt, float(op(*args)))
            u.reg_write(UC_X86_REG_XMM0, int.from_bytes(v, 'little'))
            self._ret(u.reg_read(UC_X86_REG_RAX))
        else:
            raise RuntimeError('import called: ' + name)

    def call(self, fn, *args, max_insns=500_000_000):
        u = self.u
        rsp = STACK + 0x1F0000
        stack_args = list(args[4:])
        rsp -= 8 * (len(stack_args) + 4)
        rsp &= ~0xF
        for i, a in enumerate(stack_args):
            u.mem_write(rsp + 32 + 8 * i, struct.pack('<Q', a))
        rsp -= 8
        u.mem_write(rsp, struct.pack('<Q', RET))
        u.reg_write(UC_X86_REG_RSP, rsp)
        for reg, a in zip((UC_X86_REG_RCX, UC_X86_REG_RDX, UC_X86_REG_R8, UC_X86_REG_R9), args):
            u.reg_write(reg, a)
        try:
            u.emu_start(fn, RET, count=max_insns)
        except UcError as e:
            raise RuntimeError('%s at %s; blocks: %s' % (e, hex(u.reg_read(UC_X86_REG_RIP)), ' '.join(hex(a) for a in self.trail)))
        return u.reg_read(UC_X86_REG_RAX)

    def read(self, addr, n):
        return bytes(self.u.mem_read(addr, n))

    def write(self, addr, b):
        self.u.mem_write(addr, b)
