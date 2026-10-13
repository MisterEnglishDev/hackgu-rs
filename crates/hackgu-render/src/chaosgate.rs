//! Chaos Gate menus: gate prompt -> Warp Menu -> Area Word (1st/2nd/3rd words, Area Status) -> "Initiate Area warp?".
//!
//! Images are the game's own: `cmn/data/data/xmenu_00@ENG.ccs` (word-gyro hexagons, "1st/2nd/3rd", "Area Status",
//! "Area" "LV", "Element", "Treasure", "Mission", digits, the Δ server mark, element/people icons, treasure gauge) and
//! `menu/xchaos_01.ccs` (area thumbnails). Strings come from messageInfo.bin. The in-game menu frames themselves (dark
//! panels, red/teal bars) are not textures in the data (no match among the 1,771 UI textures); they are drawn here as
//! flat shapes with colours sampled from the original. Every position was measured on reference_captures/run2_field
//! f00484 (gate prompt), f00487 (Warp Menu), f00496-f00505 (Area Word) and f00508 (confirm), 960x540: the layout below
//! is in those pixels.
//!
//! Area synthesis logic is backed by `hackgu_game::areaword`, implementing reverse-engineered Vol. 1 mechanics:
//! MT19937 PRNG, level clamping, thumbnail mapping (TEX_xsrnd_01), special area match bypass, and dynamic help strings.
//!
//! Tutorial dialogue flow implements the authentic opening Chaos Gate tutorial (messageInfo.bin rows 21728–21755)
//! where Iyoten and Asta guide Haseo through Area Words, word plate combination, gyro stock, server symbol Δ,
//! and initial warp into Δ Courageous Engaging Daydream.

use std::collections::HashMap;
use std::path::Path;

use egui::{Color32, Pos2, Rect, TextureHandle};
use hackgu_game::areaword::{self, AreaServer, AreaWordSlot, Mt19937, SynthesizedArea};
use hackgu_game::msginfo::MessageInfo;

// Base menu strings
pub const MSG_CHAOS_GATE: u32 = 0x1224B4B2;
pub const MSG_WARP_MENU: u32 = 0x58A15540;
pub const MSG_AREA_WORD: u32 = 0xE448A835;
pub const MSG_RANDOM: u32 = 0x46EF4D0D;
pub const MSG_BOOKMARK: u32 = 0x68576F81;
pub const MSG_WARP_RECORD: u32 = 0xD3131447;
pub const MSG_TO_TOWN: u32 = 0x2EDD5956;
pub const MSG_CANCEL: u32 = 0x6930A7C0;
pub const MSG_INITIATE: u32 = 0x9E622F40; // "Initiate Area warp?"
pub const MSG_HELP_GATE: u32 = 0xC97F1226;
pub const MSG_HELP_TO_TOWN: u32 = 0x6C165023;
pub const MSG_HELP_AREA_WORD: u32 = 0xC76D9EE9;
pub const MSG_HELP_RANDOM: u32 = 0xAD9DF973;
pub const MSG_HELP_BOOKMARK: u32 = 0xB7DF95DF;
pub const MSG_HELP_RECORD: u32 = 0xD7233507;
pub const MSG_HELP_CLOSE: u32 = 0x2B84B943;
pub const MSG_CONFIRM: u32 = 0xE27198F3;
pub const MSG_RETURN: u32 = 0xD07EFEF2;
pub const MSG_YES: u32 = 0x8F88A217;
pub const MSG_NO: u32 = 0x18940D56;
pub const MSG_NO_WARP_RECORD: u32 = 0x4C134BE8;

// Tutorial dialogue rows 21728..=21755
pub const MSG_TUT_21728: u32 = 0xAC66ABA7;
pub const MSG_TUT_21729: u32 = 0xA8A7B610;
pub const MSG_TUT_21730: u32 = 0xA5E490C9; // "These words..."
pub const MSG_TUT_21731: u32 = 0xA1258D7E;
pub const MSG_TUT_21732: u32 = 0x996A30C3;
pub const MSG_TUT_21733: u32 = 0x9DAB2D74;
pub const MSG_TUT_21734: u32 = 0x6D7B1CA7;
pub const MSG_TUT_21735: u32 = 0x69BA0110;
pub const MSG_TUT_21736: u32 = 0x64F927C9;
pub const MSG_TUT_21737: u32 = 0x60383A7E;
pub const MSG_TUT_21738: u32 = 0x7E7F6A7B;
pub const MSG_TUT_21739: u32 = 0x7ABE77CC;
pub const MSG_TUT_21740: u32 = 0x77FD5115;
pub const MSG_TUT_21741: u32 = 0x733C4CA2;
pub const MSG_TUT_21742: u32 = 0x4B73F11F;
pub const MSG_TUT_21743: u32 = 0x4FB2ECA8;
pub const MSG_TUT_21744: u32 = 0x34F232AE;
pub const MSG_TUT_21745: u32 = 0x30332F19;
pub const MSG_TUT_21746: u32 = 0x3D7009C0;
pub const MSG_TUT_21747: u32 = 0x39B11477;
pub const MSG_TUT_21748: u32 = 0x27F64472;
pub const MSG_TUT_21749: u32 = 0x233759C5;
pub const MSG_TUT_21750: u32 = 0x2E747F1C;
pub const MSG_TUT_21751: u32 = 0x2AB562AB;
pub const MSG_TUT_21752: u32 = 0x12FADF16;
pub const MSG_TUT_21753: u32 = 0x163BC2A1;
pub const MSG_TUT_21754: u32 = 0xE6EBF372;
pub const MSG_TUT_21755: u32 = 0xE22AEEC5;

// Guided prompt box strings (messageInfo rows 41633..41635, 41648)
pub const MSG_GUIDE_AREA_WORD: u32 = 0xEF82C5A5; // Select "Area Word."
pub const MSG_GUIDE_COURAGEOUS: u32 = 0x5CF6F906; // Select "Courageous."
pub const MSG_GUIDE_ENGAGING: u32 = 0x340DFAFA; // Select "Engaging."
pub const MSG_GUIDE_DAYDREAM: u32 = 0x0F0B64DB; // Select "Daydream."

/// Warp Menu wheel order (the wheel loops: Cancel sits above To Town).
pub const WARP_ITEMS: [(u32, &str, u32); 6] = [
    (MSG_TO_TOWN, "To Town", MSG_HELP_TO_TOWN),
    (MSG_BOOKMARK, "Bookmark", MSG_HELP_BOOKMARK),
    (MSG_AREA_WORD, "Area Word", MSG_HELP_AREA_WORD),
    (MSG_RANDOM, "Random", MSG_HELP_RANDOM),
    (MSG_WARP_RECORD, "Warp Record", MSG_HELP_RECORD),
    (MSG_CANCEL, "Cancel", MSG_HELP_CLOSE),
];

// --- measured layout (960x540 reference pixels) ---
/// Word wheel rows relative to the selection (k = -5..=4): text centre.
const WHEEL_AREA: [(f32, f32); 10] = [(25.0, 77.0), (61.0, 104.0), (89.0, 133.0), (118.0, 163.0), (139.0, 195.0), (146.0, 227.0), (117.0, 257.0), (76.0, 287.0), (40.0, 313.0), (5.0, 335.0)];
pub(crate) const WHEEL_WARP: [(f32, f32); 10] = [(130.0, -70.0), (140.0, -40.0), (150.0, -12.0), (165.0, 20.0), (170.0, 54.0), (166.0, 87.0), (151.0, 117.0), (124.0, 146.0), (88.0, 172.0), (45.0, 194.0)];
/// Gyro hexagons (xmenu_00 left block) behind each wheel.
const HEX_AREA: [f32; 4] = [-18.0, 140.0, 86.0, 284.0];
pub(crate) const HEX_WARP: [f32; 4] = [-50.0, -5.0, 54.0, 139.0];
const WORD_COL_X: [f32; 3] = [474.0, 666.0, 822.0];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TutorialSpeaker {
    Iyoten,
    Asta,
}

