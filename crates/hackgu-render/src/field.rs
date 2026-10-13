//! Fields assembled from tile kits, described by area definitions in `assets/areas/*.json` (`AreaDef` below).
//! The first one is Δ Courageous Engaging Daydream, the Vol.1 tutorial field.
//!
//! G.U. builds fields in game code from tile kits; no layout file was found. A definition's grid is read from the
//! original game's Area Map: island outlines snap to a lattice of one 16 m tile per step, the size of every
//! land/sea piece in the kit (HitMesh bounds ±8 m). Adding a field = adding a JSON file (grid, kit files, set-piece
//! positions, enemy/party setup, event ids and frames); the Chaos Gate picks it by server + word indices.
//!
//! Tiles (kit.tiles, e.g. field/sky_00.ccs) form a marching-squares set placed on the lattice corners, chosen from
//! the four cells around each corner (collision heights show land at +1 m, sea floor at -1 m):
//!   mf00 all land; mt00 land on -y (shore facing +y); ml00 land only in the +x/-y quarter;
//!   mv00 sea only in the +x/-y quarter. Other orientations are 90-degree rotations.
//! Set pieces come from kit.set (field/sf3_00.ccs): mx00 grass interior, ob01 bridge deck (7.2 x 16 m), oc00 altar
//! plinth, oi00 Beast Temple. Platforms are kit.common (field/f_common.ccs) op00.
//!
//! Not from data (marked where used): which decorated variant (mt01/ml01/mv01) a tile uses, the temple's facing,
//! and that map north is +y.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use glam::Vec3;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Kit {
    /// Ground tiles (marching-squares shore set + sea): mt00/ml00/mv00, sb01/sw01.
    pub tiles: String,
    /// Field set pieces: mx00 interior, ob01 bridge, oc00 altar, oi00 temple.
    pub set: String,
    /// Common field objects: op00 Platform.
    pub common: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Placed {
    #[serde(default)]
    pub grid: Option<[f32; 2]>,
    #[serde(default)]
    pub world: Option<[f32; 3]>,
    #[serde(default)]
    pub yaw_deg: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Temple {
    pub grid: [f32; 2],
    pub yaw_deg: f32,
    pub statue_room: String,
    pub statue: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Enemy {
    pub model: String,
    #[serde(default)]
    pub extra_anims: Option<String>,
    #[serde(default)]
    pub weapon: Option<String>,
    #[serde(default)]
    pub weapon_clump: Option<String>,
    pub name: String,
    pub hp: i32,
    pub per_altar: usize,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PartyMember {
    pub model: String,
    pub hp: i32,
    pub sp: i32,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Events {
    #[serde(default)]
    pub arrival: Option<String>,
    #[serde(default)]
    pub after_arrival: Option<String>,
    /// One per altar (None = straight to battle).
    #[serde(default)]
    pub altars: Vec<Option<String>>,
    #[serde(default)]
    pub fragments_joined: Option<String>,
    #[serde(default)]
    pub statue: Option<String>,
    #[serde(default)]
    pub statue_movie: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct PostFx {
    #[serde(default)]
    pub field: Option<(String, usize)>,
    #[serde(default)]
    pub statue_room: Option<(String, usize)>,
}

/// One area, loaded from `assets/areas/<file>.json`.
#[derive(Deserialize, Clone, Debug)]
pub struct AreaDef {
    pub id: String,
    pub name: String,
    pub server: u32,
    /// Word indices (1st, 2nd, 3rd) into the Area Word tables (crates/hackgu-game/src/areaword.rs).
    pub words: [usize; 3],
    pub kit: Kit,
    pub cell: f32,
    /// Kit tiles have their walking surface at +1 m; event scripts stand the cast at ~0 m, so tiles are lowered.
    pub ground_drop: f32,
    /// One character per cell, north at the top: '#' land, '=' bridge, '.' sea.
    pub grid: Vec<String>,
    /// Grid point that maps to world (0, 0).
    pub origin: [f32; 2],
    pub arrival_platform: Placed,
    pub exit_platform: Placed,
    pub spawn: [f32; 3],
    pub altars: Vec<[f32; 2]>,
    #[serde(default)]
    pub temple: Option<Temple>,
    pub enemy: Enemy,
    pub party: Vec<PartyMember>,
    #[serde(default)]
    pub events: Events,
    /// Event id -> [origin x, y, z (m), yaw (deg)] in this area's world frame.
    #[serde(default)]
    pub event_frames: HashMap<String, [f32; 4]>,
    #[serde(default)]
    pub postfx: PostFx,
    /// Explicitly placed pieces (any kit file + clump), e.g. dungeon rooms or props: placed as-is (no ground drop),
    /// with collision from the piece's own HitMeshes (HIT_<clump name>*).
    #[serde(default)]
    pub pieces: Vec<PieceDef>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PieceDef {
    pub file: String,
    pub clump: String,
    /// World metres.
    pub pos: [f32; 3],
    #[serde(default)]
    pub yaw_deg: f32,
}

impl AreaDef {
    /// Grid point (columns, rows) to world metres (x east, y north).
    pub fn world(&self, i: f32, j: f32) -> Vec3 {
        Vec3::new((i - self.origin[0]) * self.cell, -(j - self.origin[1]) * self.cell, 0.0)
    }
    pub fn place(&self, p: &Placed) -> Vec3 {
        match (p.world, p.grid) {
            (Some(w), _) => Vec3::from(w),
            (None, Some(g)) => self.world(g[0], g[1]),
            _ => Vec3::ZERO,
        }
    }
    pub fn altar_world(&self, k: usize) -> Vec3 {
        let a = self.altars[k];
        self.world(a[0], a[1])
    }
    pub fn cell(&self, i: i32, j: i32) -> u8 {
        if i < 0 || j < 0 {
            return b'.';
        }
        self.grid.get(j as usize).and_then(|r| r.as_bytes().get(i as usize)).copied().unwrap_or(b'.')
    }
    fn land(&self, i: i32, j: i32) -> bool {
        self.cell(i, j) == b'#'
    }
    /// All event ids that play in this area.
    pub fn event_ids(&self) -> Vec<String> {
        let e = &self.events;
        [&e.arrival, &e.after_arrival, &e.fragments_joined, &e.statue].into_iter().flatten().cloned().chain(e.altars.iter().flatten().cloned()).collect()
    }
}

static AREA: RwLock<Option<Arc<AreaDef>>> = RwLock::new(None);
pub const AREA_DIR: &str = "assets/areas";
pub const DEFAULT_AREA: &str = "vol1_tutorial_field.json";

pub fn load_file(name: &str) -> Option<AreaDef> {
    let p = std::path::Path::new(name);
    let path = if p.is_absolute() || p.exists() { p.to_path_buf() } else { std::path::Path::new(AREA_DIR).join(name) };
    let text = std::fs::read_to_string(&path).map_err(|e| eprintln!("area: {}: {}", path.display(), e)).ok()?;
    serde_json::from_str(&text).map_err(|e| eprintln!("area: {}: {}", path.display(), e)).ok()
}

/// Every area definition in assets/areas.
pub fn load_all() -> Vec<AreaDef> {
    let mut v = Vec::new();
    if let Ok(rd) = std::fs::read_dir(AREA_DIR) {
        for e in rd.flatten() {
            let n = e.file_name().to_string_lossy().to_string();
            if n.ends_with(".json") {
                if let Some(a) = load_file(&n) {
                    v.push(a);
                }
            }
        }
    }
    v
}

/// The area for a server + word combination, if one is defined.
pub fn find(server: u32, words: [usize; 3]) -> Option<AreaDef> {
    load_all().into_iter().find(|a| a.server == server && a.words == words)
}

/// The current area (the default tutorial field until another is selected).
pub fn area() -> Arc<AreaDef> {
    if let Some(a) = AREA.read().unwrap().as_ref() {
        return a.clone();
    }
    let a = Arc::new(load_file(DEFAULT_AREA).expect("assets/areas/vol1_tutorial_field.json missing or invalid"));
    *AREA.write().unwrap() = Some(a.clone());
    a
}

pub fn set_area(a: AreaDef) {
    *AREA.write().unwrap() = Some(Arc::new(a));
}

/// Grid point to world metres in the current area.
pub fn world(i: f32, j: f32) -> Vec3 {
    area().world(i, j)
}

/// A model to place: library file, clump name, world position (metres) and yaw (radians, CCW about +Z).
#[derive(Clone, Debug)]
pub struct Piece {
    pub file: String,
    pub clump: String,
    pub pos: Vec3,
    pub yaw: f32,
}

/// Rotation (in quarter turns) that brings the quadrant `q` (0 = SE, 1 = NE, 2 = NW, 3 = SW, CCW order) to SE.
fn quarter_turns_from_se(q: usize) -> f32 {
    q as f32 * std::f32::consts::FRAC_PI_2
}

/// All pieces for the current area.
pub fn layout() -> Vec<Piece> {
    let a = area();
    let mut v = layout_raw(&a);
    for p in &mut v {
        // Platforms and altars are props standing on the ground (base at 0), not ground tiles.
        if p.clump != "CMP_op00" && p.clump != "CMP_oc00" {
            p.pos.z -= a.ground_drop;
        }
    }
    for d in &a.pieces {
        v.push(Piece { file: d.file.clone(), clump: d.clump.clone(), pos: Vec3::from(d.pos), yaw: d.yaw_deg.to_radians() });
    }
    v
}

fn layout_raw(a: &AreaDef) -> Vec<Piece> {
    let mut out = Vec::new();
    let rows = a.grid.len() as i32;
    let cols = a.grid.iter().map(|r| r.len()).max().unwrap_or(0) as i32;
    let tiles = a.kit.tiles.clone();
    let set = a.kit.set.clone();
    // Marching squares on lattice corners (i, j) = top-left corner of cell (i, j).
    for j in 0..=rows {
        for i in 0..=cols {
            // Quadrants around the corner, CCW from SE: SE = cell(i, j), NE = cell(i, j-1), NW = cell(i-1, j-1), SW = cell(i-1, j).
            let q = [a.land(i, j), a.land(i, j - 1), a.land(i - 1, j - 1), a.land(i - 1, j)];
            let n = q.iter().filter(|&&b| b).count();
            let pos = a.world(i as f32, j as f32);
            let mut put = |clump: &str, yaw: f32| out.push(Piece { file: tiles.clone(), clump: clump.into(), pos, yaw });
            match n {
                0 => {}
                4 => {
                    // Plain grass interior: the set's mx00 (the tile kit's mf00 is a sandy hollow).
                    out.push(Piece { file: set.clone(), clump: "CMP_mx00".into(), pos, yaw: 0.0 });
                }
                1 => {
                    let k = q.iter().position(|&b| b).unwrap();
                    put("CMP_ml00", quarter_turns_from_se(k));
                }
                3 => {
                    let k = q.iter().position(|&b| !b).unwrap();
                    put("CMP_mv00", quarter_turns_from_se(k));
                }
                _ => {
                    // Two land quarters. Adjacent pair -> straight shore: mt00 has land on SE+SW (south).
                    let pairs = [(0, 3), (0, 1), (1, 2), (2, 3)]; // south, east, north, west
                    if let Some(p) = pairs.iter().position(|&(x, y)| q[x] && q[y]) {
                        put("CMP_mt00", p as f32 * std::f32::consts::FRAC_PI_2);
                    } else {
                        // Diagonal pair: no dedicated piece found; use two corner pieces.
                        for k in 0..4 {
                            if q[k] {
                                put("CMP_ml00", quarter_turns_from_se(k));
                            }
                        }
                    }
                }
            }
        }
    }
    // Sea surface: 32 m water tiles over the whole grid plus a 5-tile margin.
    let (w, h) = (cols as f32 * a.cell, rows as f32 * a.cell);
    let mut y = -5.0 * 2.0 * a.cell;
    while y < h + 5.0 * 2.0 * a.cell {
        let mut x = -5.0 * 2.0 * a.cell;
        while x < w + 5.0 * 2.0 * a.cell {
            let p = a.world(x / a.cell + 1.0, y / a.cell + 1.0);
            // Sea bed colour (sb01, -1 m) under the additive caustics layer (sw01, -0.5 m).
            out.push(Piece { file: tiles.clone(), clump: "CMP_sb01".into(), pos: p, yaw: 0.0 });
            out.push(Piece { file: tiles.clone(), clump: "CMP_sw01".into(), pos: p, yaw: 0.0 });
            x += 2.0 * a.cell;
        }
        y += 2.0 * a.cell;
    }
    // Bridges: one deck segment per bridge cell, oriented along the run.
    for j in 0..rows {
        for i in 0..cols {
            if a.cell(i, j) != b'=' {
                continue;
            }
            let vertical = a.cell(i, j - 1) == b'=' || a.cell(i, j + 1) == b'=' || a.land(i, j - 1) || a.land(i, j + 1);
            let pos = a.world(i as f32 + 0.5, j as f32 + 0.5);
            // Bridges sit half a cell back toward the land at each end: our hand-drawn grid's shore slopes to the
            // beach inside the land cell, while in the DLL's map the land reaches the bridge's end cell (tiles
            // 0x30..0x33, Map blobs in HANDOFF). Each deck (16 m) is centred on a cell boundary, and the end piece
            // CMP_ob00 stands on the flat grass with its gateway (local y 4..6) and deck meeting the first deck and
            // its ramp running back into the land (local -y, below the grass). FITTED: the half-cell shift.
            // Local +y of ob00 maps to world (-sin yaw, cos yaw); grid rows run south (world -y).
            use std::f32::consts::{FRAC_PI_2, PI};
            let dirs: [(i32, i32); 2] = if vertical { [(0, -1), (0, 1)] } else { [(-1, 0), (1, 0)] };
            let deck_yaw = if vertical { 0.0 } else { FRAC_PI_2 };
            let boundary = |(di, dj): (i32, i32), k: f32| a.world(i as f32 + 0.5 + di as f32 * k, j as f32 + 0.5 + dj as f32 * k);
            out.push(Piece { file: set.clone(), clump: "CMP_ob01".into(), pos: boundary(dirs[0], 0.5), yaw: deck_yaw });
            if a.land(i + dirs[1].0, j + dirs[1].1) {
                out.push(Piece { file: set.clone(), clump: "CMP_ob01".into(), pos: boundary(dirs[1], 0.5), yaw: deck_yaw });
            }
            let _ = pos;
            for (di, dj, yaw) in [(0, -1, PI), (0, 1, 0.0), (-1, 0, -FRAC_PI_2), (1, 0, FRAC_PI_2)] {
                if a.land(i + di, j + dj) {
                    out.push(Piece { file: set.clone(), clump: "CMP_ob00".into(), pos: boundary((di, dj), 1.5), yaw });
                }
            }
        }
    }
    for k in 0..a.altars.len() {
        out.push(Piece { file: set.clone(), clump: "CMP_oc00".into(), pos: a.altar_world(k), yaw: 0.0 });
    }
    if let Some(t) = &a.temple {
        let tc = a.world(t.grid[0], t.grid[1]);
        let yaw = t.yaw_deg.to_radians();
        out.push(Piece { file: set.clone(), clump: "CMP_oi00".into(), pos: tc, yaw });
        // Inner double door (CMP_oi00door01/02, leaves at x -+2 m) at DMY_door01.
        let d = temple_local_to_world(a, TEMPLE_INNER_DOOR_LOCAL);
        for c in ["CMP_oi00door01", "CMP_oi00door02"] {
            out.push(Piece { file: set.clone(), clump: c.into(), pos: Vec3::new(d.x, d.y, TEMPLE_INNER_DOOR_LOCAL.z), yaw });
        }
    }
    out.push(Piece { file: a.kit.common.clone(), clump: "CMP_op00".into(), pos: a.place(&a.arrival_platform), yaw: a.arrival_platform.yaw_deg.to_radians() });
    out.push(Piece { file: a.kit.common.clone(), clump: "CMP_op00".into(), pos: a.place(&a.exit_platform), yaw: a.exit_platform.yaw_deg.to_radians() });
    out
}

/// Arrival Platform world position of the current area.
pub fn arrival_platform() -> Vec3 {
    let a = area();
    a.place(&a.arrival_platform)
}
pub fn exit_platform() -> Vec3 {
    let a = area();
    a.place(&a.exit_platform)
}

/// Is this event one that plays in the current area?
pub fn is_area_event(id: &str) -> bool {
    area().event_ids().iter().any(|e| e == id)
}

/// Where an event's script coordinates go in the area (from the definition's event_frames).
pub fn event_frame(id: &str, _player: Vec3) -> Option<crate::cutscene::EvTransform> {
    area().event_frames.get(id).map(|f| crate::cutscene::EvTransform { origin: Vec3::new(f[0], f[1], f[2]), yaw: f[3].to_radians() })
}

/// Beast Temple statue room (field/sf3_item.ccs): DMY_start (0,-18,0), DMY_door00 (0,-24,0), DMY_item (0,32,1.43),
/// DMY_statue (0,36,3.74), DMY_ev1 (0,-18,0), DMY_ev2 (0,24,1.2).
pub const TEMPLE_INTERIOR_SPAWN: Vec3 = Vec3::new(0.0, -18.0, 0.0);
/// The temple's Inner Door: sf3_00 DMY_door01 (0, 41.1, -9.45) m at the foot of the interior stairs (MDL_oi00_i).
pub const TEMPLE_INNER_DOOR_LOCAL: Vec3 = Vec3::new(0.0, 41.1, -9.45);

fn temple_local_to_world(a: &AreaDef, l: Vec3) -> Vec3 {
    let Some(t) = &a.temple else { return Vec3::ZERO };
    let c = a.world(t.grid[0], t.grid[1]);
    let (s, co) = t.yaw_deg.to_radians().sin_cos();
    Vec3::new(c.x + co * l.x - s * l.y, c.y + s * l.x + co * l.y, l.z)
}
fn world_to_temple_local(a: &AreaDef, p: Vec3) -> Option<Vec3> {
    let t = a.temple.as_ref()?;
    let c = a.world(t.grid[0], t.grid[1]);
    let (s, co) = t.yaw_deg.to_radians().sin_cos();
    let (dx, dy) = (p.x - c.x, p.y - c.y);
    Some(Vec3::new(co * dx + s * dy, -s * dx + co * dy, p.z))
}

/// Temple interior footprint (MDL_oi00_i: local x +-11, y -11..45 m); tiles over it are hidden while the camera is
/// inside (the corridor runs below sea level and its back part through the ground).
pub fn inside_temple(p: Vec3) -> bool {
    match world_to_temple_local(&area(), p) {
        Some(l) => l.x.abs() < 11.0 && l.y > -11.5 && l.y < 45.5 && p.z < 1.0,
        None => false,
    }
}

/// Field pieces whose 16/32 m square overlaps the temple interior footprint.
pub fn over_temple_interior(p: &Piece) -> bool {
    let a = area();
    if a.temple.is_none() {
        return false;
    }
    let tile = p.clump.starts_with("CMP_m") || p.clump == "CMP_sb01" || p.clump == "CMP_sw01";
    if !tile {
        return false;
    }
    let half = if p.clump == "CMP_sb01" || p.clump == "CMP_sw01" { 16.0 } else { 8.0 };
    // Footprint corners in world space -> axis-aligned bounds (temples are placed at multiples of 90 degrees).
    let corners = [Vec3::new(-11.0, -11.5, 0.0), Vec3::new(11.0, -11.5, 0.0), Vec3::new(-11.0, 45.5, 0.0), Vec3::new(11.0, 45.5, 0.0)].map(|c| temple_local_to_world(&a, c));
    let (minx, maxx) = corners.iter().fold((f32::MAX, f32::MIN), |m, c| (m.0.min(c.x), m.1.max(c.x)));
    let (miny, maxy) = corners.iter().fold((f32::MAX, f32::MIN), |m, c| (m.0.min(c.y), m.1.max(c.y)));
    p.pos.x + half > minx && p.pos.x - half < maxx && p.pos.y + half > miny && p.pos.y - half < maxy
}

/// Passing the temple's Inner Door loads the statue room.
pub fn temple_door_world() -> Vec3 {
    let a = area();
    let d = temple_local_to_world(&a, TEMPLE_INNER_DOOR_LOCAL);
    Vec3::new(d.x, d.y, TEMPLE_INNER_DOOR_LOCAL.z - a.ground_drop)
}
