# Area definitions

Each `*.json` file here describes one field area. The Chaos Gate warps to the file whose `server` and `words` match
the Word Plate (word indices are positions in the Area Word tables, `crates/hackgu-game/src/areaword.rs`; server
0 = Δ). Loader and assembly code: `crates/hackgu-render/src/field.rs` (`AreaDef`).

| Field | Meaning |
|---|---|
| `kit.tiles` | Ground tile kit (`field/sky_0n.ccs`): marching-squares shore set mt00/ml00/mv00, sea sb01/sw01, sky dome |
| `kit.set` | Set-piece kit (`field/sf3_0n.ccs`): mx00 grass, ob01 bridge, oc00 altar, oi00 temple. Also the stage name the field loads under |
| `kit.common` | `field/f_common.ccs` (op00 Platform) |
| `cell`, `ground_drop` | Lattice step (16 m, the kit tile size) and how far tiles are lowered (kit surface is at +1 m) |
| `grid` | One row per string, north at the top: `#` land, `=` bridge, `.` sea |
| `origin` | Grid point (column, row) that becomes world (0, 0) |
| `arrival_platform`, `exit_platform` | `grid` [col,row] or `world` [x,y,z] metres, plus `yaw_deg` |
| `spawn` | Where Haseo appears when the field loads without an arrival event |
| `altars` | Grid points; each gets a plinth, a chest and `enemy.per_altar` enemies |
| `temple` | Grid point + facing, statue room stage (`sf3_item.ccs`) and statue model |
| `enemy`, `party` | Models, HP/SP (party HP/SP read from the original HUD for the tutorial) |
| `events` | Event ids: `arrival` (plays on warp), `after_arrival`, one per altar (`null` = straight to battle), `fragments_joined`, `statue`, `statue_movie` |
| `event_frames` | Event id -> [x, y, z, yaw_deg]: where that script's coordinates sit in this area |
| `postfx` | Post-filter records from `cmn/post_filter.bin` as [kind, index] (`null` = none) |

To add an area: copy `vol1_tutorial_field.json`, set the words, draw the grid from the original's Area Map (one
character per 16 m), place the set pieces, list the events. Values that are not from game data should be noted in
`notes`.

`pieces` (optional): explicitly placed pieces from any kit, e.g. dungeon rooms
(`{"file": "dungeon1.ccs", "clump": "CMP_rouka-a1", "pos": [x, y, z], "yaw_deg": 90}`), placed as-is with collision
from their own `HIT_<clump>*` HitMeshes. Dungeon layouts can be written this way once their room arrangement is known
(Gemini task 4, part C).

Testing a definition: `hackgu-render --field --area path/to/file.json` loads it directly (any path).
