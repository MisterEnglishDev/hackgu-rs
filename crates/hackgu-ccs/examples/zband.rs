//! zband file.ccs z0 z1 : XY bounding box + count of model vertices and HitMesh vertices with z in [z0, z1] metres
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    let (z0, z1): (f32, f32) = (a[2].parse().unwrap(), a[3].parse().unwrap());
    let mut acc = |pts: &mut dyn Iterator<Item = [f32; 3]>, label: &str| {
        let (mut lo, mut hi, mut n) = ([f32::MAX; 2], [f32::MIN; 2], 0);
        for p in pts { if p[2] >= z0 && p[2] <= z1 { n += 1; for k in 0..2 { lo[k] = lo[k].min(p[k]); hi[k] = hi[k].max(p[k]); } } }
        println!("{:<28} n={:<6} xy {:.1?}..{:.1?}", label, n, lo, hi);
    };
    for m in c.objects_of_type(objects::SECTION_MODEL) {
        let name = c.object_name(m.object_id).unwrap_or("?");
        if let Ok(dm) = DecodedModel::from_payload(name, &m.payload, c.header.raw_version) {
            acc(&mut dm.meshes.iter().flat_map(|me| me.vertices.iter().map(|v| v.position)), name);
        }
    }
    for h in c.hit_meshes() { acc(&mut h.records.iter().flat_map(|r| r.positions.iter().map(|v| [v[0] * 0.01, v[1] * 0.01, v[2] * 0.01])), "HIT"); }
}
