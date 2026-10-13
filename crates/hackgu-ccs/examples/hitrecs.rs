use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    for h in c.hit_meshes() {
        let name = c.object_name(h.object_id).unwrap_or("?");
        if !a[2..].iter().any(|f| name.contains(f.as_str())) { continue; }
        for r in &h.records {
            let (mut lo, mut hi) = ([f32::MAX; 3], [f32::MIN; 3]);
            let mut up = 0; let mut tri = 0;
            for t in r.normals.chunks_exact(3) { tri += 1; if t[0][2] > 0.7 { up += 1 } }
            for v in &r.positions { for k in 0..3 { lo[k] = lo[k].min(v[k] * 0.01); hi[k] = hi[k].max(v[k] * 0.01); } }
            println!("{:<12} flags {:08x} tris {:<4} upward {:<4} {:.1?}..{:.1?}", name, r.flags, tri, up, lo, hi);
        }
    }
}
