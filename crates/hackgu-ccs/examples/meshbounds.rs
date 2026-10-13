//! meshbounds file.ccs model : per-mesh texture + bounds (metres, model space)
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    let mats = c.materials();
    for m in c.objects_of_type(objects::SECTION_MODEL) {
        let name = c.object_name(m.object_id).unwrap_or("?");
        if name != a[2] { continue; }
        let dm = DecodedModel::from_payload(name, &m.payload, c.header.raw_version).unwrap();
        println!("{} vs={} type={} meshes={}", name, dm.vertex_scale, dm.model_type, dm.meshes.len());
        for (i, me) in dm.meshes.iter().enumerate() {
            let (mut lo, mut hi) = ([f32::MAX; 3], [f32::MIN; 3]);
            for v in &me.vertices { for k in 0..3 { lo[k] = lo[k].min(v.local_positions[0][k]); hi[k] = hi[k].max(v.local_positions[0][k]); } }
            let tex = mats.get(&me.material_index).and_then(|t| c.object_name(t.texture_obj_id)).unwrap_or("-");
            println!("  {:>2} {:<10} nv{:<5} x[{:7.1},{:7.1}] y[{:7.1},{:7.1}] z[{:6.1},{:6.1}]", i, tex, me.vertices.len(), lo[0], hi[0], lo[1], hi[1], lo[2], hi[2]);
        }
    }
}
