//! Story dungeons, laid out exactly as hackGU_vol1.dll builds them.
//!
//! `assets/dungeons/*.json` are written by tools/dungeon_layout.py, which runs the DLL's own dungeon builder in an
//! emulator on the Area's map blob (Peaceful Leading Freedom: ev/map/m103.ccs) and records, per floor, every cell's
//! piece id, the MDL models that id loads and the model matrix the DLL builds for the cell (0x1802E6030: rotation by
//! piece id, some pieces mirrored, y/z scale 1.001, translation (x*1600+800, y*1600+800, 0) cm; row-vector layout).
//! Each MDL_<name> is drawn through its kit clump CMP_<name>; collision from its HIT_<name>-hit* HitMeshes.

use std::sync::{Arc, RwLock};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Cell {
    pub cell: [i32; 2],
    pub id: i32,
    pub models: Vec<String>,
    /// The DLL's model matrix: row vectors, translation in elements 12..14, centimetres.
    pub matrix: [f32; 16],
}

impl Cell {
    /// Model matrix in our convention (column vectors, metres).
    pub fn mat(&self) -> hackgu_ccs::Mat4 {
        let m = &self.matrix;
        let mut o = [0.0f32; 16];
        for r in 0..4 {
            for c in 0..4 {
                o[r * 4 + c] = m[c * 4 + r];
            }
        }
        o[3] *= 0.01;
        o[7] *= 0.01;
        o[11] *= 0.01;
        hackgu_ccs::Mat4(o)
    }

    pub fn pos(&self) -> glam::Vec3 {
        glam::Vec3::new(self.matrix[12], self.matrix[13], self.matrix[14]) * 0.01
    }
}

/// A record of the map blob (loader loop 0x180675490): type, position in metres.
#[derive(Clone, Debug, Deserialize)]
pub struct Record {
    #[serde(rename = "type")]
    pub kind: i32,
    pub pos: [f32; 3],
}

#[derive(Clone, Debug, Deserialize)]
pub struct DungeonDef {
    pub words: [usize; 3],
    pub kit: String,
    pub objects: String,
    #[serde(default)]
    pub records: Vec<Record>,
    pub floors: Vec<Vec<Cell>>,
}

/// Piece ids of the floor-change halls (MDL_floorup-b01 on the lower floor, id 94; on the upper floor id 100, which
/// loads floorup-b01 and floorup-b01-2): the DLL registers a platform at their DMY_floorup-b01-plat locator
/// (post-build pass 0x1802F2FD4 for id 100).
pub const FLOOR_UP_IDS: [i32; 2] = [94, 100];

/// Piece ids of the start room (MDL_start-a..d, ids 66..69 in the builder's switch).
pub const START_IDS: [i32; 4] = [66, 67, 68, 69];

/// The dungeon being played and its floor.
pub struct Current {
    pub def: Arc<DungeonDef>,
    pub floor: usize,
}

static CURRENT: RwLock<Option<(Arc<DungeonDef>, usize)>> = RwLock::new(None);

pub fn load_file(path: &str) -> Option<DungeonDef> {
    let s = std::fs::read_to_string(path).ok()?;
    match serde_json::from_str(&s) {
        Ok(d) => Some(d),
        Err(e) => {
            eprintln!("dungeon {}: {}", path, e);
            None
        }
    }
}

/// Switch the current dungeon to another floor (the renderer reloads the stage).
pub fn set_floor(floor: usize) {
    if let Some((_, f)) = CURRENT.write().unwrap().as_mut() {
        *f = floor;
    }
}

pub fn set_current(def: DungeonDef, floor: usize) {
    *CURRENT.write().unwrap() = Some((Arc::new(def), floor));
}

pub fn clear() {
    *CURRENT.write().unwrap() = None;
}

pub fn current() -> Option<Current> {
    CURRENT.read().unwrap().as_ref().map(|(d, f)| Current { def: d.clone(), floor: *f })
}

/// The dungeon defined for these Area Words (assets/dungeons/*.json).
pub fn find(words: [usize; 3]) -> Option<DungeonDef> {
    let dir = std::fs::read_dir("assets/dungeons").ok()?;
    dir.filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|x| x == "json"))
        .filter_map(|e| load_file(&e.path().to_string_lossy()))
        .find(|d| d.words == words)
}

impl DungeonDef {
    /// Where the player starts: the blob's type-1 record (it lies in the start room; GUESS that it is the start the
    /// DLL uses for locator 0, the reader is not traced), else the start room's centre.
    pub fn start(&self) -> Option<glam::Vec3> {
        self.records.iter().find(|r| r.kind == 1).map(|r| glam::Vec3::from(r.pos)).or_else(|| self.start_centre())
    }

    /// Event locator n (frame resolver 0x1806731C0, stage type 4): 0 = start, 11..19 = the type n-5 records.
    pub fn locator(&self, n: i32) -> Option<glam::Vec3> {
        if n == 0 {
            return self.start();
        }
        self.records.iter().find(|r| r.kind == n - 5 && (6..=14).contains(&r.kind)).map(|r| glam::Vec3::from(r.pos))
    }

    /// (clump, model matrix) for every model of a floor.
    pub fn pieces(&self, floor: usize) -> Vec<(String, hackgu_ccs::Mat4)> {
        let mut v = Vec::new();
        for c in self.floors.get(floor).map(|f| f.as_slice()).unwrap_or(&[]) {
            for m in &c.models {
                v.push((format!("CMP_{}", m.trim_start_matches("MDL_")), c.mat()));
            }
        }
        v
    }

    /// The floor-change platform of a floor: the kit's DMY_floorup-b01-plat locator (`plat_cm`, centimetres in the
    /// piece's space) placed by the floor-up piece's matrix. Metres.
    pub fn floor_platform(&self, floor: usize, plat_cm: [f32; 3]) -> Option<glam::Vec3> {
        let c = self.floors.get(floor)?.iter().find(|c| FLOOR_UP_IDS.contains(&c.id))?;
        let m = &c.matrix;
        let (x, y, z) = (plat_cm[0], plat_cm[1], plat_cm[2]);
        Some(glam::Vec3::new(x * m[0] + y * m[4] + z * m[8] + m[12], x * m[1] + y * m[5] + z * m[9] + m[13], x * m[2] + y * m[6] + z * m[10] + m[14]) * 0.01)
    }

    /// Centre of the start room (the four start pieces' cells), on floor 0.
    pub fn start_centre(&self) -> Option<glam::Vec3> {
        let cells: Vec<&Cell> = self.floors.first()?.iter().filter(|c| START_IDS.contains(&c.id)).collect();
        if cells.is_empty() {
            return None;
        }
        let sum = cells.iter().fold(glam::Vec3::ZERO, |s, c| s + c.pos());
        Some(sum / cells.len() as f32)
    }
}
