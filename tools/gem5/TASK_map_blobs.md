# Gemini task 5: decode the field map blobs (ev/map/*.ccs BIN_*) end to end

Output folder: `F:\Projects\Decomp\hackgu-rs\gemini_out5\`. Scripts in `tools\gem5\`.
Read-only on everything else. Do NOT edit `crates\`, `HANDOFF.md` or `assets\`, and do NOT create git branches or
commits. Deliver data files and notes only.

## Rules (please read: last time several "VERIFIED" items were wrong)
Claude checked gemini_out4 against the binary. These were wrong:
- `town_crowd.csv` models `pc/2pc01..04.ccs` do not exist.
- The facings given for DMY_pc06/07 and DMY_ev20..23 are invented: those are 0x1300 position-only locators. The
  values were copied from PM10100's cast facings.
- The "parameter table" at 0xAC3EA8 holds 8-byte pointers (0x1807A5xxx), not `(u8,u8,i16,i16,i16)` records.
- The balloon range constant at 0x1808FB1F0 is not 2000.0 (the code at 0x1802964E4 divides by the float at
  0x1809016DC).

So:
- Mark a claim VERIFIED only if you read the value from the file or binary yourself and quote the raw bytes or
  instruction.
- When a value comes from an assumption, a reference screenshot or another table, mark it GUESS.
- Never fill a column to make a table look complete. Write `UNKNOWN` instead.
- Python and capstone are installed. Reuse `tools\gem2`..`tools\gem4` helpers. Do not name a script `dis.py`.

## Part A (main): the BIN_m101 layout format
`target\vol1\data\data\ev\map\m101.ccs` holds one BinaryBlob (section 0x2400) `BIN_m101`, 41,604 bytes. Gemini 4
found that hackGU_vol1.dll loads `ev/map/m101` / `BIN_m101` for special area 1, the tutorial
Δ Courageous Engaging Daydream, at about VA 0x180673E2E. Claude's dump of the payload:
- byte 1 = 1;
- bytes 665..1062 hold 1s and 4s, about 11 rows × 10 columns if read 40 wide;
- isolated values 1 @18116, 4 @22056, 2 @22091, 7 @25133, 3 @25134, 22 @26106;
- everything else is 0.

The tutorial field as built by Claude is `assets\areas\vol1_tutorial_field.json`. Its grid is 27 × 41 cells of
16 m each, read from the original's Area Map; `reference_captures\stills\holyground_areamap.png` is the screenshot.
The blob's land shape is much smaller, so one blob cell probably covers several 16 m kit tiles, or the 1s mark
something other than land.

Find in the DLL the code that READS this blob (start from where `BIN_m101`'s data pointer is used after the load at
0x180673E2E), and document:
1. The full byte layout: header, grid dimensions and stride, and what every section after the grid holds. Explain
   the bytes at 18116, 22056, 22091, 25133, 25134 and 26106 (e.g. object/locator tables, altar, temple, Platform,
   chest or enemy slots).
2. The meaning of each cell value (0, 1, 4 and any others) and how one blob cell turns into kit tiles: how many 16 m
   tiles per cell, which tile models (mt00/ml00/mv00/mf00/mx00/ob01/ob00 ...), and how rotations are chosen.
3. Where the arrival Platform, exit Platform, altars, temple and enemy spawns come from (the blob, the special-area
   table at RVA 0xB66174, or locators), with coordinates and units.
4. A decoded tutorial layout `m101_decoded.txt`. Draw it at 16 m per character with the same legend as the JSON
   grid ('#' land, '=' bridge, '.' sea), plus a second listing with object positions. Compare it with the JSON grid
   and the Area Map screenshot, and list every difference.

## Part B: all map blobs
Do the same decode for every file in `target\vol1\data\data\ev\map\` (m000, m101..m149, s101..s174). For each one:
- `<name>_decoded.txt`;
- a line in `maps.csv` with: file, grid size, objects found, and which special area id / Area Words /
  event script uses it.
  - The links come from the switch at 0x1806738F0 and its branch/jump tables (RVA 0x674EF8 / 0x674ED0), and from
    `gemini_out3\special_areas.csv`. Correct that csv's env/stage/event columns where they are wrong: Claude found
    the special-area table's +0x14 = area type (0 field, 2 cave, 3 castle, 4 Lost Ground) and +0x1C = field kit index.

## Part C: the procedural generator (only if A and B are done)
Gemini 4 named `FUN_180465160` (island count 3 + rand()%3, brackets) and sub-steps 0x1804657F0 / 0x180464710 /
0x180466340. Write each step as plain pseudo-code (cell counts, radii, bridge rules, and decoration variant rolls
like mt00 vs mt01) with exact constants, so it can be reimplemented. Then produce `random_example.txt`: the grid
for Δ + word ids (1st index 1 "Buzzing", 2nd index 0, 3rd index 0), seeded per gemini_out3\area_synthesis.md.
Mark which steps you actually traced.

Priority if short on time: A, then B, then C.
