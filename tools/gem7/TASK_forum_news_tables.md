# Gemini task 7: Community Forum and News Capture tables (what is visible when, and what posts give)

Output folder: `F:\Projects\Decomp\hackgu-rs\gemini_out7\`. Scripts in `tools\gem7\`. Read-only on everything else.
Do NOT edit `crates\`, `HANDOFF.md` or `assets\`, and do NOT create git branches or commits.
Rules as before: VERIFIED only with quoted bytes or instructions; otherwise GUESS or UNKNOWN. Never fill gaps.

## Known (Claude checked)
- Forum text keys in hackGU_vol1.dll near file offset 0x88122C (bbs.cpp): `bbs-<board>_<NNN>_1nickname`,
  `_2title`, `_3body` for boards apkallu (233 posts), theworld (196), news (186), officialforum (143), crimsonvs
  (99), rumors (76). The keys are messageInfo.bin keys (CRC-32/MPEG-2 of the key = message id; see
  `crates\hackgu-game\src\msginfo.rs`, `message_id`). E.g. bbs-apkallu_001 = Nogmung "Terror of Death".
- Wallpapers: accessory table at ~0x8779DD (83 records menu/xdwalNNNN + accessorydata_NNN), see HANDOFF.md.
- Reference footage reference_captures\run2_field f01097-f01198: right after the reformat the Apkallu board shows
  "No Subject" (MASARU), "A refreshing breeze..." (Maria), "A beautiful friendship" (SR), (game text)
  (Mayako (1/2)); opening Maria's post shows "Wallpaper downloaded!" (wallpaper 056 "Nice Day" then appears in
  Accessories). News Capture shows (game text) and "Nation Unified On Citizen Numbering
  System" with movies.

## Questions
1. Find the table(s) bbs.cpp uses to build each board: for every post, its key number, board, thread (parent / reply
   order), the condition that makes it visible (story flag / event id / chapter), and any reward it gives (wallpaper
   index, Area Word, item, mail). Quote the record layout from the code that reads it.
2. Same for News Capture (news.cpp or similar): items, their text keys, movies (cmn\movie files), visibility condition.
3. Which posts and news items are visible right after the reformat (the state in f01097-f01198)? List them in order
   and check against the four Apkallu threads above.
4. What sets the "NEW" marks, and what "(1/2)" after a name means.

Deliver `forum.csv` (board, post, key prefix, thread, parent, order, condition, reward), `news.csv`, and
`forum_notes.md` with the quoted instructions and record layouts.
