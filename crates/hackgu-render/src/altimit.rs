//! ALTIMIT Mine OS desktop (Last Recode), built from the game's own assets.
//!
//! Data used:
//! - Wallpaper: `cmn/data/data/menu/xdwal0000.ccs` (TEX_xdwal0000, 2048x2048; its top 2048x1152 is the 16:9 screen,
//!   mapped 1:1 to the full screen — checked against the original capture, mean error 3.5/255).
//! - Icons, ring, banners: `cmn/data/data/xddesktop@ENG.ccs`. Each application's banner is the set of clumps
//!   `CMP_xddbrd<key>NN` (teal panel, white rules, title, description, copyright/MINE OS line, glyph) posed by the last
//!   frame of `ANM_xddbrd<key>`; drawn in panel units with one transform fitted to the original (PANEL_*).
//! - NEW badge: TEX_xddicnew01 in `xdcmn@ENG.ccs`. Button glyphs: btn00 (`btn00@ENG@WIN64.ccs`).
//! - Help-bar text: messageInfo.bin ids 0x2401E845.. ("Launch The World. <iconOK>: Confirm <iconSELECT>: ...").
//!
//! Measured from the original (reference_captures/stills/altimit_desktop.png 2560x1440 and run2_field f01062.. at
//! 960x540), not found in data: the 16 orbit slots (8 applications alternating with 8 cubes, selection moves every
//! icon two slots), icon sizes per slot, the ring position/size, the bottom bar, the panel tint/opacity and the
//! initial NEW badges (News Capture, Community Forum, Movie Player after a new game).

use std::collections::HashMap;
use std::ops::Add;
use std::path::Path;

use egui::{Color32, Pos2, Rect, TextureHandle};
use hackgu_ccs::{CcsFile, DecodedModel, Mat4};
use hackgu_game::msginfo::MessageInfo;

/// Applications in ring order (selecting "next" brings the following one into the ring).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum App {
    TheWorld,
    MailStation,
    NewsCapture,
    CommunityForum,
    CrimsonVs,
    MoviePlayer,
    Accessories,
    DataManager,
}

pub const APPS: [App; 8] = [App::TheWorld, App::MailStation, App::NewsCapture, App::CommunityForum, App::CrimsonVs, App::MoviePlayer, App::Accessories, App::DataManager];
/// Banner clump/animation key per app (`CMP_xddbrd<key>NN`, `ANM_xddbrd<key>`).
const BANNER_KEY: [&str; 8] = ["tw", "mail", "nc", "yo", "cvs", "mp", "ac", "dm"];
/// Icon texture per app (identified by the icon shown in the ring when each banner is up).
const ICON_TEX: [&str; 8] = ["xddicon01", "xddicon02", "xddicon08", "xddicon03", "xddicon07", "xddicon05", "xddicon04", "xddicon06"];
/// "Launch X. <iconOK>: Confirm <iconSELECT>: Return to Title Screen" per app.
const MSG_LAUNCH: [u32; 8] = [0x2401E845, 0x23015AD3, 0x68E8CA48, 0x2628DC56, 0xD05B4E88, 0x72D3E9DF, 0x986364F1, 0x5617D57A];
const FALLBACK_NAME: [&str; 8] = ["The World", "Mail Station", "News Capture", "Community Forum", "Crimson VS", "Movie Player", "Accessories", "Data Manager"];

// --- measured layout (fractions of a 16:9 screen) ---
/// Orbit slots, front (inside the ring) first, then along the bottom to the right, up and back over the top.
const SLOTS: [[f32; 2]; 16] = [
    [0.313, 0.751],
    [0.458, 0.772],
    [0.589, 0.785],
    [0.689, 0.751],
    [0.738, 0.683],
    [0.736, 0.591],
    [0.686, 0.494],
    [0.598, 0.402],
    [0.485, 0.327],
    [0.363, 0.281],
    [0.243, 0.262],
    [0.1477, 0.284],
    [0.0809, 0.336],
    [0.064, 0.420],
    [0.102, 0.524],
    [0.189, 0.628],
];
/// Visible icon width (fraction of screen width) grows with screen y (nearer = lower): 0.031 at the back, 0.045 in front.
fn icon_size(y: f32) -> f32 {
    (0.024 + 0.0268 * y) / HSTRETCH
}
/// Icon textures fill ~87% of their square.
const ICON_TEX_SCALE: f32 = 1.15;
const CUBE_TEX_SCALE: f32 = 0.48;
const GLOW_TEX_SCALE: f32 = 1.6;
const CUBE_GLOW_TEX_SCALE: f32 = 0.6;
/// The desktop scene is drawn stretched horizontally (ring rim measured 1.082 wider than tall).
const HSTRETCH: f32 = 1.082;
/// Ring (xddfrm01) centre and full-texture height (as a fraction of width before the stretch); the selected icon.
const RING_C: [f32; 2] = [0.3139, 0.748];
const RING_W: f32 = 0.1566;
const RING_ICON_W: f32 = 0.0975;
/// The ring icon sits slightly below the ring centre (fraction of height).
const RING_ICON_DY: f32 = 0.008;
/// NEW badge on the ring icon: centre x, y and width.
const RING_BADGE: [f32; 3] = [0.293, 0.722, 0.040];
/// The ring glass is darker than its texture in the original.
const LENS_TINT: u8 = 218;
/// Opacity of the glass highlight (TEX_xddlens01) over the ring icon.
const GLASS_ALPHA: f32 = 0.3;
/// Panel units -> screen: x = PANEL_O.x + u * PANEL_S.x (of width), y = PANEL_O.y - v * PANEL_S.y (of height).
const PANEL_O: [f32; 2] = [0.6197, 0.47835];
const PANEL_S: [f32; 2] = [0.0014073, 0.0021420];
/// Teal panel: solved from the original over the known wallpaper (alpha blend).
// Solved in gamma space (egui blends in gamma space, like the original).
const PANEL_TINT: [u8; 3] = [24, 156, 188];
const PANEL_ALPHA: f32 = 0.6;
/// Bottom help bar.
const BAR: [f32; 4] = [0.0109, 0.8924, 0.9895, 0.9611];
const BAR_FILL: Color32 = Color32::from_rgb(192, 193, 195);
const BAR_LIGHT: Color32 = Color32::from_rgb(210, 210, 210);
const BAR_DARK: Color32 = Color32::from_rgb(112, 112, 112);
const BAR_TEXT_X: f32 = 0.0508;
const BAR_CAP: [f32; 2] = [0.9125, 0.9361];
const BAR_TRI: [f32; 4] = [0.0281, 0.9187, 0.0375, 0.9375];
const GLYPH_A_H: f32 = 0.041;
const GLYPH_BACK_H: f32 = 0.0455;
/// Space before a glyph beyond the text's own spacing.
const GLYPH_LEAD: f32 = 0.0055;
/// btn00 atlas pixels (512x512): A and BACK.
const BTN_A: [f32; 4] = [245.0, 4.0, 315.0, 74.0];
const BTN_BACK: [f32; 4] = [351.0, 163.0, 433.0, 237.0];
/// B, X, LB, RB (alpha bounds in btn00, inset like BTN_A). LB/RB height measured on the Mail Station help bar
/// (f01094: 31 px wide at 960).
const BTN_B: [f32; 4] = [5.0, 5.0, 75.0, 75.0];
const BTN_X: [f32; 4] = [165.0, 6.0, 235.0, 76.0];
const BTN_LB: [f32; 4] = [34.0, 171.0, 126.0, 228.0];
const BTN_RB: [f32; 4] = [34.0, 251.0, 126.0, 308.0];
const GLYPH_LB_H: f32 = 0.0356;
const GLYPH_LB_GAP: f32 = 0.006;
/// "NEW" part of TEX_xddicnew01 (the texture also holds "Log" and "!").
const NEW_UV: [f32; 4] = [0.016, 0.02, 0.49, 0.40];
/// Horizontal advance after each glyph beyond its width (fraction of width), measured on the bar.
const GLYPH_A_GAP: f32 = 0.0047;
const GLYPH_BACK_GAP: f32 = 0.0176;
/// Rotation time for one application step (seconds).
const STEP_TIME: f32 = 0.25;

