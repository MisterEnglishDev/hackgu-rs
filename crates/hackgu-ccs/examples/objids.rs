//! objids file.ccs [filter] : object id, section type and name
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    let f = a.get(2).cloned().unwrap_or_default();
    for o in &c.raw_objects {
        let n = c.object_name(o.object_id).unwrap_or("?");
        if n.contains(&f) { println!("{:4} 0x{:04x} {}", o.object_id, o.section_type, n); }
    }
    let nodes = c.scene_nodes();
    for cl in c.clumps() { println!("clump {} bones {:?}", c.object_name(cl.object_id).unwrap_or("?"), cl.bone_obj_ids); }
    let _ = nodes;
}
