//! Retail `.hack//G.U.` event-script interpreter (`ev/*.csv`).
//!
//! The retail event scripts are Shift-JIS CSV tables.  Every row is
//! `label,_,_,comment/id,category,opcode,flag,arg0,arg1,...`.  The interpreter below
//! executes them on a 60 Hz timeline exactly like the original event VM does:
//!
//! * `カット(60)` starts a camera cut (keyframed eye/target/fov), `(61)` ends its staging block.
//! * `ウエイト(4)` blocks for N frames, `メッセージ(68/5/6)` blocks for its display time.
//! * `モデル(32)` places an actor, `(7)` selects a pose/expression, `(8)` plays a motion
//!   file animation, `(9)` walks an actor to a point, `(27)` turns an actor toward another.
//! * `画面効果(39/40)` are fade in / fade out.
//!
//! All coordinates in the scripts are centimetres; the render engine works in metres.

use glam::Vec3;
use std::collections::BTreeMap;
use std::path::Path;

/// Event VM tick rate (frames per second). Checked against the voice acting (2026-10-12): PM10100's first lines
/// show for 60/45/100/90 ticks (the message op's duration column) and their Japanese voices (v_evVoiceVol1.awb cues
/// 0-3) last 1.45/1.35/2.34/1.68 s. At 60 Hz every line would vanish before its voice ends; at 30 Hz each outlasts
/// it (2.0/1.5/3.3/3.0 s). The earlier 60 Hz (not sourced) played every event at double speed.
pub const EV_FPS: f32 = 30.0;

/// One camera keyframe of a `カット` op.
#[derive(Debug, Clone, PartialEq)]
pub struct CamKey {
    pub eye: Vec3,
    pub target: Vec3,
    pub roll: f32,
    pub fov_deg: f32,
    pub frame: f32,
    pub ease: bool,
}

/// Keyframed camera for one cut.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CamTrack {
    pub keys: Vec<CamKey>,
}

/// Evaluated camera pose (still in script centimetres).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CamPose {
    pub eye: Vec3,
    pub target: Vec3,
    pub roll: f32,
    pub fov_deg: f32,
}

impl CamTrack {
    /// Evaluate the camera at `frame` frames after the cut began.
    pub fn eval(&self, frame: f32) -> Option<CamPose> {
        let first = self.keys.first()?;
        if self.keys.len() == 1 || frame <= first.frame {
            return Some(CamPose { eye: first.eye, target: first.target, roll: first.roll, fov_deg: first.fov_deg });
        }
        for w in self.keys.windows(2) {
            let (a, b) = (&w[0], &w[1]);
            if frame <= b.frame {
                let span = (b.frame - a.frame).max(1.0);
                let mut t = ((frame - a.frame) / span).clamp(0.0, 1.0);
                if b.ease {
                    t = t * t * (3.0 - 2.0 * t);
                }
                return Some(CamPose {
                    eye: a.eye.lerp(b.eye, t),
                    target: a.target.lerp(b.target, t),
                    roll: a.roll + (b.roll - a.roll) * t,
                    fov_deg: a.fov_deg + (b.fov_deg - a.fov_deg) * t,
                });
            }
        }
        let l = self.keys.last()?;
        Some(CamPose { eye: l.eye, target: l.target, roll: l.roll, fov_deg: l.fov_deg })
    }

    /// Total length in frames.
    pub fn length(&self) -> f32 {
        self.keys.last().map(|k| k.frame).unwrap_or(0.0)
    }
}

/// A decoded script instruction.
#[derive(Debug, Clone, PartialEq)]
pub enum EvOp {
    Comment,
    CastRegister { actor: String, display: String, special: bool, kind: i32 },
    CastOp { op: u32, actor: String },
    Cut { label: String, cam: CamTrack },
    CutEnd,
    Wait(u32),
    Message { op: u32, id: Option<u32>, speaker: String, text: String, frames: u32, style: i32 },
    Place { actor: String, pos: Vec3, yaw_deg: f32, visible: bool },
    Pose { actor: String, name: String, blend: f32 },
    Motion { actor: String, anim: String, blend: f32 },
    MoveTo { actor: String, pos: Vec3 },
    TurnTo { actor: String, target: String, rate: f32 },
    Remove { actor: String, pos: Option<Vec3> },
    Fade { fade_in: bool, frames: u32, dir: String },
    ScreenFx { frames: u32 },
    LoadCcs { op: u32, actor: Option<String>, name: String },
    StageLoad { name: String },
    StageStream { op: u32, name: String },
    Bgm { op: u32, args: Vec<String> },
    Special { op: u32, args: Vec<String> },
    Other { op: u32, args: Vec<String> },
}

