//! wallsides file.ccs : for each HitMesh, the centroid of its near-vertical (wall) triangles, per side
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    for h in c.hit_meshes() {
        let name = c.object_name(h.object_id).unwrap_or("?");
        let mut walls = vec![];
        for r in &h.records {
            for t in r.positions.chunks_exact(3) {
                let p: Vec<[f32; 3]> = t.iter().map(|v| [v[0] * 0.01, v[1] * 0.01, v[2] * 0.01]).collect();
                let e1 = [p[1][0] - p[0][0], p[1][1] - p[0][1], p[1][2] - p[0][2]];
                let e2 = [p[2][0] - p[0][0], p[2][1] - p[0][1], p[2][2] - p[0][2]];
                let n = [e1[1] * e2[2] - e1[2] * e2[1], e1[2] * e2[0] - e1[0] * e2[2], e1[0] * e2[1] - e1[1] * e2[0]];
                let l = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
                if l < 1e-6 || (n[2] / l).abs() > 0.3 { continue; }
                walls.push(((p[0][0] + p[1][0] + p[2][0]) / 3.0, (p[0][1] + p[1][1] + p[2][1]) / 3.0));
            }
        }
        // classify wall centroids to the nearest cell edge
        let mut sides = [0; 4]; // +x -x +y -y
        for (x, y) in &walls { if x.abs() > y.abs() { if *x > 0.0 { sides[0] += 1 } else { sides[1] += 1 } } else if *y > 0.0 { sides[2] += 1 } else { sides[3] += 1 } }
        println!("{:<20} walls {:<4} +x {:<3} -x {:<3} +y {:<3} -y {:<3}", name, walls.len(), sides[0], sides[1], sides[2], sides[3]);
    }
}
