use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    for m in c.objects_of_type(objects::SECTION_MODEL) {
        let name = c.object_name(m.object_id).unwrap_or("?");
        if !a[2..].iter().any(|f| name == f) { continue; }
        let dm = DecodedModel::from_payload(name, &m.payload, c.header.raw_version).unwrap();
        let (mut lo, mut hi) = ([f32::MAX; 3], [f32::MIN; 3]);
        for me in &dm.meshes { for v in &me.vertices { for k in 0..3 { lo[k] = lo[k].min(v.local_positions[0][k]); hi[k] = hi[k].max(v.local_positions[0][k]); } } }
        println!("{} local bounds {:.2?} .. {:.2?}", name, lo, hi);
    }
}
