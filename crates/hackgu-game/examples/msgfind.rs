//! msgfind <substring>... : print every messageInfo (eng) string containing all substrings, with its id.
use std::path::Path;
fn main() {
    let pats: Vec<String> = std::env::args().skip(1).map(|s| s.to_lowercase()).collect();
    let l = hackgu_game::msginfo::MessageInfo::load(Path::new("target"), "eng").expect("messageInfo");
    let mut v: Vec<_> = l.by_id.iter().filter(|(_, s)| pats.iter().all(|p| s.to_lowercase().contains(p))).collect();
    v.sort_by_key(|(id, _)| **id);
    for (id, s) in v {
        println!("{:08X} {}", id, s.replace('\n', " / "));
    }
}
