//! In-game Main Menu (town and field), in the Last Recode style shared with the Chaos Gate screens.
//!
//! Data used: wheel hexagons TEX_xmenu_00 (xmenu_00@ENG.ccs), party faces TEX_xf<chr>_01 (menu/xf<chr>_01.ccs,
//! 256x128 cut-corner portraits), btn00 glyphs; strings from messageInfo: items 41741-41753, help lines 42008-42019,
//! "LV" 41224, "Money:" 41910, "GP" 41911, "Party's Chim:" 41914, "Chim" 41916.
//!
//! Measured on reference_captures/run1_opening f00064-f00070 (960x540): wheel order (cyclic) Log Out, Party, Items,
//! Skill, Equipment, Customize, Skill Trigger, Status, Quest, Awakening, Strategy, Memoir, at the Warp Menu's wheel
//! positions; party panels x 593-928 at y 61 + 66 k (face x 595-704, "LV" at 588, name centred at 798, HP label 722,
//! value right at 796, SP label 830, value right at 904, bars y 110-112 at x 704-801 / 813-910); money panel
//! x 560-944, y 366-428. GUESS: the starting item (Items) and the panel outlines' exact shape. Only Log Out works yet.

use std::collections::HashMap;
use std::path::Path;

use egui::{Color32, Pos2, TextureHandle};
use hackgu_game::msginfo::MessageInfo;

use crate::chaosgate::{Draw, HEX_WARP, WHEEL_WARP};

/// (label id, help id) in wheel order.
const ITEMS: [(u32, u32); 12] = [
    (0xE4CF9A9E, 0x5499D6A7), // Log Out
    (0xBDCB3813, 0x9EEE0ECF), // Party
    (0xF15720C0, 0xF5670180), // Items
    (0xF3C711CC, 0xD0E22710), // Skill
    (0x8A12701C, 0xA93746C0), // Equipment
    (0xAD62C7D0, 0xEBE07F2A), // Customize
    (0x31225341, 0xAD74C059), // Skill Trigger
    (0x1CB4D584, 0xACE299BD), // Status
    (0x61A44974, 0x42817FA8), // Quest
    (0x68ADC65F, 0x2B4A53D0), // Awakening
    (0xB621B4B3, 0x5D5300CD), // Strategy
    (0x40F9AB4D, 0x067B13B7), // Memoir
];
/// Name keys and face textures of the characters who can be in the party in Vol.1 so far.
pub const NAME_KEYS: [&str; 5] = ["spcName_000_A", "spcName_004_A", "spcName_005_A", "spcName_019_A", "spcName_020_A"];

/// (name key, face) for a character model file ("2ast01.ccs" -> Asta).
pub fn party_identity(model: &str) -> (&'static str, &'static str) {
    match model.get(1..4).unwrap_or("") {
        "syl" => ("spcName_004_A", "xfsyl"),
        "gas" => ("spcName_005_A", "xfgas"),
        "iyo" => ("spcName_019_A", "xfiyo"),
        "ast" => ("spcName_020_A", "xfast"),
        _ => ("spcName_000_A", "xfhsw"),
    }
}

const LOG_OUT: usize = 0;
const ITEMS_ENTRY: usize = 2;
const MSG_MAIN_MENU: u32 = 0x54BDFE0C;
const MSG_LOGGING_OUT: u32 = 0xB58B7502;
const MSG_LV: u32 = 0x13737380;
const MSG_MONEY: u32 = 0x3BAE4881;
const MSG_GP: u32 = 0x5B2DB7BB;
const MSG_CHIM_LABEL: u32 = 0x3081A423;
const MSG_CHIM: u32 = 0xDC86F444;
const MSG_CONFIRM: u32 = 0xE27198F3;
const MSG_RETURN: u32 = 0xD07EFEF2;

/// One party member's line.
#[derive(Clone, Debug)]
pub struct PartyLine {
    /// messageInfo key of the name (`spcName_000_A` = Haseo).
    pub name_key: &'static str,
    /// Face texture key (`xfhsw` -> TEX_xfhsw_01 in menu/xfhsw_01.ccs).
    pub face: &'static str,
    pub level: u32,
    pub hp: i32,
    pub max_hp: i32,
    pub sp: i32,
    pub max_sp: i32,
}

