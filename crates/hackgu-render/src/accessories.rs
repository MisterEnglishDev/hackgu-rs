//! ALTIMIT Accessories: BG Setting (desktop wallpaper), built from the game's own assets.
//!
//! Data used:
//! - Wallpaper list: the DLL's accessory table (hackGU_vol1.dll file offset ~0x8779DD): 83 records
//!   `menu/xdwalNNNN` + `TEX_xdwalNNNN` + name key `accessorydata_NNN` + preview `TEX_xdwalNNNNp`, in list order
//!   (000 = xdwal0000 "Default", 056 = xdwal1530 "Nice Day", 073 = xdwal1547 (game text), checked against the
//!   pictures in reference_captures/run2_field f01206/f01207).
//! - `xddesktop@ENG.ccs`: TEX_xddactex01 "BGM Background", 02 "Background", 03 "Accessories", 05 "BG Setting",
//!   icons TEX_xddicac20 (BGM) / 30 (background), check mark TEX_xddacchk, box and arrows TEX_xddaccal.
//! - messageInfo: help 41047, "Changing..." 41162.
//!
//! Measured on f01205-f01207 (960x540): column and panel as in the Data Manager, preview (366,100)-(534,225), number
//! and name at x 534, "Background" bar y 243-270 with the owned/total count, rows at y 292 + 35 k with boxes at x
//! 385-413, selection (421..690) colour 102,150,165, arrows at x 704-714. NOT DONE: BGM (column entry only).

use std::collections::HashMap;
use std::path::Path;

use egui::{Color32, Pos2, Rect, TextureHandle};

/// Wallpaper files in the Accessories list order (DLL table).
pub const WALLPAPERS: [u16; 83] = [
    0, 1100, 1101, 1102, 1103, 1104, 1105, 1106, 1107, 1108, 1109, 1114, 1115, 1116, 1117, 1118, 1119, 1120, 1200, 1201, 1202, 1203, 1204, 1300, 1301, 1302, 1500, 1501, 1502, 1503, 1504, 1505,
    1506, 1507, 1508, 1509, 1510, 1511, 1512, 1513, 1514, 1515, 1516, 1517, 1518, 1519, 1520, 1521, 1522, 1523, 1524, 1525, 1526, 1527, 1528, 1529, 1530, 1531, 1532, 1533, 1534, 1535, 1536, 1537,
    1538, 1539, 1540, 1541, 1542, 1543, 1544, 1545, 1546, 1547, 1548, 1549, 1550, 1551, 1552, 1553, 1554, 1555, 1556,
];

const MSG_HELP: u32 = 0xD5B4776E;
const MSG_CHANGING: u32 = 0xAB26EA50;
const ROWS: usize = 6;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Mode {
    Column,
    List,
    /// "Changing..." bar for a moment before the wallpaper is applied.
    Changing(f32),
}

pub enum AcEvent {
    None,
    Close,
    SetWallpaper(usize),
}

pub struct Accessories {
    tex: HashMap<String, (TextureHandle, [usize; 2])>,
    names: Vec<String>,
    text: HashMap<u32, String>,
    mode: Mode,
    /// 0 = BGM, 1 = Background.
    background: bool,
    sel: usize,
    top: usize,
    /// Which wallpaper the "preview" texture shows.
    preview_of: Option<usize>,
    /// `xdwallsa_01.ccs`: the Vol.1 wallpaper previews (TEX_xdwalNNNNp), parsed once.
    previews: Option<hackgu_ccs::CcsFile>,
    current: usize,
}

/// Load one wallpaper's full texture into `out` under `key`.
pub fn load_wallpaper(ctx: &egui::Context, index: usize, key: &str, out: &mut HashMap<String, (TextureHandle, [usize; 2])>) {
    let Some(&n) = WALLPAPERS.get(index) else { return };
    let tex = format!("xdwal{:04}", n);
    let mut tmp = HashMap::new();
    crate::altimit::load_textures(ctx, Path::new(&format!("target/cmn/data/data/menu/xdwal{:04}.ccs", n)), &[tex.as_str()], &mut tmp);
    if let Some(t) = tmp.remove(&tex) {
        out.insert(key.to_string(), t);
    }
}

