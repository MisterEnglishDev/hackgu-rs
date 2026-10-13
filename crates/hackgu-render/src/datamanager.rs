//! ALTIMIT Data Manager (Save / Load), built from the game's own assets, and the save files behind it.
//!
//! Data used: `xddesktop@ENG.ccs` (TEX_xddsltex01 "Save/Load", 02 "Data Manager", 03 "Save", 04 "Load", icons
//! TEX_xddicsl10/20, TEX_xddiclogo02 dots, TEX_xddttex03 Mine OS lines); messageInfo strings (No./Date/Play Time
//! 41180-41182, Level 17900, Save Data 56259, "Saved." 56263, overwrite question 44523, Yes/No 41220/41221, help
//! lines 41052-41055, chapter names 43455.., location names 41183..).
//!
//! Measured on reference_captures/run2_field f01213-f01215 (960x540): the column (x 245-361, colour 127,134,142),
//! the panel (x 361-745, rows 112 grey, "Save Data" bar y 86-112 colour 62,112,121), slot rows (centres 225 + 29.4 k),
//! the popup (143,124)-(818,325) colour 45,118,125, text positions. The desktop ring stays where it is on the
//! desktop. GUESS: the selected-slot highlight (hidden under the popup in the captures) and the Load flow (not
//! captured).
//!
//! Save files are this project's own JSON (`saves/slotNN.json`), not the original's format.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use egui::{Color32, Pos2, Rect, TextureHandle};
use serde::{Deserialize, Serialize};

pub const SLOTS: usize = 9;

/// Chapter names (messageInfo 43455..): First Cry, Scars, Deja Vu, ...
pub const CHAPTER_FIRST_CRY: u32 = 0x1DE4EA55;
pub const CHAPTER_DEJA_VU: u32 = 0x1466D13B;
/// Location "Desktop" (41183).
pub const LOCATION_DESKTOP: u32 = 0x8AF6E7B2;

const MSG_NO: u32 = 0xDD59C6F0;
/// "Level" (17900).
const MSG_LEVEL: u32 = 0xAFBB6C7E;
const MSG_DATE: u32 = 0xD59EBE79;
const MSG_PLAY_TIME: u32 = 0x410BC73B;
const MSG_SAVE_DATA: u32 = 0xBC72F5BD;
const MSG_LOAD_DATA: u32 = 0xB1FC3C7B;
const MSG_SAVED: u32 = 0xAA365BA6;
const MSG_OVERWRITE: u32 = 0x7885B333;
const MSG_YES: u32 = 0x8F88A217;
const MSG_NO_ANSWER: u32 = 0x18940D56;
const MSG_HELP_SAVE_SLOT: u32 = 0xB85188B6;
const MSG_HELP_LOAD_SLOT: u32 = 0xBC909501;
const MSG_HELP_SAVE: u32 = 0x9EFAF3BE;
const MSG_HELP_LOAD: u32 = 0xA5CFCF88;
const MSG_FINISHED_LOADING: u32 = 0x75C695EA;
const MSG_FAILED_SAVE: u32 = 0x6F40D858;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SavedMail {
    pub body: u32,
    pub unread: bool,
}

/// One save slot (this project's format).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SaveFile {
    pub format: u32,
    /// Chapter name (messageInfo id) shown over the slot list.
    pub chapter: u32,
    /// Location name (messageInfo id).
    pub location: u32,
    pub level: u32,
    pub play_time: f64,
    /// MM/DD/YY as in the original's list (UTC).
    pub date: String,
    /// Seconds since the Unix epoch (picks "Load last save").
    pub saved_at: u64,
    /// Desktop story state (`altimit::Story` name).
    pub story: String,
    pub inbox: Vec<SavedMail>,
    pub new_badge: [bool; 8],
    pub reformatted: bool,
    /// Desktop wallpaper and owned wallpapers (indices into accessories::WALLPAPERS).
    #[serde(default)]
    pub wallpaper: usize,
    #[serde(default)]
    pub wallpapers: Vec<usize>,
    /// Events played so far (arms the next town events on load).
    #[serde(default)]
    pub events_done: Vec<String>,
}

