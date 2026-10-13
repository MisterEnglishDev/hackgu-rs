use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    for h in c.hit_meshes() {
        let (mut lo, mut hi, mut n) = ([f32::MAX; 3], [f32::MIN; 3], 0);
        for r in &h.records { for v in &r.positions { n += 1; for k in 0..3 { lo[k] = lo[k].min(v[k] * 0.01); hi[k] = hi[k].max(v[k] * 0.01); } } }
        let fl: Vec<String> = h.records.iter().map(|r| format!("{:x}", r.flags)).collect();
        println!("{:<22} n{:<5} {:.1?}..{:.1?} flags {}", c.object_name(h.object_id).unwrap_or("?"), n, lo, hi, fl.join(","));
    }
}