pub(crate) struct Sprite {
    pub tex: String,
    z: f32,
    alpha: f32,
    pub verts: Vec<([f32; 2], [f32; 2])>,
    pub indices: Vec<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DesktopAction {
    None,
    Launch(App),
    ReturnToTitle,
}

pub struct DesktopScreen {
    tex: HashMap<String, (TextureHandle, [usize; 2])>,
    banners: Vec<Vec<Sprite>>,
    text: HashMap<u32, String>,
    /// Selected application index into APPS.
    pub sel: usize,
    /// Displayed rotation in application steps, and where it is heading (unwrapped).
    rot: f32,
    goal: f32,
    /// Applications showing the NEW badge.
    pub new_badge: [bool; 8],
    pub status: Option<String>,
    /// Scripted desktop sequence (message id, is a system popup), advanced with A; e.g. the reboot after MM10145.
    pub intro: Vec<(u32, bool)>,
    intro_pos: usize,
    intro_text: HashMap<u32, String>,
    /// Where the story is on the desktop (the reformat and Ovan's mail).
    pub story: Story,
    mail: Option<crate::mailstation::MailStation>,
    mail_open: bool,
    dm: Option<crate::datamanager::DataManager>,
    dm_open: bool,
    ac: Option<crate::accessories::Accessories>,
    ac_open: bool,
    /// Desktop wallpaper (index into accessories::WALLPAPERS) and which ones the player owns.
    pub wallpaper: usize,
    pub owned_wallpapers: Vec<bool>,
    /// Set by the game each frame for saving: (level, play time in seconds, reformatted).
    pub save_info: (u32, f64, bool),
    /// Set by the game each frame for saving: events played so far.
    pub events_done: Vec<String>,
    /// A loaded save the game should apply (level, play time, flags).
    pub pending_load: Option<crate::datamanager::SaveFile>,
    lang: String,
}

/// Owned wallpapers at the start: 000 Default. `HACKGU_WALLPAPERS=56,73` (debug) adds others until the Community
/// Forum, which hands them out in the original, exists.
fn initial_wallpapers() -> Vec<bool> {
    let mut v = vec![false; crate::accessories::WALLPAPERS.len()];
    v[0] = true;
    if let Ok(list) = std::env::var("HACKGU_WALLPAPERS") {
        for i in list.split(',').filter_map(|s| s.trim().parse::<usize>().ok()) {
            if i < v.len() {
                v[i] = true;
            }
        }
    }
    v
}

/// Desktop story state after MM10145.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Story {
    Free,
    /// Rebooted and reformatted: the mailbox is empty.
    Reformatted,
    /// Ovan's mail has arrived.
    NewMail,
    /// Ovan's mail was read; the desktop help follows when Mail Station closes.
    ReadOvan,
}

impl Story {
    pub fn name(self) -> &'static str {
        match self {
            Story::Free => "Free",
            Story::Reformatted => "Reformatted",
            Story::NewMail => "NewMail",
            Story::ReadOvan => "ReadOvan",
        }
    }

    pub fn from_name(s: &str) -> Story {
        match s {
            "Reformatted" => Story::Reformatted,
            "NewMail" => Story::NewMail,
            "ReadOvan" => Story::ReadOvan,
            _ => Story::Free,
        }
    }
}

/// The desktop sequence after MM10145 (Haseo reformatted after Tri-Edge): messageInfo rows 21885-21892 in order.
/// System popups ("Reboot Complete", "Continuing process...", "System reformatting complete.") vs Haseo's lines:
/// 21885 popup, 21886/21889/21892 Haseo lines are seen in reference_captures/run2_field f01045-f01060; the kind of
/// 21887/21888/21890/21891 follows their wording.
pub const REBOOT_SEQUENCE: [(u32, bool); 8] = [
    (0x777EFB39, true),
    (0x7A3DDDE0, false),
    (0x7EFCC057, false),
    (0x60BB9052, true),
    (0x647A8DE5, false),
    (0x6939AB3C, true),
    (0x6DF8B68B, false),
    (0x55B70B36, false),
];

pub(crate) fn load_textures(ctx: &egui::Context, path: &Path, names: &[&str], out: &mut HashMap<String, (TextureHandle, [usize; 2])>) {
    let Some(ccs) = std::fs::read(path).ok().and_then(|d| CcsFile::parse(&d).ok()) else {
        eprintln!("altimit: cannot read {}", path.display());
        return;
    };
    load_textures_from(ctx, &ccs, names, out);
}

