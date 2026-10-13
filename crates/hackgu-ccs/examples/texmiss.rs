//! texmiss file.ccs... : materials whose texture is absent or fails to decode, with the meshes that use them.
use hackgu_ccs::*;
use std::collections::HashMap;
fn main() {
    for f in std::env::args().skip(1) {
        let c = CcsFile::parse(&std::fs::read(&f).unwrap()).unwrap();
        let mats = c.materials();
        let texs: HashMap<u32, _> = c.objects_of_type(objects::SECTION_TEXTURE).into_iter().map(|t| (t.object_id, t)).collect();
        let mut use_count: HashMap<u32, Vec<String>> = HashMap::new();
        for m in c.objects_of_type(objects::SECTION_MODEL) {
            let name = c.object_name(m.object_id).unwrap_or("?").to_string();
            match DecodedModel::from_payload(&name, &m.payload, c.header.raw_version) {
                Ok(dm) => for me in &dm.meshes { use_count.entry(me.material_index).or_default().push(name.clone()); },
                Err(e) => println!("  {}: model {} decode error {:?}", f, name, e),
            }
        }
        let mut bad = 0;
        for (mid, users) in &use_count {
            let why = match mats.get(mid) {
                None => format!("material {} not found", mid),
                Some(mat) => match texs.get(&mat.texture_obj_id) {
                    None => format!("texture obj {} ({}) not in file", mat.texture_obj_id, c.object_name(mat.texture_obj_id).unwrap_or("?")),
                    Some(t) => {
                        let clut_id = i32::from_le_bytes(t.payload[0..4].try_into().unwrap());
                        let clut = if clut_id > 0 { c.raw_objects.iter().find(|o| o.object_id == clut_id as u32 && o.section_type == objects::SECTION_CLUT).and_then(|o| o.payload.get(16..)) } else { None };
                        match DecodedTexture::from_payload_with_clut("t", &t.payload, clut) { Ok(_) => continue, Err(e) => format!("{} decode error {:?}", c.object_name(t.object_id).unwrap_or("?"), e) }
                    }
                },
            };
            bad += 1;
            let mut u = users.clone(); u.sort(); u.dedup();
            println!("  {} mat {} {}: used by {} meshes e.g. {:?}", f.rsplit('/').next().unwrap(), mid, why, users.len(), &u[..u.len().min(4)]);
        }
        println!("{}: {} bad materials of {}", f, bad, use_count.len());
    }
}