pub enum MenuAction {
    None,
    Close,
    LogOut,
}

pub struct MainMenu {
    tex: HashMap<String, (TextureHandle, [usize; 2])>,
    text: HashMap<u32, String>,
    sel: usize,
    status: Option<String>,
    names: HashMap<&'static str, String>,
}

impl MainMenu {
    pub fn new(ctx: &egui::Context, lang_code: &str) -> Self {
        let mut tex = HashMap::new();
        crate::altimit::load_textures(ctx, Path::new("target/cmn/data/data/xmenu_00@ENG.ccs"), &["xmenu_00"], &mut tex);
        crate::altimit::load_textures(ctx, Path::new("target/cmn/data/data/btn00@ENG@WIN64.ccs"), &["btn00"], &mut tex);
        for f in ["xfhsw", "xfiyo", "xfast", "xfsyl", "xfgas"] {
            crate::altimit::load_textures(ctx, Path::new(&format!("target/cmn/data/data/menu/{}_01.ccs", f)), &[format!("{}_01", f).as_str()], &mut tex);
        }
        let mut text = HashMap::new();
        if let Some(l) = MessageInfo::load(Path::new("target"), lang_code) {
            let ids = ITEMS.iter().flat_map(|&(a, b)| [a, b]).chain([MSG_MAIN_MENU, MSG_LOGGING_OUT, MSG_LV, MSG_MONEY, MSG_GP, MSG_CHIM_LABEL, MSG_CHIM, MSG_CONFIRM, MSG_RETURN]);
            for id in ids {
                if let Some(s) = l.by_id.get(&id) {
                    text.insert(id, s.clone());
                }
            }
        }
        let mut names = HashMap::new();
        if let Some(l) = MessageInfo::load(Path::new("target"), lang_code) {
            for k in NAME_KEYS {
                if let Some(n) = l.get(k) {
                    names.insert(k, n.to_string());
                }
            }
        }
        Self { tex, text, sel: ITEMS_ENTRY, status: None, names }
    }

    fn t(&self, id: u32) -> String {
        self.text.get(&id).cloned().unwrap_or_default()
    }

