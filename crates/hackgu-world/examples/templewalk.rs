//! templewalk: walk +y through the temple entrance on HIT_oi00hit alone (local metres), printing each refusal.
use glam::{Vec2, Vec3};
use hackgu_ccs::CcsFile;
use hackgu_world::CollisionMesh;
fn main() {
    let c = CcsFile::parse(&std::fs::read("target/vol1/data/data/field/sf3_00.ccs").unwrap()).unwrap();
    let col = CollisionMesh::from_hit_meshes_placed(&c, |n| n == "HIT_oi00hit", |v| v, 0.0);
    let mut p = Vec3::new(0.0, -15.0, 1.6);
    for _ in 0..400 {
        let from = Vec2::new(p.x, p.y);
        let to = from + Vec2::new(0.0, 0.1);
        let r = col.resolve_walls(from, to, p.z, 0.4, 1.8, 0.35);
        match col.get_ground_height(r.x, r.y, p.z) {
            Some(h) if (h - p.z).abs() <= 0.8 => p = Vec3::new(r.x, r.y, h),
            other => { println!("stuck at {:?}: wall-resolved {:?} ground {:?}", p, r, other); break; }
        }
    }
    println!("end {:?}", p);
    let q = Vec2::new(0.0, -12.0);
    for w in &col.walls {
        let ab = w.b - w.a;
        let t = ((q - w.a).dot(ab) / ab.length_squared().max(1e-6)).clamp(0.0, 1.0);
        let d = (w.a + ab * t - q).length();
        if d < 1.5 { println!("near wall ({:.2},{:.2})-({:.2},{:.2}) z {:.2}..{:.2} d {:.2}", w.a.x, w.a.y, w.b.x, w.b.y, w.z_min, w.z_max, d); }
    }
}
#[allow(dead_code)]
fn _unused() {}
