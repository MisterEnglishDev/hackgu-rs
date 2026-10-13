"""Export the enemy table from hackGU_vol1.dll to assets/battle/enemies.json.

Runs the DLL's static initializer at 0x1800B3410 using unicorn (tools/dllemu.py),
then reads the 154 entries (0x98 bytes each) at VA 0x180C6F3F0.

Traced by Gemini (task 8):
- Enemy table address: VA 0x180C6F3F0 (154 entries, 0x98 bytes each).
- Initializer: 0x1800B3410 populates entries 1..153 (entry 0 is in the file).
- Model dereference chain (enemy init at 0x1804B2990):
    p = *(u64*)(entry + 0) + variant * 0x18  (variant = signed byte at +0x28)
    p1 = *(u64*)p
    p2 = *(u64*)p1
    model_name = string at p2
- Scale %: int16 at +0x2E.
- Status block at +0x54 (copied to obj+0x1C0, then derived stats in obj+0x1E8..0x20A via 0x180488800):
    +0x54: level (i32) -> obj + 0x1E8
    +0x58: unknown1 / EXP (i32) -> obj + 0x1EC
    +0x5C: hp (i32) -> obj + 0x1F0
    +0x60: unknown2 / SP (i32) -> obj + 0x1F4
    +0x64: PA (int16) -> obj + 0x1F8
    +0x66: PD (int16) -> obj + 0x1FA
    +0x68: MA (int16) -> obj + 0x1FC
    +0x6A: MD (int16) -> obj + 0x1FE
    +0x6C: element 0 (int16, name UNKNOWN) -> obj + 0x200
    +0x6E: element 1 (int16, name UNKNOWN) -> obj + 0x202
    +0x70: element 2 (int16, name UNKNOWN) -> obj + 0x204
    +0x72: element 3 (int16, name UNKNOWN) -> obj + 0x206
    +0x74: element 4 (int16, name UNKNOWN) -> obj + 0x208
    +0x76: element 5 (int16, name UNKNOWN) -> obj + 0x20A
- Enemy name mapping: 0x180593CD0 reads name key from pointer table at 0x180B12120
  indexed directly by enemy index (0..153; index 0x71 takes a separate path in 0x180593CD0, not traced), then looks up string in MessageManager (messageInfo.bin).
"""
import json
import os
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from dllemu import Emu
from pe_util import PE
from gamedir import dll as _dll

TABLE_VA = 0x180C6F3F0
ENTRY_SIZE = 0x98
ENTRY_COUNT = 154
NAME_TABLE_VA = 0x180B12120


def message_id(key: str) -> int:
    crc = 0xFFFFFFFF
    for b in key.encode('ascii'):
        crc ^= (b << 24)
        for _ in range(8):
            if crc & 0x80000000:
                crc = ((crc << 1) ^ 0x04C11DB7) & 0xFFFFFFFF
            else:
                crc = (crc << 1) & 0xFFFFFFFF
    return (~crc) & 0xFFFFFFFF


def load_english_message_lut(repo_root: str):
    msg_path = os.path.join(repo_root, 'target', 'cmn', 'data', 'message', 'eng', 'messageInfo.bin')
    if not os.path.exists(msg_path):
        return {}
    with open(msg_path, 'rb') as f:
        data = f.read()
    if len(data) < 16:
        return {}
    count = struct.unpack('<I', data[4:8])[0]
    lut = {}
    for i in range(count):
        e = 16 + i * 40
        mid = struct.unpack('<I', data[e:e + 4])[0]
        rel = struct.unpack('<q', data[e + 16:e + 24])[0]
        start = e + 16 + rel
        end = data.find(b'\x00', start)
        text = data[start:end].decode('utf-8', errors='replace')
        lut[mid] = text
    return lut


