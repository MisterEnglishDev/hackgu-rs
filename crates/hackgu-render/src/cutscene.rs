//! Event playback driven only by the retail script (`ev/<ID>.csv`) through `hackgu_game::ev::EvPlayer`.
//!
//! Every frame the EvPlayer's `EvFrame` supplies the camera, the cast (placement, pose/motion, visibility),
//! fades, the current message and the stage request. This module turns that into scene state: it spawns
//! one actor per `キャスト(63)` registration, loads the animations the script asks for and maps script
//! coordinates into the town frame.
//!
//! Evidence used for the mappings below:
//! - Cast models: the script's own loads (`m10100_2hsw/2ast/2iyo` motion files are made for the
//!   `2hsw01/2ast01/2iyo01` skeletons: they drive 38/38, 37/37 and 34/34 bones).
//! - Pose names (`モデル(7)`): hackGU_vol1.dll holds a table of `{char name[96]; int code}` records at file
//!   offset 0x83bbb0 (ニュートラル=0, 走り=1, 歩き=2, 会話１=47 ... 振り返る(左180度)=64). Codes 47-64 are
//!   exactly the 18 emotes in `spc/ev/<chr>ev.ccs` (tlk/hpy/ang/sad/bad/sht/spr/trr/trl, variants 10/20).
//! - Yaw: 0 faces -Y, counter-clockwise positive (see `hackgu_game::ev::yaw_towards`).

use std::collections::HashMap;
use std::path::Path;

use glam::Vec3;
use hackgu_ccs::{CcsAnimation, CcsFile};
use hackgu_game::ev::{EvFrame, EvOp, EvPlayer, EvScript};

/// Where a cut's script coordinates live in the stage (metres / radians): world = origin + Rz(yaw) * local.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EvTransform {
    pub origin: Vec3,
    pub yaw: f32,
}

impl EvTransform {
    pub const RAW: EvTransform = EvTransform { origin: Vec3::ZERO, yaw: 0.0 };

    pub fn point(&self, cm: Vec3) -> Vec3 {
        let p = cm * 0.01;
        let (s, c) = self.yaw.sin_cos();
        self.origin + Vec3::new(c * p.x - s * p.y, s * p.x + c * p.y, p.z)
    }
}

/// Coordinate frame for each cut of a script. `overrides` (from `--evframe`) win.
pub fn frame_for_cut(script: &str, cut: &str, overrides: &[(String, EvTransform)]) -> EvTransform {
    if let Some((_, t)) = overrides.iter().rev().find(|(c, _)| c == cut || c == "*") {
        return *t;
    }
    match script {
        // Mac Anu opening: one frame for every cut, anchored at DMY_start (0, 228.5, 12 m) and turned 180 deg.
        // Matched against the original game's PM10100 (HANDOFF.md "COORDINATES"): the gate lands between the
        // cutA07 camera and Haseo, and cutA02's townspeople land on the central-plaza floor by its stairs.
        // Fallback when Mac Anu's locators are not loaded (normally app.rs places town events at DMY_ev<N>).
        "PM10100" => EvTransform { origin: Vec3::new(0.0, 228.5, 12.0), yaw: std::f32::consts::PI },
        _ => EvTransform::RAW,
    }
}

/// Map a pose name (script `モデル(7)`) to an animation-name suffix in the actor's animation set.
/// Returns (suffix, loops).
pub fn pose_anim(pose: &str) -> Option<(&'static str, bool)> {
    Some(match pose {
        "ニュートラル" => ("nut00", true),
        "走り" => ("run00", true),
        "歩き" => ("wlk00", true),
        "会話１" => ("evtlk10", true),
        "会話２" => ("evtlk20", true),
        "喜び１" => ("evhpy10", true),
        "喜び２" => ("evhpy20", true),
        "怒り１" => ("evang10", true),
        "怒り２" => ("evang20", true),
        "悲しむ１" => ("evsad10", true),
        "悲しむ２" => ("evsad20", true),
        "困る１" => ("evbad10", true),
        "困る２" => ("evbad20", true),
        "呼びかけ１" => ("evsht10", false),
        "呼びかけ２" => ("evsht20", false),
        "驚く１" => ("evspr10", false),
        "驚く２" => ("evspr20", false),
        "振り返る(右90度)" => ("evtrr10", false),
        "振り返る(左90度)" => ("evtrl10", false),
        "振り返る(右180度)" => ("evtrr20", false),
        "振り返る(左180度)" => ("evtrl20", false),
        _ => return None,
    })
}

