//! ALTIMIT Mail Station (Read mail / Greeting / Address), built from the game's own assets.
//!
//! Data used:
//! - `xddesktop@ENG.ccs`: title TEX_xddmtex01, tab icons TEX_xddicma{1,2,3}{0,1} (normal / selected), Mine OS lines
//!   TEX_xddttex03 and dots TEX_xddiclogo02 (as placed by CMP_xddcbrd01_*), the "Read mail." pill TEX_xddcbrd02,
//!   scroll arrows TEX_xddsclo, the dashed ring CMP_xddstb (vertex colour 78,144,204; ANM_xddstb animates only the
//!   icon parts), sender portraits TEX_xddface<name>.
//! - Strings: messageInfo.bin (Read mail./Greeting/Address 41115-41117, Sender/Subject 41125/41126, "Sender: "/"Subject: "
//!   41128/41129, help lines 41029-41033/41042, mails as sender/subject/body rows, e.g. Ovan's 10656-10658).
//!
//! Measured on reference_captures/run2_field f01067 (tabs), f01093/f01095 (list), f01089 (mail view) at 960x540, not
//! found in data: every position, the row pitch (29.6), the colours of the list rows and panes, the header height in
//! each mode (112 with the tabs, 55 otherwise). The open/close animations (ANM_xddmaup/madown) are not played.
//! APPROXIMATE (only seen in passing, f01078/f01079): the Greeting and Address panes.

use std::collections::HashMap;
use std::path::Path;

use egui::{Color32, Pos2, Rect, TextureHandle};
use hackgu_ccs::CcsFile;
use hackgu_game::msginfo::MessageInfo;

use crate::altimit::{build_sprites, help_bar, load_textures, load_textures_from, Sprite};

#[derive(Clone, Debug)]
pub struct Mail {
    pub sender: u32,
    pub subject: u32,
    pub body: u32,
    /// Portrait texture (TEX_xddface<name>).
    pub face: &'static str,
    pub unread: bool,
}

/// Ovan's "Choices" (messageInfo rows 10656-10658), the first mail after the reformat.
pub fn ovan_choices() -> Mail {
    Mail { sender: 0x9ED35D27, subject: 0x487A1A11, body: 0x4F983190, face: "xddfaceova", unread: true }
}

/// Story lines around the mail after the reformat (messageInfo rows 21893-21908), (id, is a system popup).
/// Placement from reference_captures/run2_field: 21895 over the empty Mail Station (f01076), 21897/21898/21900 on the
/// desktop after it closes (f01081-f01083), 21902 on entering the list with Ovan's mail (f01087), 21903/21905 back in
/// the list after reading it (f01092/f01093). GUESS: where the system popups 21893/21899/21908 sit (not captured), and
/// 21901 ((game text)) answering any other application while the mail is unread.
pub const MAIL_EMPTY_SEQ: [(u32, bool); 4] = [(0x51761681, true), (0xA1A62752, false), (0xA5673AE5, false), (0xA8241C3C, false)];
pub const DESKTOP_NEWMAIL_SEQ: [(u32, bool); 4] = [(0xACE5018B, false), (0xB2A2518E, false), (0xB6634C39, true), (0xBB206AE0, false)];
pub const CHECK_MAIL_FIRST: [(u32, bool); 1] = [(0xBFE17757, false)];
pub const OVAN_SEEN: [(u32, bool); 1] = [(0x87AECAEA, false)];
pub const AFTER_OVAN: [(u32, bool); 5] = [(0x836FD75D, false), (0xD34D7881, false), (0xD78C6536, false), (0xDACF43EF, false), (0xDE0E5E58, false)];
pub const DESKTOP_HELP: [(u32, bool); 1] = [(0xC0490E5D, true)];

