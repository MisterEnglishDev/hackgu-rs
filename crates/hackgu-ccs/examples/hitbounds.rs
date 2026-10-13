use hackgu_ccs::*;
fn main() {
    for f in std::env::args().skip(1) {
        let c = CcsFile::parse(&std::fs::read(&f).unwrap()).unwrap();
        let (mut lo, mut hi) = ([f32::MAX; 3], [f32::MIN; 3]);
        for h in c.hit_meshes() { for r in &h.records { for v in &r.positions { for k in 0..3 { lo[k] = lo[k].min(v[k] * 0.01); hi[k] = hi[k].max(v[k] * 0.01); } } } }
        print!("{} hit {:.1?}..{:.1?} |", f.rsplit('/').next().unwrap(), lo, hi);
        for m in c.objects_of_type(objects::SECTION_MODEL) {
            let s = f32::from_le_bytes(m.payload[0..4].try_into().unwrap());
            print!(" {}:vs={}", c.object_name(m.object_id).unwrap_or("?"), s);
        }
        println!();
    }
}