pub(crate) fn load_textures_from(ctx: &egui::Context, ccs: &CcsFile, names: &[&str], out: &mut HashMap<String, (TextureHandle, [usize; 2])>) {
    for t in ccs.objects_of_type(hackgu_ccs::objects::SECTION_TEXTURE) {
        let name = ccs.object_name(t.object_id).unwrap_or("").trim_start_matches("TEX_").to_string();
        if !names.contains(&name.as_str()) || out.contains_key(&name) {
            continue;
        }
        let clut_id = t.payload.get(0..4).map(|b| i32::from_le_bytes(b.try_into().unwrap())).unwrap_or(0);
        let clut = if clut_id > 0 {
            ccs.raw_objects
                .iter()
                .find(|o| o.object_id == clut_id as u32 && o.section_type == hackgu_ccs::objects::SECTION_CLUT)
                .and_then(|o| o.payload.get(16..))
        } else {
            None
        };
        if let Ok(d) = hackgu_ccs::DecodedTexture::from_payload_with_clut(&name, &t.payload, clut) {
            let img = egui::ColorImage::from_rgba_unmultiplied([d.width, d.height], &d.rgba);
            let h = ctx.load_texture(&name, img, egui::TextureOptions::LINEAR);
            out.insert(name, (h, [d.width, d.height]));
        }
    }
}

/// All sprites of one application's banner, in panel units, posed by the last frame of its animation.
fn build_banner(ccs: &CcsFile, key: &str) -> Vec<Sprite> {
    let prefix = format!("CMP_xddbrd{}", key);
    build_sprites(
        ccs,
        &|cname: &str| cname.strip_prefix(&prefix).is_some_and(|rest| !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_digit())),
        &format!("ANM_xddbrd{}", key),
    )
}

/// Sprites of the clumps accepted by `want`, in scene units, posed by the last frame of `anim_name` (bind pose when the
/// animation does not exist).
pub(crate) fn build_sprites(ccs: &CcsFile, want: &dyn Fn(&str) -> bool, anim_name: &str) -> Vec<Sprite> {
    let nodes = ccs.scene_nodes();
    let mats = ccs.materials();
    let anims = ccs.animations();
    let anim = anims.iter().find(|a| a.name == anim_name);
    let frame = anim.map(|a| a.frame_count.saturating_sub(1) as f32).unwrap_or(0.0);
    let models: HashMap<u32, _> = ccs.objects_of_type(hackgu_ccs::objects::SECTION_MODEL).into_iter().map(|m| (m.object_id, m)).collect();
    let mut out = Vec::new();
    for cl in ccs.clumps() {
        let cname = ccs.object_name(cl.object_id).unwrap_or("");
        if !want(cname) {
            continue;
        }
        // World matrix and opacity per bone, parents first (bones are listed parent-before-child).
        let mut world: HashMap<u32, Mat4> = HashMap::new();
        let mut alpha: HashMap<u32, f32> = HashMap::new();
        for (i, &oid) in cl.bone_obj_ids.iter().enumerate() {
            let bind = &cl.transforms[i];
            let name = ccs.object_name(oid).unwrap_or("");
            let ch = anim.and_then(|a| a.channel_by_name.get(name).map(|&c| &a.channels[c]));
            let t = ch.map(|c| c.sample_translation(frame, bind.translation)).unwrap_or(bind.translation);
            let q = ch.map(|c| c.sample_rotation_quat(frame, bind.rotation_deg)).unwrap_or_else(|| Mat4::euler_deg_to_quat(bind.rotation_deg));
            let s = ch.and_then(|c| c.sample_scale(frame)).unwrap_or(bind.scale);
            let a = ch.and_then(|c| c.sample_alpha(frame)).unwrap_or(1.0);
            let local = Mat4::from_translation(t).mul(&Mat4::from_quat(q)).mul(&Mat4::from_scale(s));
            let parent = nodes.get(&oid).map(|n| n.parent_obj_id).unwrap_or(0);
            // Opacity is per object: the panel's children (white rules, glyph) are opaque in the original.
            let pw = world.get(&parent).copied().unwrap_or(Mat4::IDENTITY);
            world.insert(oid, pw.mul(&local));
            alpha.insert(oid, a);
        }
        for &oid in &cl.bone_obj_ids {
            let Some(node) = nodes.get(&oid) else { continue };
            let Some(m) = models.get(&node.model_obj_id) else { continue };
            let name = ccs.object_name(m.object_id).unwrap_or("");
            let Ok(mut dm) = DecodedModel::from_payload(name, &m.payload, ccs.header.raw_version) else { continue };
            if dm.is_rigid() {
                dm.apply_rigid(&world[&oid]);
            }
            for me in &dm.meshes {
                let tex = mats
                    .get(&me.material_index)
                    .and_then(|t| ccs.object_name(t.texture_obj_id))
                    .unwrap_or("")
                    .trim_start_matches("TEX_")
                    .to_string();
                // Some quads address the texture with u in [1, 2): same image (wrap addressing).
                let umin = me.vertices.iter().map(|v| v.uv[0]).fold(f32::MAX, f32::min).add(1e-3).floor();
                let vmin = me.vertices.iter().map(|v| v.uv[1]).fold(f32::MAX, f32::min).add(1e-3).floor();
                let z = me.vertices.iter().map(|v| v.position[2]).fold(f32::MIN, f32::max);
                out.push(Sprite {
                    tex,
                    z,
                    alpha: alpha[&oid],
                    // CCS v runs bottom-up; decoded images are top-down.
                    verts: me.vertices.iter().map(|v| ([v.position[0], v.position[1]], [v.uv[0] - umin, 1.0 - (v.uv[1] - vmin)])).collect(),
                    indices: me.indices.clone(),
                });
            }
        }
    }
    out.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap_or(std::cmp::Ordering::Equal));
    out
}