pub fn slot_path(slot: usize) -> PathBuf {
    PathBuf::from(format!("saves/slot{:02}.json", slot + 1))
}

pub fn read_slot(slot: usize) -> Option<SaveFile> {
    serde_json::from_str(&std::fs::read_to_string(slot_path(slot)).ok()?).ok()
}

pub fn write_slot(slot: usize, f: &SaveFile) -> std::io::Result<()> {
    std::fs::create_dir_all("saves")?;
    std::fs::write(slot_path(slot), serde_json::to_string_pretty(f).unwrap_or_default())
}

/// The most recently written slot.
pub fn latest_slot() -> Option<(usize, SaveFile)> {
    (0..SLOTS).filter_map(|s| read_slot(s).map(|f| (s, f))).max_by_key(|(_, f)| f.saved_at)
}

/// (seconds since epoch, "MM/DD/YY") from the system clock, UTC.
pub fn now_date() -> (u64, String) {
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    // Civil date from days since 1970-01-01 (H. Hinnant's algorithm).
    let z = (secs / 86400) as i64 + 719468;
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + if m <= 2 { 1 } else { 0 };
    (secs, format!("{:02}/{:02}/{:02}", m, d, y % 100))
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Mode {
    /// The Save/Load column only.
    Column,
    Slots,
    Confirm { yes: bool },
    Message(u32),
}

pub enum DmEvent {
    None,
    Close,
    /// Write the current game to this slot (the desktop builds the SaveFile).
    Save(usize),
    Load(SaveFile),
}

pub struct DataManager {
    tex: HashMap<String, (TextureHandle, [usize; 2])>,
    text: HashMap<u32, String>,
    mode: Mode,
    /// 0 = Save, 1 = Load.
    load: bool,
    slot: usize,
    slots: Vec<Option<SaveFile>>,
}

impl DataManager {
    pub fn new(ctx: &egui::Context, lang_code: &str) -> Self {
        let mut tex = HashMap::new();
        crate::altimit::load_textures(
            ctx,
            Path::new("target/cmn/data/data/xddesktop@ENG.ccs"),
            &["xddsltex01", "xddsltex02", "xddsltex03", "xddsltex04", "xddicsl10", "xddicsl20", "xddiclogo02", "xddttex03", "xddsclo"],
            &mut tex,
        );
        crate::altimit::load_textures(ctx, Path::new("target/cmn/data/data/btn00@ENG@WIN64.ccs"), &["btn00"], &mut tex);
        let ids = [
            MSG_NO, MSG_LEVEL, MSG_DATE, MSG_PLAY_TIME, MSG_SAVE_DATA, MSG_LOAD_DATA, MSG_SAVED, MSG_OVERWRITE, MSG_YES, MSG_NO_ANSWER, MSG_HELP_SAVE_SLOT, MSG_HELP_LOAD_SLOT, MSG_HELP_SAVE,
            MSG_HELP_LOAD, MSG_FINISHED_LOADING, MSG_FAILED_SAVE, CHAPTER_FIRST_CRY, CHAPTER_DEJA_VU, LOCATION_DESKTOP,
        ];
        let mut text = HashMap::new();
        if let Some(l) = hackgu_game::msginfo::MessageInfo::load(Path::new("target"), lang_code) {
            for id in ids {
                if let Some(s) = l.by_id.get(&id) {
                    text.insert(id, s.clone());
                }
            }
        }
        Self { tex, text, mode: Mode::Column, load: false, slot: 0, slots: Vec::new() }
    }

    pub fn open(&mut self) {
        self.mode = Mode::Column;
        self.load = false;
    }

    fn refresh(&mut self) {
        self.slots = (0..SLOTS).map(read_slot).collect();
    }

    fn t(&self, id: u32) -> String {
        self.text.get(&id).cloned().unwrap_or_default()
    }

    /// After the desktop wrote the slot: report the result.
    pub fn saved(&mut self, ok: bool) {
        self.refresh();
        self.mode = Mode::Message(if ok { MSG_SAVED } else { MSG_FAILED_SAVE });
    }

    /// Input, then draw over the desktop (which has drawn the wallpaper and ring).
    pub fn show(&mut self, ctx: &egui::Context, btn: Option<egui::TextureId>) -> DmEvent {
        let (up, down, left, right, ok, back) = ctx.input(|i| {
            (
                i.key_pressed(egui::Key::ArrowUp) || i.key_pressed(egui::Key::W),
                i.key_pressed(egui::Key::ArrowDown) || i.key_pressed(egui::Key::S),
                i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::A),
                i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::D),
                i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Space),
                i.key_pressed(egui::Key::Escape) || i.key_pressed(egui::Key::Backspace),
            )
        });
        let mut ev = DmEvent::None;
        match self.mode {
            Mode::Column => {
                if up || down {
                    self.load = !self.load;
                }
                if ok {
                    self.refresh();
                    self.mode = Mode::Slots;
                }
                if back {
                    ev = DmEvent::Close;
                }
            }
            Mode::Slots => {
                if up && self.slot > 0 {
                    self.slot -= 1;
                }
                if down && self.slot + 1 < SLOTS {
                    self.slot += 1;
                }
                if ok {
                    let filled = self.slots.get(self.slot).is_some_and(|s| s.is_some());
                    if self.load {
                        if let Some(Some(f)) = self.slots.get(self.slot) {
                            ev = DmEvent::Load(f.clone());
                        }
                    } else if filled {
                        self.mode = Mode::Confirm { yes: true };
                    } else {
                        ev = DmEvent::Save(self.slot);
                    }
                }
                if back {
                    self.mode = Mode::Column;
                }
            }
            Mode::Confirm { yes } => {
                if left || right {
                    self.mode = Mode::Confirm { yes: !yes };
                }
                if ok {
                    if yes {
                        ev = DmEvent::Save(self.slot);
                    } else {
                        self.mode = Mode::Slots;
                    }
                }
                if back {
                    self.mode = Mode::Slots;
                }
            }
            Mode::Message(_) => {
                if ok || back {
                    self.mode = Mode::Slots;
                }
            }
        }
        self.draw(ctx, btn);
        ev
    }

    fn draw(&self, ctx: &egui::Context, btn: Option<egui::TextureId>) {
        let screen = ctx.screen_rect();
        let (w, h) = if screen.width() / screen.height() > 16.0 / 9.0 { (screen.height() * 16.0 / 9.0, screen.height()) } else { (screen.width(), screen.width() * 9.0 / 16.0) };
        let o = Pos2::new(screen.center().x - w * 0.5, screen.center().y - h * 0.5);
        let px = h / 540.0;
        let at = |x: f32, y: f32| Pos2::new(o.x + x * px, o.y + y * px);
        let r = |x0: f32, y0: f32, x1: f32, y1: f32| Rect::from_min_max(at(x0, y0), at(x1, y1));
        let p = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("datamanager")));
        let img = |name: &str, rect: Rect, uv: [f32; 4], tint: Color32| {
            if let Some((t, [tw, th])) = self.tex.get(name) {
                let (tw, th) = (*tw as f32, *th as f32);
                p.image(t.id(), rect, Rect::from_min_max(Pos2::new(uv[0] / tw, uv[1] / th), Pos2::new(uv[2] / tw, uv[3] / th)), tint);
            }
        };
        let font = |cap: f32| egui::FontId::proportional(cap * px / 0.70);
        // Text with its capitals' top at `cap_top`.
        let text = |x: f32, cap_top: f32, cap: f32, s: &str, c: Color32, align: egui::Align| {
            let g = p.layout_no_wrap(s.to_string(), font(cap), c);
            let lead = g.size().y * 0.5 - cap * px * 0.5;
            let x = match align {
                egui::Align::Min => at(x, 0.0).x,
                egui::Align::Center => at(x, 0.0).x - g.size().x * 0.5,
                egui::Align::Max => at(x, 0.0).x - g.size().x,
            };
            p.galley(Pos2::new(x, at(0.0, cap_top).y - lead), g, c);
        };
        let white = Color32::WHITE;

        // Column: Save/Load label, dots, the two icons (the selected one framed).
        p.rect_filled(r(245.0, 0.0, 361.0, 540.0), 0.0, Color32::from_rgb(127, 134, 142));
        img("xddiclogo02", r(259.0, 42.0, 291.0, 59.0), [189.0, 68.0, 848.0, 449.0], Color32::BLACK);
        img("xddsltex01", r(257.0, 66.0, 345.0, 78.0), alpha_box_saveload(), Color32::WHITE);
        for (k, (name, y0, y1)) in [("xddicsl10", 175.0, 221.0), ("xddicsl20", 248.0, 293.0)].into_iter().enumerate() {
            img(name, r(277.0, y0, 330.0, y1), [0.0, 0.0, 256.0, 256.0], Color32::WHITE);
            if (k == 1) == self.load {
                p.rect_stroke(r(275.0, y0 - 2.0, 332.0, y1 + 2.0), 0.0, egui::Stroke::new(2.0 * px, Color32::from_rgb(110, 182, 204)), egui::StrokeKind::Inside);
            }
        }

        if self.mode != Mode::Column {
            // Panel.
            p.rect_filled(r(361.0, 0.0, 745.0, 540.0), 0.0, Color32::from_rgb(112, 112, 112));
            img("xddsltex02", r(404.0, 35.0, 588.0, 59.0), [8.0, 14.0, 650.0, 114.0], white);
            if self.load {
                img("xddsltex04", r(606.0, 30.0, 684.0, 53.0), [79.0, 6.0, 437.0, 119.0], white);
            } else {
                img("xddsltex03", r(606.0, 30.0, 684.0, 53.0), [83.0, 3.0, 429.0, 122.0], white);
            }
            img("xddttex03", r(472.0, 67.0, 603.0, 73.0), [10.0, 14.0, 1664.0, 107.0], white);
            img("xddttex03", r(614.0, 61.0, 682.0, 73.0), [13.0, 124.0, 812.0, 284.0], white);
            p.rect_filled(r(361.0, 86.0, 745.0, 113.0), 0.0, Color32::from_rgb(62, 112, 121));
            text(367.0, 95.0, 13.0, &self.t(if self.load { MSG_LOAD_DATA } else { MSG_SAVE_DATA }), white, egui::Align::Min);
            let sel = self.slots.get(self.slot).cloned().flatten();
            if let Some(f) = &sel {
                text(376.0, 140.0, 13.0, &self.t(f.chapter), Color32::from_gray(200), egui::Align::Min);
            }
            let head = Color32::from_gray(200);
            text(382.0, 184.0, 12.0, &self.t(MSG_NO), head, egui::Align::Min);
            text(456.0, 184.0, 12.0, &self.t(MSG_LEVEL), head, egui::Align::Center);
            text(551.0, 184.0, 12.0, &self.t(MSG_DATE), head, egui::Align::Center);
            text(662.0, 184.0, 12.0, &self.t(MSG_PLAY_TIME), head, egui::Align::Center);
            for k in 0..SLOTS {
                let cy = 225.0 + 29.4 * k as f32;
                if k == self.slot {
                    // GUESS: the selection is hidden under the popup in the captures.
                    p.rect_filled(r(361.0, cy - 14.0, 727.0, cy + 14.0), 0.0, Color32::from_gray(150));
                }
                let f = self.slots.get(k).cloned().flatten();
                let c = white;
                text(388.0, cy - 6.5, 13.0, &format!("{:02}", k + 1), c, egui::Align::Min);
                text(425.0, cy - 6.5, 13.0, "LV", Color32::from_gray(190), egui::Align::Min);
                match f {
                    Some(f) => {
                        text(478.0, cy - 6.5, 13.0, &f.level.to_string(), c, egui::Align::Max);
                        text(507.0, cy - 6.5, 13.0, &f.date, c, egui::Align::Min);
                        let s = f.play_time as u64;
                        text(614.0, cy - 6.5, 13.0, &format!("{:03}:{:02}:{:02}", s / 3600, s / 60 % 60, s % 60), c, egui::Align::Min);
                    }
                    None => {
                        text(456.0, cy - 6.5, 13.0, "---", c, egui::Align::Min);
                        text(507.0, cy - 6.5, 13.0, "--/--/--", c, egui::Align::Min);
                        text(614.0, cy - 6.5, 13.0, "---:--:--", c, egui::Align::Min);
                    }
                }
            }
            // Scrollbar with TEX_xddsclo arrows (up arrows in its lower row, down arrows in the upper).
            p.rect_filled(r(727.0, 211.0, 745.0, 481.0), 0.0, Color32::from_gray(95));
            let uv = |y0: f32| [0.0, y0, 52.0, y0 + 52.0];
            img("xddsclo", r(727.0, 211.0, 745.0, 226.0), uv(52.0), white);
            img("xddsclo", r(727.0, 466.0, 745.0, 481.0), uv(0.0), white);
        }

        let help = match self.mode {
            Mode::Column => self.t(if self.load { MSG_HELP_LOAD } else { MSG_HELP_SAVE }),
            _ => self.t(if self.load { MSG_HELP_LOAD_SLOT } else { MSG_HELP_SAVE_SLOT }),
        };
        crate::altimit::help_bar(&p, btn, o, w, h, &help, false);

        // Popups (f01214 / f01215).
        let popup = |lines: &str| {
            let rect = r(143.0, 124.0, 818.0, 325.0);
            p.rect_filled(rect, 14.0 * px, Color32::from_rgba_unmultiplied(45, 118, 125, 235));
            for (i, l) in lines.lines().enumerate() {
                text(163.0, 160.0 + 35.0 * i as f32, 13.0, l, white, egui::Align::Min);
            }
        };
        match self.mode {
            Mode::Confirm { yes } => {
                popup(&self.t(MSG_OVERWRITE));
                let (ys, ns) = (self.t(MSG_YES), self.t(MSG_NO_ANSWER));
                let sel_c = white;
                let off_c = Color32::from_gray(170);
                // f01214: "Yes" x 433-468, "/" ~485, "No" x 500-527.
                text(433.0, 293.0, 13.0, &ys, if yes { sel_c } else { off_c }, egui::Align::Min);
                text(485.0, 293.0, 13.0, "/", off_c, egui::Align::Center);
                text(500.0, 293.0, 13.0, &ns, if yes { off_c } else { sel_c }, egui::Align::Min);
            }
            Mode::Message(id) => {
                popup(&self.t(id));
                if let Some(b) = btn {
                    p.image(b, Rect::from_center_size(at(789.0, 293.0), egui::vec2(22.0 * px, 22.0 * px)), Rect::from_min_max(Pos2::new(245.0 / 512.0, 4.0 / 512.0), Pos2::new(315.0 / 512.0, 74.0 / 512.0)), white);
                }
            }
            _ => {}
        }
    }
}

/// Opaque part of TEX_xddsltex01 ("Save/Load", 512x256).
fn alpha_box_saveload() -> [f32; 4] {
    [14.0, 16.0, 498.0, 95.0]
}
