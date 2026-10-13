//! hittris file.ccs HIT_name : print a HitMesh's triangles (metres) with their normal hint.
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    for h in c.hit_meshes() {
        if c.object_name(h.object_id) != Some(a[2].as_str()) { continue; }
        for r in &h.records {
            for (t, n) in r.positions.chunks_exact(3).zip(r.normals.chunks_exact(3)) {
                let m = |p: [f32; 3]| [p[0] * 0.01, p[1] * 0.01, p[2] * 0.01];
                println!("{:5.1?} {:5.1?} {:5.1?}  n {:4.1?}", m(t[0]), m(t[1]), m(t[2]), n[0]);
            }
        }
    }
}
