//! msgrows PM10310 : print the BIN_<id> message table rows (row, speaker, voice id, text).
use std::path::Path;
fn main() {
    let lang = hackgu_game::msginfo::MessageInfo::load(Path::new("target"), "eng");
    for id in std::env::args().skip(1) {
        let Some(t) = hackgu_game::msgtable::load_message_entries(Path::new("target/vol1/data/data/ev"), &id) else { println!("{}: none", id); continue };
        let mut rows: Vec<_> = t.iter().collect();
        rows.sort_by_key(|r| *r.0);
        for (n, e) in rows {
            let key = hackgu_game::msginfo::scenario_key(&e.voice).unwrap_or("");
            let prefix = &e.voice[..e.voice.len() - key.len()];
            let en = lang.as_ref().and_then(|l| l.get(prefix)).unwrap_or("-");
            println!("{:3} {:<12} {:<40} spk_en={:<12} {}", n, e.speaker, e.voice, en, e.text.chars().take(30).collect::<String>());
        }
    }
}