const MSG_PILL: [u32; 3] = [0x733F4561, 0x12D5E2FE, 0x816870D8];
const MSG_TAB_HELP: [u32; 3] = [0xBCA43938, 0xB1E71FE1, 0xB5260256];
const MSG_LIST_HELP: u32 = 0x128598A4;
const MSG_VIEW_HELP: u32 = 0xAFA04FE4;
const MSG_SENDER: u32 = 0x2705C25F;
const MSG_SUBJECT: u32 = 0x2A46E486;
const MSG_SENDER_COLON: u32 = 0x30C0A934;
const MSG_SUBJECT_COLON: u32 = 0x3401B483;
const MSG_TYPE: u32 = 0xC7717575;
const MSG_PREVIEW: u32 = 0xCA3253AC;
const MSG_NO_CARDS: u32 = 0x15891623;
const MSG_NO_ADDRESS: u32 = 0xD33C7425;
const MSG_NO_MAIL: u32 = 0x79078DDD;

const BG: Color32 = Color32::from_rgb(190, 194, 195);
const ROW_PITCH: f32 = 29.6;
const TAB_X: [f32; 3] = [364.0, 454.0, 543.0];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Mode {
    Tabs,
    List,
    View,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MailEvent {
    None,
    Close,
    /// The cursor moved from the tabs into the inbox.
    EnteredList,
    Opened(usize),
    ClosedView(usize),
}

pub struct MailStation {
    tex: HashMap<String, (TextureHandle, [usize; 2])>,
    ring: Vec<Sprite>,
    text: HashMap<u32, String>,
    pub inbox: Vec<Mail>,
    mode: Mode,
    tab: usize,
    row: usize,
    scroll: f32,
    script: Vec<(u32, bool)>,
    script_pos: usize,
    popup: Option<u32>,
}

impl MailStation {
    pub fn new(ctx: &egui::Context, lang_code: &str, extra_ids: &[u32]) -> Self {
        let mut tex = HashMap::new();
        let mut ring = Vec::new();
        if let Some(ccs) = std::fs::read("target/cmn/data/data/xddesktop@ENG.ccs").ok().and_then(|d| CcsFile::parse(&d).ok()) {
            ring = build_sprites(&ccs, &|n: &str| n == "CMP_xddstb", "");
            let names = [
                "xddmtex01", "xddmtex02", "xddicma10", "xddicma11", "xddicma20", "xddicma21", "xddicma30", "xddicma31", "xddttex03", "xddiclogo02", "xddcbrd02", "xddsclo", "xddicon02", "xddfaceova",
            ];
            load_textures_from(ctx, &ccs, &names, &mut tex);
        }
        load_textures(ctx, Path::new("target/cmn/data/data/btn00@ENG@WIN64.ccs"), &["btn00"], &mut tex);
        load_textures(ctx, Path::new("target/cmn/data/data/xdcmn@ENG.ccs"), &["xddicnew01"], &mut tex);
        let mut ids: Vec<u32> = vec![
            MSG_LIST_HELP, MSG_VIEW_HELP, MSG_SENDER, MSG_SUBJECT, MSG_SENDER_COLON, MSG_SUBJECT_COLON, MSG_TYPE, MSG_PREVIEW, MSG_NO_CARDS, MSG_NO_ADDRESS, MSG_NO_MAIL,
        ];
        ids.extend(MSG_PILL);
        ids.extend(MSG_TAB_HELP);
        let m = ovan_choices();
        ids.extend([m.sender, m.subject, m.body]);
        for seq in [&MAIL_EMPTY_SEQ[..], &OVAN_SEEN, &AFTER_OVAN] {
            ids.extend(seq.iter().map(|e| e.0));
        }
        ids.extend_from_slice(extra_ids);
        let mut text = HashMap::new();
        if let Some(l) = MessageInfo::load(Path::new("target"), lang_code) {
            for id in ids {
                if let Some(s) = l.by_id.get(&id) {
                    text.insert(id, s.clone());
                }
            }
        }
        Self { tex, ring, text, inbox: Vec::new(), mode: Mode::Tabs, tab: 0, row: 0, scroll: 0.0, script: Vec::new(), script_pos: 0, popup: None }
    }

    /// Back to the first tab, as when the application is launched.
    pub fn open(&mut self) {
        self.mode = Mode::Tabs;
        self.tab = 0;
        self.row = 0;
        self.scroll = 0.0;
        self.popup = None;
    }

    /// Play a scripted sequence of lines over the application (advanced with A).
    pub fn play(&mut self, seq: &[(u32, bool)]) {
        self.script = seq.to_vec();
        self.script_pos = 0;
    }

    pub fn busy(&self) -> bool {
        self.script_pos < self.script.len()
    }

    fn t(&self, id: u32) -> String {
        self.text.get(&id).cloned().unwrap_or_default()
    }

    pub fn show(&mut self, ctx: &egui::Context) -> MailEvent {
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
        let mut ev = MailEvent::None;
        if self.busy() {
            if ok {
                self.script_pos += 1;
            }
        } else if self.popup.is_some() {
            if ok || back {
                self.popup = None;
            }
        } else {
            match self.mode {
                Mode::Tabs => {
                    if left {
                        self.tab = (self.tab + 2) % 3;
                    }
                    if right {
                        self.tab = (self.tab + 1) % 3;
                    }
                    if ok {
                        match self.tab {
                            0 if self.inbox.is_empty() => self.popup = Some(MSG_NO_MAIL),
                            0 => {
                                self.mode = Mode::List;
                                self.row = self.row.min(self.inbox.len() - 1);
                                ev = MailEvent::EnteredList;
                            }
                            1 => self.popup = Some(MSG_NO_CARDS),
                            _ => self.popup = Some(MSG_NO_ADDRESS),
                        }
                    }
                    if back {
                        ev = MailEvent::Close;
                    }
                }
                Mode::List => {
                    if up && self.row > 0 {
                        self.row -= 1;
                    }
                    if down && self.row + 1 < self.inbox.len() {
                        self.row += 1;
                    }
                    if ok {
                        self.mode = Mode::View;
                        self.scroll = 0.0;
                        self.inbox[self.row].unread = false;
                        ev = MailEvent::Opened(self.row);
                    }
                    if back {
                        self.mode = Mode::Tabs;
                    }
                }
                Mode::View => {
                    if up {
                        self.scroll = (self.scroll - 24.0).max(0.0);
                    }
                    if down {
                        self.scroll += 24.0;
                    }
                    if back {
                        self.mode = Mode::List;
                        ev = MailEvent::ClosedView(self.row);
                    }
                }
            }
        }
        self.draw(ctx);
        if let Some(&(id, system)) = self.script.get(self.script_pos) {
            crate::altimit::draw_dialogue(ctx, self.tex.get("btn00").map(|(t, _)| t.id()), &self.t(id), system);
        } else if let Some(id) = self.popup {
            crate::altimit::draw_dialogue(ctx, self.tex.get("btn00").map(|(t, _)| t.id()), &self.t(id), true);
        }
        ev
    }

    fn draw(&self, ctx: &egui::Context) {
        let screen = ctx.screen_rect();
        let (w, h) = if screen.width() / screen.height() > 16.0 / 9.0 { (screen.height() * 16.0 / 9.0, screen.height()) } else { (screen.width(), screen.width() * 9.0 / 16.0) };
        let o = Pos2::new(screen.center().x - w * 0.5, screen.center().y - h * 0.5);
        let px = h / 540.0;
        let at = |x: f32, y: f32| Pos2::new(o.x + x * px, o.y + y * px);
        let r = |x0: f32, y0: f32, x1: f32, y1: f32| Rect::from_min_max(at(x0, y0), at(x1, y1));
        let p = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("mailstation")));
        let gray = |v: u8| Color32::from_rgb(v, v, v);
        let img_tint = |name: &str, rect: Rect, uv: [f32; 4], tint: Color32| {
            if let Some((t, [tw, th])) = self.tex.get(name) {
                let (tw, th) = (*tw as f32, *th as f32);
                p.image(t.id(), rect, Rect::from_min_max(Pos2::new(uv[0] / tw, uv[1] / th), Pos2::new(uv[2] / tw, uv[3] / th)), tint);
            }
        };
        let img = |name: &str, rect: Rect, uv: [f32; 4]| img_tint(name, rect, uv, Color32::WHITE);
        // Vertical gradient between two colours.
        let vgrad = |rect: Rect, top: Color32, bottom: Color32| {
            let mut m = egui::Mesh::default();
            m.colored_vertex(rect.left_top(), top);
            m.colored_vertex(rect.right_top(), top);
            m.colored_vertex(rect.right_bottom(), bottom);
            m.colored_vertex(rect.left_bottom(), bottom);
            m.add_triangle(0, 1, 2);
            m.add_triangle(0, 2, 3);
            p.add(egui::Shape::mesh(m));
        };
        // Text with its capitals' top at `cap_top` (Segoe UI capitals are 0.70 of the size; ascent ~0.93).
        let font = |cap: f32| egui::FontId::proportional(cap * px / 0.70);
        let text_at = |x: f32, cap_top: f32, cap: f32, s: &str, c: Color32| {
            let g = p.layout_no_wrap(s.to_string(), font(cap), c);
            let lead = g.size().y * 0.5 - cap * px * 0.5;
            p.galley(Pos2::new(o.x + x * px, o.y + cap_top * px - lead), g, c);
        };

        p.rect_filled(screen, 0.0, Color32::BLACK);
        p.rect_filled(r(0.0, 0.0, 960.0, 540.0), 0.0, BG);

        // Header: with the tabs it is 112 tall; in the list and the mail view it shrinks to 55 (everything but the
        // title moves up 56).
        let tabs = self.mode == Mode::Tabs;
        let dy = if tabs { 0.0 } else { -56.0 };
        img("xddmtex01", r(50.0, 32.0, 244.0, 56.0), [27.0, 79.0, 631.0, 188.0]);
        let ring_c = if tabs { (191.5, 86.5) } else { (378.0, 29.0) };
        self.draw_ring(&p, at(ring_c.0, ring_c.1), 45.0 / 35.14 * px);
        if let Some((t, _)) = self.tex.get("xddicon02") {
            p.image(t.id(), Rect::from_center_size(at(ring_c.0, ring_c.1), egui::vec2(30.0 * px, 30.0 * px)), Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE);
        }
        // TEX_xddttex03 is white text (white on the desktop banners); black here.
        img_tint("xddttex03", r(683.0, 84.0 + dy, 773.0, 99.0 + dy), [13.0, 124.0, 812.0, 284.0], Color32::BLACK);
        img_tint("xddttex03", r(567.0, 101.0 + dy, 779.0, 112.0 + dy), [10.0, 14.0, 1664.0, 107.0], Color32::BLACK);
        img("xddiclogo02", r(788.0, 91.0 + dy, 818.0, 108.0 + dy), [189.0, 68.0, 848.0, 449.0]);
        if tabs {
            for (k, x) in TAB_X.iter().enumerate() {
                let name = format!("xddicma{}{}", k + 1, if k == self.tab { 1 } else { 0 });
                img(&name, r(*x, 40.0, x + 53.0, 85.0), [0.0, 0.0, 256.0, 256.0]);
            }
            let cx = TAB_X[self.tab] + 26.5;
            p.add(egui::Shape::convex_polygon(vec![at(cx - 6.5, 20.0), at(cx + 6.5, 20.0), at(cx, 31.0)], Color32::from_rgb(36, 130, 191), egui::Stroke::NONE));
            // Darker than its texture in the original: fill 147 vs 180 (f01067).
            img_tint("xddcbrd02", r(634.0, 42.0, 818.0, 75.0), [15.0, 14.0, 1007.0, 471.0], Color32::from_gray(208));
            let g = p.layout_no_wrap(self.t(MSG_PILL[self.tab]), font(12.0), Color32::BLACK);
            p.galley(at(726.0, 56.5) - g.size() * 0.5, g, Color32::BLACK);
        }

        match (self.mode, self.tab) {
            (Mode::Tabs, 0) | (Mode::List, _) => self.draw_list(&p, &r, &at, &vgrad, &text_at, px),
            (Mode::Tabs, 1) => {
                // APPROXIMATE (f01078): Type | Preview columns over a dark list.
                vgrad(r(0.0, 112.0, 960.0, 118.0), BG, Color32::from_rgb(90, 92, 89));
                p.rect_filled(r(0.0, 118.0, 960.0, 150.0), 0.0, Color32::from_rgb(88, 90, 87));
                text_at(160.0, 127.0, 14.0, &self.t(MSG_TYPE), Color32::BLACK);
                text_at(620.0, 127.0, 14.0, &self.t(MSG_PREVIEW), Color32::BLACK);
                for k in 0..11 {
                    let y = 153.0 + k as f32 * ROW_PITCH;
                    p.rect_filled(r(0.0, y, 351.0, y + ROW_PITCH), 0.0, gray(if k % 2 == 0 { 71 } else { 92 }));
                }
                p.rect_filled(r(351.0, 150.0, 960.0, 481.0), 0.0, gray(50));
            }
            (Mode::Tabs, _) => {
                // APPROXIMATE (f01079): the address book's dark panels.
                vgrad(r(0.0, 112.0, 960.0, 118.0), BG, gray(60));
                p.rect_filled(r(0.0, 118.0, 960.0, 481.0), 0.0, gray(60));
                p.rect_filled(r(40.0, 130.0, 560.0, 470.0), 0.0, gray(45));
                p.rect_filled(r(575.0, 130.0, 920.0, 290.0), 0.0, gray(45));
                p.rect_filled(r(575.0, 300.0, 920.0, 470.0), 0.0, gray(45));
            }
            (Mode::View, _) => self.draw_view(&p, &r, &at, &vgrad, &text_at, px),
        }

        let help = match self.mode {
            Mode::Tabs => self.t(MSG_TAB_HELP[self.tab]),
            Mode::List => self.t(MSG_LIST_HELP),
            Mode::View => self.t(MSG_VIEW_HELP),
        };
        help_bar(&p, self.tex.get("btn00").map(|(t, _)| t.id()), o, w, h, &help, true);
    }

    fn draw_ring(&self, p: &egui::Painter, c: Pos2, scale: f32) {
        let color = Color32::from_rgb(78, 144, 204);
        for s in &self.ring {
            let mut mesh = egui::Mesh::default();
            for &(pos, _) in &s.verts {
                mesh.vertices.push(egui::epaint::Vertex { pos: Pos2::new(c.x + pos[0] * scale, c.y - pos[1] * scale), uv: egui::epaint::WHITE_UV, color });
            }
            mesh.indices = s.indices.clone();
            p.add(egui::Shape::mesh(mesh));
        }
    }

    fn scrollbar(&self, p: &egui::Painter, r: &dyn Fn(f32, f32, f32, f32) -> Rect, y0: f32, y1: f32) {
        p.rect_filled(r(891.0, y0, 909.0, y1), 0.0, Color32::from_rgb(98, 98, 98));
        p.rect_filled(r(891.0, y0, 893.0, y1), 0.0, Color32::from_rgb(46, 46, 46));
        p.rect_filled(r(907.0, y0, 909.0, y1), 0.0, Color32::from_rgb(57, 57, 57));
        if let Some((t, _)) = self.tex.get("xddsclo") {
            // TEX_xddsclo 128x128: down arrows in the top row, up arrows below.
            let uv = |x0: f32, y0: f32| Rect::from_min_max(Pos2::new(x0 / 128.0, y0 / 128.0), Pos2::new((x0 + 52.0) / 128.0, (y0 + 52.0) / 128.0));
            p.image(t.id(), r(891.0, y0, 909.0, y0 + 17.0), uv(0.0, 52.0), Color32::WHITE);
            p.image(t.id(), r(891.0, y1 - 17.0, 909.0, y1), uv(0.0, 0.0), Color32::WHITE);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_list(
        &self,
        p: &egui::Painter,
        r: &dyn Fn(f32, f32, f32, f32) -> Rect,
        at: &dyn Fn(f32, f32) -> Pos2,
        vgrad: &dyn Fn(Rect, Color32, Color32),
        text_at: &dyn Fn(f32, f32, f32, &str, Color32),
        px: f32,
    ) {
        let gray = |v: u8| Color32::from_rgb(v, v, v);
        let tabs = self.mode == Mode::Tabs;
        // Tabs: dark list under a 112 header. List: light list under a 55 header.
        let (hy0, hy1, head, rows_y, rows, pane, row_c) = if tabs {
            (118.0, 150.0, Color32::from_rgb(88, 90, 87), 153.0, 11, gray(60), [gray(71), gray(92)])
        } else {
            (61.0, 94.0, Color32::from_rgb(175, 180, 176), 96.0, 13, gray(120), [Color32::from_rgb(140, 142, 139), Color32::from_rgb(182, 184, 181)])
        };
        vgrad(r(0.0, hy0 - 6.0, 960.0, hy0), BG, head);
        p.rect_filled(r(0.0, hy0, 960.0, hy1), 0.0, head);
        p.rect_filled(r(321.0, hy0, 326.0, hy1), 0.0, Color32::from_rgb(36, 38, 35));
        p.rect_filled(r(326.0, hy0, 330.0, hy1), 0.0, Color32::from_rgb(103, 105, 102));
        let cap_off = if tabs { 9.0 } else { 10.0 };
        text_at(48.0, hy0 + cap_off, 14.0, &self.t(MSG_SENDER), Color32::BLACK);
        text_at(387.0, hy0 + cap_off, 14.0, &self.t(MSG_SUBJECT), Color32::BLACK);
        p.rect_filled(r(0.0, hy1, 960.0, 481.0), 0.0, pane);
        for k in 0..rows {
            let y = rows_y + k as f32 * ROW_PITCH;
            p.rect_filled(r(0.0, y, 891.0, (y + ROW_PITCH).min(481.0)), 0.0, row_c[k % 2]);
        }
        // Dotted column rule.
        let mut y = hy1 + 2.0;
        while y < 479.0 {
            p.rect_filled(r(324.0, y, 326.0, y + 2.0), 0.0, Color32::BLACK);
            y += 4.0;
        }
        for (k, m) in self.inbox.iter().enumerate().take(rows) {
            let y = rows_y + k as f32 * ROW_PITCH;
            text_at(48.0, y + 6.5, 14.0, &self.t(m.sender), Color32::BLACK);
            let mut sx = 387.0;
            if m.unread {
                // GUESS (f01086, small): the NEW badge in front of an unread mail's subject.
                if let Some((t, _)) = self.tex.get("xddicnew01") {
                    p.image(t.id(), r(340.0, y + 8.0, 380.0, y + 24.0), Rect::from_min_max(Pos2::new(0.016, 0.02), Pos2::new(0.49, 0.40)), Color32::WHITE);
                }
                sx = 387.0;
            }
            text_at(sx, y + 6.5, 14.0, &self.t(m.subject), Color32::BLACK);
        }
        if !tabs && !self.inbox.is_empty() {
            let y = rows_y + self.row as f32 * ROW_PITCH;
            p.rect_stroke(r(46.0, y, 889.0, y + 28.0), 0.0, egui::Stroke::new(2.0 * px, Color32::from_rgb(110, 182, 204)), egui::StrokeKind::Inside);
        }
        let _ = at;
        self.scrollbar(p, r, hy1 + 2.0, 479.0);
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_view(
        &self,
        p: &egui::Painter,
        r: &dyn Fn(f32, f32, f32, f32) -> Rect,
        at: &dyn Fn(f32, f32) -> Pos2,
        vgrad: &dyn Fn(Rect, Color32, Color32),
        text_at: &dyn Fn(f32, f32, f32, &str, Color32),
        px: f32,
    ) {
        let Some(m) = self.inbox.get(self.row) else { return };
        let page = Color32::from_rgb(187, 185, 188);
        vgrad(r(0.0, 55.0, 960.0, 61.0), BG, page);
        p.rect_filled(r(0.0, 61.0, 891.0, 481.0), 0.0, page);
        p.rect_filled(r(909.0, 61.0, 960.0, 481.0), 0.0, Color32::from_rgb(120, 120, 120));
        let clip = r(0.0, 61.0, 891.0, 481.0);
        let cp = p.with_clip_rect(clip);
        let s = self.scroll;
        // Portrait in a thin frame, sender and subject beside it, a fading rule under them (f01089).
        cp.rect_filled(r(76.0, 68.0 - s, 189.0, 181.0 - s), 0.0, Color32::from_rgb(120, 120, 120));
        if let Some((t, _)) = self.tex.get(m.face) {
            cp.image(t.id(), r(78.0, 70.0 - s, 187.0, 179.0 - s), Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE);
        }
        let ctext = |x: f32, cap_top: f32, txt: &str| {
            let g = cp.layout_no_wrap(txt.to_string(), egui::FontId::proportional(14.0 * px / 0.70), Color32::BLACK);
            let lead = g.size().y * 0.5 - 7.0 * px;
            cp.galley(Pos2::new(at(x, 0.0).x, at(0.0, cap_top).y - lead), g, Color32::BLACK);
        };
        ctext(219.0, 90.0 - s, &format!("{} {}", self.t(MSG_SENDER_COLON), self.t(m.sender)));
        ctext(219.0, 113.0 - s, &format!("{} {}", self.t(MSG_SUBJECT_COLON), self.t(m.subject)));
        let mut rule = egui::Mesh::default();
        let (a, b) = (at(76.0, 199.0 - s), at(660.0, 200.0 - s));
        rule.colored_vertex(a, Color32::from_rgb(151, 149, 152));
        rule.colored_vertex(Pos2::new(b.x, a.y), page);
        rule.colored_vertex(b, page);
        rule.colored_vertex(Pos2::new(a.x, b.y), Color32::from_rgb(151, 149, 152));
        rule.add_triangle(0, 1, 2);
        rule.add_triangle(0, 2, 3);
        cp.add(egui::Shape::mesh(rule));
        // Body: 24 px lines, first capitals at y 214. The original wraps at ~744 px; our font is ~5% narrower than the
        // original's (722 vs 685 px for the same line), so the wrap width (716) is scaled to keep the original's line breaks.
        let mut job = egui::text::LayoutJob::default();
        job.wrap.max_width = 716.0 * px;
        job.append(
            &crate::altimit::strip_tags(&self.t(m.body)),
            0.0,
            egui::TextFormat { font_id: egui::FontId::proportional(14.0 * px / 0.70), color: Color32::BLACK, line_height: Some(24.0 * px), ..Default::default() },
        );
        let g = cp.fonts(|f| f.layout_job(job));
        let lead = 12.0 * px - 7.0 * px;
        cp.galley(Pos2::new(at(76.0, 0.0).x, at(0.0, 214.0 - s).y - lead), g, Color32::BLACK);
        let _ = text_at;
        self.scrollbar(p, r, 61.0, 481.0);
    }
}
