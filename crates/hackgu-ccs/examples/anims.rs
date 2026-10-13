//! anims file.ccs [skeleton.ccs] : list animations; if a skeleton is given, report how many of its bones each anim drives
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    let sk = a.get(2).map(|p| CcsFile::parse(&std::fs::read(p).unwrap()).unwrap());
    for an in c.animations() {
        let mut s = String::new();
        if let Some(sk) = &sk {
            let cl = &sk.clumps()[0];
            let names: Vec<&str> = cl.bone_obj_ids.iter().map(|&o| sk.object_name(o).unwrap_or("")).collect();
            let hit = names.iter().filter(|n| { let suf = n.find(' ').map(|p| &n[p + 1..]).unwrap_or(n); an.channel_by_name.contains_key(**n) || an.channel_by_name.contains_key(suf) }).count();
            s = format!(" drives {}/{} bones", hit, names.len());
        }
        println!("{} frames={} ch={}{} first={:?}", an.name, an.frame_count, an.channels.len(), s, an.channels.iter().take(3).map(|c| c.bone_name.clone()).collect::<Vec<_>>());
    }
}