/// Closed Catmull-Rom through the orbit slots; `p` in slot units (wraps).
fn slot_pos(p: f32) -> [f32; 2] {
    let n = SLOTS.len() as i32;
    let i = p.floor() as i32;
    let t = p - i as f32;
    let g = |k: i32| SLOTS[k.rem_euclid(n) as usize];
    let (p0, p1, p2, p3) = (g(i - 1), g(i), g(i + 1), g(i + 2));
    let mut out = [0.0; 2];
    for c in 0..2 {
        let (a, b, cc, d) = (p0[c], p1[c], p2[c], p3[c]);
        out[c] = 0.5 * ((2.0 * b) + (-a + cc) * t + (2.0 * a - 5.0 * b + 4.0 * cc - d) * t * t + (-a + 3.0 * b - 3.0 * cc + d) * t * t * t);
    }
    out
}

impl DesktopScreen {
    pub fn new(ctx: &egui::Context, lang_code: &str) -> Self {
        let mut tex = HashMap::new();
        load_textures(ctx, Path::new("target/cmn/data/data/menu/xdwal0000.ccs"), &["xdwal0000"], &mut tex);
        let mut banners = Vec::new();
        if let Some(ccs) = std::fs::read("target/cmn/data/data/xddesktop@ENG.ccs").ok().and_then(|d| CcsFile::parse(&d).ok()) {
            let mut names: Vec<&str> = vec!["xddicon00", "xddiconbk", "xddfrm01", "xddlens", "xddlens01"];
            names.extend(ICON_TEX);
            for key in BANNER_KEY {
                let b = build_banner(&ccs, key);
                if b.is_empty() {
                    eprintln!("altimit: no banner clumps for {}", key);
                }
                if std::env::var("HACKGU_ALTIMIT_DEBUG").is_ok() {
                    for s in &b {
                        let (mut lo, mut hi) = ([f32::MAX; 4], [f32::MIN; 4]);
                        for (p, uv) in &s.verts {
                            for k in 0..2 {
                                lo[k] = lo[k].min(p[k]);
                                hi[k] = hi[k].max(p[k]);
                                lo[2 + k] = lo[2 + k].min(uv[k]);
                                hi[2 + k] = hi[2 + k].max(uv[k]);
                            }
                        }
                        eprintln!("altimit {} {:<14} z{:.1} a{:.2} pos {:.1?}..{:.1?} uv {:.3?}..{:.3?} tris {}", key, s.tex, s.z, s.alpha, &lo[..2], &hi[..2], &lo[2..], &hi[2..], s.indices.len() / 3);
                    }
                }
                banners.push(b);
            }
            let banner_tex: Vec<String> = banners.iter().flatten().map(|s| s.tex.clone()).collect();
            names.extend(banner_tex.iter().map(|s| s.as_str()));
            load_textures_from(ctx, &ccs, &names, &mut tex);
        } else {
            eprintln!("altimit: cannot read xddesktop@ENG.ccs");
            banners.resize_with(8, Vec::new);
        }
        load_textures(ctx, Path::new("target/cmn/data/data/xdcmn@ENG.ccs"), &["xddicnew01"], &mut tex);
        load_textures(ctx, Path::new("target/cmn/data/data/btn00@ENG@WIN64.ccs"), &["btn00"], &mut tex);
        let mut text = HashMap::new();
        if let Some(l) = MessageInfo::load(Path::new("target"), lang_code) {
            for id in MSG_LAUNCH {
                if let Some(s) = l.by_id.get(&id) {
                    text.insert(id, s.clone());
                }
            }
        }
        let mut new_badge = [false; 8];
        for app in [App::NewsCapture, App::CommunityForum, App::MoviePlayer] {
            new_badge[app as usize] = true;
        }
        let mut intro_text = HashMap::new();
        if let Some(l) = MessageInfo::load(Path::new("target"), lang_code) {
            use crate::mailstation as ms;
            let story = REBOOT_SEQUENCE.iter().chain(&ms::DESKTOP_NEWMAIL_SEQ).chain(&ms::CHECK_MAIL_FIRST).chain(&ms::DESKTOP_HELP);
            for (id, _) in story.copied() {
                if let Some(s) = l.by_id.get(&id) {
                    intro_text.insert(id, s.clone());
                }
            }
        }
        Self { tex, banners, text, sel: 0, rot: 0.0, goal: 0.0, new_badge, status: None, intro: Vec::new(), intro_pos: 0, intro_text, story: Story::Free, mail: None, mail_open: false, dm: None, dm_open: false, ac: None, ac_open: false, wallpaper: 0, owned_wallpapers: initial_wallpapers(), save_info: (1, 0.0, false), events_done: Vec::new(), pending_load: None, lang: lang_code.to_string() }
    }

    /// Select an application immediately (no rotation).
    pub fn select(&mut self, i: usize) {
        self.sel = i % 8;
        self.rot = self.sel as f32;
        self.goal = self.rot;
    }

    /// Handle input and draw. Returns what the game should do.
    /// Start a scripted sequence (e.g. REBOOT_SEQUENCE).
    pub fn play_intro(&mut self, seq: &[(u32, bool)]) {
        self.intro = seq.to_vec();
        self.intro_pos = 0;
    }

    fn draw_intro(&self, ctx: &egui::Context) {
        let Some(&(id, system)) = self.intro.get(self.intro_pos) else { return };
        let text = self.intro_text.get(&id).cloned().unwrap_or_default();
        draw_dialogue(ctx, self.tex.get("btn00").map(|(t, _)| t.id()), &text, system);
    }

    /// After the reboot: the mailbox is empty until Mail Station has been opened once.
    pub fn start_reformat(&mut self) {
        self.select(0);
        self.play_intro(&REBOOT_SEQUENCE);
        self.story = Story::Reformatted;
        if let Some(m) = self.mail.as_mut() {
            m.inbox.clear();
        }
    }

    fn open_mail(&mut self, ctx: &egui::Context) {
        let lang = self.lang.clone();
        let m = self.mail.get_or_insert_with(|| crate::mailstation::MailStation::new(ctx, &lang, &[]));
        m.open();
        if self.story == Story::Reformatted {
            m.play(&crate::mailstation::MAIL_EMPTY_SEQ);
        }
        self.mail_open = true;
    }

