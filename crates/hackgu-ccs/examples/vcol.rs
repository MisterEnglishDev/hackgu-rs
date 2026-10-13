use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    for m in c.objects_of_type(objects::SECTION_MODEL) {
        let name = c.object_name(m.object_id).unwrap_or("?");
        if !a[2..].iter().any(|f| name.contains(f.as_str())) { continue; }
        let dm = DecodedModel::from_payload(name, &m.payload, c.header.raw_version).unwrap();
        println!("{} ty{} flags{:#x} draw{:#x} hdr={:02x?} colors={:?}", name, dm.model_type, dm.model_flags, dm.draw_flags, &m.payload[4..16], dm.meshes.first().and_then(|me| me.vertices.iter().take(3).map(|v| v.color).collect::<Vec<_>>().into()));
    }
}
