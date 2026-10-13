use hackgu_ccs::*;
fn main() {
    for f in std::env::args().skip(1) {
        let c = CcsFile::parse(&std::fs::read(&f).unwrap()).unwrap();
        for m in c.objects_of_type(objects::SECTION_MODEL) {
            let name = c.object_name(m.object_id).unwrap_or("?");
            let declared = u16::from_le_bytes(m.payload[6..8].try_into().unwrap());
            match DecodedModel::from_payload(name, &m.payload, c.header.raw_version) {
                Ok(dm) => if dm.meshes.len() != declared as usize { println!("{} {}: declared {} decoded {} (payload {})", f.rsplit('/').next().unwrap(), name, declared, dm.meshes.len(), m.payload.len()); },
                Err(e) => println!("{} {}: ERR {:?}", f, name, e),
            }
        }
    }
}
