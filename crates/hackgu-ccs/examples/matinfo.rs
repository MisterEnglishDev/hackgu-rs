use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    let filt = a.get(2).cloned().unwrap_or_default();
    let mats = c.materials();
    for m in c.objects_of_type(objects::SECTION_MODEL) {
        let name = c.object_name(m.object_id).unwrap_or("?");
        if !name.contains(&filt) { continue; }
        let dm = DecodedModel::from_payload(name, &m.payload, c.header.raw_version).unwrap();
        for me in &dm.meshes {
            let (mut zmin, mut zmax) = (f32::MAX, f32::MIN);
            for v in &me.vertices { zmin = zmin.min(v.local_positions[0][2]); zmax = zmax.max(v.local_positions[0][2]); }
            let mo = c.raw_objects.iter().find(|o| o.object_id == me.material_index);
            let mt = mats.get(&me.material_index);
            println!("{} flags{:#x} mat {} {:?} tex {:?} z[{:.3},{:.3}] nv{} raw={:02x?}", name, dm.model_flags, me.material_index, c.object_name(me.material_index),
                mt.map(|t| c.object_name(t.texture_obj_id)), zmin, zmax, me.vertices.len(), mo.map(|o| &o.payload[..o.payload.len().min(40)]));
        }
    }
}
