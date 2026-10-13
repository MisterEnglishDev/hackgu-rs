use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    for cl in c.clumps() { println!("{} bones={} first={:?}", c.object_name(cl.object_id).unwrap_or("?"), cl.bone_count, &cl.bone_obj_ids[..cl.bone_count.min(8)]); }
    for m in c.objects_of_type(objects::SECTION_MODEL).iter().take(12) {
        let name = c.object_name(m.object_id).unwrap_or("?");
        if let Ok(dm) = DecodedModel::from_payload(name, &m.payload, c.header.raw_version) {
            for me in dm.meshes.iter().take(3) { let p = me.vertices[0].bone_indices[0] as u32;
                println!("  {} parent={} -> name {:?} sec {:?}", name, p, c.object_name(p), c.raw_objects.iter().find(|o| o.object_id==p).map(|o| format!("{:#x}",o.section_type))); }
        }
    }
}
