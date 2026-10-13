//! uvregion file.ccs MDL_name zmin : UV range of the model's vertices above zmin (metres, model space).
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    let zmin: f32 = a[3].parse().unwrap();
    for m in c.objects_of_type(objects::SECTION_MODEL) {
        if c.object_name(m.object_id) != Some(a[2].as_str()) { continue; }
        let dm = DecodedModel::from_payload(&a[2], &m.payload, c.header.raw_version).unwrap();
        let (mut lo, mut hi) = ([f32::MAX; 2], [f32::MIN; 2]);
        for me in &dm.meshes { for v in &me.vertices {
            let q = v.local_positions[0]; if q[2] < zmin || q[0].abs() > 0.35 { continue; }
            for k in 0..2 { lo[k] = lo[k].min(v.uv[k]); hi[k] = hi[k].max(v.uv[k]); }
            if std::env::args().any(|x| x == "-v") { println!("  pos {:5.2?} uv {:6.3?}", q, v.uv); }
        }}
        println!("u {:.3}..{:.3} v {:.3}..{:.3}", lo[0], hi[0], lo[1], hi[1]);
    }
}