    fn show_mail(&mut self, ctx: &egui::Context) {
        use crate::mailstation::{self as ms, MailEvent};
        let Some(m) = self.mail.as_mut() else {
            self.mail_open = false;
            return;
        };
        let ev = m.show(ctx);
        let is_ovan = |m: &ms::MailStation, i: usize| m.inbox.get(i).is_some_and(|x| x.body == ms::ovan_choices().body);
        match ev {
            MailEvent::Close => {
                self.mail_open = false;
                match self.story {
                    Story::Reformatted => {
                        self.story = Story::NewMail;
                        m.inbox.insert(0, ms::ovan_choices());
                        self.new_badge[App::MailStation as usize] = true;
                        self.play_intro(&ms::DESKTOP_NEWMAIL_SEQ);
                    }
                    Story::ReadOvan => {
                        self.story = Story::Free;
                        self.play_intro(&ms::DESKTOP_HELP);
                    }
                    _ => {}
                }
            }
            MailEvent::EnteredList if self.story == Story::NewMail && m.inbox.first().is_some_and(|x| x.unread) => m.play(&ms::OVAN_SEEN),
            MailEvent::Opened(_) => {
                self.new_badge[App::MailStation as usize] = m.inbox.iter().any(|x| x.unread);
            }
            MailEvent::ClosedView(i) if self.story == Story::NewMail && is_ovan(m, i) => {
                self.story = Story::ReadOvan;
                m.play(&ms::AFTER_OVAN);
            }
            _ => {}
        }
        ctx.request_repaint();
    }

    fn mail_station(&mut self, ctx: &egui::Context) -> &mut crate::mailstation::MailStation {
        let lang = self.lang.clone();
        self.mail.get_or_insert_with(|| crate::mailstation::MailStation::new(ctx, &lang, &[]))
    }

    /// The current game as a save file.
    fn snapshot(&mut self, ctx: &egui::Context) -> crate::datamanager::SaveFile {
        use crate::datamanager as dm;
        let (level, play_time, reformatted) = self.save_info;
        let story = self.story;
        let new_badge = self.new_badge;
        let inbox: Vec<dm::SavedMail> = self.mail_station(ctx).inbox.iter().map(|m| dm::SavedMail { body: m.body, unread: m.unread }).collect();
        // Chapter: "First Cry" before the reformat, "Deja Vu" after it (the original's slot after this point,
        // reference_captures/run2_field f01215).
        let after = reformatted || story != Story::Free || !inbox.is_empty();
        let (saved_at, date) = dm::now_date();
        dm::SaveFile {
            format: 1,
            chapter: if after { dm::CHAPTER_DEJA_VU } else { dm::CHAPTER_FIRST_CRY },
            location: dm::LOCATION_DESKTOP,
            level,
            play_time,
            date,
            saved_at,
            story: story.name().into(),
            inbox,
            new_badge,
            reformatted,
            wallpaper: self.wallpaper,
            wallpapers: self.owned_wallpapers.iter().enumerate().filter(|(_, &o)| o).map(|(i, _)| i).collect(),
            events_done: self.events_done.clone(),
        }
    }

    /// Restore the desktop part of a save (story, mailbox, badges); the game applies the rest from `pending_load`.
    pub fn apply_save(&mut self, ctx: &egui::Context, f: &crate::datamanager::SaveFile) {
        self.story = Story::from_name(&f.story);
        self.new_badge = f.new_badge;
        if !f.wallpapers.is_empty() {
            self.owned_wallpapers = vec![false; crate::accessories::WALLPAPERS.len()];
            for &i in &f.wallpapers {
                if i < self.owned_wallpapers.len() {
                    self.owned_wallpapers[i] = true;
                }
            }
        }
        if f.wallpaper != self.wallpaper && self.owned_wallpapers.get(f.wallpaper).copied().unwrap_or(false) {
            self.set_wallpaper(ctx, f.wallpaper);
        }
        let m = self.mail_station(ctx);
        m.inbox = f
            .inbox
            .iter()
            .filter_map(|s| {
                let known = crate::mailstation::ovan_choices();
                (s.body == known.body).then(|| crate::mailstation::Mail { unread: s.unread, ..known })
            })
            .collect();
        self.intro.clear();
        self.intro_pos = 0;
        self.mail_open = false;
        self.dm_open = false;
        self.pending_load = Some(f.clone());
    }

    fn show_dm(&mut self, ctx: &egui::Context) {
        use crate::datamanager::DmEvent;
        // Wallpaper, then the Data Manager's column and panel, then the ring over them (f01213/f01214).
        self.draw_parts(ctx, "altimit_desktop", true, false, false);
        let btn = self.tex.get("btn00").map(|(t, _)| t.id());
        let ev = match self.dm.as_mut() {
            Some(d) => d.show(ctx, btn),
            None => DmEvent::Close,
        };
        self.draw_parts(ctx, "altimit_ring", false, false, true);
        match ev {
            DmEvent::Close => self.dm_open = false,
            DmEvent::Save(slot) => {
                let f = self.snapshot(ctx);
                let ok = crate::datamanager::write_slot(slot, &f).is_ok();
                if let Some(d) = self.dm.as_mut() {
                    d.saved(ok);
                }
            }
            DmEvent::Load(f) => self.apply_save(ctx, &f),
            DmEvent::None => {}
        }
        ctx.request_repaint();
    }

    pub fn set_wallpaper(&mut self, ctx: &egui::Context, index: usize) {
        self.wallpaper = index;
        crate::accessories::load_wallpaper(ctx, index, "wallpaper", &mut self.tex);
    }

    fn show_ac(&mut self, ctx: &egui::Context) {
        use crate::accessories::AcEvent;
        self.draw_parts(ctx, "altimit_desktop", true, false, false);
        let btn = self.tex.get("btn00").map(|(t, _)| t.id());
        let (owned, cur) = (self.owned_wallpapers.clone(), self.wallpaper);
        let ev = match self.ac.as_mut() {
            Some(a) => a.show(ctx, btn, &owned, cur),
            None => AcEvent::Close,
        };
        self.draw_parts(ctx, "altimit_ring", false, false, true);
        match ev {
            AcEvent::Close => self.ac_open = false,
            AcEvent::SetWallpaper(i) => self.set_wallpaper(ctx, i),
            AcEvent::None => {}
        }
        ctx.request_repaint();
    }