/// A fully parsed event script.
#[derive(Debug, Clone)]
pub struct EvScript {
    pub id: String,
    pub ops: Vec<EvOp>,
    /// Header row `ID,,[x|y|z],angle,...` (centimetres, radians). Meaning not yet confirmed: an event origin
    /// and rotation. PM10100: `[0|-260|0], 3.141593`.
    pub header_origin: Option<(Vec3, f32)>,
}

/// Script yaw convention (verified on PM10100 cut01, where Asta/Iyoten face Haseo): yaw 0 faces -Y and
/// positive angles turn counter-clockwise about +Z, i.e. facing = (sin yaw, -cos yaw).
pub fn yaw_towards(d: Vec3) -> f32 {
    d.x.atan2(-d.y).to_degrees()
}

fn parse_vec3(s: &str) -> Option<Vec3> {
    let t = s.trim().trim_start_matches('[').trim_end_matches(']');
    let mut it = t.split('|').map(|p| p.trim().parse::<f32>());
    Some(Vec3::new(it.next()?.ok()?, it.next()?.ok()?, it.next()?.ok()?))
}

fn parse_cam(s: &str) -> CamTrack {
    // Ver0003:N:[eye]:[tgt]:roll:fov:near:frame:interp:...   (7 tokens per key)
    // Ver0001:N:[eye]:[tgt]:fov:frame:interp:...             (5 tokens per key)
    let toks: Vec<&str> = s.split(':').collect();
    if toks.len() < 2 {
        return CamTrack::default();
    }
    let ver3 = toks[0] == "Ver0003";
    let n: usize = toks[1].parse().unwrap_or(0);
    let stride = if ver3 { 7 } else { 5 };
    let mut keys = Vec::new();
    let mut i = 2;
    for _ in 0..n {
        if i + stride > toks.len() {
            break;
        }
        let eye = parse_vec3(toks[i]);
        let target = parse_vec3(toks[i + 1]);
        let (roll, fov, frame, interp) = if ver3 {
            (
                toks[i + 2].parse().unwrap_or(0.0),
                toks[i + 3].parse().unwrap_or(45.0),
                toks[i + 5].parse().unwrap_or(0.0),
                toks[i + 6].parse::<u32>().unwrap_or(1),
            )
        } else {
            (0.0, toks[i + 2].parse().unwrap_or(45.0), toks[i + 3].parse().unwrap_or(0.0), toks[i + 4].parse::<u32>().unwrap_or(1))
        };
        if let (Some(eye), Some(target)) = (eye, target) {
            keys.push(CamKey { eye, target, roll, fov_deg: fov, frame, ease: interp & 0x2000 != 0 });
        }
        i += stride;
    }
    CamTrack { keys }
}

/// Split the CSV text into logical records. Message text may contain raw (unquoted)
/// newlines, so a message row that is missing its trailing columns swallows the next line.
fn logical_rows(text: &str) -> Vec<Vec<String>> {
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut pending: Option<String> = None;
    for line in text.split('\n') {
        let line = line.trim_end_matches('\r');
        let candidate = match pending.take() {
            Some(prev) => format!("{}\n{}", prev, line),
            None => line.to_string(),
        };
        let fields: Vec<&str> = candidate.split(',').collect();
        let op: u32 = fields.get(5).and_then(|s| s.trim().parse().ok()).unwrap_or(0);
        let is_msg_row = matches!(op, 5 | 6 | 68 | 125 | 127) && fields.get(4).map(|c| *c == "メッセージ").unwrap_or(false);
        if is_msg_row && fields.len() < 10 {
            pending = Some(candidate);
            continue;
        }
        rows.push(candidate.split(',').map(|s| s.to_string()).collect());
    }
    rows
}