impl Accessories {
    pub fn new(ctx: &egui::Context, lang_code: &str) -> Self {
        let mut tex = HashMap::new();
        crate::altimit::load_textures(
            ctx,
            Path::new("target/cmn/data/data/xddesktop@ENG.ccs"),
            &["xddactex01", "xddactex02", "xddactex03", "xddactex05", "xddicac20", "xddicac30", "xddacchk", "xddaccal", "xddiclogo02", "xddttex03", "xddsclo"],
            &mut tex,
        );
        let mut names = Vec::new();
        let mut text = HashMap::new();
        if let Some(l) = hackgu_game::msginfo::MessageInfo::load(Path::new("target"), lang_code) {
            names = (0..WALLPAPERS.len()).map(|k| l.get(&format!("accessorydata_{:03}", k + 1)).unwrap_or("").to_string()).collect();
            for id in [MSG_HELP, MSG_CHANGING] {
                if let Some(s) = l.by_id.get(&id) {
                    text.insert(id, s.clone());
                }
            }
        }
        Self { tex, names, text, mode: Mode::Column, background: true, sel: 0, top: 0, preview_of: None, previews: None, current: 0 }
    }

    pub fn open(&mut self, current: usize) {
        // The column starts on BGM (the player moves down to Background in f01203/f01204).
        self.mode = Mode::Column;
        self.background = false;
        self.sel = current;
        self.top = current.saturating_sub(ROWS - 1).min(WALLPAPERS.len() - ROWS);
    }

    fn t(&self, id: u32) -> String {
        self.text.get(&id).cloned().unwrap_or_default()
    }