    pub fn show(&mut self, ctx: &egui::Context) -> DesktopAction {
        if self.ac_open {
            self.show_ac(ctx);
            return DesktopAction::None;
        }
        if self.dm_open {
            self.show_dm(ctx);
            return DesktopAction::None;
        }
        if self.mail_open {
            self.show_mail(ctx);
            return DesktopAction::None;
        }
        if self.intro_pos < self.intro.len() {
            let ok = ctx.input(|i| i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Space));
            if ok {
                self.intro_pos += 1;
            }
            self.draw(ctx);
            self.draw_intro(ctx);
            ctx.request_repaint();
            return DesktopAction::None;
        }
        let (prev, next, ok, back, dt) = ctx.input(|i| {
            (
                i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::A) || i.key_pressed(egui::Key::ArrowUp) || i.key_pressed(egui::Key::W),
                i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::D) || i.key_pressed(egui::Key::ArrowDown) || i.key_pressed(egui::Key::S),
                i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Space),
                i.key_pressed(egui::Key::Escape) || i.key_pressed(egui::Key::Backspace),
                i.stable_dt.min(0.1),
            )
        });
        let mut action = DesktopAction::None;
        // `goal` and `rot` are unwrapped so the ring always turns the way the player pressed.
        if next {
            self.goal += 1.0;
            self.status = None;
        }
        if prev {
            self.goal -= 1.0;
            self.status = None;
        }
        self.sel = (self.goal as i32).rem_euclid(8) as usize;
        let d = self.goal - self.rot;
        let step = dt / STEP_TIME * d.abs().max(1.0);
        self.rot = if d.abs() <= step { self.goal } else { self.rot + step * d.signum() };
        if ok && self.rot == self.goal {
            let app = APPS[self.sel];
            if app == App::MailStation {
                self.open_mail(ctx);
                ctx.request_repaint();
                return DesktopAction::None;
            } else if app == App::Accessories && self.story != Story::NewMail {
                let lang = self.lang.clone();
                let cur = self.wallpaper;
                self.ac.get_or_insert_with(|| crate::accessories::Accessories::new(ctx, &lang)).open(cur);
                self.ac_open = true;
                ctx.request_repaint();
                return DesktopAction::None;
            } else if app == App::DataManager && self.story != Story::NewMail {
                let lang = self.lang.clone();
                self.dm.get_or_insert_with(|| crate::datamanager::DataManager::new(ctx, &lang)).open();
                self.dm_open = true;
                ctx.request_repaint();
                return DesktopAction::None;
            } else if self.story == Story::NewMail {
                self.play_intro(&crate::mailstation::CHECK_MAIL_FIRST);
            } else {
                action = DesktopAction::Launch(app);
            }
        }
        if back {
            action = DesktopAction::ReturnToTitle;
        }
        if self.rot != self.goal {
            ctx.request_repaint();
        }
        self.draw(ctx);
        action
    }

    fn draw(&self, ctx: &egui::Context) {
        self.draw_parts(ctx, "altimit_desktop", true, true, true);
    }

    /// Draw the wallpaper (+ orbit, banner and help bar when `full`) and/or the ring on its own layer.
    fn draw_parts(&self, ctx: &egui::Context, layer: &str, wallpaper: bool, full: bool, ring: bool) {
        let screen = ctx.screen_rect();
        let (w, h) = if screen.width() / screen.height() > 16.0 / 9.0 {
            (screen.height() * 16.0 / 9.0, screen.height())
        } else {
            (screen.width(), screen.width() * 9.0 / 16.0)
        };
        let origin = Pos2::new(screen.center().x - w * 0.5, screen.center().y - h * 0.5);
        let at = |x: f32, y: f32| Pos2::new(origin.x + x * w, origin.y + y * h);
        let p = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new(layer)));
        let img = |name: &str, rect: Rect, uv: [f32; 4], tint: Color32| {
            if let Some((t, _)) = self.tex.get(name) {
                p.image(t.id(), rect, Rect::from_min_max(Pos2::new(uv[0], uv[1]), Pos2::new(uv[2], uv[3])), tint);
            }
        };
        let full_uv = [0.0, 0.0, 1.0, 1.0];
        // Square of side `side` (fraction of width) centred at (x, y).
        let square = |x: f32, y: f32, side: f32| Rect::from_center_size(at(x, y), egui::vec2(side * w * HSTRETCH, side * w));
        if wallpaper {
            p.rect_filled(screen, 0.0, Color32::BLACK);
            let wall = if self.tex.contains_key("wallpaper") { "wallpaper" } else { "xdwal0000" };
            img(wall, Rect::from_min_size(origin, egui::vec2(w, h)), [0.0, 0.0, 1.0, 1152.0 / 2048.0], Color32::WHITE);
        }
        let r0 = self.rot.floor();
        let f = self.rot - r0;
        let a0 = (r0 as i32).rem_euclid(8) as usize;
        let a1 = (a0 + 1) % 8;
        if full {

        // Orbit: item k (even = application k/2, odd = cube) sits at slot k - 2*rot; farther (higher) items first.
        let mut items: Vec<(f32, usize, [f32; 2], f32)> = Vec::new();
        for k in 0..16 {
            let s = (k as f32 - 2.0 * self.rot).rem_euclid(16.0);
            let pos = slot_pos(s);
            // Applications fade out as they enter the ring slot (slot 0) and in as they leave it.
            let dist = s.min(16.0 - s);
            let a = if k % 2 == 0 { dist.clamp(0.0, 1.0) } else { 1.0 };
            items.push((pos[1], k, pos, a));
        }
        items.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        for &(_, k, pos, a) in &items {
            if a <= 0.0 {
                continue;
            }
            let size = icon_size(pos[1]);
            let tint = Color32::from_white_alpha((a * 255.0) as u8);
            if k % 2 == 0 {
                let app = k / 2;
                img("xddiconbk", square(pos[0], pos[1], size * GLOW_TEX_SCALE), full_uv, tint);
                img(ICON_TEX[app], square(pos[0], pos[1], size * ICON_TEX_SCALE), full_uv, tint);
                if self.new_badge[app] {
                    self.new_badge_at(&img, &at, w, h, pos, size, tint);
                }
            } else {
                img("xddiconbk", square(pos[0], pos[1], size * CUBE_GLOW_TEX_SCALE), full_uv, tint);
                img("xddicon00", square(pos[0], pos[1], size * CUBE_TEX_SCALE), full_uv, tint);
            }
        }

        // Banner: cross-fade between the two applications either side of the displayed rotation.
        if f < 0.999 {
            self.draw_banner(&p, origin, w, h, a0, 1.0 - f);
        }
        if f > 0.001 {
            self.draw_banner(&p, origin, w, h, a1, f);
        }
        }
        if !ring {
            return;
        }

        // Ring with the selected application's icon.
        img("xddlens", square(RING_C[0], RING_C[1], RING_W), full_uv, Color32::from_gray(LENS_TINT));
        for (app, a) in [(a0, 1.0 - f), (a1, f)] {
            if a > 0.001 {
                let tint = Color32::from_white_alpha((a * 255.0) as u8);
                img(ICON_TEX[app], square(RING_C[0], RING_C[1] + RING_ICON_DY, RING_ICON_W), full_uv, tint);
                if self.new_badge[app] {
                    // Measured on News Capture in the ring: centre (0.293, 0.722), 0.040 wide.
                    let c = at(RING_BADGE[0], RING_BADGE[1]);
                    img("xddicnew01", Rect::from_center_size(c, egui::vec2(RING_BADGE[2] * w, RING_BADGE[2] * w / 2.48)), NEW_UV, tint);
                }
            }
        }
        // MDL_xddlens02 (TEX_xddlens01, z 2): translucent glass over the icon.
        img("xddlens01", square(RING_C[0], RING_C[1], RING_W), full_uv, Color32::from_white_alpha((GLASS_ALPHA * 255.0) as u8));
        img("xddfrm01", square(RING_C[0], RING_C[1], RING_W), full_uv, Color32::WHITE);

        if full {
            self.bottom_bar(&p, origin, w, h);
        }
    }

    fn new_badge_at(&self, img: &dyn Fn(&str, Rect, [f32; 4], Color32), at: &dyn Fn(f32, f32) -> Pos2, w: f32, h: f32, pos: [f32; 2], size: f32, tint: Color32) {
        // Measured on Movie Player at the back: badge centre 0.56 icon-widths left and 0.31 icon-widths up.
        let c = at(pos[0] - 0.56 * size, pos[1] - 0.31 * size * w / h);
        let bw = 0.8 * size * w;
        img("xddicnew01", Rect::from_center_size(c, egui::vec2(bw, bw / 2.48)), NEW_UV, tint);
    }

    fn draw_banner(&self, p: &egui::Painter, origin: Pos2, w: f32, h: f32, app: usize, fade: f32) {
        let Some(sprites) = self.banners.get(app) else { return };
        let to_screen = |u: f32, v: f32| Pos2::new(origin.x + (PANEL_O[0] + u * PANEL_S[0]) * w, origin.y + (PANEL_O[1] - v * PANEL_S[1]) * h);
        for s in sprites {
            let a = (s.alpha * fade).clamp(0.0, 1.0);
            let (tex_id, color) = if s.tex == "xddclt" {
                // The tint/opacity were measured on the final image, so the animation's own opacity is not applied again.
                let k = fade * PANEL_ALPHA;
                // Premultiply in gamma space (from_rgba_unmultiplied premultiplies in linear space).
                let pm = |c: u8| (c as f32 * k).round() as u8;
                let c = Color32::from_rgba_premultiplied(pm(PANEL_TINT[0]), pm(PANEL_TINT[1]), pm(PANEL_TINT[2]), (k * 255.0).round() as u8);
                (egui::TextureId::default(), c)
            } else if let Some((t, _)) = self.tex.get(&s.tex) {
                (t.id(), Color32::from_white_alpha((a * 255.0) as u8))
            } else {
                continue;
            };
            let mut mesh = egui::Mesh::with_texture(tex_id);
            for &(pos, uv) in &s.verts {
                let uv = if tex_id == egui::TextureId::default() { egui::epaint::WHITE_UV } else { Pos2::new(uv[0], uv[1]) };
                mesh.vertices.push(egui::epaint::Vertex { pos: to_screen(pos[0], pos[1]), uv, color });
            }
            mesh.indices = s.indices.clone();
            p.add(egui::Shape::mesh(mesh));
        }
    }

    fn bottom_bar(&self, p: &egui::Painter, origin: Pos2, w: f32, h: f32) {
        let msg = match &self.status {
            Some(s) => s.clone(),
            None => self.text.get(&MSG_LAUNCH[self.sel]).cloned().unwrap_or_else(|| format!("Launch {}. <iconOK>: Confirm <iconSELECT>: Return to Title Screen", FALLBACK_NAME[self.sel])),
        };
        help_bar(p, self.tex.get("btn00").map(|(t, _)| t.id()), origin, w, h, &msg, false);
    }
}

