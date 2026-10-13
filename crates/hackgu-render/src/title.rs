//! Last Recode title: volume select -> volume menu (Continue / Load Data / New Game) -> Normal Game / Cheat Mode.
//!
//! Every image is a real texture from `cmn/data/data/title/xtitle_00@ENG.ccs` (cover art, cards, logo, volume
//! plates, menu items, Haseo silhouette, copyright) and `cmn/data/data/btn00@ENG@WIN64.ccs` (A/B/X glyphs); text
//! comes from `messageInfo.bin`. The positions of these Last Recode elements are not in the CCS (the title DLL
//! places them), so the layout below was measured from the original game at 2560x1440 by template-matching each
//! texture against captured screens. Coordinates are fractions of a 16:9 screen.

use std::collections::HashMap;
use std::path::Path;

use egui::{Color32, Pos2, Rect, TextureHandle};
use hackgu_game::msginfo::MessageInfo;

/// Normalised rectangle (x0, y0, x1, y1).
type NRect = [f32; 4];
/// Texture sub-rectangle in image space (u0, v0, u1, v1).
type Uv = [f32; 4];

// --- measured layout (original game, 2560x1440) ---
const ART: NRect = [0.0047, 0.0083, 0.3969, 0.9833];
const ART_UV: Uv = [0.121, 0.234, 0.875, 0.762];
const LOGO_SELECT: NRect = [0.4258, 0.1417, 0.9508, 0.3417];
const LOGO_MENU: NRect = [0.0766, 0.1417, 0.6016, 0.3417];
const LOGO_UV: Uv = [0.2109, 0.252, 0.7871, 0.748];
const PLATE_SELECT: NRect = [0.5352, 0.3944, 0.8438, 0.4653];
const PLATE_MENU: NRect = [0.1859, 0.3944, 0.4945, 0.4653];
const PLATE_U: [f32; 2] = [0.2832, 0.7168];
const ARROW_L: NRect = [0.4359, 0.3903, 0.4578, 0.4694];
const ARROW_R: NRect = [0.9301, 0.3903, 0.9520, 0.4694];
/// Card slots in screen order: Vol.1-4, Terminal Disc (card6 texture), Parody Mode (card5 texture).
const CARD_X: [f32; 6] = [0.4375, 0.5211, 0.6039, 0.6867, 0.7695, 0.8531];
const CARD_TEX: [&str; 6] = ["xtitle_card1", "xtitle_card2", "xtitle_card3", "xtitle_card4", "xtitle_card6", "xtitle_card5"];
const CARD_W: f32 = 0.1094;
const CARD_Y: f32 = 0.6347;
const CARD_Y_SELECTED: f32 = 0.5847;
const CARD_H: f32 = 0.1945;
const SILHOUETTE: NRect = [0.7406, 0.1472, 0.9445, 0.8611];
const SILHOUETTE_UV: Uv = [0.3744, 0.089, 0.8929, 0.9682];
const COPYRIGHT: NRect = [0.0500, 0.8458, 0.3789, 0.8819];
const COPYRIGHT_UV: Uv = [0.041, 0.2188, 0.6465, 0.8281];
const BOTTOM_BAR: [f32; 2] = [0.8958, 0.9472];
const VERSION_POS: [f32; 2] = [0.9453, 0.0542];
/// Menu items (screen order Continue, Load Data, New Game): texture row, centre y, ink uv.
const MENU_ITEMS: [(usize, f32, Uv); 3] = [
    (0, 0.5281, [0.3105, 0.0352, 0.6875, 0.2168]),
    (2, 0.5858, [0.2852, 0.5312, 0.7129, 0.7148]),
    (1, 0.6472, [0.2764, 0.2852, 0.7217, 0.4668]),
];
const MENU_CENTER_X: f32 = 0.3442;
/// Screen width per texture pixel of the menu item textures (New Game: 456 px -> 0.1633).
const MENU_SCALE: f32 = 0.1633 / 456.0;
const MENU_ITEM_H: f32 = 0.0583;
const NEWGAME_HEADER: NRect = [0.2648, 0.4986, 0.4297, 0.5542];
const NEWGAME_HEADER_UV: Uv = [0.2656, 0.0352, 0.7344, 0.2168];
const NEWGAME_ITEMS_Y: [f32; 2] = [0.5875, 0.6344];
const PINK_BAR_H: f32 = 0.0417;