def main():
    repo_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    dll_path = _dll()
    pe = PE(dll_path)

    # 1. Read name key pointers from DLL
    name_table_off = pe.rva_to_offset(NAME_TABLE_VA - 0x180000000)
    name_keys = []
    for i in range(ENTRY_COUNT):
        ptr = struct.unpack_from('<Q', pe.data, name_table_off + i * 8)[0]
        if ptr != 0 and ptr >= 0x180000000:
            str_off = pe.rva_to_offset(ptr - 0x180000000)
            key_str = pe.data[str_off:str_off + 64].split(b'\0')[0].decode('ascii', 'replace')
        else:
            key_str = None
        name_keys.append(key_str)

    msg_lut = load_english_message_lut(repo_root)

    # 2. Run static initializer in emulator
    emu = Emu(dll_path)
    emu.call(0x1800B3410)

    def read_str(addr, max_len=64):
        if addr < 0x180000000 or addr > 0x300000000:
            return None
        try:
            raw = emu.u.mem_read(addr, max_len)
            return raw.split(b'\0')[0].decode('ascii', 'replace')
        except Exception:
            return None

    def read_u64(addr):
        return struct.unpack('<Q', emu.u.mem_read(addr, 8))[0]

    def read_i8(addr):
        return struct.unpack('<b', emu.u.mem_read(addr, 1))[0]

    def read_i16(addr):
        return struct.unpack('<h', emu.u.mem_read(addr, 2))[0]

    def read_i32(addr):
        return struct.unpack('<i', emu.u.mem_read(addr, 4))[0]

    entries = []
    for i in range(ENTRY_COUNT):
        entry_addr = TABLE_VA + i * ENTRY_SIZE
        raw_bytes = emu.u.mem_read(entry_addr, ENTRY_SIZE)

        p0 = read_u64(entry_addr)
        var = read_i8(entry_addr + 0x28)
        model = None
        if p0 != 0:
            p = p0 + var * 0x18
            p1 = read_u64(p)
            if p1 != 0:
                p2 = read_u64(p1)
                model = read_str(p2)

        scale = read_i16(entry_addr + 0x2E)
        level = read_i32(entry_addr + 0x54)
        unk1 = read_i32(entry_addr + 0x58)
        hp = read_i32(entry_addr + 0x5C)
        unk2 = read_i32(entry_addr + 0x60)
        stats = [read_i16(entry_addr + 0x64 + j * 2) for j in range(10)]

        key = name_keys[i]
        name_en = msg_lut.get(message_id(key)) if key else None

        entry = {
            'index': i,
            'name_key': key,
            'name_en': name_en,
            'model': model,
            'variant': var,
            'scale_pct': scale,
            'level': level,
            'unknown1': unk1,
            'hp': hp,
            'unknown2': unk2,
            'stats': stats,
            'stats_derived': {
                'pa': stats[0],
                'pd': stats[1],
                'ma': stats[2],
                'md': stats[3],
                'element0': stats[4],
                'element1': stats[5],
                'element2': stats[6],
                'element3': stats[7],
                'element4': stats[8],
                'element5': stats[9],
            },
            'raw_hex': raw_bytes.hex(),
        }
        entries.append(entry)

    # Verification checks
    e0 = entries[0]
    assert e0['model'] == 'en/egg10', f"Entry 0 model mismatch: {e0['model']}"
    assert e0['level'] == 3, f"Entry 0 level mismatch: {e0['level']}"
    assert e0['hp'] == 102, f"Entry 0 hp mismatch: {e0['hp']}"

    out_dir = os.path.join(repo_root, 'assets', 'battle')
    os.makedirs(out_dir, exist_ok=True)
    out_file = os.path.join(out_dir, 'enemies.json')

    result = {
        'source': 'hackGU_vol1.dll enemy table (0x180C6F3F0) after 0x1800B3410 static init',
        'count': len(entries),
        'stat_indices': ['pa', 'pd', 'ma', 'md', 'element0', 'element1', 'element2', 'element3', 'element4', 'element5'],
        'entries': entries,
    }

    with open(out_file, 'w', encoding='utf-8') as f:
        json.dump(result, f, indent=2, ensure_ascii=False)

    print(f'Exported {len(entries)} enemy entries to {out_file}')
    print(f"Entry 0: {e0['name_en']} ({e0['name_key']}), model: {e0['model']}, level: {e0['level']}, hp: {e0['hp']}")


if __name__ == '__main__':
    main()
