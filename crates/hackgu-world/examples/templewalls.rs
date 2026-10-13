//! templewalls: wall segments of sf3_00's temple HitMesh near the entrance (local y -14..-8, |x| < 4).
use hackgu_ccs::CcsFile;
use hackgu_world::CollisionMesh;
fn main() {
    let c = CcsFile::parse(&std::fs::read("target/vol1/data/data/field/sf3_00.ccs").unwrap()).unwrap();
    for name in ["HIT_oi00hit", "HIT_oi00_ahit", "HIT_torch_hit"] {
        let col = CollisionMesh::from_hit_meshes_placed(&c, |n| n == name, |v| v, 0.0);
        for w in &col.walls {
            let ymin = w.a.y.min(w.b.y);
            let ymax = w.a.y.max(w.b.y);
            if ymax > -14.0 && ymin < -8.0 && w.a.x.abs().min(w.b.x.abs()) < 4.0 {
                println!("{} wall ({:.2},{:.2})-({:.2},{:.2}) z {:.2}..{:.2}", name, w.a.x, w.a.y, w.b.x, w.b.y, w.z_min, w.z_max);
            }
        }
    }
}
