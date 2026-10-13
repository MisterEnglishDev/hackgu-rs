# Gemini task: reverse-engineer the Area Word system (Vol.1)

Output folder: `F:\Projects\Decomp\hackgu-rs\gemini_out3\` (create it). Put scripts in `tools\gem3\`.
Read-only on everything else: do NOT edit `crates\`, `HANDOFF.md` or the game install.

## Goal
Claude is rebuilding the Chaos Gate "Area Word" screen (crates/hackgu-render/src/chaosgate.rs). The art and
strings are known; the **logic is not**. Recover, from `F:\SteamLibrary\steamapps\common\hackGU\hackGU_vol1.dll`
(x64, Last Recode), how three Area Words (1st/2nd/3rd) plus a server (Δ Θ Λ Σ Ω) turn into an Area.

## Deliverables
1. `area_words.csv` — every Area Word in Vol.1:
   `slot(1/2/3), index, messageInfo_id(hex), english_text, japanese_text, <every attribute field you find>`
   (level contribution, element, field/dungeon type, treasure, flags such as "story only"/"available at start").
   English/Japanese text: `target\cmn\data\message\{eng,jpn}\messageInfo.bin`; ids are CRC-32/MPEG-2 of the key,
   format in `crates\hackgu-game\src\msginfo.rs`. Known anchors: "Courageous" 0xDF0C72F7 (row 18701),
   "Engaging" 0x041BDA60 (row 18852), "Daydream" 0x4D16BDED (row 18977). The word table in the DLL probably
   stores these ids or row indices — search for those constants first.
2. `area_synthesis.md` — the algorithm (with function addresses) that computes from (server, w1, w2, w3):
   Area LV, element(s), Field vs Dungeon, stage/tile kit used, thumbnail cell in `menu\xchaos_01.ccs`
   TEX_xsrnd_01 (4x4 grid of 256 px cells, index = row*4+col), Treasure LV, mission text id, the two
   "players/parties in area" counters, and the help-line choice ("too strong" .. "too weak", messageInfo
   0xA097AECB..0xBA11E379, and item lines 0x52A0FDF4..0x4826B046).
   Also: which words the player owns at the start of Vol.1 and how the gyro word list is ordered.
3. `special_areas.csv` — story/fixed areas (word triples that map to scripted areas, e.g.
   Δ Courageous Engaging Daydream = the tutorial field, Δ Hidden Forbidden Holy Ground = the Cathedral) with
   their stage files and event ids if you can find them (event scripts: `target\vol1\data\data\ev\*.csv`).
4. Ground-truth check: the original showed, for the tutorial triple, Area LV 1, Treasure LV 7, thumbnail cell 2,
   counters 000/000, mission 0x457C319B (game text); with only 1st word
   "Courageous" chosen: thumbnail cell 3, counter 002, mission 0x4E98AA45 "Defeat the boss!". Your algorithm must
   reproduce these; say explicitly whether it does.

## Rules
- Facts only from the binary/data. Mark every claim VERIFIED (traced in code/data) or GUESS.
- Give function RVAs and the exact table addresses/strides you used, so Claude can re-check them.
- Useful existing tools: `tools\gem2\*.py` (capstone disassembly helpers, caller dumps),
  `crates\hackgu-disasm`. Python with `capstone` is installed (do not name a script `dis.py`; it shadows a module).
- If you run out of time, deliver the word table (1) first; it is the most useful piece on its own.