impl EvScript {
    /// Parse already-decoded (UTF-8) script text.
    pub fn parse_text(id: &str, text: &str) -> EvScript {
        let mut ops = Vec::new();
        for row in logical_rows(text) {
            let f = |i: usize| -> &str { row.get(i).map(|s| s.as_str()).unwrap_or("") };
            let op: u32 = match f(5).trim().parse() {
                Ok(o) => o,
                Err(_) => continue,
            };
            // Column 3 = 1 disables a row (PM10120 keeps 49 such rows: earlier goblin placements and empty lines
            // that are re-issued unflagged right after).
            if f(2).trim() == "1" {
                continue;
            }
            let cat = f(4);
            let flag_spc = f(6) == "SPC";
            let a = |i: usize| f(6 + i).to_string(); // args after the flag column
            let args: Vec<String> = (7..row.len()).map(|i| row[i].clone()).filter(|s| !s.is_empty()).collect();
            let eop = match (cat, op) {
                ("コメント", _) => EvOp::Comment,
                ("カット", 60) => {
                    let cam = row.iter().skip(6).find(|s| s.starts_with("Ver000")).map(|s| parse_cam(s)).unwrap_or_default();
                    EvOp::Cut { label: f(0).to_string(), cam }
                }
                ("カット", 61) => EvOp::CutEnd,
                ("ウエイト", 4) => EvOp::Wait(f(7).trim().parse().unwrap_or(0)),
                ("メッセージ", 68) | ("メッセージ", 5) | ("メッセージ", 6) => {
                    // Columns after the op: flag, speaker, text (op 5: message-table row), duration in event ticks,
                    // style. Op 5 used to read the style as the duration (PM10110: "0,120,1" showed each line for
                    // one tick, so field scenes raced through their lines).
                    let (speaker, text, frames, style) = (a(1), a(2), f(9).trim().parse().unwrap_or(if op == 5 { 150 } else { 120 }), f(10).trim().parse().unwrap_or(0));
                    let id = f(3).trim().parse().ok();
                    EvOp::Message { op, id, speaker, text, frames, style }
                }
                ("モデル", 32) => EvOp::Place {
                    actor: a(1),
                    pos: parse_vec3(f(8)).unwrap_or(Vec3::ZERO),
                    yaw_deg: f(9).trim().parse().unwrap_or(0.0),
                    visible: f(10) != "OFF",
                },
                ("モデル", 7) => EvOp::Pose { actor: a(1), name: a(2), blend: f(9).trim().parse().unwrap_or(0.0) },
                ("モデル", 8) => EvOp::Motion { actor: a(1), anim: a(2), blend: f(9).trim().parse().unwrap_or(0.0) },
                ("モデル", 9) => EvOp::MoveTo { actor: a(1), pos: parse_vec3(f(8)).unwrap_or(Vec3::ZERO) },
                ("モデル", 27) => EvOp::TurnTo { actor: a(1), target: a(2), rate: f(9).trim().parse().unwrap_or(0.5) },
                ("モデル", 21) => EvOp::Remove { actor: a(1), pos: parse_vec3(f(8)) },
                ("画面効果", 39) => EvOp::Fade { fade_in: true, frames: f(8).trim().parse().unwrap_or(30), dir: f(9).to_string() },
                ("画面効果", 40) => EvOp::Fade { fade_in: false, frames: f(8).trim().parse().unwrap_or(30), dir: f(9).to_string() },
                ("画面効果", _) => EvOp::ScreenFx { frames: f(8).trim().parse().unwrap_or(0) },
                ("キャスト", 63) => EvOp::CastRegister {
                    actor: a(1),
                    display: a(2),
                    special: flag_spc,
                    kind: f(9).trim().parse().unwrap_or(0),
                },
                ("キャスト", o) => EvOp::CastOp { op: o, actor: a(1) },
                ("CCS操作", 50) | ("CCS操作", 51) | ("CCS操作", 53) | ("CCS操作", 55) | ("CCS操作", 56) | ("CCS操作", 58) => {
                    EvOp::LoadCcs { op, actor: None, name: a(1) }
                }
                ("CCS操作", 52) | ("CCS操作", 57) => EvOp::LoadCcs { op, actor: Some(a(1)), name: a(2) },
                ("ストリーム", 33) => EvOp::StageLoad { name: a(1) },
                ("ストリーム", o) => EvOp::StageStream { op: o, name: a(1) },
                ("BGM操作", _) => EvOp::Bgm { op, args },
                ("特殊操作", _) => EvOp::Special { op, args },
                _ => EvOp::Other { op, args },
            };
            ops.push(eop);
        }
        let header_origin = text.lines().next().and_then(|l| {
            let f: Vec<&str> = l.split(',').collect();
            Some((parse_vec3(f.get(2)?)?, f.get(3)?.trim().parse().ok()?))
        });
        EvScript { id: id.to_string(), ops, header_origin }
    }

