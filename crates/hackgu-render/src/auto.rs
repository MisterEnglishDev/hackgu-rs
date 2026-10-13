//! Command-line automation for verifying the running game: capture the real swapchain image after N frames,
//! pin the camera, place the player, or jump straight into an event script.
//!
//! `hackgu-render --shot out.png[@frame] [--shot more.png@frame ...] [--cam ex,ey,ez,tx,ty,tz[,fov]]
//!                [--pos x,y,z[,yaw_deg]] [--ev PM10100] [--ev-seek seconds] [--noui] [--hold w|a|s|d]`
//! Coordinates are metres in the game's frame (X right, Y forward, Z up). The process exits after the last shot.

use std::sync::OnceLock;

#[derive(Default, Debug, Clone)]
pub struct AutoConfig {
    /// (path, frame) pairs, sorted by frame.
    pub shots: Vec<(String, u32)>,
    /// Fixed camera eye/target in metres, plus optional vertical FOV in degrees.
    pub cam: Option<([f32; 3], [f32; 3], Option<f32>)>,
    /// Player position (metres) and optional yaw (degrees).
    pub pos: Option<([f32; 3], Option<f32>)>,
    pub ev: Option<String>,
    /// `<seconds>` or `<cutLabel>[+frames]`: fast-forward the event before the first shot.
    pub ev_seek: String,
    /// `--evframe cut:ox,oy,oz,yaw_deg` (metres, degrees; cut `*` = all cuts) overrides the event frame.
    pub evframe: Vec<(String, [f32; 4])>,
    /// Do not play voices or hold messages for them (keeps seeking deterministic).
    pub mute: bool,
    /// Text language: "eng" (default) or "jpn".
    pub lang: String,
    /// Play this movie (e.g. OP10000.usm) at startup.
    pub movie: Option<String>,
    /// Title screen state for screenshots: `select`, `menu`, `newgame`.
    pub title: String,
    pub no_ui: bool,
    /// Keys held for the whole run (movement test).
    pub hold: String,
    /// Skip the title screen and go to town exploration.
    pub town: bool,
    /// Start in the Holy Ground tutorial field.
    pub field: bool,
    /// `--keys f:Key,f:Key,...`: press egui keys (Enter, Escape, ArrowUp/Down/Left/Right, Space) on these frames.
    pub keys: Vec<(u32, String)>,
    /// `--area path.json`: use this area definition (any path) for --field.
    pub area: Option<String>,
    /// `--stage file.ccs`: load this stage file (generic loader) with Haseo.
    pub stage: Option<String>,
    /// `--dungeon path.json[:floor]`: load a story dungeon layout (assets/dungeons, tools/dungeon_layout.py).
    pub dungeon: Option<(String, usize)>,
    /// `--press f1,f2,...`: press "A" (attack/confirm) on these frames.
    pub press: Vec<u32>,
    /// `--gate gate|warp|word|word3|confirm`: open the Chaos Gate menus in that state (from Mac Anu).
    pub gate: String,
    /// `--desktop N`: start on the ALTIMIT desktop with application N (ring order, 0 = The World) selected.
    pub desktop: Option<usize>,
    /// `--reboot`: open the desktop with the post-MM10145 reboot sequence.
    pub reboot: bool,
    /// `--record dir:count`: record `count` consecutive frames to `dir/frame_%05d.png`.
    pub record: Option<(String, u32)>,
    /// `--res WxH`: override window/swapchain resolution (defaults to 1280x800, as before the 2K change).
    pub res: Option<(u32, u32)>,
}

/// Video frame rate of `--record` output: the game advances exactly 1/RECORD_FPS s per recorded frame.
pub const RECORD_FPS: f32 = 30.0;

static AUTO: OnceLock<AutoConfig> = OnceLock::new();

pub fn get() -> &'static AutoConfig {
    AUTO.get_or_init(AutoConfig::default)
}

fn floats(s: &str) -> Vec<f32> {
    s.split(',').filter_map(|v| v.trim().parse().ok()).collect()
}

