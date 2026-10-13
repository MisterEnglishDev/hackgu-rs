//! animpos file.ccs [filter] : per animation, each object's position (scene units), scale, alpha at frame 0 and the last frame
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    let filt = a.get(2).cloned().unwrap_or_default();
    for an in c.animations() {
        if !an.name.contains(&filt) { continue; }
        println!("== {} frames={}", an.name, an.frame_count);
        let last = an.frame_count.saturating_sub(1) as f32;
        for ch in &an.channels {
            if ch.translation_keys.is_empty() && ch.static_translation.is_none() && ch.alpha_keys.is_empty() && ch.scale_keys.is_empty() { continue; }
            let p = |f: f32| ch.sample_translation(f, [f32::NAN; 3]);
            println!("   {:<20} pos {:>7.1?} -> {:>7.1?}  scale {:?} -> {:?}  alpha {:?} -> {:?}", ch.bone_name.trim_start_matches("OBJ_"), p(0.0), p(last),
                ch.sample_scale(0.0).map(|v| (v[0] * 100.0).round() / 100.0), ch.sample_scale(last).map(|v| (v[0] * 100.0).round() / 100.0),
                ch.sample_alpha(0.0).map(|v| (v * 100.0).round() / 100.0), ch.sample_alpha(last).map(|v| (v * 100.0).round() / 100.0));
        }
    }
}