/// The help bar with its message (`<iconOK>`-style tags drawn as btn00 glyphs). `full`: the application screens' bar,
/// full width down to the bottom edge (measured on Mail Station f01067: top y 481, highlight 481-485, text and triangle
/// at the same place as on the desktop bar).
pub(crate) fn help_bar(p: &egui::Painter, btn: Option<egui::TextureId>, origin: Pos2, w: f32, h: f32, msg: &str, full: bool) {
    {
        let r = |x0: f32, y0: f32, x1: f32, y1: f32| Rect::from_min_max(Pos2::new(origin.x + x0 * w, origin.y + y0 * h), Pos2::new(origin.x + x1 * w, origin.y + y1 * h));
        if full {
            p.rect_filled(r(0.0, 0.8889, 1.0, 1.0), 0.0, BAR_FILL);
            p.rect_filled(r(0.0, 0.8889, 1.0, 0.8963), 0.0, Color32::from_rgb(209, 210, 212));
        } else {
            let [x0, y0, x1, y1] = BAR;
            let (bx, by) = (0.004, 0.0076);
            p.rect_filled(r(x0, y0, x1, y1), 0.0, BAR_DARK);
            p.rect_filled(r(x0, y0, x1 - bx, y1 - 0.006), 0.0, BAR_LIGHT);
            p.rect_filled(r(x0 + bx, y0 + by, x1 - bx, y1 - 0.006), 0.0, BAR_FILL);
        }
        // Green triangle.
        let [tx0, ty0, tx1, ty1] = BAR_TRI;
        p.add(egui::Shape::convex_polygon(
            vec![Pos2::new(origin.x + tx0 * w, origin.y + ty0 * h), Pos2::new(origin.x + tx1 * w, origin.y + (ty0 + ty1) * 0.5 * h), Pos2::new(origin.x + tx0 * w, origin.y + ty1 * h)],
            Color32::from_rgb(30, 160, 40),
            egui::Stroke::NONE,
        ));
        let cap = (BAR_CAP[1] - BAR_CAP[0]) * h;
        // Segoe UI capitals are 0.70 of the size; matches the original's cap height and line width (measured).
        let font = egui::FontId::proportional(cap / 0.70);
        let color = Color32::from_rgb(20, 20, 20);
        let cy = origin.y + (BAR_CAP[0] + BAR_CAP[1]) * 0.5 * h;
        let mut x = origin.x + BAR_TEXT_X * w;
        let mut rest = msg;
        while !rest.is_empty() {
            let (run, tail) = match rest.find('<') {
                Some(0) => {
                    let end = rest.find('>').map(|e| e + 1).unwrap_or(rest.len());
                    let tag = &rest[1..end.saturating_sub(1)];
                    let glyph = match tag {
                        "iconOK" => Some((BTN_A, GLYPH_A_H, GLYPH_A_GAP)),
                        "iconSELECT" => Some((BTN_BACK, GLYPH_BACK_H, GLYPH_BACK_GAP)),
                        "iconCANCEL" => Some((BTN_B, GLYPH_A_H, GLYPH_A_GAP)),
                        "iconSQUARE" => Some((BTN_X, GLYPH_A_H, GLYPH_A_GAP)),
                        "iconL1" => Some((BTN_LB, GLYPH_LB_H, GLYPH_LB_GAP)),
                        "iconR1" => Some((BTN_RB, GLYPH_LB_H, GLYPH_LB_GAP)),
                        _ => None,
                    };
                    if let (Some((uv, gh, gap)), Some(id)) = (glyph, btn) {
                        let gw = gh * h * (uv[2] - uv[0]) / (uv[3] - uv[1]);
                        x += GLYPH_LEAD * w;
                        let rect = Rect::from_min_size(Pos2::new(x, cy - gh * h * 0.5 - 0.002 * h), egui::vec2(gw, gh * h));
                        p.image(id, rect, Rect::from_min_max(Pos2::new(uv[0] / 512.0, uv[1] / 512.0), Pos2::new(uv[2] / 512.0, uv[3] / 512.0)), Color32::WHITE);
                        x += gw + gap * w;
                    }
                    ("", &rest[end..])
                }
                Some(i) => (&rest[..i], &rest[i..]),
                None => (rest, ""),
            };
            if !run.is_empty() {
                let g = p.layout_no_wrap(run.to_string(), font.clone(), color);
                // Centre the capitals (not the line box) on the measured cap band.
                let top = cy - g.size().y * 0.5;
                let adv = g.size().x;
                p.galley(Pos2::new(x, top), g, color);
                x += adv;
            }
            rest = tail;
        }
    }
}