    /// Load `<ev_dir>/<id>.csv` (Shift-JIS).
    pub fn load(ev_dir: &Path, id: &str) -> Result<EvScript, String> {
        let path = ev_dir.join(format!("{}.csv", id));
        let bytes = std::fs::read(&path).map_err(|e| format!("{}: {}", path.display(), e))?;
        Ok(Self::parse_text(id, &decode_sjis(&bytes)))
    }

    /// Ordered list of distinct actor names referenced by `Place` ops.
    pub fn placed_actors(&self) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for op in &self.ops {
            if let EvOp::Place { actor, .. } = op {
                if !out.contains(actor) {
                    out.push(actor.clone());
                }
            }
        }
        out
    }

    /// Stage CCS names requested by `ストリーム(33)` ops.
    pub fn stages(&self) -> Vec<String> {
        self.ops
            .iter()
            .filter_map(|o| if let EvOp::StageLoad { name } = o { Some(name.clone()) } else { None })
            .collect()
    }

    /// Motion CCS names (`CCS操作 52`) to load for each actor.
    pub fn motion_files(&self) -> Vec<(String, String)> {
        let mut out: Vec<(String, String)> = Vec::new();
        for op in &self.ops {
            if let EvOp::LoadCcs { op: 52, actor: Some(a), name } = op {
                let pair = (a.clone(), name.clone());
                if !out.contains(&pair) {
                    out.push(pair);
                }
            }
        }
        out
    }
}

/// Decode Shift-JIS (cp932) bytes to UTF-8 using `encoding_rs`.
pub fn decode_sjis(bytes: &[u8]) -> String {
    let (cow, _, _) = encoding_rs::SHIFT_JIS.decode(bytes);
    cow.into_owned()
}

