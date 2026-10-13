//! slopes file.ccs name... : mean height of each model's vertices per 4 m band on each side (to see which edge drops into the sea)
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    for h in c.hit_meshes() {
        let name = c.object_name(h.object_id).unwrap_or("?");
        if !a[2..].iter().any(|f| name.contains(f.as_str())) { continue; }
        // per side: floor-ish triangles' vertex heights near each edge (|x| or |y| > 6 m)
        let (mut s, mut n) = ([0f32; 4], [0u32; 4]); // +x, -x, +y, -y
        let mut centre = (0f32, 0u32);
        for r in &h.records { for v in &r.positions { let (x, y, z) = (v[0] * 0.01, v[1] * 0.01, v[2] * 0.01);
            if x > 6.0 { s[0] += z; n[0] += 1 } if x < -6.0 { s[1] += z; n[1] += 1 } if y > 6.0 { s[2] += z; n[2] += 1 } if y < -6.0 { s[3] += z; n[3] += 1 }
            if x.abs() < 3.0 && y.abs() < 3.0 { centre.0 += z; centre.1 += 1 } } }
        let m = |i: usize| if n[i] > 0 { s[i] / n[i] as f32 } else { f32::NAN };
        println!("{:<14} +x {:5.2} -x {:5.2} +y {:5.2} -y {:5.2} centre {:5.2}", name, m(0), m(1), m(2), m(3), if centre.1 > 0 { centre.0 / centre.1 as f32 } else { f32::NAN });
    }
}
