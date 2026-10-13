use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    let nodes = c.scene_nodes();
    for m in c.objects_of_type(objects::SECTION_MODEL) {
        let name = c.object_name(m.object_id).unwrap_or("?");
        let node = nodes.values().find(|n| n.model_obj_id == m.object_id);
        println!("{:<22} hdr={:02x?} nodeflags={:?}", name, &m.payload[4..24], node.map(|n| format!("{:#x}", n.flags)));
    }
}
