# Gemini task 6: where town events trigger and which coordinate frame they play in

Output folder: `F:\Projects\Decomp\hackgu-rs\gemini_out6\`. Scripts in `tools\gem6\`. Read-only on everything else.
Do NOT edit `crates\`, `HANDOFF.md` or `assets\`, and do NOT create git branches or commits.
Rules as in task 5: VERIFIED only with quoted bytes or instructions; otherwise GUESS or UNKNOWN. Never fill gaps.

## What Claude already established (verify, don't repeat)
- `hackGU_vol1.dll` file offset 0x7909BC holds a table of 240 event records, 0x74 bytes each: name (e.g. "PM10310")
  at +0, then five u32 at +0x60. Examples (+0x60 +0x64 +0x68 +0x6C +0x70):
  PM10100 200 1 0 0 1; PM10310 200 1 0 0 1; EM10320 201 3 0xE 0 1; EM10322 201 7 0xE 0 1; PM10315 201 0xA 0xE 0 1;
  PM10320 200 7 0 0 1; tutorial field events use 0x40..0x44.
- Scripts' start condition: `開始条件,75` (starts by story) vs `開始条件,79,<n>` with n = 1000/1500/2200 for
  EM10320/EM10322/PM10315 (probably a trigger radius in cm).
- Script header row 3rd column is an origin `[x|y|z]` (EM10320: [0|0|-100]) and PM10100 has a yaw (3.14159).
- PM10100's frame was solved visually: world = DMY_start (0, 228.5, 12 m) + Rz(180 deg) * local. PM10310 shares it.
- Mac Anu (town01.ccs) has locators DMY_marker01..40 (used by ccnavi.cpp, "DMY_marker*" string at file 0x895B08),
  DMY_ev20..24 in town01_09h.ccs ("DMY_ev%d" at 0x8FA868, world_man.cpp:908), DMY_start, DMY_gate, DMY_path01..05.

## Questions
1. Find the code that reads the record table (xrefs to 0x7909BC's VA). Name each of the five u32 fields from the code
   that uses them. In particular: what is +0x64 (1, 3, 7, 0xA, 0x40...) and how is it turned into a position
   (a locator name like DMY_marker%02d, a table of coordinates, or the map blob)? Quote the table if there is one.
2. Find the code for start condition 79: where the trigger centre comes from and what the argument (1000/1500/2200)
   means. Give the trigger centre in world metres for EM10320, EM10322 and PM10315.
3. Find how an event's coordinate frame is chosen when it starts (origin and yaw applied to the script's
   coordinates): from the record, the header origin row, the trigger position, the player, or a locator. Give the
   frame for PM10100 (it must come out as DMY_start + 180 deg), PM10310, EM10320, EM10322, PM10315 and PM10320.
4. List, for every record whose +0x64 is below 0x40, the resolved location (name and world position).

Deliver `event_locations.md` (findings with quoted instructions) and `event_records.csv`
(name, five fields, resolved location, trigger radius, frame origin, frame yaw; UNKNOWN where not traced).
