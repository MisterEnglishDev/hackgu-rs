//! msgkey KEY... : look keys up in the English messageInfo.bin.
fn main() {
    let l = hackgu_game::msginfo::MessageInfo::load(std::path::Path::new("target"), "eng").unwrap();
    for k in std::env::args().skip(1) {
        println!("{:<28} {:08X} {:?}", k, hackgu_game::msginfo::message_id(&k), l.get(&k));
    }
}