// messageInfo ids (eng/jpn share them).
const MSG_PLAY_VOL: [u32; 6] = [0xF5685CFF, 0xF1A94148, 0xFCEA6791, 0xF82B7A26, 0xE66C2A23, 0xE2AD3794];
const MSG_LOAD_LAST: u32 = 0x85BE8AC4;
const MSG_NO_DATA: u32 = 0xB9980433;
const MSG_LOAD_DATA_HELP: u32 = 0x69EEFF42;
const MSG_NEW_GAME_HELP: u32 = 0x9517615D;
const MSG_NORMAL: u32 = 0xCA43053F;
const MSG_CHEAT: u32 = 0xD94773E3;
const MSG_NORMAL_HELP: u32 = 0x8B2D7168;
const MSG_CHEAT_HELP: u32 = 0x982907B4;
const MSG_CONFIRM: u32 = 0xE27198F3;
const MSG_OPTIONS: u32 = 0x7175D70E;
const MSG_RETURN: u32 = 0xD07EFEF2;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TitleState {
    VolumeSelect { sel: usize },
    VolumeMenu { cursor: usize },
    NewGame { cursor: usize },
}

/// What the title asks the game to do this frame.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TitleAction {
    None,
    StartNewGame { cheat: bool },
    /// "Load last save" / "Load data": resume the most recent save slot.
    LoadLast,
}

pub struct TitleScreen {
    pub state: TitleState,
    tex: HashMap<String, (TextureHandle, [usize; 2])>,
    text: HashMap<u32, String>,
    pub status: Option<String>,
    anim: TitleAnim,
}

/// Transition state. Timings follow the title's own animation lengths in xtitle_00 (menu items: 8-frame `_01`
/// in-animations, 12-frame screen pieces, at 30 fps); GUESS which animation drives which motion (the title DLL that
/// sequences them is not traced).
#[derive(Default)]
struct TitleAnim {
    /// Seconds since the current screen opened.
    screen_t: f32,
    last_screen: u8,
    /// Pink bar centre (fraction of height), eased toward the selected item.
    bar_y: Option<f32>,
    /// Card heights (fraction of height), eased toward selected / unselected.
    card_y: [Option<f32>; 6],
    /// Volume art cross-fade: (previous volume, seconds since change).
    art_prev: Option<(usize, f32)>,
    last_sel: Option<usize>,
}

/// 8 frames at 30 fps (menu item in-animation length).
const STEP_SECS: f32 = 8.0 / 30.0;
/// 12 frames at 30 fps (screen piece animation length).
const SCREEN_SECS: f32 = 12.0 / 30.0;

/// Move `cur` toward `target` with an exponential ease that settles in about `secs`.
fn ease(cur: Option<f32>, target: f32, dt: f32, secs: f32) -> f32 {
    match cur {
        Some(c) => c + (target - c) * (1.0 - (-dt * 4.6 / secs).exp()),
        None => target,
    }
}

