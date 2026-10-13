//! effdump file.ccs : hex/float dump of Effect (0x0E00) sections and the objects that reference them.
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    let nodes = c.scene_nodes();
    for e in c.objects_of_type(objects::SECTION_EFFECT) {
        let name = c.object_name(e.object_id).unwrap_or("?");
        let users: Vec<&str> = nodes.values().filter(|n| n.model_obj_id == e.object_id).map(|n| c.object_name(n.object_id).unwrap_or("?")).collect();
        println!("== {} id {} len {} users {:?}", name, e.object_id, e.payload.len(), users);
        for (i, ch) in e.payload.chunks(4).enumerate().take(40) {
            if ch.len() < 4 { break; }
            let u = u32::from_le_bytes(ch.try_into().unwrap());
            let f = f32::from_le_bytes(ch.try_into().unwrap());
            let n = c.object_name(u).map(|s| format!(" <{}>", s)).unwrap_or_default();
            print!("[{:2}] {:08x} {:>10.4}{}  ", i, u, f, n);
            if i % 3 == 2 { println!(); }
        }
        println!();
    }
}
