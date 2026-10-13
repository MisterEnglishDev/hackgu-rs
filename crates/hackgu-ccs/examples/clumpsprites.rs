//! clumpsprites file.ccs [clump-prefix] : per clump, its placed models (bind pose) with texture, bounds, uv range,
//! vertex colour range and material float (opacity) — for 2D/UI scenes such as the ALTIMIT desktop.
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    let filt = a.get(2).cloned().unwrap_or_default();
    let owners = c.model_owner_nodes();
    let clumps = c.clumps();
    let nodes = c.scene_nodes();
    let mats = c.materials();
    let models: std::collections::HashMap<u32, _> = c.objects_of_type(objects::SECTION_MODEL).into_iter().map(|m| (m.object_id, m)).collect();
    for (ci, cl) in clumps.iter().enumerate() {
        let cname = c.object_name(cl.object_id).unwrap_or("?");
        if !cname.starts_with(&filt) { continue; }
        println!("{}", cname);
        let wm = cl.compute_world_matrices(&nodes);
        for &oid in &cl.bone_obj_ids {
            let Some(n) = nodes.get(&oid) else { continue };
            let Some(m) = models.get(&n.model_obj_id) else { continue };
            let name = c.object_name(m.object_id).unwrap_or("?");
            let Ok(mut dm) = DecodedModel::from_payload(name, &m.payload, c.header.raw_version) else { println!("  {} decode fail", name); continue };
            if dm.is_rigid() { if let Some((oc, on)) = owners.get(&m.object_id) { if *oc == ci { if let Some(mt) = wm.get(on) { dm.apply_rigid(mt); } } } }
            for me in &dm.meshes {
                let (mut lo, mut hi, mut ulo, mut uhi) = ([f32::MAX; 3], [f32::MIN; 3], [f32::MAX; 2], [f32::MIN; 2]);
                let (mut clo, mut chi) = ([255u8; 4], [0u8; 4]);
                for v in &me.vertices {
                    for k in 0..3 { lo[k] = lo[k].min(v.position[k]); hi[k] = hi[k].max(v.position[k]); }
                    for k in 0..2 { ulo[k] = ulo[k].min(v.uv[k]); uhi[k] = uhi[k].max(v.uv[k]); }
                    if let Some(col) = v.color { for k in 0..4 { clo[k] = clo[k].min(col[k]); chi[k] = chi[k].max(col[k]); } }
                }
                let mat = mats.get(&me.material_index);
                let tex = mat.and_then(|t| c.object_name(t.texture_obj_id)).unwrap_or("-");
                println!("  {:<18} df{} {:<16} nv{:<3} tri{:<3} pos {:.2?}..{:.2?} uv {:.3?}..{:.3?} col {:?}..{:?} mat {:.3}", name, dm.draw_flags, tex, me.vertices.len(), me.indices.len() / 3, lo, hi, ulo, uhi, clo, chi, mat.map(|m| m.float_param).unwrap_or(-1.0));
            }
        }
    }
}