/// Parse and remove automation flags from `args`.
pub fn init(args: &mut Vec<String>) {
    let mut cfg = AutoConfig::default();
    let mut out = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let a = args[i].as_str();
        let next = args.get(i + 1).cloned().unwrap_or_default();
        match a {
            "--shot" => {
                let (p, f) = match next.rsplit_once('@') {
                    Some((p, f)) if f.parse::<u32>().is_ok() => (p.to_string(), f.parse().unwrap()),
                    _ => (next.clone(), 120),
                };
                cfg.shots.push((p, f));
                i += 1;
            }
            "--frames" => {
                if let (Some(last), Ok(f)) = (cfg.shots.last_mut(), next.parse()) {
                    last.1 = f;
                }
                i += 1;
            }
            "--cam" => {
                let v = floats(&next);
                if v.len() >= 6 {
                    cfg.cam = Some(([v[0], v[1], v[2]], [v[3], v[4], v[5]], v.get(6).copied()));
                }
                i += 1;
            }
            "--pos" => {
                let v = floats(&next);
                if v.len() >= 3 {
                    cfg.pos = Some(([v[0], v[1], v[2]], v.get(3).copied()));
                }
                i += 1;
            }
            "--ev" => {
                cfg.ev = Some(next.clone());
                i += 1;
            }
            "--ev-seek" => {
                cfg.ev_seek = next.clone();
                i += 1;
            }
            "--evframe" => {
                if let Some((cut, v)) = next.split_once(':') {
                    let v = floats(v);
                    if v.len() >= 4 {
                        cfg.evframe.push((cut.to_string(), [v[0], v[1], v[2], v[3]]));
                    }
                }
                i += 1;
            }
            "--mute" => cfg.mute = true,
            "--title" => {
                cfg.title = next.clone();
                i += 1;
            }
            "--movie" => {
                cfg.movie = Some(next.clone());
                i += 1;
            }
            "--lang" => {
                cfg.lang = next.clone();
                i += 1;
            }
            "--hold" => {
                cfg.hold = next.clone();
                i += 1;
            }
            "--noui" => cfg.no_ui = true,
            "--town" => cfg.town = true,
            "--field" => cfg.field = true,
            "--keys" => {
                cfg.keys = next.split(',').filter_map(|e| e.split_once(':')).filter_map(|(f, k)| Some((f.trim().parse().ok()?, k.trim().to_string()))).collect();
                i += 1;
            }
            "--area" => {
                cfg.area = Some(next.clone());
                i += 1;
            }
            "--stage" => {
                cfg.stage = Some(next.clone());
                i += 1;
            }
            "--dungeon" => {
                let (p, f) = next.rsplit_once(':').filter(|(_, f)| f.parse::<usize>().is_ok()).map(|(p, f)| (p.to_string(), f.parse().unwrap())).unwrap_or((next.clone(), 0));
                cfg.dungeon = Some((p, f));
                i += 1;
            }
            "--press" => {
                cfg.press = next.split(',').filter_map(|v| v.trim().parse().ok()).collect();
                i += 1;
            }
            "--gate" => {
                cfg.gate = next.clone();
                i += 1;
            }
            "--desktop" => {
                cfg.desktop = Some(next.parse().unwrap_or(0));
                i += 1;
            }
            "--reboot" => cfg.reboot = true,
            "--record" => {
                if let Some((dir, count)) = next.rsplit_once(':') {
                    if let Ok(c) = count.parse::<u32>() {
                        cfg.record = Some((dir.to_string(), c));
                    }
                }
                i += 1;
            }
            "--res" => {
                let parts: Vec<u32> = next.split(|c: char| c == 'x' || c == ',' || c == 'X')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();
                if parts.len() == 2 {
                    cfg.res = Some((parts[0], parts[1]));
                }
                i += 1;
            }
            _ => out.push(args[i].clone()),
        }
        i += 1;
    }
    if cfg.lang.is_empty() {
        cfg.lang = "eng".into();
    }
    cfg.shots.sort_by_key(|s| s.1);
    *args = out;
    let _ = AUTO.set(cfg);
}

/// Write an RGBA8 image as PNG (stored/uncompressed deflate; no external crates).
pub fn write_png(path: &str, width: u32, height: u32, rgba: &[u8]) -> std::io::Result<()> {
    use std::io::Write;
    fn crc32(data: &[u8]) -> u32 {
        let mut c = 0xFFFF_FFFFu32;
        for &b in data {
            c ^= b as u32;
            for _ in 0..8 {
                c = if c & 1 != 0 { 0xEDB8_8320 ^ (c >> 1) } else { c >> 1 };
            }
        }
        !c
    }
    fn chunk(out: &mut Vec<u8>, ty: &[u8; 4], data: &[u8]) {
        out.extend_from_slice(&(data.len() as u32).to_be_bytes());
        let mut c = ty.to_vec();
        c.extend_from_slice(data);
        out.extend_from_slice(&c);
        out.extend_from_slice(&crc32(&c).to_be_bytes());
    }
    let stride = width as usize * 4;
    let mut raw = Vec::with_capacity((stride + 1) * height as usize);
    for y in 0..height as usize {
        raw.push(0);
        raw.extend_from_slice(&rgba[y * stride..(y + 1) * stride]);
    }
    // zlib stream with stored blocks
    let mut z = vec![0x78, 0x01];
    for (k, blk) in raw.chunks(65535).enumerate() {
        let last = (k + 1) * 65535 >= raw.len();
        z.push(last as u8);
        z.extend_from_slice(&(blk.len() as u16).to_le_bytes());
        z.extend_from_slice(&(!(blk.len() as u16)).to_le_bytes());
        z.extend_from_slice(blk);
    }
    let (mut a, mut b) = (1u32, 0u32);
    for &x in &raw {
        a = (a + x as u32) % 65521;
        b = (b + a) % 65521;
    }
    z.extend_from_slice(&((b << 16) | a).to_be_bytes());
    let mut out = b"\x89PNG\r\n\x1a\n".to_vec();
    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
    chunk(&mut out, b"IHDR", &ihdr);
    chunk(&mut out, b"IDAT", &z);
    chunk(&mut out, b"IEND", &[]);
    std::fs::File::create(path)?.write_all(&out)
}
