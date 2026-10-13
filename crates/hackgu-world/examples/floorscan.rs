//! floorscan zlevel tol : print grid cells (4 m) of Mac Anu whose collision floor is within tol of zlevel
use hackgu_ccs::CcsFile;
use hackgu_world::CollisionMesh;
fn main() {
    let a: Vec<f32> = std::env::args().skip(1).filter_map(|s| s.parse().ok()).collect();
    let (zl, tol) = (a[0], a[1]);
    let dir = "../target/vol1/data/data/town/";
    let town = CcsFile::parse(&std::fs::read(format!("{dir}town01.ccs")).unwrap()).unwrap();
    let loc = town.locators();
    let mut col = CollisionMesh::from_hit_meshes(&town, [0.0; 3]);
    for n in 0..=8 {
        let off = loc.iter().find(|l| l.0 == format!("DMY_town01_{n:02}d")).unwrap().1;
        let d = CcsFile::parse(&std::fs::read(format!("{dir}town01_{n:02}h.ccs")).unwrap()).unwrap();
        col.merge(&CollisionMesh::from_hit_meshes(&d, off));
    }
    let mut hits = vec![];
    let mut y = -300.0f32;
    while y < 300.0 { let mut x = -300.0f32; while x < 300.0 {
        if let Some(h) = col.get_ground_height(x, y, zl) { if (h - zl).abs() < tol { hits.push((x, y, h)); } }
        x += 4.0; } y += 4.0; }
    println!("{} cells", hits.len());
    for h in hits.iter().take(60) { println!("{:?}", h); }
}
