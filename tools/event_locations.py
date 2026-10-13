"""Export where every Vol.1 event plays, from hackGU_vol1.dll, to assets/events/vol1_event_locations.json.

Traced by Claude (2026-10-12):
- Event record table: 240 records of 0x74 bytes, name at +0, five u32 at +0x60 (file offset of PM10100's record found
  by walking back from "PM10310" at 0x7909BC). pddefine.cpp lookups at VA 0x180117140 (name -> index) and
  0x180117290 (name -> record+0x64, the location id).
- 0x1805DF790 calls the latter, then reads a 0x1C-byte row of the location table at RVA 0xAF13C8 indexed by the
  location id: row[0] = locator number (-1 = the player's position, 99 = special), row[1] = place type
  (1 town, 3 field, 4/5 dungeon/room...), and 0x1806731C0 resolves the frame by place type. For towns it ends in
  0x180673070, which formats "DMY_ev%d" with row[0] and copies that locator's position into the event frame.
- Start condition (script column "開始条件"): 75 = started by the story, 79 <n> = proximity trigger (n in cm;
  meaning of n inferred from the values 1000/1500/2200, not traced).
"""
import csv, io, json, os, struct, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from pe_util import PE

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from gamedir import dll as _dll  # noqa: E402
DLL = _dll()
EV_DIR = 'target/vol1/data/data/ev'
pe = PE(DLL)
d = pe.data

o = 0x7909BC
while d[o - 0x74:o - 0x72] in (b'PM', b'EM', b'ES', b'PS', b'MM'):
    o -= 0x74
records = []
while d[o:o + 2] in (b'PM', b'EM', b'ES', b'PS', b'MM'):
    name = d[o:o + 16].split(b'\0')[0].decode()
    f = struct.unpack_from('<5I', d, o + 0x60)
    records.append((name, f))
    o += 0x74

LOC_RVA = 0xAF13C8
def loc_row(i):
    return struct.unpack_from('<7i', d, pe.rva_to_offset(LOC_RVA + i * 0x1C))

def start_condition(name):
    path = os.path.join(EV_DIR, name + '.csv')
    if not os.path.exists(path):
        return None
    t = open(path, 'rb').read().decode('cp932', 'replace')
    for r in csv.reader(io.StringIO(t)):
        if len(r) > 5 and r[4] == '開始条件':
            args = [c for c in r[5:] if c]
            return [int(x) for x in args if x.lstrip('-').isdigit()]
    return None

out = []
for name, f in records:
    loc = f[1]
    row = loc_row(loc) if loc < 0x100 else None
    sc = start_condition(name)
    out.append({
        'event': name,
        'fields': list(f),
        'location': loc,
        'locator': row[0] if row else None,
        'place_type': row[1] if row else None,
        'area_words': list(row[4:7]) if row else None,
        'start': sc[0] if sc else None,
        'trigger_radius_cm': sc[1] if sc and sc[0] == 79 and len(sc) > 1 else None,
    })
os.makedirs('assets/events', exist_ok=True)
json.dump({'source': 'hackGU_vol1.dll event table + location table (tools/event_locations.py)', 'events': out},
          open('assets/events/vol1_event_locations.json', 'w', encoding='utf8'), indent=1, ensure_ascii=False)
print(len(out), 'events')