impl TutorialSpeaker {
    pub fn name(&self) -> &'static str {
        match self {
            TutorialSpeaker::Iyoten => "IYOTEN",
            TutorialSpeaker::Asta => "Asta",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TutorialHighlight {
    None,
    WordPlate,    // Red outline around top Word Plate [318, 80, 938, 136]
    WordGyro,     // Red outline around gyro wheel [0, 50, 242, 370]
    ServerSymbol, // Red circle around server symbol Δ at (360, 109)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TutorialStep {
    // Warp menu intro (rows 21728..=21732)
    Intro1, // 21728 Iyoten
    Intro2, // 21729 Asta
    Intro3, // 21730 Iyoten
    Intro4, // 21731 Iyoten
    Intro5, // 21732 Iyoten
    // Player selects Area Word
    ChooseAreaWord,
    // Area Word screen explanation (rows 21735..=21740)
    AreaWordIntro1, // 21735 Iyoten
    AreaWordIntro2, // 21736 Iyoten (Highlight: WordPlate)
    AreaWordIntro3, // 21737 Iyoten (Highlight: WordPlate)
    AreaWordIntro4, // 21738 Iyoten (Highlight: WordGyro)
    AreaWordIntro5, // 21739 Iyoten (Highlight: None)
    AreaWordIntro6, // 21740 Iyoten (Highlight: None)
    // Word picking steps
    PickWord1, // Waiting for Courageous
    PickWord2, // Waiting for Engaging
    PickWord3, // Waiting for Daydream
    // Server symbol explanation & confirmation (rows 21747..=21755)
    ServerExpl1, // 21747 Iyoten
    ServerExpl2, // 21748 Iyoten (Highlight: ServerSymbol)
    ServerExpl3, // 21749 Iyoten (Highlight: ServerSymbol)
    ServerExpl4, // 21750 Iyoten (Highlight: ServerSymbol)
    ServerExpl5, // 21751 Iyoten (Highlight: ServerSymbol)
    ServerExpl6, // 21752 Iyoten (Highlight: None)
    ServerExpl7, // 21753 Asta (Highlight: None)
    ServerExpl8, // 21754 Iyoten (Highlight: None)
    ServerExpl9, // 21755 Iyoten (Highlight: None)
    Completed,
}

impl TutorialStep {
    pub fn dialogue_info(&self) -> Option<(u32, TutorialSpeaker, TutorialHighlight)> {
        match self {
            // Speakers: only 21729 (Asta, f00487), 21738 and 21748 (Iyoten, f00496 / f00502) are confirmed by the
            // reference captures; the rest are assumptions until the gate script that plays these lines is traced.
            TutorialStep::Intro1 => Some((MSG_TUT_21728, TutorialSpeaker::Iyoten, TutorialHighlight::None)),
            TutorialStep::Intro2 => Some((MSG_TUT_21729, TutorialSpeaker::Asta, TutorialHighlight::None)),
            TutorialStep::Intro3 => Some((MSG_TUT_21730, TutorialSpeaker::Iyoten, TutorialHighlight::None)),
            TutorialStep::Intro4 => Some((MSG_TUT_21731, TutorialSpeaker::Iyoten, TutorialHighlight::None)),
            TutorialStep::Intro5 => Some((MSG_TUT_21732, TutorialSpeaker::Iyoten, TutorialHighlight::None)),
            TutorialStep::AreaWordIntro1 => Some((MSG_TUT_21735, TutorialSpeaker::Iyoten, TutorialHighlight::None)),
            TutorialStep::AreaWordIntro2 => Some((MSG_TUT_21736, TutorialSpeaker::Iyoten, TutorialHighlight::WordPlate)),
            TutorialStep::AreaWordIntro3 => Some((MSG_TUT_21737, TutorialSpeaker::Iyoten, TutorialHighlight::WordPlate)),
            TutorialStep::AreaWordIntro4 => Some((MSG_TUT_21738, TutorialSpeaker::Iyoten, TutorialHighlight::WordGyro)),
            TutorialStep::AreaWordIntro5 => Some((MSG_TUT_21739, TutorialSpeaker::Iyoten, TutorialHighlight::None)),
            TutorialStep::AreaWordIntro6 => Some((MSG_TUT_21740, TutorialSpeaker::Iyoten, TutorialHighlight::None)),
            TutorialStep::ServerExpl1 => Some((MSG_TUT_21747, TutorialSpeaker::Iyoten, TutorialHighlight::None)),
            TutorialStep::ServerExpl2 => Some((MSG_TUT_21748, TutorialSpeaker::Iyoten, TutorialHighlight::ServerSymbol)),
            TutorialStep::ServerExpl3 => Some((MSG_TUT_21749, TutorialSpeaker::Iyoten, TutorialHighlight::ServerSymbol)),
            TutorialStep::ServerExpl4 => Some((MSG_TUT_21750, TutorialSpeaker::Iyoten, TutorialHighlight::ServerSymbol)),
            TutorialStep::ServerExpl5 => Some((MSG_TUT_21751, TutorialSpeaker::Iyoten, TutorialHighlight::ServerSymbol)),
            TutorialStep::ServerExpl6 => Some((MSG_TUT_21752, TutorialSpeaker::Iyoten, TutorialHighlight::None)),
            TutorialStep::ServerExpl7 => Some((MSG_TUT_21753, TutorialSpeaker::Asta, TutorialHighlight::None)),
            TutorialStep::ServerExpl8 => Some((MSG_TUT_21754, TutorialSpeaker::Iyoten, TutorialHighlight::None)),
            TutorialStep::ServerExpl9 => Some((MSG_TUT_21755, TutorialSpeaker::Iyoten, TutorialHighlight::None)),
            _ => None,
        }
    }

    pub fn guide_prompt_id(&self) -> Option<u32> {
        match self {
            TutorialStep::ChooseAreaWord => Some(MSG_GUIDE_AREA_WORD),
            TutorialStep::PickWord1 => Some(MSG_GUIDE_COURAGEOUS),
            TutorialStep::PickWord2 => Some(MSG_GUIDE_ENGAGING),
            TutorialStep::PickWord3 => Some(MSG_GUIDE_DAYDREAM),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CgState {
    /// Side list: Warp Menu / Cancel.
    Gate { cursor: usize },
    Warp { cursor: usize },
    AreaWord { col: usize, picked: [bool; 3] },
    Confirm { yes: bool },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CgAction {
    None,
    Close,
    /// Warp to the area named by the three words.
    Warp,
}

pub struct ChaosGateScreen {
    pub state: CgState,
    tex: HashMap<String, (TextureHandle, [usize; 2])>,
    pub text: HashMap<u32, String>,
    pub status: Option<String>,
    pub server: AreaServer,
    pub owned_words: [Vec<usize>; 3],
    pub word_cursors: [usize; 3],
    pub haseo_lv: u32,

    // Tutorial state machine
    pub is_tutorial_active: bool,
    pub tutorial_step: TutorialStep,
    pub active_rejection: Option<(u32, TutorialSpeaker)>,

    // Warp history
    pub warp_records: Vec<(AreaServer, usize, usize, usize)>,
}

fn load(ctx: &egui::Context, path: &str, names: &[&str], out: &mut HashMap<String, (TextureHandle, [usize; 2])>) {
    crate::altimit::load_textures(ctx, Path::new(path), names, out);
}

impl ChaosGateScreen {
    pub fn new(ctx: &egui::Context, lang_code: &str) -> Self {
        let mut tex = HashMap::new();
        load(ctx, "target/cmn/data/data/xmenu_00@ENG.ccs", &["xmenu_00", "xmncsr_00", "xtext_00", "xicon_00"], &mut tex);
        load(ctx, "target/cmn/data/data/menu/xchaos_01@ENG.ccs", &["xsrnd_01"], &mut tex);
        load(ctx, "target/cmn/data/data/btn00@ENG@WIN64.ccs", &["btn00"], &mut tex);

        let owned_words = [
            areaword::initial_owned_words(AreaWordSlot::First),
            areaword::initial_owned_words(AreaWordSlot::Second),
            areaword::initial_owned_words(AreaWordSlot::Third),
        ];

        let mut text = HashMap::new();
        if let Some(l) = MessageInfo::load(Path::new("target"), lang_code) {
            let base_ids = [
                MSG_CHAOS_GATE, MSG_WARP_MENU, MSG_AREA_WORD, MSG_RANDOM, MSG_BOOKMARK, MSG_WARP_RECORD,
                MSG_TO_TOWN, MSG_CANCEL, MSG_INITIATE, MSG_HELP_GATE, MSG_HELP_TO_TOWN, MSG_HELP_AREA_WORD,
                MSG_HELP_RANDOM, MSG_HELP_BOOKMARK, MSG_HELP_RECORD, MSG_HELP_CLOSE, MSG_CONFIRM, MSG_RETURN,
                MSG_YES, MSG_NO, MSG_NO_WARP_RECORD,
                MSG_TUT_21728, MSG_TUT_21729, MSG_TUT_21730, MSG_TUT_21731, MSG_TUT_21732,
                MSG_TUT_21733, MSG_TUT_21734, MSG_TUT_21735, MSG_TUT_21736, MSG_TUT_21737,
                MSG_TUT_21738, MSG_TUT_21739, MSG_TUT_21740, MSG_TUT_21741, MSG_TUT_21742,
                MSG_TUT_21743, MSG_TUT_21744, MSG_TUT_21745, MSG_TUT_21746, MSG_TUT_21747,
                MSG_TUT_21748, MSG_TUT_21749, MSG_TUT_21750, MSG_TUT_21751, MSG_TUT_21752,
                MSG_TUT_21753, MSG_TUT_21754, MSG_TUT_21755,
                MSG_GUIDE_AREA_WORD, MSG_GUIDE_COURAGEOUS, MSG_GUIDE_ENGAGING, MSG_GUIDE_DAYDREAM,
                areaword::MSG_MISSION_BOSS, areaword::MSG_MISSION_TEMPLE,
                0xA097AECB, 0xADD48812, 0xA91595A5, 0xB752C5A0, 0xB393D817, 0xBED0FECE, 0xBA11E379,
                0x52A0FDF4, 0x5FE3DB2D, 0x5B22C69A, 0x4565969F, 0x41A48B28, 0x4CE7ADF1, 0x4826B046,
                0x3764FC21, 0x55557CDE,
            ];
            for id in base_ids {
                if let Some(s) = l.by_id.get(&id) {
                    text.insert(id, s.clone());
                }
            }

            for slot in [AreaWordSlot::First, AreaWordSlot::Second, AreaWordSlot::Third] {
                for w in areaword::words_for_slot(slot) {
                    if let Some(s) = l.by_id.get(&w.msg_id) {
                        text.insert(w.msg_id, s.clone());
                    } else {
                        text.insert(w.msg_id, w.word_key.to_string());
                    }
                }
            }
        }

        Self {
            state: CgState::Gate { cursor: 0 },
            tex,
            text,
            status: None,
            server: AreaServer::Delta,
            owned_words,
            word_cursors: [0, 0, 0],
            haseo_lv: 1,
            is_tutorial_active: true,
            tutorial_step: TutorialStep::Intro1,
            active_rejection: None,
            warp_records: Vec::new(),
        }
    }

    pub fn set_tutorial_active(&mut self, active: bool) {
        self.is_tutorial_active = active;
        if active {
            self.tutorial_step = TutorialStep::Intro1;
        } else {
            self.tutorial_step = TutorialStep::Completed;
        }
        self.active_rejection = None;
    }

    pub fn t(&self, id: u32, fallback: &str) -> String {
        self.text.get(&id).cloned().unwrap_or_else(|| fallback.to_string())
    }

    pub fn synthesized(&self, col: usize, picked: [bool; 3]) -> SynthesizedArea {
        let w1_idx = self.owned_words[0].get(self.word_cursors[0]).copied();
        let w2_idx = self.owned_words[1].get(self.word_cursors[1]).copied();
        let w3_idx = self.owned_words[2].get(self.word_cursors[2]).copied();

        let w1 = if picked[0] || col == 0 {
            w1_idx.and_then(|i| areaword::get_word(AreaWordSlot::First, i))
        } else {
            None
        };
        let w2 = if picked[1] || (col == 1 && picked[0]) {
            w2_idx.and_then(|i| areaword::get_word(AreaWordSlot::Second, i))
        } else {
            None
        };
        let w3 = if picked[2] || (col == 2 && picked[0] && picked[1]) {
            w3_idx.and_then(|i| areaword::get_word(AreaWordSlot::Third, i))
        } else {
            None
        };

        areaword::synthesize_area(self.server, w1, w2, w3, self.haseo_lv)
    }

    /// The chosen (server, [1st, 2nd, 3rd] word indices) on the Word Plate.
    pub fn selected_words(&self) -> (u32, [usize; 3]) {
        let w = |c: usize| self.owned_words[c].get(self.word_cursors[c]).copied().unwrap_or(0);
        (self.server as u32, [w(0), w(1), w(2)])
    }

    pub fn show(&mut self, ctx: &egui::Context) -> CgAction {
        let (up, down, left, right, ok, back) = ctx.input(|i| {
            (
                i.key_pressed(egui::Key::ArrowUp) || i.key_pressed(egui::Key::W),
                i.key_pressed(egui::Key::ArrowDown) || i.key_pressed(egui::Key::S),
                i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::A),
                i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::D),
                i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Space) || i.key_pressed(egui::Key::E),
                i.key_pressed(egui::Key::Escape) || i.key_pressed(egui::Key::Backspace) || i.key_pressed(egui::Key::Q),
            )
        });

        let mut action = CgAction::None;
        if up || down || ok || back {
            self.status = None;
        }

        // 1. If an active rejection dialogue is up, dismiss on OK or Back
        if self.active_rejection.is_some() {
            if ok || back {
                self.active_rejection = None;
            }
            self.draw(ctx);
            return action;
        }

        // 2. If tutorial dialogue is currently playing, advance dialogue on OK. The tutorial lines belong to the Warp
        // Menu and later screens (they are drawn there); on the first "Chaos Gate" screen OK must open the Warp Menu
        // (it used to step invisible tutorial lines, which made the menu look frozen).
        if self.is_tutorial_active && !matches!(self.state, CgState::Gate { .. }) {
            if let Some((_msg, _speaker, _hl)) = self.tutorial_step.dialogue_info() {
                if ok {
                    self.tutorial_step = match self.tutorial_step {
                        TutorialStep::Intro1 => TutorialStep::Intro2,
                        TutorialStep::Intro2 => TutorialStep::Intro3,
                        TutorialStep::Intro3 => TutorialStep::Intro4,
                        TutorialStep::Intro4 => TutorialStep::Intro5,
                        TutorialStep::Intro5 => TutorialStep::ChooseAreaWord,
                        TutorialStep::AreaWordIntro1 => TutorialStep::AreaWordIntro2,
                        TutorialStep::AreaWordIntro2 => TutorialStep::AreaWordIntro3,
                        TutorialStep::AreaWordIntro3 => TutorialStep::AreaWordIntro4,
                        TutorialStep::AreaWordIntro4 => TutorialStep::AreaWordIntro5,
                        TutorialStep::AreaWordIntro5 => TutorialStep::AreaWordIntro6,
                        TutorialStep::AreaWordIntro6 => TutorialStep::PickWord1,
                        TutorialStep::ServerExpl1 => TutorialStep::ServerExpl2,
                        TutorialStep::ServerExpl2 => TutorialStep::ServerExpl3,
                        TutorialStep::ServerExpl3 => TutorialStep::ServerExpl4,
                        TutorialStep::ServerExpl4 => TutorialStep::ServerExpl5,
                        TutorialStep::ServerExpl5 => TutorialStep::ServerExpl6,
                        TutorialStep::ServerExpl6 => TutorialStep::ServerExpl7,
                        TutorialStep::ServerExpl7 => TutorialStep::ServerExpl8,
                        TutorialStep::ServerExpl8 => TutorialStep::ServerExpl9,
                        TutorialStep::ServerExpl9 => {
                            self.state = CgState::Confirm { yes: true };
                            TutorialStep::Completed
                        }
                        other => other,
                    };
                }
                self.draw(ctx);
                return action;
            }
        }

        // 3. Normal State Transitions with Tutorial Constraints & Warp Actions
        self.state = match self.state {
            CgState::Gate { mut cursor } => {
                if up || down {
                    cursor = 1 - cursor;
                }
                if back || (ok && cursor == 1) {
                    action = CgAction::Close;
                    CgState::Gate { cursor: 0 }
                } else if ok {
                    // Warp Menu entry
                    CgState::Warp { cursor: 1 }
                } else {
                    CgState::Gate { cursor }
                }
            }
            CgState::Warp { mut cursor } => {
                let n = WARP_ITEMS.len();
                if up {
                    cursor = (cursor + n - 1) % n;
                }
                if down {
                    cursor = (cursor + 1) % n;
                }

                if self.is_tutorial_active && self.tutorial_step == TutorialStep::ChooseAreaWord {
                    if back {
                        // Rejection 21734 if trying to cancel during tutorial
                        self.active_rejection = Some((MSG_TUT_21734, TutorialSpeaker::Iyoten));
                        CgState::Warp { cursor }
                    } else if ok {
                        if WARP_ITEMS[cursor].0 == MSG_AREA_WORD {
                            self.tutorial_step = TutorialStep::AreaWordIntro1;
                            CgState::AreaWord { col: 0, picked: [false; 3] }
                        } else {
                            // Rejection 21733 if selecting any other menu item
                            self.active_rejection = Some((MSG_TUT_21733, TutorialSpeaker::Iyoten));
                            CgState::Warp { cursor }
                        }
                    } else {
                        CgState::Warp { cursor }
                    }
                } else if back {
                    CgState::Gate { cursor: 0 }
                } else if ok {
                    match WARP_ITEMS[cursor].0 {
                        MSG_AREA_WORD => CgState::AreaWord { col: 0, picked: [false; 3] },
                        MSG_RANDOM => {
                            // Pick 3 random words with MT19937 PRNG from owned stock
                            let mut rng = Mt19937::new(
                                std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap_or_default()
                                    .as_millis() as u32,
                            );
                            for c in 0..3 {
                                let len = self.owned_words[c].len();
                                if len > 0 {
                                    self.word_cursors[c] = (rng.rand() as usize) % len;
                                }
                            }
                            CgState::AreaWord { col: 2, picked: [true; 3] }
                        }
                        MSG_WARP_RECORD => {
                            if self.warp_records.is_empty() {
                                self.status = Some(self.t(MSG_NO_WARP_RECORD, ""));
                                CgState::Warp { cursor }
                            } else {
                                let &(server, w1, w2, w3) = self.warp_records.last().unwrap();
                                self.server = server;
                                self.word_cursors = [w1, w2, w3];
                                CgState::AreaWord { col: 2, picked: [true; 3] }
                            }
                        }
                        MSG_CANCEL => {
                            action = CgAction::Close;
                            CgState::Gate { cursor: 0 }
                        }
                        _ => {
                            self.status = Some(format!("{} is not implemented yet.", WARP_ITEMS[cursor].1));
                            CgState::Warp { cursor }
                        }
                    }
                } else {
                    CgState::Warp { cursor }
                }
            }
            CgState::AreaWord { mut col, mut picked } => {
                let n = self.owned_words[col].len();
                if up && n > 0 {
                    self.word_cursors[col] = (self.word_cursors[col] + n - 1) % n;
                }
                if down && n > 0 {
                    self.word_cursors[col] = (self.word_cursors[col] + 1) % n;
                }

                if self.is_tutorial_active {
                    match self.tutorial_step {
                        TutorialStep::PickWord1 => {
                            if ok {
                                // Validate Courageous (index 0)
                                if self.word_cursors[0] == 0 {
                                    picked[0] = true;
                                    col = 1;
                                    self.tutorial_step = TutorialStep::PickWord2;
                                } else {
                                    self.active_rejection = Some((MSG_TUT_21742, TutorialSpeaker::Iyoten));
                                }
                            }
                            CgState::AreaWord { col, picked }
                        }
                        TutorialStep::PickWord2 => {
                            if back {
                                col = 0;
                                picked[0] = false;
                                self.tutorial_step = TutorialStep::PickWord1;
                            } else if ok {
                                // Validate Engaging (index 0)
                                if self.word_cursors[1] == 0 {
                                    picked[1] = true;
                                    col = 2;
                                    self.tutorial_step = TutorialStep::PickWord3;
                                } else {
                                    self.active_rejection = Some((MSG_TUT_21744, TutorialSpeaker::Iyoten));
                                }
                            }
                            CgState::AreaWord { col, picked }
                        }
                        TutorialStep::PickWord3 => {
                            if back {
                                col = 1;
                                picked[1] = false;
                                self.tutorial_step = TutorialStep::PickWord2;
                            } else if ok {
                                // Validate Daydream (index 0)
                                if self.word_cursors[2] == 0 {
                                    picked[2] = true;
                                    self.tutorial_step = TutorialStep::ServerExpl1;
                                } else {
                                    self.active_rejection = Some((MSG_TUT_21746, TutorialSpeaker::Iyoten));
                                }
                            }
                            CgState::AreaWord { col, picked }
                        }
                        _ => CgState::AreaWord { col, picked },
                    }
                } else if back {
                    if col > 0 {
                        col -= 1;
                        picked[col] = false;
                        CgState::AreaWord { col, picked }
                    } else {
                        CgState::Warp { cursor: 2 }
                    }
                } else if ok {
                    picked[col] = true;
                    if col == 2 {
                        CgState::Confirm { yes: true }
                    } else {
                        CgState::AreaWord { col: col + 1, picked }
                    }
                } else {
                    let _ = (left, right);
                    CgState::AreaWord { col, picked }
                }
            }
            CgState::Confirm { mut yes } => {
                if up || down {
                    yes = !yes;
                }
                if back || (ok && !yes) {
                    CgState::AreaWord { col: 2, picked: [true, true, false] }
                } else if ok {
                    action = CgAction::Warp;
                    self.is_tutorial_active = false;
                    self.tutorial_step = TutorialStep::Completed;
                    let record = (self.server, self.word_cursors[0], self.word_cursors[1], self.word_cursors[2]);
                    if self.warp_records.last() != Some(&record) {
                        self.warp_records.push(record);
                    }
                    CgState::Gate { cursor: 0 }
                } else {
                    CgState::Confirm { yes }
                }
            }
        };

        self.draw(ctx);
        action
    }

    fn draw(&self, ctx: &egui::Context) {
        let screen = ctx.screen_rect();
        let (w, h) = if screen.width() / screen.height() > 16.0 / 9.0 {
            (screen.height() * 16.0 / 9.0, screen.height())
        } else {
            (screen.width(), screen.width() * 9.0 / 16.0)
        };
        let origin = Pos2::new(screen.center().x - w * 0.5, screen.center().y - h * 0.5);
        let d = Draw {
            p: ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("chaos_gate"))),
            origin,
            sx: w / 960.0,
            sy: h / 540.0,
            tex: &self.tex,
        };

        match self.state {
            CgState::Gate { cursor } => {
                d.header(&self.t(MSG_CHAOS_GATE, "Chaos Gate"), &self.t(MSG_CONFIRM, "Confirm"), &self.t(MSG_RETURN, "Return"));
                d.side_list(&self.t(MSG_CHAOS_GATE, "Chaos Gate"), &[self.t(MSG_WARP_MENU, "Warp Menu"), self.t(MSG_CANCEL, "Cancel")], cursor);
                let help = if cursor == 0 { self.t(MSG_HELP_GATE, "") } else { self.t(MSG_HELP_CLOSE, "") };
                d.message_box(&[self.status.clone().unwrap_or(help)]);
            }
            CgState::Warp { cursor } => {
                d.fill(Color32::from_rgba_unmultiplied(20, 6, 4, 150));
                // The wheel runs behind the header bar (as in the Main Menu, run1_opening f00064).
                let items: Vec<String> = WARP_ITEMS.iter().map(|(id, fb, _)| self.t(*id, fb)).collect();
                d.wheel(&items, cursor, &WHEEL_WARP, HEX_WARP);
                d.header(&self.t(MSG_WARP_MENU, "Warp Menu"), &self.t(MSG_CONFIRM, "Confirm"), &self.t(MSG_RETURN, "Return"));

                if let Some((rej_id, spk)) = self.active_rejection {
                    d.formatted_message_box(Some(spk.name()), &self.t(rej_id, ""), true);
                } else if self.is_tutorial_active {
                    if let Some((tut_id, spk, _)) = self.tutorial_step.dialogue_info() {
                        d.formatted_message_box(Some(spk.name()), &self.t(tut_id, ""), true);
                    } else {
                        let help = self.status.clone().unwrap_or_else(|| self.t(WARP_ITEMS[cursor].2, ""));
                        d.message_box(&[help]);
                        if let Some(guide_id) = self.tutorial_step.guide_prompt_id() {
                            d.guide_box(&self.t(guide_id, "Select \"Area Word.\""));
                        }
                    }
                } else {
                    let help = self.status.clone().unwrap_or_else(|| self.t(WARP_ITEMS[cursor].2, ""));
                    d.message_box(&[help]);
                }
            }
            CgState::AreaWord { col, picked } => self.draw_area_word(&d, col, picked, None),
            CgState::Confirm { yes } => self.draw_area_word(&d, 2, [true; 3], Some(yes)),
        }
    }

    fn draw_area_word(&self, d: &Draw, col: usize, picked: [bool; 3], confirm: Option<bool>) {
        d.fill(Color32::from_rgba_unmultiplied(34, 6, 6, 215));
        d.header(&self.t(MSG_AREA_WORD, "Area Word"), &self.t(MSG_CONFIRM, "Confirm"), &self.t(MSG_RETURN, "Return"));

        // Evaluate synthesized area dynamically from player's selections
        let synth = self.synthesized(col, picked);
        let full = picked[0] && picked[1] && (picked[2] || col == 2);

        // Gyro wheel holds the owned words for the active slot in table order
        let cur_slot = match col {
            0 => AreaWordSlot::First,
            1 => AreaWordSlot::Second,
            _ => AreaWordSlot::Third,
        };
        let owned = &self.owned_words[col];
        let sel = self.word_cursors[col];
        let items: Vec<String> = if !owned.is_empty() {
            owned
                .iter()
                .map(|&idx| {
                    let w = areaword::get_word(cur_slot, idx).unwrap();
                    self.t(w.msg_id, w.word_key)
                })
                .collect()
        } else {
            vec!["".to_string()]
        };
        d.wheel(&items, sel, &WHEEL_AREA, HEX_AREA);

        // Slot header ("1st"/"2nd"/"3rd" over the active column).
        let hdr_uv = [[0.0156, 0.0195, 0.2520, 0.2227], [0.2773, 0.0156, 0.6348, 0.2227], [0.6543, 0.0156, 0.9922, 0.2266]];
        let uv = hdr_uv[col];
        let hw = (uv[2] - uv[0]) * 512.0 * 0.18;
        let cx = WORD_COL_X[col];
        d.img("xmncsr_00", [cx - hw * 0.5, 62.0, cx + hw * 0.5, 81.0], uv, Color32::WHITE);

        // Word slots bar with the Δ server mark.
        d.poly(&[(330.0, 92.0), (918.0, 92.0), (930.0, 108.0), (918.0, 124.0), (330.0, 124.0)], Color32::from_rgba_unmultiplied(16, 12, 12, 235));
        d.rect([330.0, 92.0, 930.0, 93.0], Color32::from_rgb(70, 58, 58));
        d.rect([330.0, 123.0, 930.0, 124.0], Color32::from_rgb(70, 58, 58));
        d.rect([336.0, 95.0, 388.0, 121.0], Color32::from_rgb(28, 24, 24));
        d.img("xtext_00", [343.0, 98.0, 381.0, 119.0], [0.5664, 0.0938, 0.6855, 0.1562], Color32::WHITE);
        d.rect([cx - 88.0, 94.0, cx + 88.0, 122.0], Color32::from_rgb(22, 58, 58));

        for c in 0..3 {
            let slot_c = match c {
                0 => AreaWordSlot::First,
                1 => AreaWordSlot::Second,
                _ => AreaWordSlot::Third,
            };
            if picked[c] || c == col {
                if let Some(&word_idx) = self.owned_words[c].get(self.word_cursors[c]) {
                    if let Some(w) = areaword::get_word(slot_c, word_idx) {
                        d.text_c(WORD_COL_X[c], 108.0, &self.t(w.msg_id, w.word_key), 12.5, Color32::from_rgb(205, 205, 205));
                    }
                }
            }
        }

        // Area Status panel.
        d.rect([348.0, 128.0, 910.0, 422.0], Color32::from_rgba_unmultiplied(22, 16, 16, 200));
        d.frame([352.0, 133.0, 906.0, 418.0], Color32::from_rgb(88, 78, 78));
        d.frame([358.0, 139.0, 900.0, 412.0], Color32::from_rgb(52, 46, 46));
        d.img("xtext_00", [548.0, 132.0, 692.0, 150.0], [0.2646, 0.8838, 0.7150, 0.9375], Color32::WHITE);
        d.img("xtext_00", [410.0, 160.0, 478.0, 183.0], [0.4902, 0.6992, 0.6953, 0.7676], Color32::WHITE);
        d.img("xtext_00", [478.0, 160.0, 528.0, 183.0], [0.7051, 0.7051, 0.8369, 0.7676], Color32::WHITE);

        // Area LV is shown by the large digit on the thumbnail (original f00496/f00502), not next to the label.

        // Thumbnail (xsrnd_01 4x4 cells; stage_idx / cell from SynthesizedArea).
        let cell = synth.thumbnail_cell;
        let (cu, cv) = ((cell % 4) as f32 * 0.25, (cell / 4) as f32 * 0.25);
        d.img(
            "xsrnd_01",
            [382.0, 187.0, 562.0, 323.0],
            [cu + 5.0 / 1024.0, cv + 65.0 / 1024.0, cu + 251.0 / 1024.0, cv + 251.0 / 1024.0],
            Color32::WHITE,
        );
        d.img("xtext_00", [512.0, 196.0, 541.0, 233.0], [0.8496, 0.2227, 0.9365, 0.3223], Color32::WHITE);

        // Players / party counts.
        d.rect([630.0, 165.0, 855.0, 193.0], Color32::from_rgba_unmultiplied(48, 40, 40, 220));
        d.img("xicon_00", [636.0, 166.0, 676.0, 192.0], [0.7676, 0.0117, 0.9727, 0.1367], Color32::WHITE);
        d.img("xicon_00", [748.0, 166.0, 790.0, 192.0], [0.7676, 0.1465, 0.9668, 0.2832], Color32::WHITE);
        let party_count = format!("{:03}", synth.party_count);
        let player_count = format!("{:03}", synth.player_count);
        d.rect([680.0, 169.0, 724.0, 189.0], Color32::from_rgb(28, 24, 26));
        d.rect([794.0, 169.0, 838.0, 189.0], Color32::from_rgb(28, 24, 26));
        d.text_c(702.0, 179.0, &party_count, 12.5, Color32::from_rgb(220, 220, 220));
        d.text_c(816.0, 179.0, &player_count, 12.5, Color32::from_rgb(220, 220, 220));

        // Element.
        d.rect([618.0, 228.0, 855.0, 256.0], Color32::from_rgba_unmultiplied(48, 40, 40, 160));
        d.img("xtext_00", [620.0, 230.0, 700.0, 250.0], [0.5107, 0.6094, 0.7344, 0.6660], Color32::WHITE);
        for i in 0..6 {
            let u0 = [0.0215, 0.1465, 0.2715, 0.3965, 0.5215, 0.6465][i];
            let x = 696.0 + i as f32 * 20.0;
            let y = if i % 2 == 0 { 233.0 } else { 240.0 };
            let tint = if synth.elements[i] { Color32::WHITE } else { Color32::from_rgba_unmultiplied(255, 255, 255, 40) };
            d.img("xicon_00", [x, y, x + 19.0, y + 19.0], [u0, 0.1465, u0 + 0.082, 0.2285], tint);
        }

        // Treasure LV and gauge.
        d.rect([618.0, 283.0, 855.0, 308.0], Color32::from_rgba_unmultiplied(48, 40, 40, 160));
        d.img("xtext_00", [621.0, 287.0, 706.0, 304.0], [0.7500, 0.6064, 0.9922, 0.6631], Color32::WHITE);
        d.img("xtext_00", [705.0, 285.0, 760.0, 306.0], [0.7051, 0.7051, 0.8369, 0.7676], Color32::WHITE);
        d.draw_number_sprites(833.0, 285.0, synth.treasure_lv);

        // Gauge: three slanted segments either side of the -/+ marker.
        for i in 0..6 {
            let x = if i < 3 { 665.0 + i as f32 * 21.0 } else { 757.0 + (i - 3) as f32 * 21.0 };
            let is_lit = if i < 3 {
                synth.treasure_gauge < 0 && (2 - i as i32) < (-synth.treasure_gauge)
            } else {
                synth.treasure_gauge > 0 && (i as i32 - 3) < synth.treasure_gauge
            };
            let col = if is_lit { Color32::from_rgb(110, 205, 180) } else { Color32::from_rgb(70, 66, 70) };
            d.poly(&[(x + 3.0, 314.0), (x + 20.0, 314.0), (x + 17.0, 324.0), (x, 324.0)], col);
            d.poly_outline(&[(x + 3.0, 314.0), (x + 20.0, 314.0), (x + 17.0, 324.0), (x, 324.0)], Color32::from_rgb(170, 165, 170));
        }
        d.img("xicon_00", [728.0, 311.0, 758.0, 327.0], [0.7344, 0.4004, 0.9551, 0.5176], Color32::WHITE);

        // Mission.
        d.img("xtext_00", [385.0, 343.0, 495.0, 366.0], [0.6787, 0.5078, 0.9961, 0.5781], Color32::WHITE);
        let mission = if synth.mission_id != 0 { self.t(synth.mission_id, "Defeat the boss!") } else { String::new() };
        d.text_l(387.0, 385.0, &mission, 13.0, Color32::from_rgb(215, 215, 215));

        // Tutorial Highlight overlays
        if self.is_tutorial_active {
            if let Some((_, _, hl)) = self.tutorial_step.dialogue_info() {
                match hl {
                    TutorialHighlight::WordPlate => {
                        d.highlight_rect([318.0, 80.0, 938.0, 136.0]);
                    }
                    TutorialHighlight::WordGyro => {
                        d.highlight_rect([0.0, 50.0, 242.0, 370.0]);
                    }
                    TutorialHighlight::ServerSymbol => {
                        d.highlight_circle((360.0, 109.0), 30.0);
                    }
                    TutorialHighlight::None => {}
                }
            }
        }

        // Bottom Message Box & Guided Prompt Box
        if let Some((rej_id, spk)) = self.active_rejection {
            d.formatted_message_box(Some(spk.name()), &self.t(rej_id, ""), true);
        } else if self.is_tutorial_active {
            if let Some((tut_id, spk, _)) = self.tutorial_step.dialogue_info() {
                d.formatted_message_box(Some(spk.name()), &self.t(tut_id, ""), true);
            } else {
                let help = if full {
                    vec![self.t(synth.monster_help_msg_id, ""), self.t(synth.item_help_msg_id, "")]
                } else {
                    vec![self.t(MSG_HELP_AREA_WORD, "")]
                };
                d.message_box(&help);
                if let Some(guide_id) = self.tutorial_step.guide_prompt_id() {
                    d.guide_box(&self.t(guide_id, ""));
                }
            }
        } else {
            let help = if full {
                vec![self.t(synth.monster_help_msg_id, ""), self.t(synth.item_help_msg_id, "")]
            } else {
                vec![self.t(MSG_HELP_AREA_WORD, "")]
            };
            d.message_box(&help);
        }

        // Confirmation Dialog
        if let Some(yes) = confirm {
            d.poly(
                &[(262.0, 238.0), (698.0, 238.0), (712.0, 252.0), (712.0, 316.0), (698.0, 330.0), (262.0, 330.0), (248.0, 316.0), (248.0, 252.0)],
                Color32::from_rgba_unmultiplied(12, 10, 10, 235),
            );
            d.poly_outline(
                &[(262.0, 238.0), (698.0, 238.0), (712.0, 252.0), (712.0, 316.0), (698.0, 330.0), (262.0, 330.0), (248.0, 316.0), (248.0, 252.0)],
                Color32::from_rgb(120, 110, 110),
            );
            d.poly(&[(394.0, 222.0), (566.0, 222.0), (580.0, 237.0), (566.0, 253.0), (394.0, 253.0), (380.0, 237.0)], Color32::from_rgb(22, 20, 20));
            d.poly_outline(&[(394.0, 222.0), (566.0, 222.0), (580.0, 237.0), (566.0, 253.0), (394.0, 253.0), (380.0, 237.0)], Color32::from_rgb(120, 110, 110));
            d.text_c(480.0, 238.0, &self.t(MSG_INITIATE, "Initiate Area warp?"), 13.0, Color32::from_rgb(230, 230, 230));
            let sel_y = if yes { 275.0 } else { 311.0 };
            d.teal_bar([428.0, sel_y - 12.0, 540.0, sel_y + 12.0]);
            d.text_c(482.0, 275.0, &self.t(MSG_YES, "Yes"), 13.5, Color32::WHITE);
            d.text_c(482.0, 311.0, &self.t(MSG_NO, "No"), 13.5, Color32::WHITE);
        }
    }
}

/// Helper struct for parsed formatted text spans.
#[derive(Clone, Debug)]
pub struct ColoredSpan {
    pub text: String,
    pub color: Color32,
}

pub fn parse_formatted_text(s: &str, default_color: Color32) -> Vec<ColoredSpan> {
    let mut spans = Vec::new();
    let mut cur_color = default_color;
    let mut cur_text = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '<' {
            let mut tag = String::new();
            for tc in chars.by_ref() {
                if tc == '>' {
                    break;
                }
                tag.push(tc);
            }
            if tag.starts_with("color") && tag.len() == 11 {
                if let Ok(rgb) = u32::from_str_radix(&tag[5..11], 16) {
                    if !cur_text.is_empty() {
                        spans.push(ColoredSpan { text: cur_text, color: cur_color });
                        cur_text = String::new();
                    }
                    let r = ((rgb >> 16) & 0xFF) as u8;
                    let g = ((rgb >> 8) & 0xFF) as u8;
                    let b = (rgb & 0xFF) as u8;
                    cur_color = Color32::from_rgb(r, g, b);
                    continue;
                }
            } else if tag.starts_with("ruby") {
                if let Some(pipe_idx) = tag.find('|') {
                    let kanji = &tag[4..pipe_idx];
                    cur_text.push_str(kanji);
                    continue;
                }
            }
        } else {
            cur_text.push(c);
        }
    }
    if !cur_text.is_empty() {
        spans.push(ColoredSpan { text: cur_text, color: cur_color });
    }
    spans
}

