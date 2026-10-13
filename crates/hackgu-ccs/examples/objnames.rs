//! objnames file.ccs substring : list objects whose name contains the substring (id, section type, name).
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    let nodes = c.scene_nodes();
    for o in &c.raw_objects {
        let n = c.object_name(o.object_id).unwrap_or("?");
        if n.contains(a[2].as_str()) {
            let node = nodes.get(&o.object_id).map(|nd| format!("parent {} ({}) model {} ({})", nd.parent_obj_id, c.object_name(nd.parent_obj_id).unwrap_or("?"), nd.model_obj_id, c.object_name(nd.model_obj_id).unwrap_or("?"))).unwrap_or_default();
            println!("{:5} {:#06x} {} {}", o.object_id, o.section_type, n, node);
        }
    }
}
