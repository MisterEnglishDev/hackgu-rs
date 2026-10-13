use std::env;
use std::path::PathBuf;
use winit::event_loop::{ControlFlow, EventLoop};

use hackgu_render::app::App;

fn main() {
    // A crash writes its message, location and backtrace to crash.log next to the game (the console window closes).
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let bt = std::backtrace::Backtrace::force_capture();
        let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        let text = format!("=== crash at unix time {} ===
{}

backtrace:
{}

", secs, info, bt);
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("crash.log") {
            use std::io::Write;
            let _ = f.write_all(text.as_bytes());
        }
        default_hook(info);
    }));
    let mut args: Vec<String> = env::args().skip(1).collect();
    hackgu_render::auto::init(&mut args);

    let mut consumed_indices = std::collections::HashSet::new();
    let mut headless_out = None;
    let mut anim_idx = None;
    let mut frame_val = None;
    let mut weapon_path = None;
    let mut stage_path = None;
    let mut explore_arg = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--headless" => {
                consumed_indices.insert(i);
                if i + 1 < args.len() {
                    consumed_indices.insert(i + 1);
                    headless_out = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--explore" | "-e" => {
                consumed_indices.insert(i);
                explore_arg = true;
            }
            "--weapon" => {
                consumed_indices.insert(i);
                if i + 1 < args.len() {
                    consumed_indices.insert(i + 1);
                    weapon_path = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                }
            }
            "--stage" => {
                consumed_indices.insert(i);
                if i + 1 < args.len() {
                    consumed_indices.insert(i + 1);
                    stage_path = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                }
            }
            "--anim" => {
                consumed_indices.insert(i);
                if i + 1 < args.len() {
                    consumed_indices.insert(i + 1);
                    anim_idx = args[i + 1].parse::<usize>().ok();
                    i += 1;
                }
            }
            "--frame" => {
                consumed_indices.insert(i);
                if i + 1 < args.len() {
                    consumed_indices.insert(i + 1);
                    frame_val = args[i + 1].parse::<f32>().ok();
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    let ccs_path = args
        .iter()
        .enumerate()
        // Only a .ccs file counts: values of the automation flags (`--title newgame`...) are not model paths.
        .find(|(idx, a)| !consumed_indices.contains(idx) && !a.starts_with("--") && a.to_ascii_lowercase().ends_with(".ccs"))
        .map(|(_, a)| PathBuf::from(a));

    let explore = explore_arg || ccs_path.is_none();

    if let Some(out_png) = headless_out {
        println!("Running in Headless GPU Mode -> {}", out_png);
        if let Err(e) = hackgu_render::headless::render_with_weapons(
            ccs_path.as_ref(),
            weapon_path.as_ref(),
            stage_path.as_ref(),
            out_png,
            1440,
            1440,
            None,
            anim_idx,
            frame_val,
        ) {
            eprintln!("Error rendering offscreen GPU image: {}", e);
            std::process::exit(1);
        }
        return;
    }

    println!("============================================================");
    println!("     .hack//G.U. Last Recode - Real-Time Engine Studio      ");
    println!("============================================================");
    if explore {
        println!("🎮 Mode: Playable Mac Anu Exploration");
        println!("Character: Haseo (1st Form) | Weapons: Twin Blades");
        println!("Stage: Mac Anu Central Plaza (town01_00h.ccs)");
    } else if let Some(path) = &ccs_path {
        println!("Opening CCS Container: {}", path.display());
    }
    println!();
    println!("Controls:");
    println!("  W / A / S / D              : Run / Move Player in Camera Direction");
    println!("  Left Mouse Button + Drag   : Orbit Camera (360° Follow)");
    println!("  Right/Middle Mouse + Drag  : Pan Camera");
    println!("  Scroll Wheel               : Zoom In / Out");
    println!("  Tab                        : Toggle WASD Player Mode / Free Camera");
    println!("  1 - 5                      : Emote Animations (Idle, Walk, Run, Spell, Gesture)");
    println!("  [ / ]                      : Cycle Previous / Next Animation Track");
    println!("  P                          : Pause / Resume Animation Playback");
    println!("  , / .                      : Step 1 Frame Backward / Forward");
    println!("  B                          : Toggle Background (Sky / Silhouettes)");
    println!("  Spacebar                   : Reset Camera to Center");
    println!("  Escape                     : Exit Viewer");
    println!("============================================================");

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new(ccs_path, explore);
    event_loop.run_app(&mut app).unwrap();
}
