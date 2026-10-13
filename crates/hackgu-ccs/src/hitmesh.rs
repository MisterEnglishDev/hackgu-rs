//! HitMesh (0x0B00) collision data.
//!
//! This is the game's own collision geometry (invisible walls, floors, slopes), authored separately from the
//! visible models. Layout (all little-endian), verified against town01_01h/02h/05h where the record sizes
//! add up to the section size with zero bytes left over:
//!
//! ```text
//! u32 unknown            (varies: 6, 8, 71 ...)
//! u32 record_count
//! u32 total_vertex_count (sum of every record's N)
//! record_count x {
//!     u32 N              vertex count, always a multiple of 3 (triangle list)
//!     u32 flags          surface/type bits (e.g. 0x00e100ef walls, 0x00606067 floor)
//!     N x [f32;3]        positions, centimetres, Z up
//!     N x [f32;3]        per-vertex normals
//! }
//! ```

#[derive(Clone, Debug)]
pub struct HitRecord {
    pub flags: u32,
    /// Triangle list, positions in centimetres.
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
}

#[derive(Clone, Debug)]
pub struct HitMesh {
    pub object_id: u32,
    pub header_unknown: u32,
    pub records: Vec<HitRecord>,
}

impl HitMesh {
    pub fn from_payload(object_id: u32, payload: &[u8]) -> Option<Self> {
        if payload.len() < 12 {
            return None;
        }
        let rd = |o: usize| u32::from_le_bytes(payload[o..o + 4].try_into().unwrap());
        let header_unknown = rd(0);
        let record_count = rd(4) as usize;
        let total = rd(8) as usize;
        let mut off = 12usize;
        let mut records = Vec::with_capacity(record_count);
        let mut seen = 0usize;
        for _ in 0..record_count {
            if off + 8 > payload.len() {
                return None;
            }
            let n = rd(off) as usize;
            let flags = rd(off + 4);
            let need = 8 + n * 24;
            if n % 3 != 0 || off + need > payload.len() {
                return None;
            }
            let f = |o: usize| f32::from_le_bytes(payload[o..o + 4].try_into().unwrap());
            let mut positions = Vec::with_capacity(n);
            let mut normals = Vec::with_capacity(n);
            let pb = off + 8;
            let nb = pb + n * 12;
            for i in 0..n {
                positions.push([f(pb + i * 12), f(pb + i * 12 + 4), f(pb + i * 12 + 8)]);
                normals.push([f(nb + i * 12), f(nb + i * 12 + 4), f(nb + i * 12 + 8)]);
            }
            seen += n;
            off += need;
            records.push(HitRecord { flags, positions, normals });
        }
        if seen != total || off != payload.len() {
            return None; // layout did not match exactly: refuse rather than guess
        }
        Some(Self { object_id, header_unknown, records })
    }

    pub fn triangle_count(&self) -> usize {
        self.records.iter().map(|r| r.positions.len() / 3).sum()
    }
}
