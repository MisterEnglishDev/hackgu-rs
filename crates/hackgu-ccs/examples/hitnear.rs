//! hitnear file.ccs x y r : HitMesh vertex heights (metres) within r metres (XY) of (x, y) metres, per record flags
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    let (x, y, r): (f32, f32, f32) = (a[2].parse().unwrap(), a[3].parse().unwrap(), a[4].parse().unwrap());
    for h in c.hit_meshes() {
        for rec in &h.records {
            let mut zs: Vec<f32> = rec.positions.iter().filter(|v| ((v[0] * 0.01 - x).powi(2) + (v[1] * 0.01 - y).powi(2)).sqrt() < r).map(|v| v[2] * 0.01).collect();
            if zs.is_empty() { continue; }
            zs.sort_by(|a, b| a.partial_cmp(b).unwrap()); zs.dedup_by(|a, b| (*a - *b).abs() < 0.05);
            println!("{} flags {:#010x} z: {:.2?}", c.object_name(h.object_id).unwrap_or("?"), rec.flags, &zs[..zs.len().min(12)]);
        }
    }
}