    pub fn show(&mut self, ctx: &egui::Context, party: &[PartyLine], money: u32, chim: u32) -> MenuAction {
        let (up, down, ok, back) = ctx.input(|i| {
            (
                i.key_pressed(egui::Key::ArrowUp) || i.key_pressed(egui::Key::W),
                i.key_pressed(egui::Key::ArrowDown) || i.key_pressed(egui::Key::S),
                i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Space),
                i.key_pressed(egui::Key::Escape) || i.key_pressed(egui::Key::Backspace),
            )
        });
        let n = ITEMS.len();
        let mut act = MenuAction::None;
        if up {
            self.sel = (self.sel + n - 1) % n;
            self.status = None;
        }
        if down {
            self.sel = (self.sel + 1) % n;
            self.status = None;
        }
        if ok {
            match self.sel {
                LOG_OUT => act = MenuAction::LogOut,
                _ => self.status = Some(format!("{} is not implemented yet.", self.t(ITEMS[self.sel].0))),
            }
        }
        if back {
            act = MenuAction::Close;
            self.status = None;
        }
        self.draw(ctx, party, money, chim);
        act
    }

    fn draw(&self, ctx: &egui::Context, party: &[PartyLine], money: u32, chim: u32) {
        let screen = ctx.screen_rect();
        let (w, h) = if screen.width() / screen.height() > 16.0 / 9.0 { (screen.height() * 16.0 / 9.0, screen.height()) } else { (screen.width(), screen.width() * 9.0 / 16.0) };
        let origin = Pos2::new(screen.center().x - w * 0.5, screen.center().y - h * 0.5);
        let d = Draw { p: ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("main_menu"))), origin, sx: w / 960.0, sy: h / 540.0, tex: &self.tex };
        // The scene behind is dimmed (f00064).
        d.p.rect_filled(screen, 0.0, Color32::from_rgba_unmultiplied(20, 6, 4, 170));
        // The wheel runs behind the header bar (f00064).
        let items: Vec<String> = ITEMS.iter().map(|&(id, _)| self.t(id)).collect();
        d.wheel(&items, self.sel, &WHEEL_WARP, HEX_WARP);
        d.header(&self.t(MSG_MAIN_MENU), &self.t(MSG_CONFIRM), &self.t(MSG_RETURN));

        let white = Color32::from_rgb(235, 235, 235);
        for (k, m) in party.iter().take(3).enumerate() {
            let y0 = 61.0 + 66.0 * k as f32;
            let panel = [(605.0, y0), (916.0, y0), (928.0, y0 + 12.0), (928.0, y0 + 48.0), (916.0, y0 + 60.0), (605.0, y0 + 60.0), (593.0, y0 + 48.0), (593.0, y0 + 12.0)];
            d.poly(&panel, Color32::from_rgba_unmultiplied(20, 7, 6, 235));
            d.poly_outline(&panel, Color32::from_rgb(70, 60, 58));
            d.img(&format!("{}_01", m.face), [595.0, y0 + 3.0, 705.0, y0 + 60.0], [0.0, 0.0, 1.0, 1.0], Color32::WHITE);
            if m.level > 0 {
                d.text_l(588.0, y0 + 8.0, &self.t(MSG_LV), 7.5, white);
                d.text_l(640.0, y0 + 8.0, &m.level.to_string(), 9.5, white);
            }
            d.text_c(798.0, y0 + 16.5, self.names.get(m.name_key).map(|s| s.as_str()).unwrap_or(""), 13.0, white);
            d.text_l(722.0, y0 + 40.0, "HP", 10.0, white);
            d.p.text(d.pt(796.0, y0 + 40.0), egui::Align2::RIGHT_CENTER, m.hp.to_string(), d.font(10.0), white);
            d.text_l(830.0, y0 + 40.0, "SP", 10.0, white);
            d.p.text(d.pt(904.0, y0 + 40.0), egui::Align2::RIGHT_CENTER, m.sp.to_string(), d.font(10.0), white);
            let bar = |x0: f32, x1: f32, frac: f32, c: Color32| {
                d.rect([x0, y0 + 49.0, x1, y0 + 51.0], Color32::from_rgb(50, 40, 38));
                d.rect([x0, y0 + 49.0, x0 + (x1 - x0) * frac.clamp(0.0, 1.0), y0 + 51.0], c);
            };
            bar(704.0, 801.0, m.hp as f32 / m.max_hp.max(1) as f32, Color32::from_rgb(225, 180, 60));
            bar(813.0, 910.0, m.sp as f32 / m.max_sp.max(1) as f32, Color32::from_rgb(70, 190, 200));
        }

        let mp = [(572.0, 366.0), (944.0, 366.0), (944.0, 428.0), (572.0, 428.0), (560.0, 416.0), (560.0, 378.0)];
        d.poly(&mp, Color32::from_rgba_unmultiplied(36, 6, 6, 235));
        d.poly_outline(&mp, Color32::from_rgb(90, 76, 72));
        d.text_l(589.0, 383.5, &self.t(MSG_MONEY), 11.0, white);
        d.p.text(d.pt(841.0, 383.5), egui::Align2::RIGHT_CENTER, money.to_string(), d.font(11.0), white);
        d.p.text(d.pt(910.0, 383.5), egui::Align2::RIGHT_CENTER, self.t(MSG_GP), d.font(11.0), white);
        d.text_l(589.0, 411.5, &self.t(MSG_CHIM_LABEL), 11.0, white);
        d.p.text(d.pt(841.0, 411.5), egui::Align2::RIGHT_CENTER, chim.to_string(), d.font(11.0), white);
        d.p.text(d.pt(910.0, 411.5), egui::Align2::RIGHT_CENTER, self.t(MSG_CHIM), d.font(11.0), white);

        let help = self.status.clone().unwrap_or_else(|| self.t(ITEMS[self.sel].1));
        d.message_box(&[help]);
    }
}
