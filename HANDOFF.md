# .hack//G.U. Last Recode — Rust recreation: handoff (for Claude Code on the PC)

Goal: 1:1 Rust reimplementation of the PC (Steam) game so it can be modded. Game install: `F:\SteamLibrary\steamapps\common\hackGU`. Project: `F:\Projects\Decomp\hackgu-rs`. Extracted assets live under `target\` (vol1 / hackGU_vol2_a / cmn ...). Build: `cargo build --release`, run via `play_game.bat`.
Priority now: opening scenes (PM10100 etc.) and the first dungeon playable and faithful, for a playtest.

## State of the code (what is trustworthy)
VERIFIED (checked with a CPU renderer / plots):
- `hackgu-ccs` parser: section size handling fixed (plausible-header chaining, texture size from field at payload+20). All 1,387 CCS files parse to the stream marker (Gemini audit_v3.csv).
- Animation quaternions are conjugated relative to bind-pose Euler convention (anim.rs) — fixes Haseo's pose.
- HitMesh (0xB00) parsing (`hitmesh.rs`), `CollisionMesh::from_hit_meshes` (walls + floors), `resolve_walls`, `clip_camera` in `hackgu-world/collision.rs`.
- Mac Anu = `town01.ccs` + `town01_00h..08h` placed by `DMY_town01_XXd` locators (cm -> m). Spawn candidates `DMY_pc01..` ; `DMY_event01`=(0,-400) cm, `DMY_start`/`DMY_gate` (0,22850..23414,1200) = gate hall at north end (floor z=+12 m).
WRITTEN BUT NOT VISUALLY VERIFIED IN-GAME (never run by the author — no GPU in the authoring env):
- render/app.rs town assembly, player wall collision, camera clipping, reverse-Z depth (camera.rs uses infinite reverse-Z; depth_compare Greater, clear 0.0), in-process HCA audio (sound.rs: cridecoder + rodio; BGM loops on HCA loop points; English voice bank default), CJK font fallback (gui.rs).
KNOWN BAD / GEMINI-MADE (not 1:1, replace gradually): hackgu-game runtime (hardcoded cutscene cameras/actors, fake dialogue), gui.rs menus, much of app.rs's cutscene hooks.

## User-reported bugs from first playtest (to fix first)
1. Opening cutscene: Haseo just stands there; other cast missing; spawn wrong; camera far; ground and water texture flicker (water meshes sit coplanar in ground slots — dark strips with yellow pattern in the plaza); most text shows squares (fixed with CJK font fallback, unverified); audio janky (replaced with in-process decoder, unverified).
2. In the software render of the plaza there are floating house pieces and an untextured white cube near the ship monument — suspect object transform/material bug in stage loading. Investigate in-game.

## Opening cutscene (PM10100) facts
- Real script: `target\vol1\data\data\ev\PM10100.csv` (Shift-JIS CSV). `hackgu-game/src/ev.rs` already parses it (EvScript/EvPlayer, EvFrame with camera/actors/message/black/stage). `app.rs` consumes the EvPlayer camera but then OVERWRITES it each frame from hardcoded `GameState::Cutscene.camera_eye` (runtime.rs). Remove that override; drive everything from `EvFrame`.
- Ops: カット(60/61) camera keys `Ver0003:<n>:[eye]:[target]:roll:fov:near:frame:flags...` (cm), モデル(32 place / 7 pose / 8 motion / 9 move / 27 turn / 21 remove), キャスト(63 register / 96 / 100), CCS操作(50-58 load model/motion), ストリーム(33 stage, 35-37), 画面効果(39 fade-in/40 fade-out), BGM操作, メッセージ(68).
- Cast: ハセヲ (SPC, loads `1hsw01` + morph `f10100_hsw`, motion `m10100_2hsw` in ev\epc), アスタ (`m10100_2ast`), Iyoten (`m10100_2iyo`), 街人男1/2, 街人女1/2 (generic PCs, kind 11). Motion files: `ev\epc\m10100_2{hsw,ast,iyo}.ccs`. Stream file `ev\estr\m10100_s01.ccs` (only a Haseo "trall" clump + effects, not stage geometry).
- COORDINATES: cutA01 camera (-243,13348,1516)cm looks into the domed gate hall — rendered raw (no transform) it looks right (domed ceiling over dark water). cutA02 people stand at z=-1215 cm while the hall floor is +12 m: z sign/offset unresolved (script z maybe negated/offset). cutA06+ Haseo is placed at [0,-400,0] = `DMY_event01` exactly, so those coordinates appear to be absolute town coordinates, BUT camera poses for cutA06/A09 rendered raw land inside geometry — frame still unresolved (test: header row `PM10100,,[0,-260,0],3.14159` is an origin/yaw; applying it also did not look right). Needs in-game visual experimentation: render each cut with candidate transforms and compare to the original (movies at `hackGU\cmn\movie\vol1`, OP/MM .usm; the gameplay opening can be compared to YouTube footage).
- Voice: English bank `cmn\data\data\sound\PC\US\v_evVoiceVol1.awb` (AFS2/HCA, unencrypted); cue index = row N-1 for PM10100 (M101_{N-1:03d}); voice ids in msg blobs like `spcName_019_AMainScenario00002`.
- English text: `target\cmn\data\message\eng\messageInfo.bin` (5.2 MB, header `e9030000 68dd0000 08000000 ...`, strings with markup `<color76E1FF>`, `<rubyThe World|ザ　ワールド>`, `<iconTRIANGLE>`). NOT yet parsed; the msg_*.ccs blobs in `ev\msg` are Japanese only. A Gemini job is reverse engineering the format (gemini_out2\messageInfo_format.md).

## Dungeon (first field)
Not started beyond hitmesh/locator census (gemini_out\stages.csv, locators.csv). The tutorial "Delta Hidden Forbidden Holy Ground" arrival event is PM10105; Gemini job D identifies its stage files and spawn/warp/chest/enemy locators.

## Data from Gemini (F:\Projects\Decomp\hackgu-rs\gemini_out and gemini_out2)
audit_v3.csv, locators.csv (4,328), hitmesh.csv + hitmesh_flags.csv (72 surface flag values; no ceilings in data), stages.csv, unknown_sections.csv (12 unparsed section types), text_hits.csv, README_gemini.md. Treat Gemini claims as leads; verify against data.

## Useful dev tools (crates/hackgu-ccs/examples)
soft.rs (CPU renderer: `soft out.ppm eye=x,y,z target=x,y,z fov=45 size=700 file.ccs offset=x,y,z other.ccs`; metres; ~45 s for full town), dmy.rs (locators), inv.rs (section inventory), near.rs/hitcsv.rs (collision), cov.rs (texture coverage), gh.rs in hackgu-world (floor heights).

## Suggested next steps
1. Build and run; check town visuals (water z-fight, floating houses, white cube), player collision, follow camera distance (default 4.2 m, should feel like the original third-person camera), spawn at the original start.
2. Make EvPlayer the only driver of cutscenes (camera, cast spawn by model, motions, fades, stage), resolve the coordinate frames per cut visually.
3. Parse messageInfo.bin; show English text with correct fonts/markup; keep Japanese available.
4. First dungeon: stage + HitMesh collision + locator-driven spawns/warps/chests; enemies via real data.
5. Replace Gemini's fake runtime/UI with data-driven equivalents.

## Session 2026-10-10 (Claude) — verified in the running game
Tools: `hackgu-render --town|--ev PM10100 [--ev-seek cutA06+100] [--evframe cut:ox,oy,oz,yaw] [--cam ex,ey,ez,tx,ty,tz,fov] [--pos x,y,z,yaw] [--noui] [--mute] [--lang jpn] --shot out.png@frame` captures the real swapchain. Reference footage of the ORIGINAL game's PM10100 was captured (scratch: cap/*.jpg); keyboard in the original: Enter=A, Esc=B, E=Y (menu), arrows; injected keys need KEYEVENTF_SCANCODE.
Data: target/ was missing all of hackGU_vol1_a.cpk and hackGU_cmn_a.cpk (+ _pc packs). Now extracted (no overwrites).

DONE / VERIFIED
- Rigid models now use the world matrix of the Object node that references them (the per-mesh u32 was a name id, not a bone). Fixed floating pieces (e.g. MDL_kabu02, 58 m off).
- Model header byte 8 = draw flags: bit0 additive (all water/caustic/reflection layers), bit1 alpha decal (shadows/graffiti). Material float <1 = opacity. Water z-fighting gone.
- Vertex colours now rendered (sky dome, silhouettes, baked shadows were white). Sky drawn first, no depth.
- town01.ccs is a prop LIBRARY at the origin (doors, ships, sea tiles, box01 = the white cube). Only sky clumps are loaded. DLL strings show placement: ships at `DMY_ship_a*/b*/c*` with ANM_s_ship/st_ship/m_ship, sea `MDL_see_*` at DMY_see*, extra districts at `DMY_town%02d_%02dd_*` — not implemented yet.
- Spawn DMY_start (in front of the gate). Follow camera matched to the original (6.0 m, aim 1.9 m).
- COORDINATES (PM10100): ONE frame for the whole script: world = (0, 228.5, 12) m [DMY_start] + Rz(180°)·local. Verified vs real footage: cutA01 dome/sun, cutA02 townspeople on the central-plaza stairs (explains z=-12.15), cutA07 gate between camera and Haseo, cutA10/cut01 compositions. Header `[0|-260|0],π`: π matches, offset meaning unknown.
- Yaw convention: 0 faces -Y, CCW positive. Pose names -> anims via DLL table (file off 0x83bbb0, {name[96], code}); codes 47-64 = spc/ev/<chr>ev.ccs emotes.
- English text: messageInfo.bin (CRC-32/MPEG-2 ids). Script line -> BIN_ table row -> voice key MainScenarioNNNNN (text) / prefix spcName_NNN_A (speaker). US voices default.

STILL WRONG / UNKNOWN
- Gate hall INTERIOR is missing: town01_01h has the hall's collision (floor 12 m, x±20, y 17..70 local) but no visible interior; town01h.ccs/town01h_01.ccs are only HD textures + a stub model referencing 01h. Not found in any extracted vol1/cmn file. Top priority.
- MDL_obj_02h (00h, slab at y107-150 z12-13) is not visible in the real cutA04; no flag found that hides it.
- Townspeople: DLL name table (VA 0x180790250) gives indices ライダーチョップ#14 狼一刀#17 マーガレット#142 すみれ#143; appearance = base body (2pcm00/2pcf00…) + colour set pc/2pcc0N, mapping table not found. Shown untextured.
- HG Haseo 1hsw01: m10100 motions only drive 28/63 bones; 2hsw01 used instead.
- Real game shows event dialogue as small bottom subtitles, in-game talk as a bottom bar with speaker tab; our box is Gemini's.
- Audio stutter not verified (all tests ran --mute). Item 5 (dungeon) not started: PM10105 frame header is [0,0,0], yaw 162°; sf3_00 is the Tri-Edge cathedral (PM10140), the arrival field is likely field/sky_00 or sky_02 (DLL: "field/sky_%02d") — unverified. gemini_out2/dungeon1.csv columns are shifted (x column = payload length).
- VR mod moved out of the game folder to F:\SteamLibrary\steamapps\common\hackGU_vrmod_removed (user asked to remove it).

## Session 2026-10-11 (Claude) — verified in the running game
Reference footage of the ORIGINAL game (local, not in git): `reference_captures/` — run1_opening (title → PM10100 →
Mac Anu gate), run2_field (Chaos Gate → PM10105 → whole Holy Ground incl. fights and Beast Temple), stills (title
screens, gate hall, Area Map, ALTIMIT desktop). Frame names end in seconds since capture start.
New automation: `--field`, `--title select|menu|newgame`, `--movie` (New Game path), `--lang jpn`,
`HACKGU_TRACE_PLAYER=1`, `HACKGU_FIELD_SKIP=CMP_x,...`, `HACKGU_AUDIO_LOG=1`.

DONE
- Movie audio in-process (ffmpeg → f32 → rodio, video clock starts with audio); ffplay glitches gone (loopback
  recording: no dropouts/clicks). Cutscene animations crossfade using the script's blend column (pose/motion ops).
- Title: Last Recode volume select → Vol.1 menu → Normal/Cheat built from title/xtitle_00@ENG.ccs + btn00@ENG@WIN64.ccs,
  layout measured from the original by template matching (title.rs). Strings from messageInfo.bin ids 44296-44321.
- Mac Anu gate hall interior = town01_00h placed at DMY_town01_01d (NOT DMY_town01_00d); evidence in commit 64bfd24.
- CCS parser: legacy textures with oversized declared size no longer swallow the next section (501 files gain a texture).
- Holy Ground (field.rs): layout from the original's Area Map (16 m lattice = kit tile size); sky_00 shore tiles by
  marching squares, sf3_00 mx00 interiors / ob01 bridges / oc00 altars (+ itembox01 chests on DMY_oc00item) / oi00
  temple, f_common op00 platforms, sb01+sw01 sea. Ground lowered 1 m to match PM10105's cast heights. PM10105 plays
  in the field and hands over to play at Haseo's mark. Walking, floors and shore walls verified.

STILL WRONG / UNKNOWN
- Field: exact tile variants (mt01/ml01/mv01 decorated, mf01 mounds), grass tufts, bridge gate posts (ob00) and torches,
  temple orientation (assumed facing its bridge), crystal glow effect, fog/lighting — not from data; compare with
  reference_captures/run2_field. Goblin positions approximate (3 per altar); egg10 looks darker than the original's
  Goblin Rookie (maybe a colour variant). No battle system (Gemini's runtime only); party members vanish after PM10105.
- Story wiring after PM10100 still uses Gemini's Chaos Gate selector UI (original: Word Plate keyword screen, see
  run2_field ~f00495). ALTIMIT desktop is Gemini's (original reference: stills/altimit_desktop.png + run2 tail).
- Title backdrop animation (stripes/watermark), Continue/Load not implemented.

## Session 2026-10-11 (Claude, part 2) — verified in the running game
DONE
- ALTIMIT desktop (altimit.rs, `--desktop N`): wallpaper menu/xdwal0000.ccs (1:1, mean error 3.5/255 vs original),
  8 apps + 8 cubes on 16 measured orbit slots, rotation two slots per app, ring (xddfrm01/xddlens/xddlens01),
  banners drawn from each app's CMP_xddbrd<key> clumps posed by ANM_xddbrd<key> (data-driven positions), panel tint
  solved from the original (gamma alpha 0.6 over (24,156,188)), help bar with messageInfo strings + btn00 glyphs.
  Compared side by side for The World, Mail Station, News Capture, Community Forum, Accessories.
  Launch: The World -> login_to_the_world; other apps show "not implemented"; BACK -> title.
- egui now renders through a non-sRGB view of the sRGB swapchain = gamma-space blending, like the original 2D.
  Note: Color32::from_rgba_unmultiplied premultiplies in LINEAR space; premultiply by hand for measured colours.
- Chaos Gate menus (chaosgate.rs, `--town --gate gate|warp|word|word3|confirm`): gate prompt, Warp Menu wheel,
  Area Word (gyro wheel, 1st/2nd/3rd, Area Status with real xmenu_00@ENG labels/icons, xchaos_01 thumbnails),
  "Initiate Area warp?" -> PM10105. Replaces Gemini's selector UI. Gate prompt opens within 8 m of DMY_gate
  (old code tested distance to the origin and could never fire at the real gate).
- The tutorial field is "Δ Courageous Engaging Daydream" (area name + words in the original), not Hidden Forbidden
  Holy Ground (that is the Cathedral).

FINDINGS / OPEN
- The red in-game menu frames are not textures (no ORB match in 1,771 UI textures; no DDS/PNG in the DLLs): drawn
  as flat shapes with sampled colours. Area Word logic (word table, area synthesis) is in hackGU_vol1.dll: Gemini
  task tools/gem3/TASK_area_words.md -> gemini_out3/. Until then only the tutorial triple is offered, with the values
  the original showed. Not built: player status box on the Area Word screen, tutorial dialogue during the gate
  (Asta/Iyoten lines), Bookmark/Random/Warp Record/To Town.
- Game fonts exist (target/cmn/font: FOT-NewRodin Pro DB/EB, TsukuGo Pro D, AnitoStd L). NewRodin DB in egui did
  not match the original's measured cap height/width; Segoe UI does, so UI text stays on Segoe.

## Gemini area-word branches (merged 2026-10-11 after review)
Merged `feature/area-word-synthesis` + `feature/chaosgate-tutorial-flow` (fast-forward), then corrected.
VERIFIED against hackGU_vol1.dll by Claude: word tables (RVA 0xB446C0/0xB43D60/0xB43040, 48-byte entries, 66/50/70
words), WORD_TYPE_n_xxx keys = messageInfo CRCs, special-area table (RVA 0xB66174, 29 x 72 bytes), MT19937 first
value for seed 0x03E90000 (452509106), help-line thresholds (+-5/+-3/+-1 in sub_27DD10), tutorial line texts
(messageInfo rows 21726-21756, 41633-41648). Treasure LV in the tutorial area is 1 (the original's italic "1"
glyph; Claude's task note "7" was a misread).
CORRECTED: special-area +0x14 is the area type (0 field, 2/3 dungeon, 4 = the six Lost Grounds) and +0x1C the field
kit; Gemini had set all 29 to field/kit 0 (gemini_out3/special_areas.csv still has the wrong env/stage/event columns —
the event_script column is invented). Removed a stray Area LV digit beside the label.
NOT TRACED (marked in code): party counter "002" is a constant (wanderer generator sub_67C080 not ported); Lost
Ground stage/thumbnail mapping; tutorial speakers except rows 21729 (Asta), 21738/21748 (Iyoten).

## Beast Temple and statue room (2026-10-11, Claude) — verified in the running game
- Temple (sf3_00 CMP_oi00) = exterior + MDL_oi00_i, the interior corridor/stairs down to the Inner Door at DMY_door01
  (local 0, 41.1, -9.45). HIT_oi00_ahit is a seal wall across the doorway: kept until the Symbol Fragments are joined
  (story ToTemple), then the open collision is swapped in. Door HitMeshes are not used (entrance is open in the
  original). Field tiles/sea over the interior are hidden while the camera is inside (field::inside_temple) and their
  collision is dropped.
- Statue room = field/sf3_item.ccs (CMP_mi01*, HIT_mi01hit) with field/statue.ccs CMP_statue at DMY_statue and the
  offering chest at DMY_item; the DLL names field/sf3_item, DMY_statue and field/statue with the temple door code.
  Passing the Inner Door loads it and plays PM10140 (frame (0,14.1,1.2): z from data, y fitted). Opening the chest
  plays MM10145 (the PK ambush movie). Matched to reference f00803-f00850.
- Not yet: light shafts/blue grade of the room, followers inside the temple after PM10140, chest open animation,
  Mission Reward screen; Lost Ground thumbnails. Debug: HACKGU_STORY=temple starts the field with the temple open;
  --stage file.ccs loads any stage.

## Session 2026-10-11 (Claude, part 3) — playthrough checks and additions
Verified joins in the running game: PM10100 end -> Chaos Gate prompt; Chaos Gate tutorial (Enter + one Down on the
Warp Menu, as the original's cursor starts on Bookmark) -> PM10105 -> PM10110 -> free play; altar 1 PM10120 -> fight
-> results; altar 2 PM10125; last chest -> "Symbol Fragments joined" -> PM10135; temple (opens after PM10135) ->
Inner Door -> statue room PM10140 -> offering chest opens -> MM10145 -> Mac Anu (end of the playable part; the next
chapter's ordering is not known).
Added: Platform orb (CCS Effect 0x0E00 sprites on props), chest open animation (ANM_itembox01_open), statue-room
light shafts (ANM_mi01shine) and fitted post filter (dungeon record 1), field minimap (panel measured; zoom
estimated), --keys automation, HACKGU_STORY=chests|temple and HACKGU_FRAGMENTS=n debug starts, HACKGU_POSTFX override.
Gemini task 4 written: tools/gem4/TASK_town_crowd_and_rules.md (town crowd PCs with chatter, post-filter record
selection, field layout generator).
Open: town crowd at the gate (original has chatting PCs; data not traced), field post filter (selection rule unknown;
several records come out near-black, possibly night/cave moods), followers inside the temple, Mission Reward screen,
Game Over, menus (items/status/area map), real battle formulas.

## Data-driven areas (2026-10-11, Claude)
Areas are now `assets/areas/*.json` (field.rs `AreaDef`; format in assets/areas/README.md). The tutorial field was
exported with identical values and verified (same 2066 pieces, same event chain). The Chaos Gate warps to the area
whose server+words match; event chaining (arrival -> after_arrival -> free play), altars, enemies, party, temple,
statue room/movie, event frames and post filter all come from the file. `pieces` places arbitrary kit pieces
(dungeon rooms) with their HitMesh collision. `--area path.json` loads any definition for testing.
Still tutorial-shaped: the story controller (fieldplay Story: altars -> fragments -> temple) assumes the
"Symbol Fragment" mission; other missions (Defeat the boss) need a second controller once traced.

## Gemini task 4 review (2026-10-11, Claude)
gemini_out4 partly verified. TRUE: PC speech tags (PCSpeechBubble01007-01009, SPCSpeechBubble00395/00396/00707)
CRC to the reference chatter rows; DMY_ev%d / DMY_pc%d format strings exist; ev/map/m101.ccs BIN_m101 exists and is
loaded for special area 1. WRONG: crowd models pc/2pc01..04 (no such files), crowd facings (0x1300 locators have no
rotation), "param table" at 0xAC3EA8 (it is a pointer array), balloon range constant. Actor<->line<->locator pairing
UNVERIFIED. tutorial_grid.txt is a raw 40-wide read of BIN_m101 whose cell scale is unknown.
Next Gemini task: tools/gem5/TASK_map_blobs.md (decode BIN_* map blobs -> field layouts from data).

## Mail Station and the post-reformat desktop (after MM10145)
- `crates/hackgu-render/src/mailstation.rs`: Mail Station (tabs Read/Greeting/Address, inbox list, mail view with
  portrait and scrolling body). All textures and strings from the game (xddesktop@ENG.ccs: xddmtex01, xddicma*,
  xddttex03, xddiclogo02, xddcbrd02, xddsclo, CMP_xddstb ring, xddface*; messageInfo 41029-41033, 41042, 41115-41139;
  Ovan's mail rows 10656-10658). Layout measured on run2_field f01067/f01089/f01093/f01095 at 960x540.
  Body text wraps at 716 px (not the measured ~744) because our font is ~5% narrower: this keeps the original's
  line breaks (checked line by line against f01089).
  APPROXIMATE: Greeting/Address panes (seen only in passing, f01078/f01079); no open/close animations
  (ANM_xddmaup/madown); selected tab icon is TEX_xddicma*1 (the original looks paler, probably mid cross-fade).
- Story (`altimit::Story`): reboot -> Mail Station empty (21893 popup, 21894-21896) -> on close, desktop lines
  21897/21898, popup 21899, 21900, Ovan's mail arrives (NEW badge) -> entering the list "Ovan?!" (21902) ->
  after reading, 21903-21907 -> on close, desktop help popup 21908. Other apps while the mail is unread answer
  21901 ((game text)). GUESS: where the system popups sit inside Mail Station (centred), and
  the 21901 trigger.
- help_bar() now draws X/B/LB/RB glyphs (iconSQUARE/iconCANCEL/iconL1/iconR1) and a full-width variant for apps.
- Test: `hackgu-render --reboot --keys "60:Enter,...,190:Right,230:Enter,..."` (see the commit for the full key list).
- Next in the reference (f01097+): News Capture, Community Forum, wallpaper setting, Data Manager save.
- First log-in after the reformat: `runtime.reformatted` (set when the reboot sequence plays) makes
  `login_to_the_world` start PM10310 instead of the old invented "Welcome to The World R:2" box. Events outside the
  current area with no stage of their own load Mac Anu (app.rs start_cutscene). PM10310 uses PM10100's frame
  (DMY_start + 180 deg): it reuses PM10100's camera poses (cutA01/A02/A04-A06). Plays through to free movement at
  the gate; verified visually that Haseo, the gate and the two townspeople are in frame (no reference footage of
  PM10310 in reference_captures to compare against). Speaker labels ＰＣ女１/２ are still the raw Japanese ids.
- Skipping MM10145 now leads to the desktop reboot too (it used to drop into town with an invented Silabus line).
- Test `test_initial_cutscenes_prologue_chain` rewritten: it asserted Gemini's invented MM10145 -> MM10210 -> ...
  -> MM10240 -> Silabus chain.

## Review of Gemini task 5 (gemini_out5, map blobs) — NOT merged
Checked by Claude against the DLL and the blob bytes:
- TRUE: VA 0x180673F06 `lea r13,[rax+0x14]` and 0x180673F0F `lea r12,[rax+0x654]` exist (rax = return of
  0x1801620A0, which returns `[obj+0x38]` of the loaded BIN object), and two 0x12C0 buffers are allocated after.
- WRONG: grid 1 is placed at payload+0x14. Read that way, the land wraps across row ends (rows 18-20 run off column
  39 and continue at column 0). With stride 40, the shape is only contiguous when rows start at payload offset
  32..39 (mod 40), e.g. payload+16+0x14 = 36. So `rax` most likely points 16 bytes into our payload (blob header);
  the BinaryBlob loader that sets `[obj+0x38]` was never traced.
- WRONG: "Grid 2 is 200x200 at +0x654": that ends at 41,620 bytes; the blob is 41,604 (and 41,588 after a 16-byte
  header). Dimensions unknown.
- WRONG: the arrival Platform (value 1) lands on a sea cell of their own grid 1 (tile 19,16).
- UNVERIFIED (labels look invented): "Area Lighting / Sound Anchor", "Special Effect / Boundary Colliders", the
  3-byte tile records (model id / rotation / elevation 4.0 m), random_example.txt (seed, island radii).
- The JSON comparison quotes our altars/arrival values correctly but its "distances match within sub-tile
  tolerance" claim has no numbers behind it.
Usable as leads: the jump table at 0x1806741F3 over grid-2 bytes 1..56 and its handler addresses.

## Town events after PM10310 (in progress)
- Next story events: EM10320 / EM10322 (townspeople chatter, both start with (game text)), PM10315 (PKs
  in an alley), PM10320 (Silabus and Gaspard). Start condition `開始条件,79,<n>` (n = 1000/1500/2200) vs 75 (story).
- DLL event record table at file 0x7909BC (240 x 0x74 bytes; +0x64 = location id: 1 = gate log-in for
  PM10100/PM10310/PM10370..., 0x40-0x44 tutorial field, 3/7/0xA for EM10320/EM10322/PM10315). DMY_marker01..40 in
  town01.ccs are the navigation-cursor spots (ccnavi.cpp); location id N -> DMY_markerNN is a lead, not verified.
- The event frames for these are NOT solved: EM10320 at marker03 yaw 0 looks plausible, others unclear. Gemini task 6
  (tools/gem6/TASK_event_locations.md) asks for the code path. Not wired into the game until verified.
- Generic speakers (ＰＣ女１ etc., voice prefix SpeakerNNNNN) have no English name key in messageInfo; still raw.
- `--ev <id>` now starts that script (it used to fall back to PM10100); examples msgrows (BIN table rows with voice
  ids) and msgkey (messageInfo key lookup).

## Data Manager (save/load)
- `crates/hackgu-render/src/datamanager.rs`: Save/Load column, slot list (01-09: No./Level/Date/Play Time, chapter
  name of the selected slot), overwrite question, "Saved." popup. Textures xddsltex01-04, xddicsl10/20,
  xddiclogo02, xddttex03, xddsclo; strings from messageInfo. Layout measured on run2_field f01213-f01215; the desktop
  ring stays in place and is drawn over the column. GUESS: selected-slot highlight, the Load flow (not captured).
- Saves are this project's JSON in `saves/slotNN.json` (gitignored): chapter (First Cry before the reformat, Deja Vu
  after it, as in f01215), location Desktop, level, play time, UTC date, desktop story state, inbox, NEW badges,
  `reformatted`. Title "Continue"/"Load Data" resume the newest slot on the desktop (no slot picker yet); with no
  saves the title shows the game's own "There is no data to continue from..." line.
- Play time counts everything except the title screen (`runtime.play_time`).
- Data Manager can't be opened while Ovan's mail is unread (answers (game text)).

## Accessories: BG Setting (wallpapers)
- `crates/hackgu-render/src/accessories.rs`. Wallpaper list = the DLL's accessory table (83 records: file
  `menu/xdwalNNNN`, name key `accessorydata_NNN` in messageInfo, preview `TEX_xdwalNNNNp` in `xdwallsa_01.ccs`
  (512x512, picture in the top 512x384)). The messageInfo CSV row order of these names is NOT the list order (off by
  one after ~003): always use the keys. Checked: 056 = xdwal1530 "Nice Day", 073 = xdwal1547 (game text).
- Owned wallpapers start as 000 Default only; in the original they come from Community Forum posts ("Wallpaper
  downloaded!", f01190s), not built yet. Debug: `HACKGU_WALLPAPERS=56,73`. Current/owned wallpapers are saved.
- GUESS: "Changing..." duration (0.6 s). NOT DONE: BGM, Randomize (X).

## Area Map (field, SELECT) — parked, findings
- Reference: reference_captures/stills/holyground_areamap.png. Strings: "Area Map" 41773, help line 43754
  ((game text)), legend 41882 Current Position, 41885 Beast
  Statue, 41884 Altar, 41782 Platform (legend lists only the marker types present).
- Assets: `vol1/field/mapchip.ccs` TEX_mapchip (32x32 land/bridge chips: left 256x128 block = field land set,
  0..64 x 128..256 bridges; right = dungeon set) + TEX_mapback; `field/mapmenu@ENG.ccs` TEX_mapicon (arrow, Platform
  blue dot, green/magenta/grey dots, red corner brackets, N E W S letters). The gold Altar / Beast Statue map icons
  were NOT found (not in mapicon, xficon, xicon, xmenu_00).
- Measured (960x540): red frame (124,87)-(539,424), inner dark panel from x 136, N (324,117)-(339,133),
  W (143,306)-(161,322), E (509,306)-(520,321), legend panel (542,106)-(820,262) rows at y ~133/167/203/238,
  land (97,109,94) spanning (344,218)-(423,371).
- Why parked: the still's land does not fit our JSON grid at one scale (3.2 px/cell across vs 3.9 down) and looks
  like the coarse BIN_m101 grid drawn with mapchip chips. Needs Gemini 5b (blob layout) and the chip-selection rule.

## Fixes from the user's playthrough (2026-10-12)
- Menus lock the player and take the gamepad (A=Enter, B=Escape, D-pad=arrows) on the title, desktop, Chaos Gate
  and movies. Before, the player walked around under the Chaos Gate menu and D-pad Right equipped the broadsword
  (the "greatsword + weird animation" after PM10100).
- Event text now looks like the original (run2_field f00104/f00128): centred white subtitles with an outline, no
  name and no box; op-6 notices ((game text)) in a dark band mid-screen in pale gold. The
  invented navy box, coloured name badges, letterbox bars and "◈ Prologue ... [PM10100]" header are gone.
- Speaker names: Japanese labels with no voice entry map to the English name other lines with the same label use
  (all ev/msg tables); full-width letters made plain; generic extras without an English name get no name (GUESS).
- Event actors snap to the floor within 1 m of the scripted height (PM10100 had Asta at -60 cm and Iyoten at -55 cm,
  sunk to the knees).
- Music: script BGM numbers are bgm.acb CUE indices, not AWB indices (new `acb.rs` parses the @UTF tables:
  cue 3 = "rtown0" Mac Anu = AWB 135, cue 62 = "ev06" = AWB 53). Ops: 123 play, 134 stop, 135 fade out (2 s GUESS),
  138 back to the place's own music. Place music: Mac Anu "rtown0", desktop "TD_desktop"; fields: unknown (none).
  Battle theme is cue "btl0" (was raw AWB 62 = ev15).
- Title: eased transitions (cursor bar 8 frames, cards, art cross-fade, screen fade-in 12 frames at 30 fps; the
  lengths are xtitle_00's animation lengths, which animation drives what is a GUESS until the title DLL is traced).
- main.rs: only a `.ccs` argument is taken as a model path (automation flag values disabled explore mode).
- Test hooks: `--keys N:Skip` skips a playing movie.
- Chaos Gate "can't proceed" bug: the Warp-menu tutorial (rows 21728-21732) intercepted A on the first "Chaos Gate"
  screen, where its lines are not drawn, so the menu looked frozen for five presses. The tutorial now only takes
  input from the Warp Menu on (verified: Enter opens the Warp Menu and the IYOTEN/Asta lines show).

## In-game Main Menu (2026-10-12)
- `crates/hackgu-render/src/mainmenu.rs`: Esc / gamepad Start in town or the field. Wheel (cyclic order from
  run1_opening f00064-f00070: Log Out, Party, Items, Skill, Equipment, Customize, Skill Trigger, Status, Quest,
  Awakening, Strategy, Memoir), help lines 42008-42019, party panels with TEX_xf<chr>_01 faces and names from
  spcName keys, Money / Party's Chim panel. The field HUD is hidden while it is open. Only Log Out works (to the
  desktop); the others say "not implemented yet". Followers' levels are not known yet (not shown).
- Esc no longer sends the desktop, town, field or Chaos Gate to the title screen (keyboard players were thrown out
  of Mail Station / Data Manager when pressing Esc for "Return"). Esc in town/field opens the Main Menu.
- New games start with 0 GP (run1_opening shows 0 GP; the 1000 GP default was invented).
- Header bars are drawn over the wheels (Main Menu and the Chaos Gate Warp Menu), as in f00064.
- Test hook: `--keys N:Menu` presses the menu button.

## Town events: locations and triggers SOLVED from the DLL (supersedes Gemini task 6)
- Code path (traced by Claude): pddefine.cpp event table (240 x 0x74, name + five u32 at +0x60; +0x64 = location id)
  -> 0x1805DF790 -> location table at RVA 0xAF13C8 (0x1C rows: [0] locator number, -1 = player position, 99 =
  special; [1] place type 1 town / 3 field / 4,5 dungeon/room) -> 0x1806731C0 switches on the place type; for towns
  0x180673070 formats "DMY_ev%d" with row[0] and copies that locator's position into the event frame.
- `tools/event_locations.py` exports all 240 events to `assets/events/vol1_event_locations.json` (location, locator,
  place type, start condition 75/79 and the 79 trigger radius in cm). Loaded by `crates/hackgu-render/src/events.rs`.
- Town events play at DMY_ev<N> (town01.ccs) turned by the script header's yaw (PM10100 pi; others 0, checked by
  eye: EM10320 lane, EM10322 harbour, PM10315 alley, PM10320 harbour plaza). Header origin offsets are not applied.
- Triggers: after the latest story-started (75) event played, the following start-79 town events are armed; walking
  within the radius of their DMY_ev locator plays them. A PM proximity event leads into the next story event
  (PM10315 -> PM10320). `runtime.events_done` records played events and is saved.
- Debug: `HACKGU_EVENTS_DONE=PM10100,PM10310` seeds it (e.g. `--town --pos -125,-40,4` stands by PM10315).
- Cast models added: Silabus 2syl01, Gaspard 2gas01, Bordeaux 2bor01 (not in the Vol.1 archive; the DLL's character
  -> model table is not traced), Negimaru 2neg01, Grein 2gri01, Ovan 2ova01.
- Not done: field-type (3) frames from the field's own objects (the tutorial still uses its fitted event_frames);
  start condition 80; what happens after PM10320 (PM10330 is place type 4).

## Map blobs (BIN_mNNN / BIN_sNNN): what Claude traced (2026-10-12, corrected)
- Layout of the extracted payload (41,604 bytes): 4-byte header (00 01 00 00), grid 1 = 40x40 bytes at +4, grid 2 =
  200x200 bytes at +1604 (ends exactly at the file end). The DLL's rax+0x14 / rax+0x654 point there (rax = payload-16).
  Checked: with these offsets every grid-2 object of m101 (6) and m104 (24) sits on a land cell; the earlier "+36"
  reading was wrong (it only looked right because the land is centred).
- Axes: grid index [r][c] -> world x from r, world y from c (0x1806741E0: x = sub_r*320+160, y = sub_c*320+160; grid-2
  sub-cell = 1/5 of a grid-1 cell). The Area Map (holyground_areamap.png = Courageous Engaging Daydream) is this grid
  with x to the right and +c up (north).
- Grid 1 values: 0 sea, 1 land, 2 raised land, 3 ramp, 4 bridge (builder 0x180673F99, shore codes 0x180465f20, see
  below). Raised objects get z = 400 (cm) and the kit's raised shore pieces (sf3_00 mt10/ml10/mv10) span z 1..5 m.
- Tutorial check (tools: scratchpad blobgen): our hand-drawn tutorial grid (from the Area Map, 16 m per cell) is
  reproduced 1104/1107 cells by: each grid-1 land cell (br, bc) -> a 5x5 block of '#' centred on JSON col 2*br-30,
  row 55-2*bc; bridge cells -> '=' along the run where not already land. The 3 differences are stray hand-drawn cells.
  Objects then land at: Platform (1) grid (2.5, 11.5) [ours 2.9, 12.9 fitted]; altars 7/22/2 at (16.5, 4.7),
  (18.5, 15.5), (10.5, 21.5) [ours (16.8,4.5), (18.8,16.5), (10.8,22.7)]; Beast temple (4) at (10.5, 35.5) [ours 32.4].
  OPEN: this means one grid-1 cell = 32 m in our world, while the DLL's sub-cell maths (320 per 1/5 cell) gives 16 m
  if its units are cm. The footage-fitted tutorial supports 32 m; not resolved.
- Grid 2 object types (jump table 0x1806741F3): 1 Platform (0x180434710), 2/22 and 7..16 altars/event points
  (7..16 store a position slot: rsi + (v+0x74)*16), 4 Beast temple, 23 (0x180432990), 24 (0x180433020, z 400),
  5, 6, 46, 47, 55, 56 own handlers; 3, 17..21, 25, 48..54 ignored here.
- Special-area table used by the loader: rows of 0x48 bytes at RVA 0xB6616C (29 rows): +4 map id (-> obj+0x2E4),
  +0xC/+0x10/+0x14 Area Words, +0x1C set kit (0 sf3_00, 1 sf2_00, other -> sf3_00), +0x24 sky index (field/sky_%02d,
  +20 when obj+0x1C == 1; ordinary areas use random % 5). Map id -> file (switch 0x180673E00): 1 m101, 9 m104,
  10 m107, 11 m110, 16 s101, 17 s104, 19 s110, 20 s113, 24 s134, 29 s104; all others have no map blob.
  (gem areaword.rs SPECIAL_AREAS reads a different offset: its special_id is not this map id.)
- Peaceful Leading Freedom (11/27/21) is row 7: map id 8 = NO field map, env 3 (dungeon, menu/xsd_02). So it is a
  dungeon-only Area and PM10330 (place type 4, locator 0) plays at the dungeon start. m104 is row 8
  (Submissive Tragedy's 1000 Oaks). Next: trace the dungeon room layout builder.

## Story dungeons (2026-10-12, Claude) — built by the DLL's own code
- tools/dllemu.py: unicorn emulator for hackGU_vol1.dll (image at its base, IAT stubs, bump allocator for
  0x18067F1B0, memset/memcpy and libm in Python; any other import stops the run with its name).
- tools/dungeon_layout.py <w1> <w2> <w3> <map ccs> <out>: runs area setup 0x18067C8B0, the dungeon builder
  (ctor 0x1804679F0, 0x18046A510 on the BIN_M1xx blob) and the placement routine 0x1802E6030, and writes per floor every
  cell's piece id, its MDL list (switch in 0x1802DFD30) and the DLL's model matrix (row vectors, cm; some pieces
  mirrored, y/z scale 1.001). assets/dungeons/m103.json = Peaceful Leading Freedom (map id 8 -> ev/map/m103, kit 3 ->
  dungeon/dungeon2 + d02_obj; 2 floors, 90 and 105 cells). dungeon.rs + app.rs build_dungeon draw it with HitMesh
  collision; `--dungeon assets/dungeons/m103.json[:floor]` to test. Verified by screenshots: start hall with
  waterfalls and stairs, tunnel corridors, rooms; walking works.
- Dungeon map per map id (switch 0x180675066): 8 m103, 12 m149, 13 m114, 14 m118, 15 m123, 18 s107, 21-23 s131-133,
  25-28 s171-174; others random (generator 0x180467CA0, not exported yet).
- Blob layout (payload): +4 room map int16[40][40][5], +0x3E84 second grid, +0xBB84 record count, +0xBB88 records of
  0x28 bytes: int16 type, x, y, z (pos = v*400+200 cm for x/y, v*400 for z), +0x14 parameter. Types (jump table at
  0x180675840): 2 -> 0x1802F3D20, 4/23/24/25 -> objects 0x1802F4CE0, 6..14 -> event locator slots 11..19 (area obj
  +(type+5+0x77)*16, read by the frame resolver 0x1806731C0 for locators 11..40), 15..20/26..61 -> dungeon obj
  +0x5176.. slots. m103: type 1 at (418, 510) m is in the start room (likely the start point; reader not traced);
  type 6 -> locator 11 (PM10333) at (346, 514); 7 -> 12 (PM10334) (254, 514); 8 -> 13 (PM10335) (290, 362);
  9 -> 14 (PM10337) (294, 174); 10 -> 15 (PM10340) (374, 266) (all z 0, floor not yet attributed).
- Frame resolver 0x1806731C0 for stage type 4 (dungeon): locator 0 -> dungeon obj +0x40/+0x44 (start), 11..40 ->
  the slots above, 50/51 -> obj +(n-0x32+0xA3)*16.
- Area Words for Peaceful Leading Freedom are given after PM10320 (events.rs GIVEN_WORDS; text 1E0E6590 "Area Word
  given by Silabus and Gaspard"; the granting code is NOT traced).
- Field blob grid scale is still open (see Map blobs above).

## Gemini task 6 / 7 review (2026-10-12, Claude)
- Task 6 (event locations): superseded by tools/event_locations.py (traced by Claude); not merged.
- Task 7 (forum/news), gemini_out7: TRUE: board tables (apkallu @0xAB7990 233 x 0x70, officialforum @0xABDF80 143...),
  record layout (+0 title key ptr, +8 nickname, +0x18 body, +0x38 int16 condition, +0x5C thread root index),
  visibility 0x18024B1A0 (-1 = always, else story flag condition+0x1A). "(1/2)" is part of the nickname string
  ("Mayako (1/2)", 0C1ECB5E). WRONG: "exactly 4 Apkallu thread heads have condition -1 after the reformat": 28 root
  posts have -1 (186 posts in all), so another gate limits the board to the 4 threads seen in f01097-f01198.
  UNPROVEN: reward record at file 0x902F10 (type 3, id 56) — the bytes match, but the "magic" 0x19930522 occurs 1773
  times in the DLL (a generic record stamp) and the link to Maria's post is not shown. news.csv not checked yet.
- Story wiring (2026-10-12): Gate words 11/27/21 (owned after PM10320) -> assets/dungeons match -> arrival event =
  first start-75 event whose location row has those Area Words (events.rs arrival_event: PM10330) at locator 0.
  Verified end to end by a scripted Gate run (gate -> confirm -> PM10330 in the start hall).
- GUESS (not traced): start-80 dungeon events (PM10333, PM10334, PM10335, PM10337, PM10340) play one at a time in
  table order when Haseo enters the 16 m cell of their locator record. Start condition 80 also covers the tutorial's
  altar/fragment events, i.e. "started by game code"; the dungeon code that starts them is not found yet (the event
  name pointer table at RVA 0x7EF870 (478 names) is read by name lookups at 0x1805C6CC1 / 0x1805CB19C).
  PM10345..PM10360 use locator 50 (last room), not filled from the blob records; floor changes (floorup pieces) and
  dungeon enemies are not done, so the chain stops after PM10340.
- Chaos Gate (2026-10-12): now spawned as a prop from Vol.1's town/xcg.ccs and loops ANM_xcg_nut (the rings turn as in
  run1_opening f00000). Its glowing orb is NOT in xcg.ccs, town01*.ccs (EFF_efe00-02 there are warm lamp halos on
  OBJ_obu01..), town/xw.ccs (the town warp-point pad at DMY_warp01..05) or xetim.ccs (Chim Chims). The gate loader
  0x1803728EA loads xetim, field/itembox01, town/xw, town/xcg; the orb is probably a code-built effect. Not done.
- Event clock (2026-10-12): 30 ticks/s (was an unsourced 60). Evidence: PM10100's message durations 60/45/100/90 vs
  JP voice cues 0-3 of 1.45/1.35/2.34/1.68 s (only 30 Hz keeps each line up until its voice ends). All events, waits,
  camera moves and script blends were playing at double speed. MOVE_SPEED_CM_S (260) is still unsourced.
- 2026-10-12 later: field message op 5 read the style column as its duration (field lines flashed by); events at 30
  ticks/s; camera and walking actors evaluated between ticks; "Press <iconTRIANGLE>" in PM10100 waits for Y/E/Esc and
  opens the Main Menu (GUESS: flow after it untraced, PM10100 now ends in town); bridges: decks on cell boundaries +
  ob00 gateway on the grass (FITTED half-cell shift), walk height = highest floor within 0.5 m, sea walls cut under
  bridges; files with header bytes 2..4 = 0xCCCC (town/xcg, town/xw) store V top-down (UV_TOP_DOWN flag).
- Run speed OPEN: player 5.5 m/s is unsourced; the planted foot in ANM_2hsw01run00 slides ~0.16 m per animation frame
  (4.8 m/s at 30 fps, 9.6 at 60). The user reports the original is faster than 5.5. Needs a timed comparison.
- Warp effect TODO (user, from the original): players warping in/out at the gate get a swirl of lights around them
  (seen on others, usually not on yourself). Effect not identified yet.
- Dungeon floors (2026-10-12): the floor-up hall (floorup-b01, ids 94/100) is walkable down to its bottom, where the
  kit's DMY_floorup-b01-plat sits (the DLL registers a platform there, 0x1802F2FD4). Fixes: visible floor meshes
  (TEX_fl*) are walkable too (the hall's HitMesh has floor only at its far end); walls are classified by triangle
  geometry, not the stored normal hints (the ~14 deg ramp was a wall); player step height 0.7 m (FITTED, entrance lip).
  GUESS: standing on the platform (shown with f_common CMP_op00) switches floors, arriving 3 m off the other floor's
  platform; it re-arms after stepping off. The original's platform model/prompt are not known.
- Last room (2026-10-12): locators 50/51 = the kit's DMY_lastroom-ev1/-ev2 placed by the lastroom-a cell (DLL
  0x1802E5BF9/0x1802E5D53 write them to the slots the frame resolver reads); triggers at ev1 (PM10345 plays on entering).
  FITTED: the script frame is the room's centre line (piece x -8 m, y 0) turned with the piece. Beast Statue
  (dungeon/statue.ccs CMP_statue) and offering chest (dungeon/itembox21.ccs, GUESS) at DMY_lastroom-a-statue/-item.
  Not done: PM10350/PM10355/PM10360 (chest opening and what follows), dungeon enemies.
- Last room events: A at the offering chest (GUESS trigger; PM10350 has no script and counts as played) plays PM10355
  (Bordeaux's ambush), which leads into PM10360 (Pi) via the table order (dungeon_follow_up). Pi = 2pai01. After it:
  free movement; PM10365 is the next Area (12/28/22, Submissive Tragedy's 1000 Oaks, field m104), not built.

## Public GitHub, Asset Extractor & Patreon Setup (2026-10-12)

### 1. Clean-Room Repository Hardening
- **Legal Compliance**: Added `LEGAL.md` documenting intellectual property/trademarks (Bandai Namco, CyberConnect2), clean-room development principles, and the Bring-Your-Own-Game (BYOG) policy.
- **License**: Added standard `LICENSE` (MIT License) matching `Cargo.toml`.
- **Git Ignore**: Updated `.gitignore` to strictly exclude all game containers (`*.cpk`, `*.iso`, `*.bin`, `*.ccs`, `*.cvm`, `*.afs`), dumps, crash logs, scratch outputs (`gemini_out*`), and untracked images (`*.png`, `*.bmp`).
- **README Overhaul**: Rewrote `README.md` for public consumption, including architecture tables, quickstart guide, build instructions, BYOG disclaimers, and Patreon sponsorship placeholders.

### 2. Universal Asset Extraction Pipeline (Drag & Drop)
- **Tool**: Added `tools/extract_assets.py` (pure Python 3 stdlib, zero external dependencies):
  - Last Recode (Steam/PC): Automatically finds default Steam installs or accepts user paths/drag-and-dropped CPKs; extracts `hackGU_vol1_a.cpk` (`target/vol1`), `hackGU_cmn_a.cpk` (`target/cmn`), and Vol 2/3 packs via `cargo run --release -p hackgu-cpk -- extract`.
  - PlayStation 2 ISOs: Includes a custom Python ISO 9660 sector reader (`Iso9660Extractor`) that inspects PVD/directories and extracts disc files directly to `target/ps2_extracted/<iso_name>/`.
  - Batch wrapper: `Extract_Game_Assets.bat` supports direct Windows Explorer drag-and-drop of folders, CPK files, or ISO images.
### 3. Patreon Setup Guidelines
- Formatted as a personal tip jar / open-source developer research fund (never sold as a commercial product or paid game access).
- Strict 100% free open-source policy: zero paywalled code, zero early-access binaries.
- Avoid Bandai Namco / CyberConnect2 trademarks and character art in Patreon banner branding.

### 4. Engine Video Recording System & 2K Preview Video
- **Renderer Automation**: Added `--record <dir>:<count>` and `--res <width>x<height>` to `auto.rs` and `app.rs`. Engine now defaults to native **2K (2560x1440)** rendering.
- **Generated 2K Clips** (`clips/`):
  - `clip1_town_2k.mp4`: Haseo running in Mac Anu with third-person follow cam tracking (2560x1440, 13 Mbps).
  - `clip2_chaos_gate_2k.mp4`: Real-time celestial rotation of the Chaos Gate's rings (2560x1440).
  - `clip3_field_2k.mp4`: Holy Ground grass field exploration towards distant ruins (2560x1440, 43 Mbps).
  - `clip4_dungeon_2k.mp4`: Dungeon exploration and platform traversal (2560x1440).
  - `clip5_cutscene_2k.mp4`: Real event cutscene playback with cast models (2560x1440).
  - `preview_hackgu_rs_2K.mp4`: Master 30-second 2560x1440 showcase video combining all 5 clips with authentic game BGM (`target/bgm_3.hca`), visually lossless CRF 17 encoding.




## Combat from the DLL (2026-10-12, Claude) — traced, not yet ported
Recording note first: `--record dir:count` now advances the game 1/30 s per recorded frame (auto.rs `RECORD_FPS`);
Gemini's 2K clips played back sped up because each 2K PNG save took several frames of wall time.

**Enemy table**: 154 (0x9A) entries of 0x98 bytes at 0x180C6F3F0, filled by the C++ static initializer 0x1800B3410
(only entry 0 is in the file). Run it with tools/dllemu.py (`Emu().call(0x1800B3410)`, then read the table).
Entry 0 = Goblin Rookie (en/egg10). Entry layout so far:
- +0x00..0x27 pointers (model/anim name lists, see the enemy init 0x1804B2990), +0x28 byte: variant index into them.
- +0x2E int16 model scale in % (init multiplies it by a float into the object's scale).
- +0x54 status block (the object keeps a pointer to it at obj+0x1C0): i32 level (Goblin Rookie 3), i32 unknown
  (1/40/330 for entries 0/1/2; maybe EXP), i32 HP (102 = the tutorial goblin's HP box), i32 unknown (18/64/65; SP?),
  then int16 x10 at +0x64: 4 stats + 6 elements (Goblin Rookie 2 5 1 3 / 1 1 1 1 1 1). Stat order NOT confirmed
  (the object's derived stats are PA 0x1F8, PD 0x1FA, MA 0x1FC, MD 0x1FE, elements 0x200..0x20A; the copy from the
  status block to those fields is not traced yet).

**Damage**: 0x180490380(attacker, target, attack_type, _, power_percent):
- attack_type 0x16..0x34 = elemental (magic path 0x1804902E0 with the element pair 0x200..0x20A by type range
  0x16-0x1A, 0x1B-0x1F, 0x20-0x24, 0x25-0x29, 0x2A-0x2E, 0x2F-0x33), < 0x37 otherwise physical, else 0.
- physical base = 0x180490170(attLv, att.PA, defLv, def.PD, flag, cap). flag = attacker obj+0x20 == 1 and target
  obj+0x20 == 5; cap = 9 when attacker obj+0x20 == 1 and target obj+0x28 is 1 or 3 (obj+0x28 kind: 3 = ccGobjPc,
  5 = enemy; obj+0x20 is a team value passed by the creator, values not yet mapped).
- final = clamp(base * power_percent / 100 (integer, toward zero), 1, 9999).
0x180490170: d = attLv - defLv (the global [0x180F82CA8]+0x320 is added to the defender's level when flag, else to
the attacker's), d = max(d, -3.5), if cap > 0 then d = min(d, cap);
level = 1 + 0.2 * (0.032 d^3 - 0.08 d^2 + 0.085 d)   (all three coefficients 0.05 when [[0x180F82CA0]+0xC] == 2;
level = 1 when cap < 0);
D = max(0.5, def); base = ((4.5 A - D) / (0.5 D) + c A) * level + 0.5, c = 0.5 when flag else 0.3; truncate, clamp 1..9999.
Floating damage numbers: ccDamageNormal/Crittical/Element (spawn 0x1802354C0, colour per type).

## Gemini task 8 report (enemy table, stat copy function & name mapping)

### 1. Enemy Table Generator (`tools/enemy_table.py`)
- Extracted 154 entries (size `0x98` bytes each) starting at VA `0x180C6F3F0` after running static initializer `0x1800B3410` via `tools/dllemu.py`.
- Model pointer dereferencing follows `0x1804B2990`:
  `p = *(u64*)(entry + 0) + variant * 0x18` (variant = signed byte at `entry + 0x28`), then `p1 = *(u64*)p`, `p2 = *(u64*)p1`, model name string at `p2`.
  Verified: all 154 entries resolve to valid model strings with 0 null pointers.
- Entry 0 verification: `model = "en/egg10"`, `scale_pct = 100%`, `level = 3`, `hp = 102`, `unknown1 = 1`, `unknown2 = 18`, `stats = [2, 5, 1, 3, 1, 1, 1, 1, 1, 1]`.
- Output generated at `assets/battle/enemies.json` (gitignored under `/assets/battle/`).
- Pipeline integrated into `generate_derived_data` in `tools/extract_assets.py`.

### 2. Status Block Copy Function (`0x180488800`) & Stat Order Proof
During enemy initialization at `0x1804B2990`:
- `0x1804B2A01: mov qword ptr [rbp + 0x3f0], rax` (table entry pointer)
- `0x1804B2A0B: add rax, 0x54`
- `0x1804B2A0F: mov qword ptr [rbp + 0x1c0], rax` (status block pointer at `entry + 0x54` stored to `obj + 0x1C0`)
- `0x1804B2A16: call 0x180488800`

Function `0x180488800` copies the status block into the object's derived fields using SSE vector instructions (`movups`):
```x86asm
0x180488800: mov      rax, qword ptr [rcx + 0x1c0]    ; rax = entry + 0x54
0x180488807: movups   xmm0, xmmword ptr [rax]         ; loads 16 bytes: level, unk1, hp, unk2
0x18048880A: movups   xmmword ptr [rcx + 0x1e8], xmm0 ; copies to obj + 0x1E8..0x1F7
0x180488811: movups   xmm1, xmmword ptr [rax + 0x10]  ; loads 16 bytes from entry + 0x64
0x180488815: movups   xmmword ptr [rcx + 0x1f8], xmm1 ; copies to obj + 0x1F8..0x207
0x18048881C: mov      eax, dword ptr [rax + 0x20]     ; loads 4 bytes from entry + 0x74
0x18048881F: mov      dword ptr [rcx + 0x208], eax    ; copies to obj + 0x208..0x20B
```

Stat ordering confirmed by field offset mapping:
- `entry + 0x54` -> `obj + 0x1E8`: Level (i32)
- `entry + 0x58` -> `obj + 0x1EC`: Unknown1 (i32, likely EXP)
- `entry + 0x5C` -> `obj + 0x1F0`: HP (i32)
- `entry + 0x60` -> `obj + 0x1F4`: Unknown2 (i32, likely SP)
- `entry + 0x64` -> `obj + 0x1F8`: PA (Physical Attack, int16)
- `entry + 0x66` -> `obj + 0x1FA`: PD (Physical Defense, int16)
- `entry + 0x68` -> `obj + 0x1FC`: MA (Magic Attack, int16)
- `entry + 0x6A` -> `obj + 0x1FE`: MD (Magic Defense, int16)
- `entry + 0x6C` -> `obj + 0x200`: element 0 (int16)
- `entry + 0x6E` -> `obj + 0x202`: element 1 (int16)
- `entry + 0x70` -> `obj + 0x204`: element 2 (int16)
- `entry + 0x72` -> `obj + 0x206`: element 3 (int16)
- `entry + 0x74` -> `obj + 0x208`: element 4 (int16)
- `entry + 0x76` -> `obj + 0x20A`: element 5 (int16)

Matches the elemental ranges used in magic damage path `0x1804902E0`. Claude's review: the copy at 0x180488800 is verified; the element NAMES are not traced (Gemini's Fire/Water/Wind/Earth/Light/Dark labels were removed), so the tool writes element0..element5.

### 3. Enemy Name Mapping (SOLVED)
- Enemy name resolution function: `0x180593CD0(enemy_index)` (verified by Claude; index 0x71 takes a separate path that reads a pointer at +0x2FC0 of a global object, not traced).
  Called directly from enemy init `0x1804B2990`:
  ```x86asm
  0x1804B29B0: mov      ecx, edi        ; edi = enemy table index (0..153)
  0x1804B29B5: call     0x180593cd0     ; returns localized enemy name string pointer in rax
  0x1804B29BD: mov      qword ptr [rsp + 0x20], rax
  ```
- Implementation inside `0x180593CD0`:
  - Reads pointer at `0x1807B6898`, which points to array `0x180B12120` (file offset `0xB10F20`).
  - `0x180B12120` is an array of 64-bit pointers to `enemyName_NNN` strings (for indices 0..153) and `spcName_*_arena` strings (for indices 154+).
  - Retrieves key `const char* key = ((const char**)0x180B12120)[enemy_index]`.
  - Calls `MessageManager::lookup` (`0x18012dac0`) with `key` to look up the localized name from `messageInfo.bin` (using CRC-32 / MPEG-2 of the key string).
  - Several enemies share name keys (e.g. indices 3..6 share `enemyName_003` = "Goblin Mage", indices 7..10 share `enemyName_007` = "Goblin Sage", unassigned/dummy slots map to `enemyName_129`).

