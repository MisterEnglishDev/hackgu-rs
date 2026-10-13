//! rotkeys file.ccs ANM_name : per channel, rotation keys (frame, quaternion) and static rotations.
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    for an in c.animations() {
        if an.name != a[2] { continue; }
        println!("== {} frames {}", an.name, an.frame_count);
        for ch in &an.channels {
            println!("  {} static_rot {:?} keys {}", ch.bone_name, ch.static_rotation, ch.rotation_keys.len());
            for k in ch.rotation_keys.iter().step_by((ch.rotation_keys.len() / 6).max(1)) {
                println!("     f{:4} q {:6.3?}", k.frame, k.quat);
            }
        }
    }
}
