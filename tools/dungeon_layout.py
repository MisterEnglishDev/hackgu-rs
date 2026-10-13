"""Export a story dungeon's piece layout by running hackGU_vol1.dll's own builder in an emulator (tools/dllemu.py).

    python tools/dungeon_layout.py <w1> <w2> <w3> <map ccs> <out json>
    python tools/dungeon_layout.py 11 27 21 target/vol1/data/data/ev/map/m103.ccs assets/dungeons/m103.json

Traced by Claude (2026-10-12):
- Area setup 0x18067C8B0(area, w1, w2, w3, 0) fills the current-Area object (global 0x180F84C10): +0x2E4 map id,
  +0x2FC kit (3 -> dungeon/dungeon2 + d02_obj), from the special-area table at RVA 0xB6616C.
- Dungeon map per map id (switch 0x180675066): 8 -> ev/map/m103 (BIN_M103, else BIN_m103), 12 m149, 13 m114,
  14 m118, 15 m123, 18 s107, 21..23 s131..s133, 25..28 s171..s174; other ids generate a random dungeon.
- Builder: object 0x370E0 bytes, ctor 0x1804679F0, then 0x18046A510(gen, blob+4, blob+0x3E84) (blob = BinaryBlob
  payload). Result: piece ids as int16 [40 x][40 y][5 floor] at gen+0x2BB8C (copied to the dungeon object +0x2B0
  by 0x1802E7A90).
- Cell placement: 0x1802E6030 builds each cell's model matrix (row-major 4x4, translation in the last row, cm),
  including mirrored pieces (x scale -1); 0x1802E3720 / 0x1802E3770 are the position / yaw helpers used for locators. Models per id: switch in 0x1802DFD30 (table 0x2E36A8 / 0x2E35B4); listed below as
  MODELS, read from that switch by tools/scratch dcase (every MDL the case loads, base first).
"""
import json
import os
import struct
import sys

sys.path.insert(0, os.path.dirname(__file__))
from dllemu import Emu  # noqa: E402

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from pe_util import PE  # noqa: E402
import capstone  # noqa: E402

AREA_GLOBAL = 0x180F84C10
NEEDED_GLOBALS = [0x180F82CA0, 0x180F82C48]  # server/session objects read (zeroed: server 0 = Delta, no quest state)


def models_by_id(pe):
    md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
    rd = lambda a, n: pe.data[pe.rva_to_offset(a):][:n]
    out = {}
    for v in range(4, 0x75):
        b = rd(0x2E36A8 + v - 4, 1)[0]
        t = struct.unpack('<I', rd(0x2E35B4 + b * 4, 4))[0]
        names = []
        for i in md.disasm(rd(t, 0x200), 0x180000000 + t):
            if i.mnemonic == 'lea' and 'rip +' in i.op_str:
                tg = i.address + i.size + int(i.op_str.split('rip + ')[1].rstrip(']'), 16) - 0x180000000
                s = pe.read_string(tg)
                if s.startswith('MDL_'):
                    names.append(s)
            if i.mnemonic == 'jmp' and i.op_str.startswith('0x1802e3'):
                break
        out[v] = names
    return out


def first_blob(path):
    # The first BinaryBlob (0x2400) section's payload, via the hackgu-ccs blobdump example (or a .bin given directly).
    if path.endswith('.bin'):
        return open(path, 'rb').read()
    import subprocess
    import tempfile
    tmp = os.path.join(tempfile.gettempdir(), 'dungeon_blob.bin')
    subprocess.run(['cargo', 'run', '--release', '-q', '-p', 'hackgu-ccs', '--example', 'blobdump', '--', path, tmp],
                   check=True, stdout=subprocess.DEVNULL)
    return open(tmp, 'rb').read()


def main():
    w = [int(x) for x in sys.argv[1:4]]
    src, out_path = sys.argv[4], sys.argv[5]
    e = Emu()
    area = e.alloc(0x1000)
    e.write(AREA_GLOBAL, struct.pack('<Q', area))
    for g in NEEDED_GLOBALS:
        e.write(g, struct.pack('<Q', e.alloc(0x100000)))
    e.call(0x18067C8B0, area, w[0], w[1], w[2], 0)
    a = e.read(area, 0x400)
    map_id, kit = struct.unpack_from('<i', a, 0x2E4)[0], struct.unpack_from('<i', a, 0x2FC)[0]
    blob = first_blob(src)
    b = e.alloc(len(blob) + 64)
    e.write(b, blob)
    gen = e.alloc(0x370E0)
    e.call(0x1804679F0, gen)
    e.call(0x18046A510, gen, b + 4, b + 0x3E84)
    grid = e.read(gen + 0x2BB8C, 16000)
    # Placement: 0x1802E6030(obj) writes one matrix per cell (all four model slots alike) into the cell records at
    # [obj+0x4E5F0] (0x180 bytes per cell, cells in x-major order, slot k matrix at +k*0x60+0x20): x scale (-1 for
    # mirrored pieces), y/z scale 1.001, rotation about z by piece id, translation (x*1600+800, y*1600+800, 0) cm.
    # The floor is obj+0x10; the grid is read from obj+0x2B0.
    obj = e.alloc(0x50000)
    cells_buf = e.alloc(40 * 40 * 0x180)
    e.write(obj + 0x2B0, grid)
    e.write(obj + 0x4E5F0, struct.pack('<Q', cells_buf))
    models = models_by_id(e.pe)
    floors = []
    for f in range(5):
        e.write(obj + 0x10, struct.pack('<i', f))
        e.write(cells_buf, bytes(40 * 40 * 0x180))
        e.call(0x1802E6030, obj, 0)
        cells = []
        for x in range(40):
            for y in range(40):
                pid = struct.unpack_from('<h', grid, ((x * 40 + y) * 5 + f) * 2)[0]
                if not pid:
                    continue
                m = struct.unpack('<16f', e.read(cells_buf + (x * 40 + y) * 0x180 + 0x20, 64))
                cells.append({'cell': [x, y], 'id': pid, 'models': models.get(pid, []),
                              'matrix': [round(v, 6) for v in m]})
        if cells:
            floors.append(cells)
    # Blob records (0x28 bytes from +0xBB88, count at +0xBB84): int16 type, x, y, z; x/y = v*400+200 cm, z = v*400 cm
    # (loader loop 0x180675490). Types 6..14 fill event locators 11..19 (type+5); type 1 lies in the start room.
    records = []
    for i in range(struct.unpack_from('<i', blob, 0xBB84)[0]):
        t, x, y, z = struct.unpack_from('<4h', blob, 0xBB88 + i * 0x28)
        records.append({'type': t, 'pos': [x * 4.0 + 2.0, y * 4.0 + 2.0, z * 4.0],
                        'param': struct.unpack_from('<h', blob, 0xBB88 + i * 0x28 + 0x14)[0]})
    kit_file = {3: 'dungeon2.ccs'}.get(kit, 'dungeon1.ccs')
    obj_file = {3: 'd02_obj.ccs'}.get(kit, 'd01_obj.ccs')
    doc = {
        'source': 'hackGU_vol1.dll dungeon builder run on %s (tools/dungeon_layout.py)' % os.path.basename(src),
        'words': w, 'map_id': map_id, 'kit_id': kit, 'kit': kit_file, 'objects': obj_file, 'cell_m': 16.0,
        'records': records,
        'floors': floors,
    }
    os.makedirs(os.path.dirname(out_path), exist_ok=True)
    json.dump(doc, open(out_path, 'w'), indent=1)
    print('map id', map_id, 'kit', kit, 'floors', [len(f) for f in floors])


if __name__ == '__main__':
    main()
