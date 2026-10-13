# Gemini task 4: town crowd, post-filter selection, field layout generator (Vol.1)

Output folder: `F:\Projects\Decomp\hackgu-rs\gemini_out4\`. Scripts in `tools\gem4\`.
Read-only on everything else: do NOT edit `crates\`, `HANDOFF.md`, the game install, and do NOT create git branches
or commits this time. Deliver data files + notes only; Claude will implement.

Rules (same as before, stricter):
- Every value must come from the binary or data files. Mark each claim VERIFIED (traced: give RVA / table address /
  stride / file offset) or GUESS. Do not "fill in" anything to make a test pass.
- If a part cannot be traced, say so and stop on that part. An honest "not found" is more useful than a guess.
- Python + capstone is installed; reuse `tools\gem2`/`tools\gem3` helpers. Do not name a script `dis.py`.

## Part A (highest value): Mac Anu ambient player characters ("town crowd")
In the original, player characters stand around the Mac Anu gate hall and say short lines in speech balloons
(reference_captures/run1_opening f00000-f00023: "Come on now, double time! / Hut two three four!",
(game text), (game text)). These lines are messageInfo rows 51628, 54122, 53811
(ids 0x20918A70, 0xA2317463, 0xF9C35946). Names like ライダーチョップ / 狼一刀 / マーガレット / すみれ appear as a name
pool in hackGU_vol1.dll (pointer list around file offset 0x78F4C0 / 0x78F8C0).
Find in hackGU_vol1.dll:
1. The table(s) that define town crowd members: which town / district, position (locator name such as
   DMY_ev20..23, DMY_pc06/07, DMY_shop_npc.., or coordinates), facing, model (pc/2pcXnn.ccs and any colour/part
   variant), idle animation, name, and their chatter line ids (row numbers or CRC ids).
2. How the line is chosen and when the balloon shows (timer, distance).
3. Whether the set changes with story progress (e.g. a flag/chapter index) and the values for the very start of
   Vol.1 (right after PM10100).
Output: `town_crowd.csv` (one row per crowd member) + `town_crowd.md` (algorithm, addresses).

## Part B: post-filter record selection
`cmn/post_filter.bin` has per-scene tables `gu::<field|dungeon|lostground|town|...>::s_post_filter_param`
(132-byte records; layout in crates/hackgu-render/src/postfx.rs). The selector is FUN_18017e560 in hackGU_vol1.dll:
it reads `[rsi+0x2fc]` / `[rsi+0x304]` (field: index = tableA[x]*5 + tableB[y], tableA at RVA 0x79b9f8, tableB at
0x79bbe0; dungeon: table at 0x79b9d0 indexed by [rsi+0x2fc]; town: table 0x79b698).
Find what object `rsi` is and what the fields at +0x2fc and +0x304 mean (time of day? weather? field kit?), and the
values for: the tutorial field Δ Courageous Engaging Daydream, and its Beast Temple statue room (field/sf3_item).
Output: `postfilter_selection.md`.

## Part C: field layout generator
Fields are built at runtime from tile kits (field/sky_00 + sf3_00 etc.; no layout file). Claude rebuilt the
tutorial field by hand from the Area Map screenshot (crates/hackgu-render/src/field.rs GRID). Find the code that
generates or loads a field layout (grid of land/sea/bridge cells, altar/temple/platform positions) from the area
synthesis result (see gemini_out3/area_synthesis.md; seed MT19937). Deliver the algorithm and, if possible, the
generated grid for the tutorial area (special area 0, kit 0) so it can be compared with field.rs GRID.
Output: `field_layout.md` (+ `tutorial_grid.txt` if produced).

If you run short of time: A first, then B, then C.