/// Drawing in 960x540 reference pixels.
pub(crate) struct Draw<'a> {
    pub p: egui::Painter,
    pub origin: Pos2,
    pub sx: f32,
    pub sy: f32,
    pub tex: &'a HashMap<String, (TextureHandle, [usize; 2])>,
}

impl Draw<'_> {
    pub(crate) fn pt(&self, x: f32, y: f32) -> Pos2 {
        Pos2::new(self.origin.x + x * self.sx, self.origin.y + y * self.sy)
    }
    pub(crate) fn r(&self, b: [f32; 4]) -> Rect {
        Rect::from_min_max(self.pt(b[0], b[1]), self.pt(b[2], b[3]))
    }
    pub(crate) fn fill(&self, c: Color32) {
        self.p.rect_filled(self.r([0.0, 0.0, 960.0, 540.0]), 0.0, c);
    }
    pub(crate) fn rect(&self, b: [f32; 4], c: Color32) {
        self.p.rect_filled(self.r(b), 0.0, c);
    }
    pub(crate) fn frame(&self, b: [f32; 4], c: Color32) {
        self.p.rect_stroke(self.r(b), 0.0, egui::Stroke::new(1.5_f32, c), egui::StrokeKind::Middle);
    }
    pub(crate) fn poly(&self, pts: &[(f32, f32)], c: Color32) {
        self.p.add(egui::Shape::convex_polygon(pts.iter().map(|&(x, y)| self.pt(x, y)).collect(), c, egui::Stroke::NONE));
    }
    pub(crate) fn poly_outline(&self, pts: &[(f32, f32)], c: Color32) {
        self.p.add(egui::Shape::closed_line(pts.iter().map(|&(x, y)| self.pt(x, y)).collect(), egui::Stroke::new(1.2_f32, c)));
    }
    pub(crate) fn img(&self, name: &str, b: [f32; 4], uv: [f32; 4], tint: Color32) {
        if let Some((t, _)) = self.tex.get(name) {
            self.p.image(t.id(), self.r(b), Rect::from_min_max(Pos2::new(uv[0], uv[1]), Pos2::new(uv[2], uv[3])), tint);
        }
    }
    /// `cap` = capital height in reference pixels.
    pub(crate) fn font(&self, cap: f32) -> egui::FontId {
        egui::FontId::proportional(cap * self.sy / 0.70)
    }
    pub(crate) fn text_c(&self, x: f32, y: f32, s: &str, cap: f32, c: Color32) {
        self.p.text(self.pt(x, y), egui::Align2::CENTER_CENTER, s, self.font(cap), c);
    }
    pub(crate) fn text_l(&self, x: f32, y: f32, s: &str, cap: f32, c: Color32) {
        self.p.text(self.pt(x, y), egui::Align2::LEFT_CENTER, s, self.font(cap), c);
    }
    pub(crate) fn glyph(&self, x: f32, cy: f32, size: f32, b: usize) {
        // btn00 first row: 0 B, 1 Y, 2 X, 3 A (70 px circles at 5 + 80 i).
        if let Some((t, _)) = self.tex.get("btn00") {
            let u0 = [5.0, 85.0, 165.0, 245.0][b] / 512.0;
            self.p.image(
                t.id(),
                self.r([x, cy - size * 0.5, x + size, cy + size * 0.5]),
                Rect::from_min_max(Pos2::new(u0, 4.0 / 512.0), Pos2::new(u0 + 70.0 / 512.0, 74.0 / 512.0)),
                Color32::WHITE,
            );
        }
    }
    pub(crate) fn teal_bar(&self, b: [f32; 4]) {
        let [x0, y0, x1, y1] = b;
        self.rect(b, Color32::from_rgb(28, 96, 84));
        self.rect([x0, y0, x1, y0 + 2.0], Color32::from_rgb(110, 205, 180));
        self.rect([x0, y1 - 2.0, x1, y1], Color32::from_rgb(70, 160, 140));
        self.rect([x0 + 2.0, y0 + 4.0, x0 + 5.0, y1 - 4.0], Color32::from_rgb(140, 225, 200));
        self.rect([x1 - 5.0, y0 + 4.0, x1 - 2.0, y1 - 4.0], Color32::from_rgb(140, 225, 200));
    }

    /// Red tutorial highlight box with rounded corners
    pub(crate) fn highlight_rect(&self, b: [f32; 4]) {
        self.p.rect_stroke(
            self.r(b),
            6.0 * self.sy,
            egui::Stroke::new(2.5 * self.sy, Color32::from_rgb(215, 30, 30)),
            egui::StrokeKind::Middle,
        );
    }

    /// Red tutorial highlight circle
    pub(crate) fn highlight_circle(&self, center: (f32, f32), radius: f32) {
        self.p.circle_stroke(
            self.pt(center.0, center.1),
            radius * self.sy,
            egui::Stroke::new(2.5 * self.sy, Color32::from_rgb(215, 30, 30)),
        );
    }

    /// Bottom-right guided prompt box (Select "...")
    pub(crate) fn guide_box(&self, text: &str) {
        let b = [682.0, 452.0, 911.0, 509.0];
        self.p.rect_filled(self.r(b), 6.0 * self.sy, Color32::from_rgba_unmultiplied(32, 14, 14, 240));
        self.p.rect_stroke(
            self.r(b),
            6.0 * self.sy,
            egui::Stroke::new(2.0 * self.sy, Color32::from_rgb(180, 25, 25)),
            egui::StrokeKind::Middle,
        );
        self.text_c(796.5, 480.5, text, 13.5, Color32::WHITE);
    }

    /// Top bar: title at the left, "A: Confirm  B: Return" at the right.
    pub(crate) fn header(&self, title: &str, confirm: &str, ret: &str) {
        self.rect([0.0, 24.0, 960.0, 58.0], Color32::from_rgba_unmultiplied(14, 10, 10, 225));
        self.rect([0.0, 24.0, 960.0, 25.0], Color32::from_rgb(60, 50, 50));
        self.rect([0.0, 57.0, 960.0, 58.0], Color32::from_rgb(60, 50, 50));
        self.poly(&[(248.0, 41.0), (266.0, 25.0), (292.0, 25.0), (274.0, 41.0), (292.0, 57.0), (266.0, 57.0)], Color32::from_rgb(28, 22, 22));
        self.text_l(48.0, 41.0, title, 13.5, Color32::from_rgb(225, 225, 225));
        let font = self.font(13.5);
        let col = Color32::from_rgb(225, 225, 225);
        let mut x = 912.0 * self.sx + self.origin.x;
        for (b, label) in [(3usize, confirm), (0usize, ret)].iter().rev() {
            let g = self.p.layout_no_wrap(format!(": {}", label), font.clone(), col);
            x -= g.size().x;
            let y = self.pt(0.0, 41.0).y - g.size().y * 0.5;
            self.p.galley(Pos2::new(x, y), g, col);
            x -= 20.0 * self.sx;
            self.glyph((x - self.origin.x) / self.sx, 41.0, 19.0, *b);
            x -= 8.0 * self.sx;
        }
    }

    pub(crate) fn message_box(&self, lines: &[String]) {
        self.rect([8.0, 432.0, 952.0, 530.0], Color32::from_rgba_unmultiplied(6, 5, 5, 220));
        self.frame([8.0, 432.0, 952.0, 530.0], Color32::from_rgb(64, 56, 56));
        for (i, l) in lines.iter().enumerate() {
            self.text_l(48.0, 455.0 + i as f32 * 25.0, l, 13.5, Color32::from_rgb(230, 230, 230));
        }
    }

    /// Render message box with speaker name tab and color tags `<color...>`
    pub(crate) fn formatted_message_box(&self, speaker: Option<&str>, text: &str, show_prompt_icon: bool) {
        // Speaker tab
        if let Some(spk) = speaker {
            let tab_poly = [
                (112.0, 410.0),
                (338.0, 410.0),
                (356.0, 426.0),
                (356.0, 432.0),
                (94.0, 432.0),
                (94.0, 426.0),
            ];
            self.poly(&tab_poly, Color32::from_rgba_unmultiplied(18, 15, 15, 240));
            self.poly_outline(&tab_poly, Color32::from_rgb(120, 40, 40));
            self.text_c(225.0, 421.0, spk, 13.5, Color32::WHITE);
        }

        // Message body box
        self.rect([8.0, 432.0, 952.0, 530.0], Color32::from_rgba_unmultiplied(6, 5, 5, 225));
        self.frame([8.0, 432.0, 952.0, 530.0], Color32::from_rgb(72, 48, 48));

        // Format body text lines
        let raw_lines: Vec<&str> = text.split('\n').collect();
        let font = self.font(13.5);
        for (line_idx, line_str) in raw_lines.iter().enumerate() {
            let spans = parse_formatted_text(line_str, Color32::from_rgb(230, 230, 230));
            let mut cur_x = 48.0;
            let cur_y = 458.0 + line_idx as f32 * 25.0;
            for span in spans {
                let galley = self.p.layout_no_wrap(span.text.clone(), font.clone(), span.color);
                let w = galley.size().x / self.sx;
                let text_pos = self.pt(cur_x, cur_y);
                self.p.galley(Pos2::new(text_pos.x, text_pos.y - galley.size().y * 0.5), galley, span.color);
                cur_x += w;
            }
        }

        // Blinking prompt button (A) on bottom-right
        if show_prompt_icon {
            self.glyph(920.0, 515.0, 19.0, 3);
        }
    }

    /// Right-hand list used by the gate prompt.
    pub(crate) fn side_list(&self, title: &str, items: &[String], cursor: usize) {
        self.poly(&[(640.0, 103.0), (960.0, 103.0), (960.0, 135.0), (640.0, 135.0), (628.0, 119.0)], Color32::from_rgba_unmultiplied(18, 15, 15, 235));
        self.text_l(647.0, 119.0, title, 13.5, Color32::from_rgb(225, 225, 225));
        self.poly(&[(652.0, 138.0), (960.0, 138.0), (960.0, 218.0), (652.0, 218.0), (640.0, 206.0), (640.0, 150.0)], Color32::from_rgba_unmultiplied(10, 8, 8, 200));
        for (i, s) in items.iter().enumerate() {
            let y = 159.0 + i as f32 * 32.0;
            if i == cursor {
                self.teal_bar([644.0, y - 13.0, 960.0, y + 13.0]);
            }
            self.text_l(671.0, y, s, 13.5, Color32::from_rgb(230, 230, 230));
        }
    }

    /// Looping word/command wheel: rows at `rows[k + 5]` for k = -5..=4 around the selection.
    pub(crate) fn wheel(&self, items: &[String], sel: usize, rows: &[(f32, f32); 10], hex: [f32; 4]) {
        self.img("xmenu_00", hex, [0.0098, 0.0059, 0.4766, 0.7812], Color32::WHITE);
        let n = items.len() as i32;
        if n == 0 {
            return;
        }
        for k in -5..=4i32 {
            let (x, y) = rows[(k + 5) as usize];
            let idx = (sel as i32 + k).rem_euclid(n) as usize;
            let fade = (1.0 - (k.abs() as f32 - 1.0).max(0.0) * 0.2).clamp(0.15, 1.0);
            let pill = if k == 0 { Color32::from_rgba_unmultiplied(122, 22, 28, 235) } else { Color32::from_rgba_unmultiplied(14, 11, 11, (235.0 * fade) as u8) };
            self.poly(&[(x - 140.0, y - 14.0), (x + 92.0, y - 14.0), (x + 106.0, y), (x + 92.0, y + 14.0), (x - 140.0, y + 14.0)], pill);
            let col = if k == 0 { Color32::from_rgb(240, 222, 150) } else { Color32::from_rgba_unmultiplied(190, 186, 186, (255.0 * fade) as u8) };
            self.text_c(x, y, &items[idx], 13.0, col);
        }
    }

    /// Number sprite digit UVs in `xtext_00`.
    pub(crate) fn digit_uv(d: u32) -> [f32; 4] {
        match d {
            0 => [0.0117, 0.0059, 0.0996, 0.0684],
            1 => [0.1377, 0.0068, 0.1963, 0.0674],
            2 => [0.2285, 0.0059, 0.3232, 0.0674],
            3 => [0.3389, 0.0059, 0.4287, 0.0684],
            4 => [0.4463, 0.0059, 0.5361, 0.0674],
            5 => [0.5576, 0.0068, 0.6494, 0.0684],
            6 => [0.6680, 0.0088, 0.7549, 0.0674],
            7 => [0.7715, 0.0068, 0.8672, 0.0713],
            8 => [0.8848, 0.0088, 0.9746, 0.0674],
            9 => [0.0293, 0.0684, 0.1787, 0.1455],
            _ => [0.0117, 0.0059, 0.0996, 0.0684],
        }
    }

    /// Draw a number using the game's authentic digit sprites from `xtext_00`.
    pub(crate) fn draw_number_sprites(&self, x: f32, y: f32, num: u32) {
        let s = num.to_string();
        let dw = 19.0;
        let dh = 21.0;
        for (i, ch) in s.chars().enumerate() {
            if let Some(digit) = ch.to_digit(10) {
                let dx = x + i as f32 * (dw + 2.0);
                self.img("xtext_00", [dx, y, dx + dw, y + dh], Self::digit_uv(digit), Color32::WHITE);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_screen() -> ChaosGateScreen {
        let mut text = HashMap::new();
        text.insert(MSG_CHAOS_GATE, "Chaos Gate".to_string());
        text.insert(MSG_WARP_MENU, "Warp Menu".to_string());
        text.insert(MSG_AREA_WORD, "Area Word".to_string());
        text.insert(MSG_RANDOM, "Random".to_string());
        text.insert(MSG_BOOKMARK, "Bookmark".to_string());
        text.insert(MSG_WARP_RECORD, "Warp Record".to_string());
        text.insert(MSG_TO_TOWN, "To Town".to_string());
        text.insert(MSG_CANCEL, "Cancel".to_string());
        text.insert(MSG_INITIATE, "Initiate Area warp?".to_string());
        text.insert(MSG_YES, "Yes".to_string());
        text.insert(MSG_NO, "No".to_string());
        text.insert(MSG_NO_WARP_RECORD, "".to_string());

        text.insert(MSG_TUT_21728, "".to_string());
        text.insert(MSG_TUT_21729, "".to_string());
        text.insert(MSG_TUT_21730, "These words...".to_string());
        text.insert(MSG_TUT_21731, "".to_string());
        text.insert(MSG_TUT_21732, "".to_string());
        text.insert(MSG_TUT_21733, "".to_string());
        text.insert(MSG_TUT_21734, "".to_string());

        text.insert(MSG_TUT_21735, "".to_string());
        text.insert(MSG_TUT_21736, "".to_string());
        text.insert(MSG_TUT_21737, "".to_string());
        text.insert(MSG_TUT_21738, "".to_string());
        text.insert(MSG_TUT_21739, "".to_string());
        text.insert(MSG_TUT_21740, "".to_string());
        text.insert(MSG_TUT_21741, "".to_string());
        text.insert(MSG_TUT_21742, "".to_string());
        text.insert(MSG_TUT_21743, "".to_string());
        text.insert(MSG_TUT_21744, "".to_string());
        text.insert(MSG_TUT_21745, "".to_string());
        text.insert(MSG_TUT_21746, "".to_string());

        text.insert(MSG_TUT_21747, "".to_string());
        text.insert(MSG_TUT_21748, "".to_string());
        text.insert(MSG_TUT_21749, "".to_string());
        text.insert(MSG_TUT_21750, "".to_string());
        text.insert(MSG_TUT_21751, "".to_string());
        text.insert(MSG_TUT_21752, "".to_string());
        text.insert(MSG_TUT_21753, "".to_string());
        text.insert(MSG_TUT_21754, "".to_string());
        text.insert(MSG_TUT_21755, "".to_string());

        text.insert(MSG_GUIDE_AREA_WORD, "Select \"Area Word.\"".to_string());
        text.insert(MSG_GUIDE_COURAGEOUS, "Select \"Courageous.\"".to_string());
        text.insert(MSG_GUIDE_ENGAGING, "Select \"Engaging.\"".to_string());
        text.insert(MSG_GUIDE_DAYDREAM, "Select \"Daydream.\"".to_string());

        ChaosGateScreen {
            state: CgState::Gate { cursor: 0 },
            tex: HashMap::new(),
            text,
            status: None,
            server: AreaServer::Delta,
            owned_words: [
                areaword::initial_owned_words(AreaWordSlot::First),
                areaword::initial_owned_words(AreaWordSlot::Second),
                areaword::initial_owned_words(AreaWordSlot::Third),
            ],
            word_cursors: [0, 0, 0],
            haseo_lv: 1,
            is_tutorial_active: true,
            tutorial_step: TutorialStep::Intro1,
            active_rejection: None,
            warp_records: Vec::new(),
        }
    }

    #[test]
    fn test_chaos_gate_screen_synthesis_flow() {
        let cg = create_test_screen();

        // 1. Initial slot 0 (Courageous alone selected)
        let s1 = cg.synthesized(0, [false, false, false]);
        assert_eq!(s1.area_lv, 1);
        assert_eq!(s1.thumbnail_cell, 3);
        assert_eq!(s1.party_count, 2);
        assert_eq!(s1.player_count, 0);
        assert_eq!(s1.mission_id, areaword::MSG_MISSION_BOSS);
        assert!(!s1.is_special);

        // 2. All 3 slots picked (Courageous Engaging Daydream)
        let s3 = cg.synthesized(2, [true, true, true]);
        assert_eq!(s3.area_lv, 1);
        assert_eq!(s3.thumbnail_cell, 2);
        assert_eq!(s3.party_count, 0);
        assert_eq!(s3.player_count, 0);
        assert_eq!(s3.mission_id, areaword::MSG_MISSION_TEMPLE);
        assert!(s3.is_special);
        assert_eq!(s3.special_id, Some(2));
    }

    #[test]
    fn test_formatted_text_parsing() {
        let raw = "Go to <color76E1FF>the gate<colorFFFFFF>, then pick three words.";
        let spans = parse_formatted_text(raw, Color32::WHITE);
        assert_eq!(spans.len(), 3);
        assert_eq!(spans[0].text, "Go to ");
        assert_eq!(spans[0].color, Color32::WHITE);
        assert_eq!(spans[1].text, "the gate");
        assert_eq!(spans[1].color, Color32::from_rgb(0x76, 0xE1, 0xFF));
        assert_eq!(spans[2].text, ", then pick three words.");
        assert_eq!(spans[2].color, Color32::WHITE);
    }

    #[test]
    fn test_tutorial_dialogue_and_step_progression() {
        let mut cg = create_test_screen();
        assert!(cg.is_tutorial_active);
        assert_eq!(cg.tutorial_step, TutorialStep::Intro1);

        // Verify dialogue sequence step-by-step
        let expected_steps = [
            (TutorialStep::Intro1, MSG_TUT_21728, TutorialSpeaker::Iyoten),
            (TutorialStep::Intro2, MSG_TUT_21729, TutorialSpeaker::Asta),
            (TutorialStep::Intro3, MSG_TUT_21730, TutorialSpeaker::Iyoten),
            (TutorialStep::Intro4, MSG_TUT_21731, TutorialSpeaker::Iyoten),
            (TutorialStep::Intro5, MSG_TUT_21732, TutorialSpeaker::Iyoten),
        ];

        for (step, expected_msg, expected_speaker) in expected_steps {
            assert_eq!(cg.tutorial_step, step);
            let (msg, speaker, _) = cg.tutorial_step.dialogue_info().unwrap();
            assert_eq!(msg, expected_msg);
            assert_eq!(speaker, expected_speaker);

            // Advance step
            cg.tutorial_step = match cg.tutorial_step {
                TutorialStep::Intro1 => TutorialStep::Intro2,
                TutorialStep::Intro2 => TutorialStep::Intro3,
                TutorialStep::Intro3 => TutorialStep::Intro4,
                TutorialStep::Intro4 => TutorialStep::Intro5,
                TutorialStep::Intro5 => TutorialStep::ChooseAreaWord,
                _ => unreachable!(),
            };
        }

        assert_eq!(cg.tutorial_step, TutorialStep::ChooseAreaWord);
        assert_eq!(cg.tutorial_step.guide_prompt_id(), Some(MSG_GUIDE_AREA_WORD));

        // Area Word explanation steps
        cg.tutorial_step = TutorialStep::AreaWordIntro1;
        assert_eq!(cg.tutorial_step.dialogue_info().unwrap().0, MSG_TUT_21735);
        cg.tutorial_step = TutorialStep::AreaWordIntro2;
        assert_eq!(cg.tutorial_step.dialogue_info().unwrap().2, TutorialHighlight::WordPlate);
        cg.tutorial_step = TutorialStep::AreaWordIntro4;
        assert_eq!(cg.tutorial_step.dialogue_info().unwrap().2, TutorialHighlight::WordGyro);

        // Word picking guides
        cg.tutorial_step = TutorialStep::PickWord1;
        assert_eq!(cg.tutorial_step.guide_prompt_id(), Some(MSG_GUIDE_COURAGEOUS));
        cg.tutorial_step = TutorialStep::PickWord2;
        assert_eq!(cg.tutorial_step.guide_prompt_id(), Some(MSG_GUIDE_ENGAGING));
        cg.tutorial_step = TutorialStep::PickWord3;
        assert_eq!(cg.tutorial_step.guide_prompt_id(), Some(MSG_GUIDE_DAYDREAM));

        // Server symbol steps
        cg.tutorial_step = TutorialStep::ServerExpl2;
        assert_eq!(cg.tutorial_step.dialogue_info().unwrap().2, TutorialHighlight::ServerSymbol);
        cg.tutorial_step = TutorialStep::ServerExpl7;
        assert_eq!(cg.tutorial_step.dialogue_info().unwrap().1, TutorialSpeaker::Asta);
    }

    #[test]
    fn test_random_and_warp_record_actions() {
        let mut cg = create_test_screen();
        cg.set_tutorial_active(false);

        // 1. Warp record initially empty
        assert!(cg.warp_records.is_empty());

        // 2. Add history record
        cg.server = AreaServer::Delta;
        cg.word_cursors = [0, 0, 0];
        let rec = (cg.server, cg.word_cursors[0], cg.word_cursors[1], cg.word_cursors[2]);
        cg.warp_records.push(rec);

        assert_eq!(cg.warp_records.len(), 1);
        assert_eq!(cg.warp_records[0], (AreaServer::Delta, 0, 0, 0));
    }
}
