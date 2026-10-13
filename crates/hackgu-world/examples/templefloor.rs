//! templefloor: every walkable surface height of the field Beast Temple (sf3_00 HIT_oi00*), temple-local metres.
use hackgu_ccs::CcsFile;
use hackgu_world::CollisionMesh;
fn main() {
    let c = CcsFile::parse(&std::fs::read(&std::env::args().nth(1).unwrap_or("target/vol1/data/data/field/sf3_00.ccs".into())).unwrap()).unwrap();
    let col = CollisionMesh::from_hit_meshes_placed(&c, |n| n.starts_with(&std::env::args().nth(2).unwrap_or("HIT_oi00".into())), |v| v, 0.0);
    for x in [0.0f32, -6.0, 6.0] {
        println!("== x {}", x);
        for y in (-22..=46).step_by(2) {
            let mut hs: Vec<f32> = Vec::new();
            let mut z = 30.0;
            while z > -15.0 {
                if let Some(g) = col.get_ground_height(x, y as f32, z) {
                    if !hs.iter().any(|h| (h - g).abs() < 0.05) { hs.push(g); }
                }
                z -= 0.5;
            }
            println!("y {:4}: {:?}", y, hs.iter().map(|h| (h * 100.0).round() / 100.0).collect::<Vec<_>>());
        }
    }
}