/// Strip `<colorRRGGBB>`, `<iconXXX>` tags and resolve `<ruby 漢字|よみ>` to the base text.
pub fn strip_markup(s: &str) -> String {
    let mut out = String::new();
    let mut rest = s;
    while let Some(start) = rest.find('<') {
        out.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        match after.find('>') {
            Some(end) => {
                let tag = &after[..end];
                if let Some(body) = tag.strip_prefix("ruby") {
                    if let Some((base, _)) = body.split_once('|') {
                        out.push_str(base);
                    }
                } else if tag.starts_with("icon") {
                    out.push_str(match &tag[4..] {
                        "TRIANGLE" => "△",
                        "CIRCLE" => "○",
                        "CROSS" => "×",
                        "SQUARE" => "□",
                        _ => "",
                    });
                }
                rest = &after[end + 1..];
            }
            None => {
                out.push('<');
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}

// --------------------------------------------------------------------------------------
// Executor
// --------------------------------------------------------------------------------------

/// Live state of one scripted actor.
#[derive(Debug, Clone)]
pub struct ActorState {
    pub pos: Vec3,
    /// `pos` advanced by the fraction of an event tick that has elapsed (for drawing: walks look smooth at any frame
    /// rate instead of stepping 30 times a second).
    pub smooth_pos: Vec3,
    pub yaw_deg: f32,
    pub visible: bool,
    /// Current pose / expression name (Japanese script name, e.g. `ニュートラル`).
    pub pose: String,
    /// Currently playing motion file animation (e.g. `ANM_2hswm10100_08a`), if any.
    pub motion: Option<String>,
    /// Bumped whenever the render side has to restart the actor's animation.
    pub anim_serial: u32,
    /// Crossfade length (event ticks) requested by the last pose/motion op (its blend column).
    pub blend_frames: f32,
    pub move_target: Option<Vec3>,
    pub moving: bool,
}

/// Dialogue currently on screen.
#[derive(Debug, Clone)]
pub struct MessageState {
    /// Message number from the script's id column (PM10100: 1..N; voice cue = N-1).
    pub id: Option<u32>,
    pub speaker: String,
    pub text: String,
    pub frames_total: u32,
    pub frames_left: f32,
    pub style: i32,
    /// Script op: 6 = notice ("Acquired ...'s Member Address!", drawn in a band mid-screen), else a spoken line.
    pub op: u32,
}

/// Everything the renderer needs to draw the current event frame.
#[derive(Debug, Clone, Default)]
pub struct EvFrame {
    pub camera: Option<CamPose>,
    pub actors: BTreeMap<String, ActorState>,
    pub message: Option<MessageState>,
    /// 0 = fully visible, 1 = fully black.
    pub black: f32,
    pub stage: Option<String>,
    pub cut_label: String,
    pub finished: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Block {
    None,
    Wait(f32),
    Message,
}

/// Walking speed used for `モデル(9)` moves (cm / s).
const MOVE_SPEED_CM_S: f32 = 260.0;

/// Time-driven event interpreter.
pub struct EvPlayer {
    pub script: EvScript,
    pc: usize,
    block: Block,
    cut: CamTrack,
    cut_frame: f32,
    frame_accum: f32,
    fade_from: f32,
    fade_to: f32,
    fade_len: f32,
    fade_t: f32,
    pub state: EvFrame,
    pub total_frames: f32,
    /// Playback speed multiplier (1.0 = retail speed).
    pub speed: f32,
    pub msg_table: Option<std::collections::HashMap<u32, crate::msgtable::MessageEntry>>,
    /// Localized message database (e.g. English). When set, lines are looked up through their voice key.
    pub lang: Option<std::sync::Arc<crate::msginfo::MessageInfo>>,
    /// While true, a displayed message does not time out (the renderer holds it while its voice plays).
    pub hold_message: bool,
}

impl EvPlayer {
    pub fn new(script: EvScript) -> Self {
        Self::new_with_ev_dir(script, None)
    }

    pub fn new_with_ev_dir(script: EvScript, ev_dir: Option<&Path>) -> Self {
        let msg_table = ev_dir.and_then(|dir| crate::msgtable::load_message_entries(dir, &script.id));
        let mut p = EvPlayer {
            script,
            pc: 0,
            block: Block::None,
            cut: CamTrack::default(),
            cut_frame: 0.0,
            frame_accum: 0.0,
            fade_from: 1.0,
            fade_to: 1.0,
            fade_len: 0.0,
            fade_t: 0.0,
            state: EvFrame { black: 1.0, ..Default::default() },
            total_frames: 0.0,
            speed: 1.0,
            msg_table,
            hold_message: false,
            lang: None,
        };
        p.run_ops();
        p
    }

    /// Find the speaker and text to display for a script message. Lines are matched to their row in the
    /// `BIN_<id>` table (by number for numeric references, otherwise by Japanese text, falling back to row
    /// N-1), whose voice id `<prefix><key>` (e.g. `spcName_019_A` + `MainScenario00002`) keys both the
    /// localized line (`key`) and the speaker name (`prefix`) in messageInfo.bin. Markup is kept.
    fn resolve_message(&self, speaker: &str, text: &str, id: Option<u32>) -> (String, String) {
        let entry = self.msg_table.as_ref().and_then(|table| {
            if let Ok(n) = text.trim().parse::<u32>() {
                return table.get(&n);
            }
            let want = strip_markup(text);
            table.values().find(|e| strip_markup(&e.text) == want).or_else(|| id.and_then(|n| table.get(&n.saturating_sub(1))))
        });
        let (mut spk, mut txt) = match entry {
            Some(e) => (if e.speaker.is_empty() { speaker.to_string() } else { e.speaker.clone() }, e.text.clone()),
            None => (speaker.to_string(), text.to_string()),
        };
        let mut named = false;
        if let (Some(lang), Some(e)) = (&self.lang, entry) {
            if let Some(key) = crate::msginfo::scenario_key(&e.voice) {
                if let Some(t) = lang.get(key) {
                    txt = t.to_string();
                }
                let prefix = &e.voice[..e.voice.len() - key.len()];
                if let Some(n) = lang.get(prefix) {
                    spk = n.to_string();
                    named = true;
                }
            }
        }
        if !named {
            if let Some(lang) = &self.lang {
                spk = english_speaker(lang, &spk);
            }
        }
        (spk, txt)
    }

    /// Index of the next op to execute.
    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn is_finished(&self) -> bool {
        self.state.finished
    }

    /// Player pressed the confirm button: finish the current message early.
    pub fn press_advance(&mut self) {
        if self.block == Block::Message {
            self.state.message = None;
            self.block = Block::None;
            self.run_ops();
        }
    }

    /// Jump straight to the end of the script (skip).
    pub fn skip_all(&mut self) {
        self.pc = self.script.ops.len();
        self.block = Block::None;
        self.state.message = None;
        self.state.finished = true;
        self.state.black = 1.0;
    }

    /// Advance the VM by `dt` seconds of wall-clock time.
    pub fn update(&mut self, dt: f32) {
        if self.state.finished {
            return;
        }
        self.frame_accum += dt.min(0.25) * EV_FPS * self.speed;
        while self.frame_accum >= 1.0 && !self.state.finished {
            self.frame_accum -= 1.0;
            self.tick_frame();
        }
        // Between ticks, the camera and walking actors are evaluated at the fractional tick so they move smoothly.
        let frac = self.frame_accum.clamp(0.0, 1.0);
        if let Some(pose) = self.cut.eval(self.cut_frame + frac) {
            self.state.camera = Some(pose);
        }
        let step = MOVE_SPEED_CM_S / EV_FPS;
        for a in self.state.actors.values_mut() {
            a.smooth_pos = match a.move_target {
                Some(t) => {
                    let d = t - a.pos;
                    let len = d.length();
                    if len > 1e-4 { a.pos + d / len * (step * frac).min(len) } else { a.pos }
                }
                None => a.pos,
            };
        }
    }

    fn tick_frame(&mut self) {
        self.total_frames += 1.0;
        self.cut_frame += 1.0;

        // fade
        if self.fade_t < self.fade_len {
            self.fade_t += 1.0;
            let t = (self.fade_t / self.fade_len.max(1.0)).clamp(0.0, 1.0);
            self.state.black = self.fade_from + (self.fade_to - self.fade_from) * t;
        }

        // actor movement
        let step = MOVE_SPEED_CM_S / EV_FPS;
        for a in self.state.actors.values_mut() {
            if let Some(t) = a.move_target {
                let d = t - a.pos;
                let len = d.length();
                if len <= step {
                    a.pos = t;
                    a.move_target = None;
                    a.moving = false;
                } else {
                    a.pos += d / len * step;
                    a.moving = true;
                    a.yaw_deg = yaw_towards(d);
                }
            }
        }

        match self.block {
            Block::Wait(left) => {
                if left <= 1.0 {
                    self.block = Block::None;
                } else {
                    self.block = Block::Wait(left - 1.0);
                }
            }
            Block::Message => {
                if let Some(m) = self.state.message.as_mut() {
                    if !self.hold_message || m.frames_left > 1.0 {
                        m.frames_left -= 1.0;
                    }
                    if m.frames_left <= 0.0 {
                        self.state.message = None;
                        self.block = Block::None;
                    }
                } else {
                    self.block = Block::None;
                }
            }
            Block::None => {}
        }
        if self.block == Block::None {
            self.run_ops();
        }
    }

    fn actor_mut(&mut self, name: &str) -> &mut ActorState {
        self.state.actors.entry(name.to_string()).or_insert_with(|| ActorState {
            pos: Vec3::ZERO,
            smooth_pos: Vec3::ZERO,
            yaw_deg: 0.0,
            visible: false,
            pose: "ニュートラル".into(),
            motion: None,
            anim_serial: 0,
            blend_frames: 0.0,
            move_target: None,
            moving: false,
        })
    }

    fn run_ops(&mut self) {
        while self.block == Block::None && self.pc < self.script.ops.len() {
            let op = self.script.ops[self.pc].clone();
            self.pc += 1;
            match op {
                EvOp::Cut { label, cam } => {
                    self.cut = cam;
                    self.cut_frame = 0.0;
                    self.state.cut_label = label;
                    if let Some(p) = self.cut.eval(0.0) {
                        self.state.camera = Some(p);
                    }
                }
                EvOp::Wait(n) => {
                    if n > 0 {
                        self.block = Block::Wait(n as f32);
                    }
                }
                EvOp::Message { op, id, speaker, text, frames, style } => {
                    let (resolved_spk, resolved_txt) = self.resolve_message(&speaker, &text, id);
                    self.state.message = Some(MessageState {
                        id,
                        speaker: resolved_spk,
                        text: resolved_txt,
                        frames_total: frames,
                        frames_left: frames as f32,
                        style,
                        op,
                    });
                    self.block = Block::Message;
                }
                EvOp::Place { actor, pos, yaw_deg, visible } => {
                    let a = self.actor_mut(&actor);
                    a.pos = pos;
                    a.smooth_pos = pos;
                    a.yaw_deg = yaw_deg;
                    a.visible = visible;
                    a.move_target = None;
                    a.moving = false;
                }
                EvOp::Pose { actor, name, blend } => {
                    let a = self.actor_mut(&actor);
                    a.blend_frames = blend;
                    if a.pose != name || a.motion.is_some() {
                        a.anim_serial += 1;
                    }
                    a.pose = name;
                    a.motion = None;
                }
                EvOp::Motion { actor, anim, blend } => {
                    let a = self.actor_mut(&actor);
                    a.blend_frames = blend;
                    a.motion = Some(anim);
                    a.anim_serial += 1;
                }
                EvOp::MoveTo { actor, pos } => {
                    let a = self.actor_mut(&actor);
                    a.move_target = Some(pos);
                }
                EvOp::TurnTo { actor, target, .. } => {
                    let tp = self.state.actors.get(&target).map(|t| t.pos);
                    if let Some(tp) = tp {
                        let a = self.actor_mut(&actor);
                        let d = tp - a.pos;
                        a.yaw_deg = yaw_towards(d);
                    }
                }
                EvOp::Remove { actor, .. } => {
                    self.actor_mut(&actor).visible = false;
                }
                EvOp::CastOp { op, actor } => {
                    if matches!(op, 96 | 99) {
                        if let Some(a) = self.state.actors.get_mut(&actor) {
                            a.visible = false;
                        }
                    }
                }
                EvOp::Fade { fade_in, frames, .. } => {
                    self.fade_from = self.state.black;
                    self.fade_to = if fade_in { 0.0 } else { 1.0 };
                    self.fade_len = frames.max(1) as f32;
                    self.fade_t = 0.0;
                }
                EvOp::StageLoad { name } => {
                    self.state.stage = Some(name);
                }
                _ => {}
            }
        }
        if self.pc >= self.script.ops.len() && self.block == Block::None {
            self.state.finished = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev_dir() -> Option<std::path::PathBuf> {
        for base in [".", "..", "../..", "../../.."] {
            let p = Path::new(base).join("target/vol1/data/data/ev");
            if p.join("PM10100.csv").exists() {
                return Some(p);
            }
        }
        None
    }

    #[test]
    fn markup_is_stripped() {
        assert_eq!(strip_markup("これが<ruby審神者|さにわ>の神"), "これが審神者の神");
        assert_eq!(strip_markup("<color76E1FF>『The World』<colorFFFFFF>でメニュー<iconTRIANGLE>"), "『The World』でメニュー△");
    }

    #[test]
    fn camera_track_parses_and_interpolates() {
        let t = parse_cam("Ver0003:2:[0.00|0.00|0.00]:[100.00|0.00|0.00]:0.00:60.00:5.00:0:1:[100.00|0.00|0.00]:[100.00|0.00|0.00]:0.00:30.00:5.00:100:1:");
        assert_eq!(t.keys.len(), 2);
        let mid = t.eval(50.0).unwrap();
        assert!((mid.eye.x - 50.0).abs() < 1e-3);
        assert!((mid.fov_deg - 45.0).abs() < 1e-3);
        let v1 = parse_cam("Ver0001:2:[1|2|3]:[4|5|6]:50.00:0:1:[7|8|9]:[1|1|1]:50.00:399:1:");
        assert_eq!(v1.keys.len(), 2);
        assert_eq!(v1.length(), 399.0);
    }

    #[test]
    fn all_retail_scripts_parse_and_run_to_completion() {
        let Some(dir) = ev_dir() else { return };
        for entry in std::fs::read_dir(&dir).unwrap() {
            let p = entry.unwrap().path();
            if p.extension().and_then(|e| e.to_str()) != Some("csv") {
                continue;
            }
            let id = p.file_stem().unwrap().to_string_lossy().to_string();
            let script = EvScript::load(&dir, &id).unwrap();
            assert!(!script.ops.is_empty(), "{} parsed empty", id);
            let mut player = EvPlayer::new(script);
            player.speed = 1.0;
            let mut guard = 0;
            while !player.is_finished() && guard < 60 * 60 * 20 {
                player.update(1.0 / 60.0);
                guard += 1;
            }
            assert!(player.is_finished(), "{} never finished", id);
        }
    }

    #[test]
    fn camera_moves_between_ticks() {
        let Some(dir) = ev_dir() else { return };
        let script = EvScript::load(&dir, "PM10100").unwrap();
        let mut p = EvPlayer::new_with_ev_dir(script, Some(&dir));
        for _ in 0..240 {
            p.update(1.0 / 120.0);
        }
        // At 120 frames/s there are ~4 frames per event tick; a moving camera must change on every one of them.
        let mut changes = 0;
        let mut last = p.state.camera.map(|c| c.eye);
        for _ in 0..40 {
            p.update(1.0 / 120.0);
            let e = p.state.camera.map(|c| c.eye);
            if e != last {
                changes += 1;
            }
            last = e;
        }
        assert!(changes > 30, "camera changed on only {} of 40 frames", changes);
    }

    #[test]
    fn pm10100_has_the_real_tutorial_dialogue_and_cast() {
        let Some(dir) = ev_dir() else { return };
        let script = EvScript::load(&dir, "PM10100").unwrap();
        let cuts = script.ops.iter().filter(|o| matches!(o, EvOp::Cut { .. })).count();
        assert!(cuts >= 20, "expected the full cut list, got {}", cuts);
        let msgs: Vec<_> = script
            .ops
            .iter()
            .filter_map(|o| if let EvOp::Message { speaker, text, .. } = o { Some((speaker.clone(), text.clone())) } else { None })
            .collect();
        assert!(msgs.iter().any(|(s, t)| s == "ハセヲ" && t.contains("わぁ")));
        assert!(msgs.iter().any(|(_, t)| t.contains("今日が初めて")), "multi-line message must be merged");
        let actors = script.placed_actors();
        assert!(actors.contains(&"ハセヲ".to_string()) && actors.contains(&"アスタ".to_string()));
        assert!(script.stages().contains(&"m10100_s01.ccs".to_string()));
        // total running time should be a couple of minutes, not seconds
        let mut p = EvPlayer::new(script);
        let mut frames = 0u32;
        while !p.is_finished() && frames < 60 * 600 {
            p.update(1.0 / 60.0);
            frames += 1;
        }
        assert!(frames > 60 * 60, "PM10100 ran only {} frames", frames);
    }
}

/// English name for a speaker label that had no voice entry of its own: the name other lines with the same Japanese
/// label use (learned from every message table's `spcName_NNN_A` voice prefix), else the label with full-width letters
/// made plain and matched case-insensitively to a known English name ("Ｉｙｏｔｅｎ" -> "IYOTEN").
pub fn english_speaker(lang: &crate::msginfo::MessageInfo, label: &str) -> String {
    use std::collections::HashMap;
    use std::sync::OnceLock;
    static NAMES: OnceLock<HashMap<String, String>> = OnceLock::new();
    let names = NAMES.get_or_init(|| {
        let mut m: HashMap<String, String> = HashMap::new();
        for e in crate::msgtable::all_entries(Path::new("target/vol1/data/data/ev")) {
            let Some(key) = crate::msginfo::scenario_key(&e.voice) else { continue };
            let prefix = &e.voice[..e.voice.len() - key.len()];
            if let Some(n) = lang.get(prefix) {
                m.entry(e.speaker.clone()).or_insert_with(|| n.to_string());
            }
        }
        m
    });
    if let Some(n) = names.get(label) {
        return n.clone();
    }
    let plain = crate::msgtable::halfwidth(label);
    if let Some(n) = names.values().find(|n| n.eq_ignore_ascii_case(&plain)) {
        return n.clone();
    }
    // GUESS: generic extras (ＰＣ女１...) have no English name in the data; show no name plate rather than a
    // half-Japanese label.
    if plain.chars().any(|c| !c.is_ascii()) {
        String::new()
    } else {
        plain
    }
}
