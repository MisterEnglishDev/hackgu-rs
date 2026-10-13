//! evtext PM10110 ... : step an event script and print its cuts, stage changes and (English) lines in order.
use hackgu_game::ev::{EvPlayer, EvScript};
use std::path::Path;
fn main() {
    let lang = hackgu_game::msginfo::MessageInfo::load(Path::new("target"), "eng").map(std::sync::Arc::new);
    let dir = Path::new("target/vol1/data/data/ev");
    for id in std::env::args().skip(1) {
        let script = match EvScript::load(dir, &id) { Ok(s) => s, Err(e) => { println!("{}: {}", id, e); continue } };
        println!("== {} origin {:?} stages {:?}", id, script.header_origin, script.stages());
        let mut p = EvPlayer::new_with_ev_dir(script, Some(dir));
        p.lang = lang.clone();
        let (mut last_cut, mut last_msg, mut last_stage) = (String::new(), String::new(), None);
        for _ in 0..200000 {
            p.update(1.0 / 30.0);
            let f = p.state.clone();
            if f.cut_label != last_cut {
                last_cut = f.cut_label.clone();
                let cam = f.camera.as_ref().map(|c| format!(" eye {:.0?} target {:.0?}", c.eye, c.target)).unwrap_or_default();
                println!("  [cut {}]{}", last_cut, cam);
            }
            if f.stage != last_stage { last_stage = f.stage.clone(); println!("  [stage {:?}]", last_stage); }
            if let Some(m) = &f.message {
                let s = format!("{}: {}", m.speaker, m.text.replace('\n', " / "));
                if s != last_msg { println!("    {}", s); last_msg = s; }
                p.press_advance();
            }
            if p.is_finished() { break; }
        }
    }
}