/// One line of a scripted desktop sequence. `system`: Mine OS popup (measured on f01045 for "Reboot Complete": x 386-607,
/// y 236-325, colour 42,109,117; longer messages widen it and wrap). Otherwise the player's line in a dark bar above
/// the help bar (f01053: y 463-493).
pub(crate) fn draw_dialogue(ctx: &egui::Context, btn: Option<egui::TextureId>, text: &str, system: bool) {
    let screen = ctx.screen_rect();
    let (w, h) = if screen.width() / screen.height() > 16.0 / 9.0 { (screen.height() * 16.0 / 9.0, screen.height()) } else { (screen.width(), screen.width() * 9.0 / 16.0) };
    let o = Pos2::new(screen.center().x - w * 0.5, screen.center().y - h * 0.5);
    let at = |x: f32, y: f32| Pos2::new(o.x + x / 960.0 * w, o.y + y / 540.0 * h);
    let px = h / 540.0;
    let p = ctx.layer_painter(egui::LayerId::new(egui::Order::Tooltip, egui::Id::new("altimit_dialogue")));
    let font = egui::FontId::proportional(10.5 * px / 0.70);
    let plain = strip_tags(text);
    let glyph = |c: Pos2, size: f32| {
        if let Some(id) = btn {
            p.image(id, Rect::from_center_size(c, egui::vec2(size, size)), Rect::from_min_max(Pos2::new(BTN_A[0] / 512.0, BTN_A[1] / 512.0), Pos2::new(BTN_A[2] / 512.0, BTN_A[3] / 512.0)), Color32::WHITE);
        }
    };
    if system {
        let g = p.layout(plain, font, Color32::from_rgb(225, 235, 235), 420.0 * px);
        let half_w = (g.size().x * 0.5 + 40.0 * px).max(110.5 * px);
        let half_h = (g.size().y * 0.5 + 30.0 * px).max(44.5 * px);
        let c = at(496.5, 280.5);
        let r = Rect::from_center_size(c, egui::vec2(half_w * 2.0, half_h * 2.0));
        p.rect_filled(r, 10.0 * px, Color32::from_rgba_unmultiplied(42, 109, 117, 240));
        p.galley(Pos2::new(c.x - g.size().x * 0.5, c.y - g.size().y * 0.5 + 4.5 * px), g, Color32::WHITE);
        glyph(Pos2::new(r.max.x - 33.0 * px, r.max.y - 18.0 * px), 20.0 * px);
    } else {
        p.rect_filled(Rect::from_min_max(at(40.0, 463.0), at(920.0, 493.0)), 0.0, Color32::from_rgba_unmultiplied(40, 40, 40, 235));
        p.text(at(480.0, 478.0), egui::Align2::CENTER_CENTER, &plain, font, Color32::WHITE);
        glyph(at(896.0, 478.0), 20.0 * px);
    }
}

/// Text with `<...>` markup tags removed.
pub(crate) fn strip_tags(text: &str) -> String {
    let mut out = String::new();
    let mut skip = false;
    for c in text.chars() {
        match c {
            '<' => skip = true,
            '>' => skip = false,
            _ if !skip => out.push(c),
            _ => {}
        }
    }
    out
}
