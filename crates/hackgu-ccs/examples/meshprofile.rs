//! meshprofile file.ccs MDL_name mesh : max/min z of the mesh's vertices per 1 m band of local y (metres).
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    let k: usize = a[3].parse().unwrap();
    for m in c.objects_of_type(objects::SECTION_MODEL) {
        if c.object_name(m.object_id) != Some(a[2].as_str()) { continue; }
        let dm = DecodedModel::from_payload(&a[2], &m.payload, c.header.raw_version).unwrap();
        let mut bands: std::collections::BTreeMap<i32, (f32, f32)> = Default::default();
        for v in &dm.meshes[k].vertices {
            let p = v.local_positions[0];
            let e = bands.entry(p[1].floor() as i32).or_insert((f32::MAX, f32::MIN));
            e.0 = e.0.min(p[2]); e.1 = e.1.max(p[2]);
        }
        for (y, (lo, hi)) in bands { println!("y {:3} z {:5.2}..{:5.2}", y, lo, hi); }
    }
}