/// Model file for a cast entry. SPC characters use their standard model; generic PCs (kind 11) are built
/// in the game from a base body plus colour sets per NPC (table not decoded yet), so the plain base body
/// stands in for them.
pub fn cast_model(actor: &str, display: &str, kind: i32) -> Option<(&'static str, &'static str)> {
    // (model file, animation base code)
    Some(match actor {
        "ハセヲ" => ("2hsw01.ccs", "2hsw"),
        "アスタ" => ("2ast01.ccs", "2ast"),
        "Iyoten" | "Ｉｙｏｔｅｎ" => ("2iyo01.ccs", "2iyo"),
        // Mac Anu events after the reformat (PM10315, PM10320). First-form models by file name; the DLL's own
        // character -> model table is not traced (Bordeaux: 2bor01 is not in the Vol.1 archive, only 2bor02 is).
        "シラバス" => ("2syl01.ccs", "2syl"),
        "ガスパー" => ("2gas01.ccs", "2gas"),
        "ボルドー" => ("2bor01.ccs", "2bor"),
        "ネギ丸" => ("2neg01.ccs", "2neg"),
        "グリン" => ("2gri01.ccs", "2gri"),
        "オーヴァン" => ("2ova01.ccs", "2ova"),
        "パイ" => ("2pai01.ccs", "2pai"),
        // ゴブナイト (ENEMY): the event motions m10120_egg2 drive en/egg10's bones (OBJ_egg10t0 ...).
        _ if actor.starts_with("ゴブナイト") || display.starts_with("ゴブナイト") => ("egg10.ccs", "egg2"),
        _ if kind == 11 && display.contains('女') => ("2pcf00.ccs", "2pcf"),
        _ if kind == 11 => ("2pcm00.ccs", "2pcm"),
        _ => return None,
    })
}

pub struct CastActor {
    pub scene_idx: usize,
    pub ccs: CcsFile,
    pub anims: Vec<CcsAnimation>,
    pub cur: Option<(usize, bool)>,
    pub t: f32,
    pub serial: u32,
    pub shown: bool,
    /// Pose being faded out after an animation switch, with elapsed and total fade time (seconds).
    pub fade_from: Option<(hackgu_ccs::LocalPose, f32, f32)>,
    /// Local pose shown last frame (snapshot source for the next crossfade).
    pub last_pose: Option<hackgu_ccs::LocalPose>,
    pub clump: Option<hackgu_ccs::CcsClump>,
    pub nodes: HashMap<u32, hackgu_ccs::CcsObjectNode>,
}

/// Crossfade used when an actor starts or stops walking (the script gives no blend for these).
pub const WALK_BLEND_SECONDS: f32 = 0.2;

impl CastActor {
    pub fn find_anim(&self, pred: impl Fn(&str) -> bool) -> Option<usize> {
        self.anims.iter().position(|a| pred(&a.name))
    }
}

pub struct Cutscene {
    pub player: EvPlayer,
    pub cast: HashMap<String, CastActor>,
    pub overrides: Vec<(String, EvTransform)>,
    pub last_message_key: Option<String>,
    pub message_index: u32,
    pub bgm_requests: Vec<(u32, Vec<String>)>,
    pc_seen: usize,
}

pub fn ev_dir() -> &'static Path {
    Path::new("target/vol1/data/data/ev")
}

impl Cutscene {
    pub fn load(id: &str) -> Option<Cutscene> {
        let script = EvScript::load(ev_dir(), id).map_err(|e| eprintln!("event {}: {}", id, e)).ok()?;
        let player = EvPlayer::new_with_ev_dir(script, Some(ev_dir()));
        Some(Cutscene {
            player,
            cast: HashMap::new(),
            overrides: Vec::new(),
            last_message_key: None,
            message_index: 0,
            bgm_requests: Vec::new(),
            pc_seen: 0,
        })
    }

    pub fn id(&self) -> &str {
        &self.player.script.id
    }

    /// Cast registrations: (script actor name, display name used by モデル ops, kind).
    pub fn cast_list(&self) -> Vec<(String, String, i32)> {
        self.player
            .script
            .ops
            .iter()
            .filter_map(|o| if let EvOp::CastRegister { actor, display, kind, .. } = o { Some((actor.clone(), display.clone(), *kind)) } else { None })
            .collect()
    }

    /// Motion files requested by `CCS操作(52)` for a cast member (by display or actor name).
    pub fn motion_files_for(&self, actor: &str, display: &str) -> Vec<String> {
        self.player.script.motion_files().into_iter().filter(|(a, _)| a == actor || a == display).map(|(_, f)| f).collect()
    }

    pub fn frame(&self) -> &EvFrame {
        &self.player.state
    }

    pub fn transform(&self) -> EvTransform {
        frame_for_cut(self.id(), &self.player.state.cut_label, &self.overrides)
    }

    /// BGM ops the script executed since the last call (op, args). EvPlayer ignores BGM, so they are read
    /// straight from the ops the program counter passed.
    pub fn take_new_bgm_ops(&mut self) -> Vec<(u32, Vec<String>)> {
        let pc = self.player.pc();
        let mut out = Vec::new();
        for op in &self.player.script.ops[self.pc_seen.min(pc)..pc] {
            if let EvOp::Bgm { op, args } = op {
                out.push((*op, args.clone()));
            }
        }
        self.pc_seen = pc;
        out
    }
}

/// Load a CCS by bare file name through the renderer's search path.
pub fn load_ccs(name: &str) -> Option<CcsFile> {
    let p = crate::app::resolve_ccs_path(Path::new(name))?;
    CcsFile::parse(&std::fs::read(p).ok()?).ok()
}
