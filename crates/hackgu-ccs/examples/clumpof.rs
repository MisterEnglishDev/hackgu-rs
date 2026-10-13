//! clumpof file.ccs NAME... : for each named object (e.g. a DMY locator), print the scene nodes that reference it
//! and the clumps those nodes are bones of.
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    let nodes = c.scene_nodes();
    let clumps = c.clumps();
    for want in &a[2..] {
        let ids: Vec<u32> = c.raw_objects.iter().filter(|o| c.object_name(o.object_id) == Some(want.as_str())).map(|o| o.object_id).collect();
        for n in nodes.values() {
            if ids.contains(&n.model_obj_id) || ids.contains(&n.object_id) {
                let owners: Vec<&str> = clumps.iter().filter(|cl| cl.bone_obj_ids.contains(&n.object_id)).map(|cl| c.object_name(cl.object_id).unwrap_or("?")).collect();
                println!("{} node {} ({}) parent {} ({}) clumps {:?}", want, n.object_id, c.object_name(n.object_id).unwrap_or("?"), n.parent_obj_id, c.object_name(n.parent_obj_id).unwrap_or("?"), owners);
            }
        }
    }
}
