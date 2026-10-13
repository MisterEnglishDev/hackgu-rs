//! For each model: which object node references it (structural link), which clump owns that node,
//! and which clump the name heuristic (CMP_<suffix>) would pick. Also flags missing materials/textures.
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let d = std::fs::read(&a[1]).unwrap();
    let c = CcsFile::parse(&d).unwrap();
    let nodes = c.scene_nodes();
    let clumps = c.clumps();
    let mats = c.materials();
    let texs: std::collections::HashSet<u32> = c.objects_of_type(objects::SECTION_TEXTURE).iter().map(|o| o.object_id).collect();
    for m in c.objects_of_type(objects::SECTION_MODEL) {
        let name = c.object_name(m.object_id).unwrap_or("?");
        let refs: Vec<u32> = nodes.values().filter(|n| n.model_obj_id == m.object_id).map(|n| n.object_id).collect();
        let owner: Vec<String> = refs.iter().map(|&r| {
            let ci = clumps.iter().position(|cl| cl.bone_obj_ids.contains(&r));
            format!("{}@{}", c.object_name(r).unwrap_or("?"), ci.map(|i| c.object_name(clumps[i].object_id).unwrap_or("?").to_string()).unwrap_or("-".into()))
        }).collect();
        let suf = name.strip_prefix("MDL_").unwrap_or(name);
        let heur = if clumps.len() == 1 { Some(0) } else { clumps.iter().position(|cl| { let cn = c.object_name(cl.object_id).unwrap_or(""); cn == format!("CMP_{}", suf) || cn.ends_with(suf) }) };
        let dm = DecodedModel::from_payload(name, &m.payload, c.header.raw_version).ok();
        let (ty, lk, nmesh, missing) = dm.as_ref().map(|dm| {
            let miss = dm.meshes.iter().filter(|me| mats.get(&me.material_index).map(|mt| !texs.contains(&mt.texture_obj_id)).unwrap_or(true)).count();
            (dm.model_type, dm.lookup_list.len(), dm.meshes.len(), miss)
        }).unwrap_or((255, 0, 0, 0));
        let bi: Vec<u16> = dm.as_ref().map(|dm| { let mut v: Vec<u16> = dm.meshes.iter().flat_map(|me| me.vertices.iter().map(|v| v.bone_indices[0])).collect(); v.sort(); v.dedup(); v.truncate(6); v }).unwrap_or_default();
        println!("{:<28} ty{:<3} lk{:<3} meshes{:<3} nomat{:<2} bones{:?} refs[{}] heur={}", name, ty, lk, nmesh, missing, bi, owner.join(","), heur.map(|i| c.object_name(clumps[i].object_id).unwrap_or("?")).unwrap_or("-"));
    }
}
