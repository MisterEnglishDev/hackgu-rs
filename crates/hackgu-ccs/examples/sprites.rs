//! sprites file.ccs : for every model, owner-placed bounds (bind pose) + texture + uv range
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    let owners = c.model_owner_nodes(); let clumps = c.clumps(); let nodes = c.scene_nodes();
    let wms: Vec<_> = clumps.iter().map(|cl| cl.compute_world_matrices(&nodes)).collect();
    let mats = c.materials();
    for m in c.objects_of_type(objects::SECTION_MODEL) {
        let name = c.object_name(m.object_id).unwrap_or("?");
        let Ok(mut dm) = DecodedModel::from_payload(name, &m.payload, c.header.raw_version) else { println!("{} decode fail", name); continue };
        if dm.is_rigid() { if let Some((ci, n)) = owners.get(&m.object_id) { if let Some(mt) = wms[*ci].get(n) { dm.apply_rigid(mt); } } }
        for me in &dm.meshes {
            let (mut lo, mut hi, mut ulo, mut uhi) = ([f32::MAX; 3], [f32::MIN; 3], [f32::MAX; 2], [f32::MIN; 2]);
            for v in &me.vertices { for k in 0..3 { lo[k] = lo[k].min(v.position[k]); hi[k] = hi[k].max(v.position[k]); } for k in 0..2 { ulo[k] = ulo[k].min(v.uv[k]); uhi[k] = uhi[k].max(v.uv[k]); } }
            let tex = mats.get(&me.material_index).and_then(|t| c.object_name(t.texture_obj_id)).unwrap_or("-");
            println!("{:<20} {:<18} nv{:<3} pos {:.2?}..{:.2?} uv {:.3?}..{:.3?}", name, tex, me.vertices.len(), lo, hi, ulo, uhi);
        }
    }
}