fn load_textures(ctx: &egui::Context, path: &Path, names: &[&str], out: &mut HashMap<String, (TextureHandle, [usize; 2])>) {
    let Some(ccs) = std::fs::read(path).ok().and_then(|d| hackgu_ccs::CcsFile::parse(&d).ok()) else {
        eprintln!("title: cannot read {}", path.display());
        return;
    };
    for t in ccs.objects_of_type(hackgu_ccs::objects::SECTION_TEXTURE) {
        let name = ccs.object_name(t.object_id).unwrap_or("").trim_start_matches("TEX_").to_string();
        if !names.contains(&name.as_str()) {
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

impl TitleScreen {
    pub fn new(ctx: &egui::Context, lang_code: &str) -> Self {
        let lang_db = MessageInfo::load(Path::new("target"), lang_code);
        let lang = lang_db.as_ref();
        let mut tex = HashMap::new();
        let mut names: Vec<&str> = vec!["xtitle_18", "xtitle_10", "xtitle_16r", "xtitle_17r", "xtitle_19", "xtitle_13r", "xtitle_14r", "xtitle_15r", "xtitle_card7"];
        names.extend(["xtitle_pic1", "xtitle_pic2", "xtitle_pic3", "xtitle_pic4", "xtitle_pic5", "xtitle_pic6"]);
        names.extend(["xtitle_hsw1", "xtitle_hsw2", "xtitle_hsw3", "xtitle_hsw4", "xtitle_hsw5"]);
        names.extend(CARD_TEX);
        load_textures(ctx, Path::new("target/cmn/data/data/title/xtitle_00@ENG.ccs"), &names, &mut tex);
        load_textures(ctx, Path::new("target/cmn/data/data/btn00@ENG@WIN64.ccs"), &["btn00"], &mut tex);
        let mut text = HashMap::new();
        if let Some(l) = lang {
            for id in MSG_PLAY_VOL.iter().chain(&[MSG_LOAD_LAST, MSG_LOAD_DATA_HELP, MSG_NEW_GAME_HELP, MSG_NORMAL, MSG_CHEAT, MSG_NORMAL_HELP, MSG_CHEAT_HELP, MSG_CONFIRM, MSG_OPTIONS, MSG_RETURN, MSG_NO_DATA]) {
                if let Some(s) = l.by_id.get(id) {
                    text.insert(*id, s.clone());
                }
            }
        }
        Self { state: TitleState::VolumeSelect { sel: 0 }, tex, text, status: None, anim: TitleAnim::default() }
    }

    pub fn no_data_text(&self) -> Option<String> {
        Some(self.t(MSG_NO_DATA, "There is no data to continue from."))
    }

    fn t(&self, id: u32, fallback: &str) -> String {
        self.text.get(&id).cloned().unwrap_or_else(|| fallback.to_string())
    }

    /// Handle input and draw. Returns what the game should do.
    pub fn show(&mut self, ctx: &egui::Context) -> TitleAction {
        let mut action = TitleAction::None;
        let (left, right, up, down, ok, back) = ctx.input(|i| {
            (
                i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::A),
                i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::D),
                i.key_pressed(egui::Key::ArrowUp) || i.key_pressed(egui::Key::W),
                i.key_pressed(egui::Key::ArrowDown) || i.key_pressed(egui::Key::S),
                i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Space),
                i.key_pressed(egui::Key::Escape) || i.key_pressed(egui::Key::Backspace),
            )
        });
        self.state = match self.state {
            TitleState::VolumeSelect { mut sel } => {
                if left {
                    sel = (sel + 5) % 6;
                }
                if right {
                    sel = (sel + 1) % 6;
                }
                if ok {
                    if sel == 0 {
                        self.status = None;
                        TitleState::VolumeMenu { cursor: 2 }
                    } else {
                        self.status = Some("Only Vol. 1 is implemented.".into());
                        TitleState::VolumeSelect { sel }
                    }
                } else {
                    TitleState::VolumeSelect { sel }
                }
            }
            TitleState::VolumeMenu { mut cursor } => {
                if up {
                    cursor = (cursor + 2) % 3;
                }
                if down {
                    cursor = (cursor + 1) % 3;
                }
                if back {
                    TitleState::VolumeSelect { sel: 0 }
                } else if ok && cursor == 2 {
                    TitleState::NewGame { cursor: 0 }
                } else {
                    if ok {
                        // GUESS: "Load data" should open a slot list; both entries resume the newest save for now.
                        action = TitleAction::LoadLast;
                    }
                    TitleState::VolumeMenu { cursor }
                }
            }
            TitleState::NewGame { mut cursor } => {
                if up || down {
                    cursor = 1 - cursor;
                }
                if back {
                    TitleState::VolumeMenu { cursor: 2 }
                } else if ok {
                    action = TitleAction::StartNewGame { cheat: cursor == 1 };
                    TitleState::VolumeSelect { sel: 0 }
                } else {
                    TitleState::NewGame { cursor }
                }
            }
        };
        self.animate(ctx);
        self.draw(ctx);
        action
    }

    fn animate(&mut self, ctx: &egui::Context) {
        let dt = ctx.input(|i| i.stable_dt.min(0.1));
        let a = &mut self.anim;
        let screen_id = match self.state {
            TitleState::VolumeSelect { .. } => 0,
            TitleState::VolumeMenu { .. } => 1,
            TitleState::NewGame { .. } => 2,
        };
        if screen_id != a.last_screen {
            a.last_screen = screen_id;
            a.screen_t = 0.0;
            a.bar_y = None;
        } else {
            a.screen_t += dt;
        }
        let bar_target = match self.state {
            TitleState::VolumeMenu { cursor } => Some(MENU_ITEMS[cursor].1),
            TitleState::NewGame { cursor } => Some(NEWGAME_ITEMS_Y[cursor]),
            _ => None,
        };
        if let Some(t) = bar_target {
            a.bar_y = Some(ease(a.bar_y, t, dt, STEP_SECS));
        }
        if let TitleState::VolumeSelect { sel } = self.state {
            for i in 0..6 {
                let t = if i == sel { CARD_Y_SELECTED } else { CARD_Y };
                a.card_y[i] = Some(ease(a.card_y[i], t, dt, STEP_SECS));
            }
            if a.last_sel.is_some_and(|l| l != sel) {
                a.art_prev = a.last_sel.map(|l| (l, 0.0));
            }
            a.last_sel = Some(sel);
            if let Some((v, t)) = a.art_prev {
                a.art_prev = if t + dt >= SCREEN_SECS { None } else { Some((v, t + dt)) };
            }
        }
        if a.screen_t < SCREEN_SECS || a.art_prev.is_some() || a.bar_y.zip(bar_target).is_some_and(|(b, t)| (b - t).abs() > 0.0005) || a.card_y.iter().enumerate().any(|(i, y)| {
            let t = match self.state { TitleState::VolumeSelect { sel } if sel == i => CARD_Y_SELECTED, _ => CARD_Y };
            y.is_some_and(|y| (y - t).abs() > 0.0005)
        }) {
            ctx.request_repaint();
        }
    }

    fn draw(&self, ctx: &egui::Context) {
        let screen = ctx.screen_rect();
        // 16:9 area centred in the window.
        let (w, h) = if screen.width() / screen.height() > 16.0 / 9.0 {
            (screen.height() * 16.0 / 9.0, screen.height())
        } else {
            (screen.width(), screen.width() * 9.0 / 16.0)
        };
        let origin = Pos2::new(screen.center().x - w * 0.5, screen.center().y - h * 0.5);
        let r = |n: NRect| Rect::from_min_max(Pos2::new(origin.x + n[0] * w, origin.y + n[1] * h), Pos2::new(origin.x + n[2] * w, origin.y + n[3] * h));
        let p = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("lr_title")));
        p.rect_filled(screen, 0.0, Color32::WHITE);

        let img = |name: &str, rect: Rect, uv: Uv, tint: Color32| {
            if let Some((h, _)) = self.tex.get(name) {
                p.image(h.id(), rect, Rect::from_min_max(Pos2::new(uv[0], uv[1]), Pos2::new(uv[2], uv[3])), tint);
            }
        };
        let full: Uv = [0.0, 0.0, 1.0, 1.0];
        let plate_uv = |vol: usize| -> Uv { [PLATE_U[0], vol as f32 * 0.25 + 0.0137, PLATE_U[1], vol as f32 * 0.25 + 0.2383] };

        match self.state {
            TitleState::VolumeSelect { sel } => {
                if let Some((prev, t)) = self.anim.art_prev {
                    img(&format!("xtitle_pic{}", prev + 1), r(ART), ART_UV, Color32::WHITE);
                    let k = (t / SCREEN_SECS).clamp(0.0, 1.0);
                    img(&format!("xtitle_pic{}", sel + 1), r(ART), ART_UV, Color32::from_white_alpha((k * 255.0) as u8));
                } else {
                    img(&format!("xtitle_pic{}", sel + 1), r(ART), ART_UV, Color32::WHITE);
                }
                img("xtitle_10", r(LOGO_SELECT), LOGO_UV, Color32::WHITE);
                if sel < 4 {
                    img("xtitle_16r", r(PLATE_SELECT), plate_uv(sel), Color32::WHITE);
                }
                img("xtitle_17r", r(COPYRIGHT), COPYRIGHT_UV, Color32::WHITE);
                img("xtitle_19", r(ARROW_L), full, Color32::WHITE);
                img("xtitle_19", r(ARROW_R), [1.0, 0.0, 0.0, 1.0], Color32::WHITE);
                for (i, (&x, &name)) in CARD_X.iter().zip(CARD_TEX.iter()).enumerate() {
                    let y = self.anim.card_y[i].unwrap_or(if i == sel { CARD_Y_SELECTED } else { CARD_Y });
                    let rect = r([x, y, x + CARD_W, y + CARD_H]);
                    img(name, rect, full, Color32::WHITE);
                    if i == sel {
                        img("xtitle_card7", rect, full, Color32::WHITE);
                        // Red check badge, measured at the selected card's top-left corner.
                        img("xtitle_18", r([x + 0.0141, y - 0.0222, x + 0.0391, y + 0.0222]), full, Color32::WHITE);
                    }
                }
                self.prompts(&p, origin, w, h, &[(3, self.t(MSG_CONFIRM, "Confirm")), (2, self.t(MSG_OPTIONS, "Options")), (0, "Quit Game".into())]);
                let help = self.status.clone().unwrap_or_else(|| self.t(MSG_PLAY_VOL[sel], ""));
                self.bottom_bar(&p, screen, origin, w, h, &help);
                p.text(Pos2::new(origin.x + VERSION_POS[0] * w, origin.y + VERSION_POS[1] * h), egui::Align2::RIGHT_TOP, "Ver. 1.1.01", bold(0.027 * h), Color32::from_rgb(20, 20, 20));
            }
            TitleState::VolumeMenu { cursor } => {
                self.menu_backdrop(&p, &img, &r, &plate_uv);
                let (_, cy, _) = MENU_ITEMS[cursor];
                self.pink_bar(&p, screen, origin, w, h, self.anim.bar_y.unwrap_or(cy));
                for (i, &(row, y, uv)) in MENU_ITEMS.iter().enumerate() {
                    let (tex, tint) = if i == cursor { ("xtitle_13r", Color32::WHITE) } else { ("xtitle_14r", Color32::from_white_alpha(110)) };
                    let half_w = (uv[2] - uv[0]) * 1024.0 * MENU_SCALE * 0.5;
                    let _ = row;
                    img(tex, r([MENU_CENTER_X - half_w, y - MENU_ITEM_H * 0.5, MENU_CENTER_X + half_w, y + MENU_ITEM_H * 0.5]), uv, tint);
                }
                self.prompts(&p, origin, w, h, &[(3, self.t(MSG_CONFIRM, "Confirm")), (0, self.t(MSG_RETURN, "Return"))]);
                let help = self.status.clone().unwrap_or_else(|| match cursor {
                    0 => self.t(MSG_LOAD_LAST, ""),
                    1 => self.t(MSG_LOAD_DATA_HELP, ""),
                    _ => self.t(MSG_NEW_GAME_HELP, ""),
                });
                self.bottom_bar(&p, screen, origin, w, h, &help);
            }
            TitleState::NewGame { cursor } => {
                self.menu_backdrop(&p, &img, &r, &plate_uv);
                img("xtitle_15r", r(NEWGAME_HEADER), NEWGAME_HEADER_UV, Color32::WHITE);
                self.pink_bar(&p, screen, origin, w, h, self.anim.bar_y.unwrap_or(NEWGAME_ITEMS_Y[cursor]));
                for (i, (id, fb)) in [(MSG_NORMAL, "Normal Game"), (MSG_CHEAT, "Cheat Mode")].iter().enumerate() {
                    let pos = Pos2::new(origin.x + MENU_CENTER_X * w, origin.y + NEWGAME_ITEMS_Y[i] * h);
                    let (fill, edge) = if i == cursor { (Color32::from_rgb(10, 10, 10), Color32::from_rgb(235, 235, 235)) } else { (Color32::from_rgb(110, 110, 110), Color32::from_rgb(235, 235, 235)) };
                    outlined_text(&p, pos, &self.t(*id, fb), 0.044 * h, fill, edge);
                }
                self.prompts(&p, origin, w, h, &[(3, self.t(MSG_CONFIRM, "Confirm")), (0, self.t(MSG_RETURN, "Return"))]);
                let help = if cursor == 0 { self.t(MSG_NORMAL_HELP, "") } else { self.t(MSG_CHEAT_HELP, "") };
                self.bottom_bar(&p, screen, origin, w, h, help.lines().next().unwrap_or(""));
            }
        }
        // Each screen comes in from the white background.
        let k = (self.anim.screen_t / SCREEN_SECS).clamp(0.0, 1.0);
        if k < 1.0 {
            let e = 1.0 - k * k * (3.0 - 2.0 * k);
            p.rect_filled(screen, 0.0, Color32::from_white_alpha((e * 255.0) as u8));
        }
    }

    fn menu_backdrop(&self, _p: &egui::Painter, img: &dyn Fn(&str, Rect, Uv, Color32), r: &dyn Fn(NRect) -> Rect, plate_uv: &dyn Fn(usize) -> Uv) {
        img("xtitle_hsw1", r(SILHOUETTE), SILHOUETTE_UV, Color32::WHITE);
        img("xtitle_10", r(LOGO_MENU), LOGO_UV, Color32::WHITE);
        img("xtitle_16r", r(PLATE_MENU), plate_uv(0), Color32::WHITE);
        img("xtitle_17r", r(COPYRIGHT), COPYRIGHT_UV, Color32::WHITE);
    }

    /// Selection bar: fades in from the left edge, full pink from x = 0.25, crimson lines near its top.
    fn pink_bar(&self, p: &egui::Painter, screen: Rect, origin: Pos2, w: f32, h: f32, center_y: f32) {
        let y0 = origin.y + (center_y - PINK_BAR_H * 0.5) * h;
        let y1 = y0 + PINK_BAR_H * h;
        let pink = Color32::from_rgb(200, 78, 138);
        let fade_end = origin.x + 0.25 * w;
        let steps = 24;
        for i in 0..steps {
            let a = i as f32 / steps as f32;
            let xa = screen.left() + (fade_end - screen.left()) * a;
            let xb = screen.left() + (fade_end - screen.left()) * (a + 1.0 / steps as f32);
            let c = lerp_color(Color32::from_rgb(248, 231, 239), pink, a + 0.5 / steps as f32);
            p.rect_filled(Rect::from_min_max(Pos2::new(xa, y0), Pos2::new(xb + 0.5, y1)), 0.0, c);
        }
        p.rect_filled(Rect::from_min_max(Pos2::new(fade_end, y0), Pos2::new(screen.right(), y1)), 0.0, pink);
        let line = |a: f32, b: f32, c: Color32| {
            p.rect_filled(Rect::from_min_max(Pos2::new(screen.left(), y0 + a * h), Pos2::new(screen.right(), y0 + b * h)), 0.0, c);
        };
        line(0.0014, 0.0090, Color32::from_rgb(155, 10, 68));
        line(0.0174, 0.0201, Color32::from_rgb(167, 12, 72));
    }

    fn bottom_bar(&self, p: &egui::Painter, screen: Rect, origin: Pos2, _w: f32, h: f32, text: &str) {
        let y0 = origin.y + BOTTOM_BAR[0] * h;
        let y1 = origin.y + BOTTOM_BAR[1] * h;
        p.rect_filled(Rect::from_min_max(Pos2::new(screen.left(), y0), Pos2::new(screen.right(), y1)), 0.0, Color32::from_black_alpha(189));
        p.text(Pos2::new(origin.x + 0.034 * _w, (y0 + y1) * 0.5), egui::Align2::LEFT_CENTER, text, bold(0.03 * h), Color32::WHITE);
    }

    /// Right-aligned button prompts; `btn` indexes the first row of btn00 (0 B, 1 Y, 2 X, 3 A).
    fn prompts(&self, p: &egui::Painter, origin: Pos2, w: f32, h: f32, items: &[(usize, String)]) {
        let Some((btn, _)) = self.tex.get("btn00") else { return };
        let glyph_h = 0.047 * h;
        let font = bold(0.033 * h);
        let color = Color32::from_rgb(17, 17, 17);
        let right = origin.x + 0.9488 * w;
        let cy = origin.y + 0.8653 * h;
        let mut x = right;
        for (b, label) in items.iter().rev() {
            let galley = p.layout_no_wrap(format!(": {}", label), font.clone(), color);
            x -= galley.size().x;
            p.galley(Pos2::new(x, cy - galley.size().y * 0.5), galley, color);
            x -= glyph_h + 0.002 * w;
            let u0 = [5.0, 85.0, 165.0, 245.0][*b] / 512.0;
            p.image(btn.id(), Rect::from_min_size(Pos2::new(x, cy - glyph_h * 0.5), egui::vec2(glyph_h, glyph_h)), Rect::from_min_max(Pos2::new(u0, 4.0 / 512.0), Pos2::new(u0 + 70.0 / 512.0, 75.0 / 512.0)), Color32::WHITE);
            x -= 0.012 * w;
        }
    }
}

fn bold(size: f32) -> egui::FontId {
    egui::FontId::new(size, egui::FontFamily::Name("ui_bold".into()))
}

fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let l = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t.clamp(0.0, 1.0)) as u8;
    Color32::from_rgb(l(a.r(), b.r()), l(a.g(), b.g()), l(a.b(), b.b()))
}

fn outlined_text(p: &egui::Painter, center: Pos2, text: &str, size: f32, fill: Color32, edge: Color32) {
    let font = bold(size);
    let o = (size * 0.06).max(1.0);
    for (dx, dy) in [(-o, 0.0), (o, 0.0), (0.0, -o), (0.0, o), (-o, -o), (o, o), (-o, o), (o, -o)] {
        p.text(center + egui::vec2(dx, dy), egui::Align2::CENTER_CENTER, text, font.clone(), edge);
    }
    p.text(center, egui::Align2::CENTER_CENTER, text, font, fill);
}