    pub fn show(&mut self, ctx: &egui::Context, btn: Option<egui::TextureId>, owned: &[bool], current: usize) -> AcEvent {
        self.current = current;
        let (up, down, ok, back, dt) = ctx.input(|i| {
            (
                i.key_pressed(egui::Key::ArrowUp) || i.key_pressed(egui::Key::W),
                i.key_pressed(egui::Key::ArrowDown) || i.key_pressed(egui::Key::S),
                i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Space),
                i.key_pressed(egui::Key::Escape) || i.key_pressed(egui::Key::Backspace),
                i.stable_dt.min(0.1),
            )
        });
        let mut ev = AcEvent::None;
        let prev_sel = self.sel;
        match self.mode {
            Mode::Column => {
                if up || down {
                    self.background = !self.background;
                }
                if ok && self.background {
                    self.mode = Mode::List;
                }
                if back {
                    ev = AcEvent::Close;
                }
            }
            Mode::List => {
                if up && self.sel > 0 {
                    self.sel -= 1;
                }
                if down && self.sel + 1 < WALLPAPERS.len() {
                    self.sel += 1;
                }
                if self.sel < self.top {
                    self.top = self.sel;
                }
                if self.sel >= self.top + ROWS {
                    self.top = self.sel + 1 - ROWS;
                }
                if ok && owned.get(self.sel).copied().unwrap_or(false) && self.sel != current {
                    self.mode = Mode::Changing(0.0);
                }
                if back {
                    self.mode = Mode::Column;
                }
            }
            Mode::Changing(t) => {
                // GUESS: the bar's duration (seen for one capture frame, f01207).
                let t = t + dt;
                if t >= 0.6 {
                    self.mode = Mode::List;
                    ev = AcEvent::SetWallpaper(self.sel);
                } else {
                    self.mode = Mode::Changing(t);
                }
                ctx.request_repaint();
            }
        }
        let _ = prev_sel;
        if self.mode != Mode::Column && owned.get(self.sel).copied().unwrap_or(false) && self.preview_of != Some(self.sel) {
            if self.previews.is_none() {
                self.previews = std::fs::read("target/cmn/data/data/xdwallsa_01.ccs").ok().and_then(|d| hackgu_ccs::CcsFile::parse(&d).ok());
            }
            if let Some(ccs) = &self.previews {
                let name = format!("xdwal{:04}p", WALLPAPERS[self.sel]);
                let mut tmp = HashMap::new();
                crate::altimit::load_textures_from(ctx, ccs, &[name.as_str()], &mut tmp);
                if let Some(t) = tmp.remove(&name) {
                    self.tex.insert("preview".into(), t);
                }
            }
            self.preview_of = Some(self.sel);
        }
        self.draw(ctx, btn, owned);
        ev
    }

    fn draw(&self, ctx: &egui::Context, btn: Option<egui::TextureId>, owned: &[bool]) {
        let screen = ctx.screen_rect();
        let (w, h) = if screen.width() / screen.height() > 16.0 / 9.0 { (screen.height() * 16.0 / 9.0, screen.height()) } else { (screen.width(), screen.width() * 9.0 / 16.0) };
        let o = Pos2::new(screen.center().x - w * 0.5, screen.center().y - h * 0.5);
        let px = h / 540.0;
        let at = |x: f32, y: f32| Pos2::new(o.x + x * px, o.y + y * px);
        let r = |x0: f32, y0: f32, x1: f32, y1: f32| Rect::from_min_max(at(x0, y0), at(x1, y1));
        let p = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("accessories")));
        let img = |name: &str, rect: Rect, uv: [f32; 4], tint: Color32| {
            if let Some((t, [tw, th])) = self.tex.get(name) {
                let (tw, th) = (*tw as f32, *th as f32);
                p.image(t.id(), rect, Rect::from_min_max(Pos2::new(uv[0] / tw, uv[1] / th), Pos2::new(uv[2] / tw, uv[3] / th)), tint);
            }
        };
        let text = |x: f32, cap_top: f32, cap: f32, s: &str, c: Color32, right: bool| {
            let g = p.layout_no_wrap(s.to_string(), egui::FontId::proportional(cap * px / 0.70), c);
            let lead = g.size().y * 0.5 - cap * px * 0.5;
            let x = if right { at(x, 0.0).x - g.size().x } else { at(x, 0.0).x };
            p.galley(Pos2::new(x, at(0.0, cap_top).y - lead), g, c);
        };
        let white = Color32::WHITE;

        // Column: dots, "BGM / Background", the two icons (the selected one framed).
        p.rect_filled(r(245.0, 0.0, 361.0, 540.0), 0.0, Color32::from_rgb(127, 134, 142));
        img("xddiclogo02", r(253.0, 43.0, 285.0, 58.0), [189.0, 68.0, 848.0, 449.0], Color32::BLACK);
        img("xddactex01", r(255.0, 55.0, 348.0, 95.0), [21.0, 30.0, 496.0, 230.0], white);
        for (k, (name, x0, y0, x1, y1)) in [("xddicac20", 277.0, 176.0, 330.0, 221.0), ("xddicac30", 291.0, 249.0, 344.0, 294.0)].into_iter().enumerate() {
            img(name, r(x0, y0, x1, y1), [0.0, 0.0, 256.0, 256.0], white);
            if (k == 1) == self.background {
                p.rect_stroke(r(x0 - 2.0, y0 - 2.0, x1 + 2.0, y1 + 2.0), 0.0, egui::Stroke::new(2.0 * px, Color32::from_rgb(110, 182, 204)), egui::StrokeKind::Inside);
            }
        }

        if self.mode != Mode::Column {
            p.rect_filled(r(361.0, 0.0, 745.0, 540.0), 0.0, Color32::from_rgb(112, 112, 112));
            img("xddactex03", r(381.0, 33.0, 545.0, 56.0), [28.0, 6.0, 921.0, 118.0], white);
            img("xddactex05", r(564.0, 31.0, 718.0, 59.0), [18.0, 12.0, 498.0, 115.0], white);
            img("xddttex03", r(472.0, 67.0, 603.0, 73.0), [10.0, 14.0, 1664.0, 107.0], white);
            img("xddttex03", r(614.0, 61.0, 682.0, 73.0), [13.0, 124.0, 812.0, 284.0], white);
            // Preview area.
            p.rect_filled(r(361.0, 84.0, 745.0, 241.0), 0.0, Color32::from_rgb(83, 93, 102));
            p.rect_filled(r(366.0, 100.0, 535.0, 226.0), 0.0, white);
            if owned.get(self.sel).copied().unwrap_or(false) {
                if let Some((t, _)) = self.tex.get("preview") {
                    // Previews are 512x512 with the 4:3 picture in the top 512x384.
                    p.image(t.id(), r(369.0, 103.0, 532.0, 223.0), Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 0.75)), white);
                }
                text(534.0, 98.0, 13.0, &format!("{:03}", self.sel), white, false);
                text(535.0, 157.0, 13.0, self.names.get(self.sel).map(|s| s.as_str()).unwrap_or(""), white, false);
            }
            // "Background" bar and the owned count.
            img("xddactex02", r(371.0, 250.0, 471.0, 268.0), [6.0, 64.0, 506.0, 172.0], white);
            let n = owned.iter().filter(|&&b| b).count();
            text(709.0, 250.0, 13.0, &format!("{}/{}", n, WALLPAPERS.len()), white, true);
            p.line_segment([at(361.0, 271.0), at(745.0, 271.0)], egui::Stroke::new(1.0 * px, Color32::from_gray(90)));
            for k in 0..ROWS {
                let i = self.top + k;
                let cy = 292.0 + 35.0 * k as f32;
                if i == self.sel {
                    p.rect_filled(r(421.0, cy - 15.0, 690.0, cy + 14.0), 0.0, Color32::from_rgb(102, 150, 165));
                }
                // Check box (TEX_xddaccal top-left frame) and the check mark on the current wallpaper.
                p.rect_stroke(r(385.0, cy - 12.0, 413.0, cy + 13.0), 0.0, egui::Stroke::new(1.5 * px, Color32::from_gray(150)), egui::StrokeKind::Inside);
                if i == self.current {
                    // f01207: red check (TEX_xddacchk) over the current wallpaper's box, x 391-408, y 284-300.
                    img("xddacchk", r(388.0, cy - 10.0, 412.0, cy + 9.0), [0.0, 0.0, 128.0, 64.0], white);
                }
                if owned.get(i).copied().unwrap_or(false) {
                    text(422.0, cy - 6.5, 13.0, &format!("{:03} {}", i, self.names.get(i).map(|s| s.as_str()).unwrap_or("")), white, false);
                } else {
                    text(429.0, cy - 6.5, 13.0, "--------", Color32::from_gray(225), false);
                }
            }
            // Blue arrows when more rows lie above / below (TEX_xddaccal: blue triangle at its top right).
            let blue = Color32::from_rgb(20, 150, 200);
            if self.top > 0 {
                p.add(egui::Shape::convex_polygon(vec![at(709.0, 284.0), at(714.0, 296.0), at(704.0, 296.0)], blue, egui::Stroke::NONE));
            }
            if self.top + ROWS < WALLPAPERS.len() {
                p.add(egui::Shape::convex_polygon(vec![at(704.0, 463.0), at(714.0, 463.0), at(709.0, 474.0)], blue, egui::Stroke::NONE));
            }
            p.rect_filled(r(727.0, 277.0, 745.0, 481.0), 0.0, Color32::from_gray(95));
            let uv = |y0: f32| [0.0, y0, 52.0, y0 + 52.0];
            img("xddsclo", r(727.0, 277.0, 745.0, 292.0), uv(52.0), white);
            img("xddsclo", r(727.0, 466.0, 745.0, 481.0), uv(0.0), white);
        }
        crate::altimit::help_bar(&p, btn, o, w, h, &self.t(MSG_HELP), false);

        if let Mode::Changing(t) = self.mode {
            // f01207: grey bar (123..836, 257..281) with "Changing..." and a progress bar.
            p.rect_filled(r(123.0, 257.0, 836.0, 282.0), 8.0 * px, Color32::from_rgb(184, 186, 185));
            p.rect_stroke(r(123.0, 257.0, 836.0, 282.0), 8.0 * px, egui::Stroke::new(1.5 * px, Color32::from_gray(60)), egui::StrokeKind::Inside);
            text(305.0, 263.0, 13.0, &self.t(MSG_CHANGING), Color32::BLACK, false);
            p.rect_filled(r(443.0, 266.0, 657.0, 275.0), 0.0, Color32::from_gray(120));
            p.rect_filled(r(443.0, 266.0, 443.0 + 214.0 * (t / 0.6).min(1.0), 275.0), 0.0, Color32::from_rgb(130, 160, 200));
        }
    }
}
