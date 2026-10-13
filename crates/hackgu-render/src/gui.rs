use std::path::{Path, PathBuf};
use winit::window::Window;

use crate::mesh::GpuScene;
use hackgu_ccs::CcsFile;
use hackgu_game::{GameRuntime, GameState};

pub struct GuiModelPreset {
    pub name: &'static str,
    pub path: &'static str,
    pub category: &'static str,
}

pub const PRESETS: &[GuiModelPreset] = &[
    // Towns & Cities
    GuiModelPreset { name: "Mac Anu Grand Canal (01h)", path: "target/vol1/data/data/town/town01_01h.ccs", category: "Towns" },
    GuiModelPreset { name: "Raven Guild Pavilion (00h)", path: "target/vol1/data/data/town/town01_00h.ccs", category: "Towns" },
    GuiModelPreset { name: "Mac Anu Central Dome (02h)", path: "target/vol1/data/data/town/town01_02h.ccs", category: "Towns" },
    GuiModelPreset { name: "Mac Anu Complete Stage", path: "target/vol1/data/data/town/town01.ccs", category: "Towns" },
    GuiModelPreset { name: "Mac Anu Harbor (04h)", path: "target/vol1/data/data/town/town01_04h.ccs", category: "Towns" },
    GuiModelPreset { name: "Grand Arena Bridge (05h)", path: "target/vol1/data/data/town/town01_05h.ccs", category: "Towns" },
    GuiModelPreset { name: "Mercenary Guild Shop", path: "target/vol1/data/data/town/gshop01.ccs", category: "Towns" },
    GuiModelPreset { name: "Royal Airship", path: "target/vol1/data/data/town/l_ship.ccs", category: "Towns" },

    // Dungeons & Stages
    GuiModelPreset { name: "Underground Ruins Stage", path: "target/vol1/data/data/dungeon/dungeon2.ccs", category: "Dungeons" },
    GuiModelPreset { name: "Dungeon Relic Props", path: "target/vol1/data/data/dungeon/d02_obj.ccs", category: "Dungeons" },

    // Characters
    GuiModelPreset { name: "Haseo (1st Form)", path: "VRChat/work/haseo/raw/2hsw01.ccs", category: "Characters" },
    GuiModelPreset { name: "Haseo (2nd Form)", path: "VRChat/work/haseo/raw/2hsw02.ccs", category: "Characters" },
    GuiModelPreset { name: "Haseo (3rd Form)", path: "VRChat/work/haseo/raw/2hsw03.ccs", category: "Characters" },
    GuiModelPreset { name: "Haseo (4th Form)", path: "VRChat/work/haseo/raw/2hsw05.ccs", category: "Characters" },
    GuiModelPreset { name: "Atoli", path: "target/vol1/data/data/spc/2atl01.ccs", category: "Characters" },
    GuiModelPreset { name: "Kuhn", path: "target/cmn/data/data/spc/2ast01.ccs", category: "Characters" },
    GuiModelPreset { name: "Endrance", path: "target/cmn/data/data/spc/2iyo01.ccs", category: "Characters" },
    GuiModelPreset { name: "Gaspard", path: "target/cmn/data/data/spc/2gas01.ccs", category: "Characters" },
    GuiModelPreset { name: "Silabus", path: "target/vol1/data/data/spc/2syl01.ccs", category: "Characters" },
    GuiModelPreset { name: "Pi", path: "target/vol1/data/data/spc/2pai01.ccs", category: "Characters" },
    GuiModelPreset { name: "Ovan", path: "target/vol1/data/data/spc/2ova01.ccs", category: "Characters" },
    GuiModelPreset { name: "Bordeaux", path: "target/vol1/data/data/spc/2bor01.ccs", category: "Characters" },
    GuiModelPreset { name: "Tabby", path: "target/cmn/data/data/spc/2tbl01.ccs", category: "Characters" },
    GuiModelPreset { name: "Negimaru", path: "target/cmn/data/data/spc/2neg01.ccs", category: "Characters" },
    GuiModelPreset { name: "Gremlin", path: "target/cmn/data/data/spc/2gri01.ccs", category: "Characters" },

    // Weapons
    GuiModelPreset { name: "Haseo Twin Blades (w2dsw00l)", path: "target/vol1/data/data/eq/w2dsw00l.ccs", category: "Weapons" },
    GuiModelPreset { name: "Broadsword (w22sw01)", path: "target/vol1/data/data/eq/w22sw01.ccs", category: "Weapons" },
    GuiModelPreset { name: "Reaper Scythe (w22sc01)", path: "VRChat/work/haseo/wraw/w22sc01.ccs", category: "Weapons" },
    GuiModelPreset { name: "Dual Guns (w2dgn00)", path: "VRChat/work/haseo/wraw/w2dgn00.ccs", category: "Weapons" },
];

pub const STEAM_CPKS: &[(&str, &str)] = &[
    ("Vol 1 Index (hackGU_vol1_i.cpk)", "cpk/hackGU_vol1_i.cpk"),
    ("Vol 1 Main Archive (hackGU_vol1_a.cpk)", "cpk/hackGU_vol1_a.cpk"),
    ("Vol 2 Main Archive (hackGU_vol2_a.cpk)", "cpk/hackGU_vol2_a.cpk"),
    ("Vol 3 Main Archive (hackGU_vol3_a.cpk)", "cpk/hackGU_vol3_a.cpk"),
    ("Vol 4 Main Archive (hackGU_vol4_a.cpk)", "cpk/hackGU_vol4_a.cpk"),
    ("Vol 4 Index (hackGU_vol4_i.cpk)", "cpk/hackGU_vol4_i.cpk"),
    ("Common Index (hackGU_cmn_i.cpk)", "cpk/hackGU_cmn_i.cpk"),
    ("Common Archive (hackGU_cmn_a.cpk)", "cpk/hackGU_cmn_a.cpk"),
];

pub struct GuiState {
    pub custom_path: String,
    pub cpk_path: String,
    pub status_message: String,
    /// In-game Main Menu (town / field), open while Some; party lines are filled in by the renderer each frame.
    pub main_menu: Option<crate::mainmenu::MainMenu>,
    pub main_menu_open: bool,
    pub party_lines: Vec<crate::mainmenu::PartyLine>,
    /// A save chosen on the title screen, applied when the desktop opens.
    pub pending_title_load: Option<crate::datamanager::SaveFile>,
    pub pending_load: Option<PathBuf>,
    pub pending_explore: Option<(&'static str, &'static str)>,
    pub pending_export: bool,
    pub pending_batch: bool,
    pub pending_cpk_extract: bool,
    pub pending_equip: Option<String>,
    pub pending_play_movie: Option<String>,
    pub show_sidebar: bool,
    /// Last Recode title screen (created on first use).
    pub title: Option<crate::title::TitleScreen>,
    /// ALTIMIT Mine OS desktop (created on first use).
    pub altimit: Option<crate::altimit::DesktopScreen>,
    /// Chaos Gate menus (created on first use).
    pub chaos_gate: Option<crate::chaosgate::ChaosGateScreen>,
    pub selected_preset_tab: usize,
    pub player_mode: bool,
    pub game_runtime: GameRuntime,
    pub show_story_window: bool,
    pub show_movie_theater: bool,
    pub selected_email_idx: usize,
    pub selected_bbs_idx: usize,
    pub pending_advance_cutscene: bool,
}

impl GuiState {
    pub fn new() -> Self {
        Self {
            custom_path: String::new(),
            cpk_path: crate::gamedir::game_path("cpk/hackGU_vol1_i.cpk").to_string_lossy().into_owned(),
            status_message: "Ready. Select New Game or Continue.".to_string(),
            pending_title_load: None,
            main_menu: None,
            main_menu_open: false,
            party_lines: Vec::new(),
            pending_load: None,
            pending_explore: None,
            pending_export: false,
            pending_batch: false,
            pending_cpk_extract: false,
            pending_equip: None,
            pending_play_movie: None,
            show_sidebar: false,
            title: None,
            altimit: None,
            chaos_gate: None,
            selected_preset_tab: 0,
            player_mode: false,
            game_runtime: GameRuntime::new_game("Haseo"),
            show_story_window: false,
            show_movie_theater: false,
            selected_email_idx: 0,
            selected_bbs_idx: 0,
            pending_advance_cutscene: false,
        }
    }
}

pub struct EguiManager {
    pub context: egui::Context,
    pub state: egui_winit::State,
    pub renderer: egui_wgpu::Renderer,
    pub gui_state: GuiState,
}

/// egui's built-in fonts have no Japanese glyphs (they render as boxes). Add a system CJK font as a
/// fallback for both families; the first one found wins.
fn install_cjk_fonts(ctx: &egui::Context) {
    let candidates: [(&str, u32); 9] = [
        ("C:/Windows/Fonts/YuGothM.ttc", 0),
        ("C:/Windows/Fonts/yugothic.ttf", 0),
        ("C:/Windows/Fonts/meiryo.ttc", 0),
        ("C:/Windows/Fonts/msgothic.ttc", 0),
        ("C:/Windows/Fonts/BIZ-UDGothicR.ttc", 0),
        ("C:/Windows/Fonts/msmincho.ttc", 0),
        ("/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc", 0),
        ("/System/Library/Fonts/ヒラギノ角ゴシック W3.ttc", 0),
        ("/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc", 0),
    ];
    let mut fonts = egui::FontDefinitions::default();
    let mut found = false;
    for (path, index) in candidates {
        if let Ok(bytes) = std::fs::read(path) {
            let mut data = egui::FontData::from_owned(bytes);
            data.index = index;
            fonts.font_data.insert("cjk".to_owned(), std::sync::Arc::new(data));
            for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
                fonts.families.entry(family).or_default().push("cjk".to_owned());
            }
            println!("font: loaded CJK fallback {path}");
            found = true;
            break;
        }
    }
    if !found {
        eprintln!("font: no CJK system font found; Japanese text will show as boxes");
    }
    // Bold UI face for the title screen's prompts and help line ("ui_bold"); falls back to the normal fonts.
    let mut bold = fonts.families.get(&egui::FontFamily::Proportional).cloned().unwrap_or_default();
    if let Ok(bytes) = std::fs::read("C:/Windows/Fonts/segoeuib.ttf") {
        fonts.font_data.insert("ui_bold".to_owned(), std::sync::Arc::new(egui::FontData::from_owned(bytes)));
        bold.insert(0, "ui_bold".to_owned());
    }
    fonts.families.insert(egui::FontFamily::Name("ui_bold".into()), bold);
    ctx.set_fonts(fonts);
}

impl EguiManager {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat, window: &Window) -> Self {
        let context = egui::Context::default();
        install_cjk_fonts(&context);
        let viewport_id = egui::ViewportId::ROOT;
        let state = egui_winit::State::new(
            context.clone(),
            viewport_id,
            window,
            Some(window.scale_factor() as f32),
            None,
            None,
        );

        let renderer = egui_wgpu::Renderer::new(device, format, None, 1, false);

        Self {
            context,
            state,
            renderer,
            gui_state: GuiState::new(),
        }
    }

    pub fn on_window_event(&mut self, window: &Window, event: &winit::event::WindowEvent) -> bool {
        let res = self.state.on_window_event(window, event);
        match event {
            winit::event::WindowEvent::KeyboardInput { .. } => {
                // If an egui text edit widget is active and taking input, egui consumes the keystroke.
                // Otherwise, pass it through to the engine/game (for WASD, Tab, Space, numbers, brackets, etc.)
                self.context.wants_keyboard_input()
            }
            winit::event::WindowEvent::MouseInput { .. } => {
                // Only consume clicks if clicking on egui UI panels/widgets
                self.context.wants_pointer_input()
            }
            winit::event::WindowEvent::MouseWheel { .. } => {
                self.context.wants_pointer_input()
            }
            _ => res.consumed,
        }
    }
}

impl GuiState {
    pub fn draw_ui(
        &mut self,
        ctx: &egui::Context,
        scene: Option<&mut GpuScene>,
        ccs: Option<&CcsFile>,
        queue: &wgpu::Queue,
        show_background: &mut bool,
        recenter_camera: &mut bool,
        current_file_path: Option<&Path>,
    ) {
        // Play time (Data Manager): everything but the title screen.
        if !matches!(self.game_runtime.state, GameState::TitleScreen) {
            self.game_runtime.play_time += ctx.input(|i| i.stable_dt.min(0.1)) as f64;
        }
        // Handle files dropped anywhere in the egui window
        ctx.input(|i| {
            for dropped_file in &i.raw.dropped_files {
                if let Some(path) = &dropped_file.path {
                    self.pending_load = Some(path.clone());
                }
            }
        });

        // 0. Title: the Last Recode title built from the game's own textures (title.rs).
        if matches!(self.game_runtime.state, GameState::TitleScreen) {
            let lang = crate::auto::get().lang.clone();
            let title = self.title.get_or_insert_with(|| {
                let mut t = crate::title::TitleScreen::new(ctx, &lang);
                t.state = match crate::auto::get().title.as_str() {
                    "menu" => crate::title::TitleState::VolumeMenu { cursor: 2 },
                    "newgame" => crate::title::TitleState::NewGame { cursor: 0 },
                    _ => t.state,
                };
                t
            });
            match title.show(ctx) {
                crate::title::TitleAction::StartNewGame { .. } => {
                    // New Game: opening movie OP10000, then the PM10100 event (story order of the original).
                    self.game_runtime.start_prologue_movie();
                }
                crate::title::TitleAction::LoadLast => match crate::datamanager::latest_slot() {
                    // Saves are made on the desktop (Data Manager), so loading resumes there.
                    Some((_, f)) => {
                        self.pending_title_load = Some(f);
                        self.game_runtime.to_desktop();
                    }
                    None => title.status = title.no_data_text(),
                },
                crate::title::TitleAction::None => {}
            }
            if !self.show_sidebar {
                return;
            }
        }

        // 0b. ALTIMIT desktop built from the game's own assets (altimit.rs).
        if matches!(self.game_runtime.state, GameState::Desktop) {
            let lang = crate::auto::get().lang.clone();
            let desk = self.altimit.get_or_insert_with(|| {
                let mut d = crate::altimit::DesktopScreen::new(ctx, &lang);
                if let Some(i) = crate::auto::get().desktop {
                    d.select(i);
                }
                d
            });
            if let Some(f) = self.pending_title_load.take() {
                desk.select(0);
                desk.apply_save(ctx, &f);
            }
            desk.save_info = (self.game_runtime.save.level, self.game_runtime.play_time, self.game_runtime.reformatted);
            if desk.events_done.len() != self.game_runtime.events_done.len() {
                desk.events_done = self.game_runtime.events_done.clone();
            }
            if self.game_runtime.pending_desktop_sequence.take().as_deref() == Some("reboot") {
                desk.start_reformat();
                self.game_runtime.reformatted = true;
            }
            match desk.show(ctx) {
                crate::altimit::DesktopAction::Launch(crate::altimit::App::TheWorld) => self.game_runtime.login_to_the_world(),
                crate::altimit::DesktopAction::Launch(app) => {
                    // Mail/Forum still live in the F1 tools panel (Gemini's lists); the real application screens are not built yet.
                    desk.status = Some(format!("{:?} is not implemented yet.", app));
                }
                crate::altimit::DesktopAction::ReturnToTitle => self.game_runtime.state = GameState::TitleScreen,
                crate::altimit::DesktopAction::None => {}
            }
            if let Some(f) = desk.pending_load.take() {
                self.game_runtime.save.level = f.level;
                self.game_runtime.play_time = f.play_time;
                self.game_runtime.reformatted = f.reformatted;
                self.game_runtime.events_done = f.events_done.clone();
            }
            if !self.show_sidebar {
                return;
            }
        }

        // 0b2. In-game Main Menu (mainmenu.rs).
        if self.main_menu_open && matches!(self.game_runtime.state, GameState::Town { .. } | GameState::Field { .. }) {
            let lang = crate::auto::get().lang.clone();
            let mm = self.main_menu.get_or_insert_with(|| crate::mainmenu::MainMenu::new(ctx, &lang));
            let party = self.party_lines.clone();
            let (gp, chim) = (self.game_runtime.save.gp, self.game_runtime.save.chim_count);
            match mm.show(ctx, &party, gp, chim) {
                crate::mainmenu::MenuAction::Close => self.main_menu_open = false,
                crate::mainmenu::MenuAction::LogOut => {
                    self.main_menu_open = false;
                    self.game_runtime.logout_to_desktop();
                }
                crate::mainmenu::MenuAction::None => {}
            }
            if !self.show_sidebar {
                return;
            }
        } else {
            self.main_menu_open = false;
        }

        // 0c. Chaos Gate menus from the game's own menu textures (chaosgate.rs).
        if matches!(self.game_runtime.state, GameState::ChaosGateSelector { .. }) {
            let lang = crate::auto::get().lang.clone();
            let cg = self.chaos_gate.get_or_insert_with(|| {
                let mut c = crate::chaosgate::ChaosGateScreen::new(ctx, &lang);
                c.state = match crate::auto::get().gate.as_str() {
                    "warp" => crate::chaosgate::CgState::Warp { cursor: 1 },
                    "word" => crate::chaosgate::CgState::AreaWord { col: 0, picked: [false; 3] },
                    "word3" => crate::chaosgate::CgState::AreaWord { col: 2, picked: [true, true, false] },
                    "confirm" => crate::chaosgate::CgState::Confirm { yes: true },
                    _ => c.state,
                };
                c
            });
            // The gate tutorial runs once, on the first visit (the tutorial field's arrival PM10105 follows it).
            if cg.is_tutorial_active && (self.game_runtime.reformatted || self.game_runtime.events_done.iter().any(|e| e == "PM10105")) {
                cg.set_tutorial_active(false);
            }
            for (c, words) in crate::events::given_words(&self.game_runtime.events_done).into_iter().enumerate() {
                for w in words {
                    if !cg.owned_words[c].contains(&w) {
                        cg.owned_words[c].push(w);
                        cg.owned_words[c].sort();
                    }
                }
            }
            match cg.show(ctx) {
                crate::chaosgate::CgAction::Close => self.game_runtime.return_to_town(),
                crate::chaosgate::CgAction::Warp => {
                    self.player_mode = true;
                    self.pending_equip = Some("twin_blades".into());
                    // The area defined for these words (assets/areas/*.json). Its arrival event loads the field itself
                    // (start_cutscene); without one, the field is entered directly.
                    let (server, words) = cg.selected_words();
                    let dungeon = if server == 0 { crate::dungeon::find(words) } else { None };
                    match (dungeon, crate::field::find(server, words)) {
                        (Some(d), _) => {
                            // A dungeon-only Area (assets/dungeons): its arrival event (PM10330) or free play there.
                            crate::dungeon::set_current(d, 0);
                            println!("chaos gate: warping to dungeon {:?}", words);
                            match crate::events::arrival_event(words, &self.game_runtime.events_done) {
                                Some(ev) => self.game_runtime.start_script(&ev.event),
                                None => self.game_runtime.resume_field(),
                            }
                        }
                        (None, Some(area)) => {
                            let arrival = area.events.arrival.clone();
                            println!("chaos gate: warping to {} ({})", area.name, area.id);
                            crate::dungeon::clear();
                            crate::field::set_area(area);
                            match arrival {
                                Some(ev) => self.game_runtime.start_script(&ev),
                                None => self.game_runtime.resume_field(),
                            }
                        }
                        (None, None) => {
                            cg.status = Some("This Area is not built yet (no assets/areas definition for these words).".into());
                        }
                    }
                }
                crate::chaosgate::CgAction::None => {}
            }
            if !self.show_sidebar {
                return;
            }
        }

        if !self.show_sidebar {
            // Discreet F1 toggle pill in top-left
            egui::Area::new(egui::Id::new("top_f1_pill"))
                .fixed_pos(egui::pos2(15.0, 15.0))
                .show(ctx, |ui| {
                    if ui.small_button(egui::RichText::new("⚙ [F1] Tools").color(egui::Color32::from_rgba_unmultiplied(190, 210, 235, 200))).clicked() {
                        self.show_sidebar = true;
                    }
                });
        }

        if self.show_sidebar {
            egui::SidePanel::left("side_panel")
                .default_width(360.0)
                .resizable(true)
                .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(egui::RichText::new("⚔ .hack//G.U. Studio").strong().color(egui::Color32::from_rgb(255, 180, 50)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.small_button("✖ Hide").clicked() {
                            self.show_sidebar = false;
                        }
                    });
                });
                ui.separator();

                // Active Model Indicator
                if let Some(path) = current_file_path {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Active:").strong());
                        ui.label(
                            egui::RichText::new(path.file_name().unwrap_or_default().to_string_lossy().to_string())
                                .color(egui::Color32::from_rgb(120, 220, 255))
                                .strong(),
                        );
                    });
                    ui.separator();
                }

                // 1. TOP-LEVEL: PLAYABLE WORLD EXPLORATION (WASD)
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.heading(egui::RichText::new("🎮 PLAYABLE WORLD (WASD)").strong().color(egui::Color32::from_rgb(100, 255, 120)));
                    });
                    ui.label("Explore .hack//G.U. stages with 3rd-person WASD character controls:");

                    ui.add_space(3.0);
                    if ui.button(egui::RichText::new("🏰 Explore Mac Anu (Playable WASD)").strong().size(14.0)).clicked() {
                        self.pending_explore = Some(("target/vol1/data/data/town/town01_00h.ccs", "VRChat/work/haseo/raw/2hsw01.ccs"));
                    }

                    ui.horizontal_wrapped(|ui| {
                        if ui.button("🌊 Grand Canal (01h)").clicked() {
                            self.pending_explore = Some(("target/vol1/data/data/town/town01_01h.ccs", "VRChat/work/haseo/raw/2hsw01.ccs"));
                        }
                        if ui.button("🏛 Central Dome (02h)").clicked() {
                            self.pending_explore = Some(("target/vol1/data/data/town/town01_02h.ccs", "VRChat/work/haseo/raw/2hsw01.ccs"));
                        }
                        if ui.button("⚔ Ruins Dungeon").clicked() {
                            self.pending_explore = Some(("target/vol1/data/data/dungeon/dungeon2.ccs", "VRChat/work/haseo/raw/2hsw01.ccs"));
                        }
                    });

                    ui.separator();
                    let mode_color = if self.player_mode {
                        egui::Color32::from_rgb(60, 230, 80)
                    } else {
                        egui::Color32::from_rgb(170, 170, 170)
                    };
                    let mode_text = if self.player_mode {
                        "🟢 WASD Controller: ACTIVE (Tab to toggle)"
                    } else {
                        "⚪ WASD Controller: INACTIVE (Tab to toggle)"
                    };
                    if ui.button(egui::RichText::new(mode_text).strong().color(mode_color)).clicked() {
                        self.player_mode = !self.player_mode;
                    }

                    ui.add_space(2.0);
                    ui.label(egui::RichText::new("Quick Equip Weapons:").small().strong());
                    ui.horizontal_wrapped(|ui| {
                        if ui.button("⚔ Twin Blades").clicked() {
                            self.pending_equip = Some("twin_blades".into());
                        }
                        if ui.button("🗡 Broadsword").clicked() {
                            self.pending_equip = Some("broadsword".into());
                        }
                        if ui.button("☠ Scythe").clicked() {
                            self.pending_equip = Some("scythe".into());
                        }
                        if ui.button("🔫 Dual Guns").clicked() {
                            self.pending_equip = Some("guns".into());
                        }
                        if ui.button("👊 Unarmed").clicked() {
                            self.pending_equip = Some("none".into());
                        }
                    });
                });

                ui.add_space(6.0);

                // 2. Model & Stage Presets
                ui.collapsing(egui::RichText::new("📁 Asset Library & Presets").strong(), |ui| {
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.selected_preset_tab, 0, "🏙 Towns");
                        ui.selectable_value(&mut self.selected_preset_tab, 1, "👤 Characters");
                        ui.selectable_value(&mut self.selected_preset_tab, 2, "🏰 Dungeons");
                        ui.selectable_value(&mut self.selected_preset_tab, 3, "⚔ Weapons");
                    });
                    ui.separator();

                    let target_cat = match self.selected_preset_tab {
                        0 => "Towns",
                        1 => "Characters",
                        2 => "Dungeons",
                        _ => "Weapons",
                    };

                    let category_presets: Vec<_> = PRESETS.iter().filter(|p| p.category == target_cat).collect();
                    egui::Grid::new("presets_grid").num_columns(2).spacing([6.0, 6.0]).show(ui, |ui| {
                        for (i, p) in category_presets.iter().enumerate() {
                            if ui.button(p.name).clicked() {
                                self.pending_load = Some(PathBuf::from(p.path));
                            }
                            if (i + 1) % 2 == 0 {
                                ui.end_row();
                            }
                        }
                    });

                    ui.add_space(8.0);
                    ui.label("Open / Search Custom .CCS File:");
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.custom_path);
                        if ui.button("Load").clicked() && !self.custom_path.is_empty() {
                            self.pending_load = Some(PathBuf::from(&self.custom_path));
                        }
                    });
                    ui.label(egui::RichText::new("💡 Hint: You can also drag & drop any .ccs/.css file directly into this window!").small().color(egui::Color32::GRAY));
                });

                ui.add_space(6.0);

                // 3. Animation Controls
                ui.collapsing(egui::RichText::new("🎬 Animation Studio").strong(), |ui| {
                    if let (Some(scene), Some(ccs)) = (scene, ccs) {
                        if !scene.animations.is_empty() {
                            let cur_idx = scene.active_anim_idx.unwrap_or(0);
                            let anim_name = &scene.animations[cur_idx].name;
                            let frame_count = scene.animations[cur_idx].frame_count;

                            ui.label(format!("Track [{}]: {}", cur_idx, anim_name));

                            let anim_info: Vec<(usize, String, u32)> = scene
                                .animations
                                .iter()
                                .enumerate()
                                .map(|(i, a)| (i, a.name.clone(), a.frame_count))
                                .collect();

                            egui::ComboBox::from_label("Track Selector")
                                .selected_text(format!("[{}] {}", cur_idx, anim_name))
                                .show_ui(ui, |ui| {
                                    for (i, name, frames) in anim_info {
                                        if ui.selectable_label(cur_idx == i, format!("[{}] {} ({} frames)", i, name, frames)).clicked() {
                                            scene.select_animation(i, queue, ccs);
                                        }
                                    }
                                });

                            ui.horizontal(|ui| {
                                if ui.button(if scene.is_playing { "⏸ Pause" } else { "▶ Play" }).clicked() {
                                    scene.is_playing = !scene.is_playing;
                                }
                                if ui.button("⏮ Prev").clicked() {
                                    scene.cycle_animation(false, queue, ccs);
                                }
                                if ui.button("⏭ Next").clicked() {
                                    scene.cycle_animation(true, queue, ccs);
                                }
                            });

                            let max_frame = (frame_count.saturating_sub(1)).max(1) as f32;
                            let mut frame = scene.current_frame;
                            if ui.add(egui::Slider::new(&mut frame, 0.0..=max_frame).text("Frame")).changed() {
                                scene.seek_frame(frame, queue, ccs);
                            }

                            ui.add(egui::Slider::new(&mut scene.anim_speed, 0.1..=3.0).text("Speed"));
                        } else {
                            ui.label("No skeletal animations in this container.");
                        }
                    } else {
                        ui.label("No model loaded.");
                    }
                });

                ui.add_space(6.0);

                // 4. Environment & View Controls
                ui.collapsing(egui::RichText::new("👁 View & Player Controller").strong(), |ui| {
                    ui.checkbox(&mut self.player_mode, egui::RichText::new("🎮 Player Controller Mode (WASD to run / Tab)").color(egui::Color32::from_rgb(100, 220, 100)));
                    ui.checkbox(show_background, "Show Sky / Background Sphere");
                    if ui.button("🎯 Recenter Camera (Spacebar)").clicked() {
                        *recenter_camera = true;
                    }
                });

                ui.add_space(6.0);

                // 5. Exporters & CPK Decompressor
                ui.collapsing(egui::RichText::new("💾 Exporters & CPK Decompressor").strong(), |ui| {
                    ui.label(egui::RichText::new("glTF 2.0 Export:").strong());
                    if ui.button("⬇ Export Active Model (.glb)").clicked() {
                        self.pending_export = true;
                    }

                    ui.separator();
                    ui.label(egui::RichText::new("Steam CPK Decompressor / Batch:").strong());

                    let current_cpk = self.cpk_path.clone();
                    let current_name = STEAM_CPKS.iter()
                        .find(|(_, p)| crate::gamedir::game_path(p).to_string_lossy() == current_cpk.as_str())
                        .map(|(n, _)| *n)
                        .unwrap_or("Custom CPK Path");

                    egui::ComboBox::from_label("Target CPK")
                        .selected_text(current_name)
                        .show_ui(ui, |ui| {
                            for (name, path) in STEAM_CPKS {
                                let full = crate::gamedir::game_path(path).to_string_lossy().into_owned();
                                if ui.selectable_label(self.cpk_path == full, *name).clicked() {
                                    self.cpk_path = full;
                                }
                            }
                        });

                    ui.text_edit_singleline(&mut self.cpk_path);

                    ui.horizontal(|ui| {
                        if ui.button("📂 Extract All .CCS to target/").clicked() {
                            self.pending_cpk_extract = true;
                        }
                        if ui.button("📦 Batch Convert CPK to .GLB").clicked() {
                            self.pending_batch = true;
                        }
                    });

                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Status:").strong());
                        ui.label(&self.status_message);
                    });
                });

                ui.add_space(10.0);
                ui.separator();
                ui.label(egui::RichText::new("Controls:").small().strong());
                ui.label(egui::RichText::new("• Tab: Player Mode (WASD to run around world)\n• W/A/S/D: Run in camera direction\n• Left Mouse: Orbit 360°\n• Right Mouse: Pan\n• Scroll: Zoom In/Out\n• 1-5 / [ ]: Quick Animations\n• Space: Recenter Camera\n• Drag & Drop: Open any .ccs/.css").small());
            });
        }

        // 6. On-Screen HUD Overlay for Quick Controls (only if sidebar open or player mode active)
        if self.show_sidebar && !matches!(self.game_runtime.state, GameState::TitleScreen | GameState::Movie { .. }) {
            egui::Area::new(egui::Id::new("hud_overlay"))
                .fixed_pos(egui::pos2(if self.show_sidebar { 385.0 } else { 15.0 }, 15.0))
                .show(ctx, |ui| {
                    egui::Frame::new()
                        .fill(egui::Color32::from_rgba_unmultiplied(20, 25, 35, 210))
                        .corner_radius(6.0)
                        .inner_margin(egui::Margin::symmetric(10, 6))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                if self.player_mode {
                                    ui.label(egui::RichText::new("🎮 [WASD] Run around Mac Anu").strong().color(egui::Color32::from_rgb(100, 255, 120)));
                                    ui.label(egui::RichText::new("|").color(egui::Color32::GRAY));
                                    ui.label(egui::RichText::new("[Mouse] Orbit Camera").color(egui::Color32::WHITE));
                                    ui.label(egui::RichText::new("|").color(egui::Color32::GRAY));
                                    ui.label(egui::RichText::new("[Tab] Free Orbit").color(egui::Color32::from_rgb(180, 210, 255)));
                                } else {
                                    ui.label(egui::RichText::new("👁 [Free Orbit Camera]").strong().color(egui::Color32::from_rgb(255, 210, 100)));
                                    ui.label(egui::RichText::new("|").color(egui::Color32::GRAY));
                                    ui.label(egui::RichText::new("Press [Tab] to activate WASD Player Controls").color(egui::Color32::WHITE));
                                }
                            });
                        });
                });
        }

        // 7. Authentic 1:1 CyberConnect2 Dialogue & Cinematic Overlay
        self.draw_authentic_dialogue_overlay(ctx);

        // 8. Interactive Story & Gameplay Runtime Overlay
        if self.show_story_window {
            self.draw_gameplay_ui(ctx);
        }
    }

    /// Render the interactive .hack//G.U. Playable Story Runtime (Prologue)
    pub fn draw_gameplay_ui(&mut self, ctx: &egui::Context) {
        let mut open = self.show_story_window;
        egui::Window::new("🎮 .hack//G.U. Story Mode - The World R:2")
            .open(&mut open)
            .default_width(420.0)
            .default_pos(egui::pos2(390.0, 60.0))
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Haseo").strong().color(egui::Color32::from_rgb(255, 120, 80)));
                    ui.label(format!("Lv {}", self.game_runtime.save.level));
                    ui.label(format!("HP: {}/{}", self.game_runtime.save.hp, self.game_runtime.save.max_hp));
                    ui.label(format!("GP: {}", self.game_runtime.save.gp));
                    ui.label(format!("⚡ Chim: {}", self.game_runtime.save.chim_count));
                });
                ui.separator();

                match self.game_runtime.state.clone() {
                    GameState::TitleScreen => {
                        ui.heading(egui::RichText::new("◈ .hack//G.U. Title Screen").strong().color(egui::Color32::from_rgb(255, 205, 70)));
                        ui.label("Main Menu active. Select New Game or Continue from the Title Screen.");
                        if ui.button(egui::RichText::new("▶ Start New Game").strong().color(egui::Color32::from_rgb(80, 255, 120))).clicked() {
                            self.pending_play_movie = Some("OP10000.usm".to_string());
                            self.game_runtime.start_prologue_cutscene();
                            self.pending_explore = Some(("target/vol1/data/data/town/town01.ccs", "VRChat/work/haseo/raw/2hsw01.ccs"));
                            self.player_mode = true;
                            self.pending_equip = Some("twin_blades".into());
                            self.show_sidebar = false;
                        }
                    }
                    GameState::Desktop => {
                        ui.heading(egui::RichText::new("🖥 ALTIMIT OS Desktop").strong().color(egui::Color32::from_rgb(100, 180, 255)));
                        ui.label("Outside the game: manage emails, forums, and system settings.");
                        ui.add_space(4.0);

                        ui.collapsing(format!("📬 Mailer ({} Unread)", self.game_runtime.desktop.unread_mail_count), |ui| {
                            let emails = self.game_runtime.desktop.emails.clone();
                            for (_idx, msg) in emails.iter().enumerate() {
                                ui.group(|ui| {
                                    let read_badge = if msg.is_read { "✔" } else { "✉ NEW" };
                                    ui.label(egui::RichText::new(format!("{} From: {} - {}", read_badge, msg.sender, msg.subject)).strong());
                                    ui.label(&msg.body);
                                    if !msg.reply_options.is_empty() && msg.selected_reply.is_none() {
                                        ui.horizontal(|ui| {
                                            for (opt_idx, opt) in msg.reply_options.iter().enumerate() {
                                                if ui.button(&opt.text).clicked() {
                                                    let _ = self.game_runtime.desktop.reply_email(msg.mail_id, opt_idx, &mut self.game_runtime.save);
                                                }
                                            }
                                        });
                                    } else if let Some(sel) = msg.selected_reply {
                                        ui.label(egui::RichText::new(format!("Replied: {}", msg.reply_options[sel].follow_up_response)).italics().color(egui::Color32::LIGHT_GREEN));
                                    }
                                });
                            }
                        });

                        ui.collapsing("🌐 The World Forum (BBS)", |ui| {
                            let threads = self.game_runtime.desktop.forum_threads.clone();
                            for thread in threads {
                                ui.group(|ui| {
                                    ui.label(egui::RichText::new(&thread.title).strong().color(egui::Color32::from_rgb(255, 210, 100)));
                                    for post in &thread.posts {
                                        ui.label(format!("#{}: {} [{}]", post.post_number, post.author, post.timestamp));
                                        ui.label(&post.content);
                                    }
                                    if let Some((ref w1, ref w2, ref w3)) = thread.unlocks_keyword {
                                        if ui.button(format!("🔍 Record Area Keywords: {} {} {}", w1, w2, w3)).clicked() {
                                            self.game_runtime.desktop.read_bbs_thread(thread.thread_id, &mut self.game_runtime.save);
                                            self.game_runtime.message_log.push(format!("Saved keywords: {} {} {}", w1, w2, w3));
                                        }
                                    }
                                });
                            }
                        });

                        ui.add_space(8.0);
                        if ui.button(egui::RichText::new("🎬 PLAY PROLOGUE FROM INTRO MOVIE (OP10000.usm -> PM10100)").strong().size(14.0).color(egui::Color32::from_rgb(255, 200, 80))).clicked() {
                            self.game_runtime.start_prologue_movie();
                            self.pending_explore = Some(("target/vol1/data/data/town/town01.ccs", "VRChat/work/haseo/raw/2hsw01.ccs"));
                            self.player_mode = true;
                            self.pending_equip = Some("twin_blades".into());
                        }
                        if ui.button(egui::RichText::new("🎭 PLAY PROLOGUE CUTSCENE (PM10100: Welcome to Mac Anu)").strong().size(13.0).color(egui::Color32::from_rgb(255, 170, 60))).clicked() {
                            self.game_runtime.start_prologue_cutscene();
                            self.pending_explore = Some(("target/vol1/data/data/town/town01.ccs", "VRChat/work/haseo/raw/2hsw01.ccs"));
                            self.player_mode = true;
                            self.pending_equip = Some("twin_blades".into());
                        }
                        if ui.button(egui::RichText::new("🚀 QUICK LOGIN TO \"THE WORLD\" (Mac Anu Central Plaza)").strong().size(13.0).color(egui::Color32::from_rgb(80, 255, 120))).clicked() {
                            self.game_runtime.login_to_the_world();
                            self.pending_explore = Some(("target/vol1/data/data/town/town01_00h.ccs", "VRChat/work/haseo/raw/2hsw01.ccs"));
                            self.player_mode = true;
                            self.pending_equip = Some("twin_blades".into());
                        }
                    }

                    GameState::Town { ref district, ref active_dialogue, ref active_shop } => {
                        ui.heading(egui::RichText::new(format!("📍 {}", district)).strong().color(egui::Color32::from_rgb(120, 220, 255)));

                        if let Some(dlg) = active_dialogue {
                            ui.group(|ui| {
                                ui.label(egui::RichText::new(format!("💬 {}", dlg.speaker)).strong().color(egui::Color32::YELLOW));
                                ui.label(&dlg.text);
                                ui.horizontal(|ui| {
                                    for opt in &dlg.options {
                                        if ui.button(opt).clicked() {
                                            self.game_runtime.close_popup();
                                        }
                                    }
                                    if ui.button("Close").clicked() {
                                        self.game_runtime.close_popup();
                                    }
                                });
                            });
                        }

                        if let Some(shop) = active_shop {
                            ui.group(|ui| {
                                ui.heading(egui::RichText::new(&shop.name).strong());
                                for item in &shop.listings {
                                    ui.horizontal(|ui| {
                                        ui.label(format!("{} ({} GP)", item.name, item.buy_price));
                                        if ui.button("Buy 1x").clicked() {
                                            let mut temp_shop = shop.clone();
                                            let _ = temp_shop.buy(item.item_id, 1, &mut self.game_runtime.save);
                                        }
                                        if ui.button("Sell 1x").clicked() {
                                            let mut temp_shop = shop.clone();
                                            let _ = temp_shop.sell(item.item_id, 1, &mut self.game_runtime.save);
                                        }
                                    });
                                }
                                if ui.button("Close Shop").clicked() {
                                    self.game_runtime.close_popup();
                                }
                            });
                        }

                        ui.add_space(4.0);
                        ui.label(egui::RichText::new("Town Interactions:").strong());
                        ui.horizontal_wrapped(|ui| {
                            if ui.button("💬 Talk to Atoli").clicked() {
                                self.game_runtime.talk_to_npc("Atoli");
                            }
                            if ui.button("💬 Talk to Silabus").clicked() {
                                self.game_runtime.talk_to_npc("Silabus");
                            }
                            if ui.button("💬 Talk to Gaspard").clicked() {
                                self.game_runtime.talk_to_npc("Gaspard");
                            }
                            if ui.button("🛍 Merchant Shop").clicked() {
                                self.game_runtime.open_shop();
                            }
                        });

                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            if ui.button(egui::RichText::new("🌀 Enter Chaos Gate").strong().size(14.0).color(egui::Color32::from_rgb(100, 200, 255))).clicked() {
                                self.game_runtime.open_chaos_gate();
                            }
                            if ui.button("💻 Log Out to Desktop").clicked() {
                                self.game_runtime.logout_to_desktop();
                            }
                        });
                    }

                    GameState::ChaosGateSelector { selected_word_a, selected_word_b, selected_word_c, ref preview_area } => {
                        ui.heading(egui::RichText::new("🌀 Chaos Gate: Keyword Selection").strong().color(egui::Color32::from_rgb(100, 200, 255)));
                        ui.label("Select three keywords to open the gate to an instanced area:");

                        ui.add_space(6.0);
                        let mut curr_a = selected_word_a;
                        let mut curr_b = selected_word_b;
                        let mut curr_c = selected_word_c;
                        let mut changed = false;

                        // 1. Keyword 1
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("1st Word:").strong().color(egui::Color32::from_rgb(255, 200, 100)));
                            egui::ComboBox::from_id_salt("word_a_combo")
                                .selected_text(&self.game_runtime.area_db.first_words[curr_a].name)
                                .show_ui(ui, |ui| {
                                    for (idx, w) in self.game_runtime.area_db.first_words.iter().enumerate() {
                                        if ui.selectable_value(&mut curr_a, idx, &w.name).clicked() {
                                            changed = true;
                                        }
                                    }
                                });
                        });

                        // 2. Keyword 2
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("2nd Word:").strong().color(egui::Color32::from_rgb(100, 220, 255)));
                            egui::ComboBox::from_id_salt("word_b_combo")
                                .selected_text(&self.game_runtime.area_db.second_words[curr_b].name)
                                .show_ui(ui, |ui| {
                                    for (idx, w) in self.game_runtime.area_db.second_words.iter().enumerate() {
                                        if ui.selectable_value(&mut curr_b, idx, &w.name).clicked() {
                                            changed = true;
                                        }
                                    }
                                });
                        });

                        // 3. Keyword 3
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("3rd Word:").strong().color(egui::Color32::from_rgb(200, 150, 255)));
                            egui::ComboBox::from_id_salt("word_c_combo")
                                .selected_text(&self.game_runtime.area_db.third_words[curr_c].name)
                                .show_ui(ui, |ui| {
                                    for (idx, w) in self.game_runtime.area_db.third_words.iter().enumerate() {
                                        if ui.selectable_value(&mut curr_c, idx, &w.name).clicked() {
                                            changed = true;
                                        }
                                    }
                                });
                        });

                        if changed {
                            self.game_runtime.select_keywords(curr_a, curr_b, curr_c);
                        }

                        ui.add_space(6.0);
                        ui.separator();
                        ui.group(|ui| {
                            ui.label(egui::RichText::new(format!("Target Area: {}", preview_area.full_name)).strong().size(15.0).color(egui::Color32::WHITE));
                            ui.label(format!("Area Level: {} | Element: {:?} | Stage: {:?}", preview_area.area_level, preview_area.element, preview_area.field_kind));
                            ui.label(format!("Weather: {:?} | Mission: {:?}", preview_area.weather, preview_area.mission));
                        });

                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            if ui.button("🌟 Quick Preset: Delta Hidden Forbidden Holy Ground").clicked() {
                                self.game_runtime.select_keywords(0, 0, 0);
                            }
                        });

                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            if ui.button(egui::RichText::new("⚡ GATE IN (Warp to Area)").strong().size(15.0).color(egui::Color32::from_rgb(80, 255, 120))).clicked() {
                                // The arrival event loads the field itself (start_cutscene); a second load here would wipe its cast.
                                self.player_mode = true;
                                self.pending_equip = Some("twin_blades".into());
                                // Trigger authentic retail arrival cutscene PM10105
                                self.game_runtime.start_prologue_field_cutscene();
                            }
                            if ui.button("Cancel").clicked() {
                                self.game_runtime.return_to_town();
                            }
                        });
                    }

                    GameState::Field { ref area, ref active_encounter, ref active_dialogue, chests: _, barrels: _, is_cleared, .. } => {
                        ui.heading(egui::RichText::new(format!("⚔ {}", area.full_name)).strong().color(egui::Color32::from_rgb(255, 200, 80)));
                        ui.label(format!("Area Level: {} | Gott Statue: {}", area.area_level, if is_cleared { "CLEARED 🏆" } else { "Undiscovered" }));

                        if let Some(dlg) = active_dialogue {
                            ui.group(|ui| {
                                ui.label(egui::RichText::new(&dlg.text).strong().color(egui::Color32::YELLOW));
                                if ui.button("Close").clicked() {
                                    self.game_runtime.close_popup();
                                }
                            });
                        }

                        if let Some(enc) = active_encounter {
                            ui.group(|ui| {
                                ui.heading(egui::RichText::new(format!("⚔ COMBAT: {} (Lv {})", enc.enemy.name, enc.enemy.stats.level)).strong().color(egui::Color32::from_rgb(255, 80, 80)));
                                ui.label(format!("Enemy HP: {} / {}", enc.enemy.stats.current_hp, enc.enemy.stats.max_hp));

                                let rengeki_color = if enc.rengeki_ready { egui::Color32::from_rgb(200, 80, 255) } else { egui::Color32::GRAY };
                                ui.label(egui::RichText::new(format!("Rengeki Gauge: {:.0}% {}", enc.rengeki_meter, if enc.rengeki_ready { "★ READY!" } else { "" })).strong().color(rengeki_color));
                                ui.label(format!("Combo: {}", enc.hit_combo));

                                ui.horizontal(|ui| {
                                    if ui.button(egui::RichText::new("⚔ Attack Combo").strong()).clicked() {
                                        self.game_runtime.combat_attack();
                                    }
                                    if enc.rengeki_ready {
                                        if ui.button(egui::RichText::new("💥 RENGEKI FINISH!").strong().color(egui::Color32::from_rgb(220, 100, 255))).clicked() {
                                            self.game_runtime.combat_rengeki();
                                        }
                                    }
                                });

                                for log in enc.log_messages.iter().rev().take(3) {
                                    ui.label(egui::RichText::new(log).small());
                                }
                            });
                        } else {
                            ui.add_space(4.0);
                            ui.label(egui::RichText::new("Field Actions:").strong());
                            ui.horizontal_wrapped(|ui| {
                                if ui.button("📦 Open Treasure Chest").clicked() {
                                    self.game_runtime.interact_nearest(glam::Vec3::new(15.0, 10.0, 0.0));
                                }
                                if ui.button("⚡ Kick Chim Barrel").clicked() {
                                    self.game_runtime.interact_nearest(glam::Vec3::new(8.0, 5.0, 0.0));
                                }
                                if ui.button("⚔ Engage Blade Goblin").clicked() {
                                    self.game_runtime.engage_combat(0);
                                }
                                if ui.button("🏆 Clear Gott Statue").clicked() {
                                    self.game_runtime.interact_nearest(glam::Vec3::new(50.0, 0.0, 0.0));
                                }
                            });

                            ui.add_space(6.0);
                            if ui.button("🌀 Gate Out (Return to Mac Anu)").clicked() {
                                self.game_runtime.return_to_town();
                                self.pending_explore = Some(("target/vol1/data/data/town/town01_00h.ccs", "VRChat/work/haseo/raw/2hsw01.ccs"));
                                self.player_mode = true;
                            }
                        }
                    }

                    GameState::Cutscene { script_id, scene_title, current_step, total_steps, ref dialogue, .. } => {
                        ui.heading(egui::RichText::new(format!("🎬 {}", scene_title)).strong().color(egui::Color32::from_rgb(255, 215, 80)));
                        ui.label(format!("Script: {} | Step: {}/{}", script_id, current_step + 1, total_steps));

                        ui.add_space(8.0);
                        if let Some(dlg) = dialogue {
                            ui.group(|ui| {
                                let speaker_color = match dlg.speaker.as_str() {
                                    "Haseo" => egui::Color32::from_rgb(255, 120, 80),
                                    "Iyoten" => egui::Color32::from_rgb(100, 200, 255),
                                    "Asta" => egui::Color32::from_rgb(180, 140, 255),
                                    "Ovan" => egui::Color32::from_rgb(255, 80, 80),
                                    "Silabus" => egui::Color32::from_rgb(100, 255, 120),
                                    _ => egui::Color32::YELLOW,
                                };
                                ui.label(egui::RichText::new(format!("🗣 {}", dlg.speaker)).strong().size(15.0).color(speaker_color));
                                ui.add_space(4.0);
                                ui.label(egui::RichText::new(&dlg.text).size(14.0));
                            });
                        }

                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            if ui.button(egui::RichText::new("▶ Advance (Space / A Button)").strong().size(14.0).color(egui::Color32::from_rgb(80, 255, 120))).clicked() {
                                self.game_runtime.advance_cutscene();
                            }
                            if ui.button("⏩ Skip Cutscene (Esc / B Button)").clicked() {
                                self.game_runtime.skip_cutscene();
                            }
                        });
                    }

                    GameState::Movie { movie_file, movie_title, .. } => {
                        ui.heading(egui::RichText::new(format!("🎬 {}", movie_title)).strong().color(egui::Color32::from_rgb(255, 215, 80)));
                        ui.label(format!("USM Video: {}", movie_file));
                        ui.label(egui::RichText::new("Full-motion pre-rendered FMV stream with native video/audio.").italics().color(egui::Color32::LIGHT_GRAY));
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            if ui.button(egui::RichText::new("▶ Watch Movie (Fullscreen)").strong().size(14.0).color(egui::Color32::from_rgb(255, 200, 80))).clicked() {
                                self.pending_play_movie = Some(movie_file.clone());
                            }
                            if ui.button(egui::RichText::new("▶ Advance to Next Scene").size(13.0).color(egui::Color32::from_rgb(80, 255, 120))).clicked() {
                                self.game_runtime.advance_cutscene();
                            }
                            if ui.button("⏩ Skip Movie").clicked() {
                                self.game_runtime.skip_cutscene();
                            }
                        });
                    }
                }

                ui.add_space(6.0);
                ui.separator();
                ui.collapsing("🎬 Prologue Movie Theater (Full-Motion Cutscenes)", |ui| {
                    ui.label("Stream authentic high-definition video cutscenes (.usm) from game files:");
                    ui.add_space(4.0);
                    if ui.button(egui::RichText::new("▶ 1. OP10000.usm - Opening Movie (1m 11s)").strong().color(egui::Color32::from_rgb(255, 200, 80))).clicked() {
                        self.pending_play_movie = Some("OP10000.usm".to_string());
                    }
                    if ui.button(egui::RichText::new("▶ 2. MM10145.usm - Ovan's Intervention (3m 30s)").strong().color(egui::Color32::from_rgb(255, 140, 80))).clicked() {
                        self.pending_play_movie = Some("MM10145.usm".to_string());
                    }
                    if ui.button(egui::RichText::new("▶ 3. MM10210.usm - Six Months Later: The Terror of Death (4m 20s)").strong().color(egui::Color32::from_rgb(255, 80, 80))).clicked() {
                        self.pending_play_movie = Some("MM10210.usm".to_string());
                    }
                    if ui.button(egui::RichText::new("▶ 4. MM10220.usm - Mac Anu PK Confrontation (3m 03s)").strong().color(egui::Color32::from_rgb(180, 140, 255))).clicked() {
                        self.pending_play_movie = Some("MM10220.usm".to_string());
                    }
                    if ui.button(egui::RichText::new("▶ 5. MM10230.usm - The Cathedral at Hulle Granz (3m 41s)").strong().color(egui::Color32::from_rgb(120, 200, 255))).clicked() {
                        self.pending_play_movie = Some("MM10230.usm".to_string());
                    }
                    if ui.button(egui::RichText::new("▶ 6. MM10240.usm - Tri-Edge Showdown & Data Drain (4m 53s)").strong().color(egui::Color32::from_rgb(255, 60, 60))).clicked() {
                        self.pending_play_movie = Some("MM10240.usm".to_string());
                    }
                });

                ui.add_space(6.0);
                ui.separator();
                ui.collapsing("📜 Adventure Log", |ui| {
                    for msg in self.game_runtime.message_log.iter().rev().take(6) {
                        ui.label(egui::RichText::new(msg).small());
                    }
                });
            });
        self.show_story_window = open;
    }

    /// Render authentic 1:1 CyberConnect2 Dialogue Box & Cinematic Widescreen Presentation
    pub fn draw_authentic_dialogue_overlay(&mut self, ctx: &egui::Context) {
        let (speaker, text, is_cutscene, scene_title, script_id, step_str) = match &self.game_runtime.state {
            // Movies play full screen with nothing over them, as in the original.
            GameState::Movie { .. } => return,
            GameState::Cutscene { dialogue, .. } => {
                // Event text as in the original (reference_captures/run2_field f00104, f00128): no box, no name.
                if let Some(dlg) = dialogue {
                    draw_event_text(ctx, &dlg.text, dlg.options.iter().any(|o| o == "notice"));
                }
                return;
            }
            GameState::Town { active_dialogue: Some(dlg), .. } => {
                (dlg.speaker.clone(), dlg.text.clone(), false, String::new(), String::new(), String::new())
            }
            GameState::Field { active_dialogue: Some(dlg), .. } => {
                (dlg.speaker.clone(), dlg.text.clone(), false, String::new(), String::new(), String::new())
            }
            _ => return,
        };

        let screen_rect = ctx.screen_rect();
        let screen_w = screen_rect.width();
        let screen_h = screen_rect.height();

        // 1. Cinematic 16:9 Letterbox Bars (Top and Bottom)
        if is_cutscene {
            let bar_h = (screen_h * 0.08).max(35.0);
            let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("letterbox_bars")));
            // Top letterbox bar
            painter.rect_filled(
                egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(screen_w, bar_h)),
                0.0,
                egui::Color32::from_rgb(5, 5, 8),
            );
            // Scene header on top letterbox
            if !scene_title.is_empty() {
                painter.text(
                    egui::pos2(25.0, bar_h * 0.45),
                    egui::Align2::LEFT_CENTER,
                    format!("◈ {}  [{}]", scene_title, script_id),
                    egui::FontId::proportional(13.0),
                    egui::Color32::from_rgb(180, 190, 210),
                );
                painter.text(
                    egui::pos2(screen_w - 25.0, bar_h * 0.45),
                    egui::Align2::RIGHT_CENTER,
                    format!("Scene {}", step_str),
                    egui::FontId::proportional(12.0),
                    egui::Color32::from_rgb(140, 150, 170),
                );
            }
            // Bottom letterbox bar
            painter.rect_filled(
                egui::Rect::from_min_size(egui::pos2(0.0, screen_h - bar_h), egui::vec2(screen_w, bar_h)),
                0.0,
                egui::Color32::from_rgb(5, 5, 8),
            );
        }

        // 2. Authentic CyberConnect2 Bottom-Third Dialogue Banner
        let box_w = (screen_w * 0.76).clamp(520.0, 960.0);
        let box_h = 135.0;
        let box_x = (screen_w - box_w) * 0.5;
        let box_y = screen_h - box_h - (if is_cutscene { (screen_h * 0.08).max(35.0) + 12.0 } else { 25.0 });

        let box_rect = egui::Rect::from_min_size(egui::pos2(box_x, box_y), egui::vec2(box_w, box_h));

        // Speaker color palette
        let (speaker_accent, badge_bg) = match speaker.as_str() {
            "Haseo" | "ハセヲ" => (egui::Color32::from_rgb(255, 110, 50), egui::Color32::from_rgba_unmultiplied(60, 20, 10, 245)),
            "Iyoten" | "Ｉｙｏｔｅｎ" => (egui::Color32::from_rgb(50, 180, 255), egui::Color32::from_rgba_unmultiplied(10, 35, 60, 245)),
            "Asta" | "アスタ" => (egui::Color32::from_rgb(175, 110, 255), egui::Color32::from_rgba_unmultiplied(40, 15, 65, 245)),
            "Ovan" | "オーヴァン" => (egui::Color32::from_rgb(255, 50, 50), egui::Color32::from_rgba_unmultiplied(65, 10, 15, 245)),
            "Silabus" | "シラバス" => (egui::Color32::from_rgb(60, 225, 110), egui::Color32::from_rgba_unmultiplied(15, 55, 25, 245)),
            "Gaspard" | "ガスパー" => (egui::Color32::from_rgb(255, 180, 50), egui::Color32::from_rgba_unmultiplied(60, 40, 10, 245)),
            "Atoli" | "アトリ" => (egui::Color32::from_rgb(255, 140, 180), egui::Color32::from_rgba_unmultiplied(60, 20, 35, 245)),
            _ => (egui::Color32::from_rgb(230, 210, 70), egui::Color32::from_rgba_unmultiplied(50, 45, 15, 245)),
        };

        let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("dialogue_banner")));

        // Dialogue background: Dark translucent navy/slate with border
        painter.rect_filled(
            box_rect,
            8.0,
            egui::Color32::from_rgba_unmultiplied(10, 14, 24, 235),
        );
        painter.rect_stroke(
            box_rect,
            8.0,
            egui::Stroke::new(1.8_f32, egui::Color32::from_rgba_unmultiplied(60, 130, 210, 220)),
            egui::StrokeKind::Inside,
        );

        // Accent corner notches (CyberConnect2 UI aesthetic)
        let notch_size = 10.0;
        painter.line_segment(
            [box_rect.left_top() + egui::vec2(notch_size, 0.0), box_rect.left_top() + egui::vec2(0.0, notch_size)],
            egui::Stroke::new(2.5_f32, speaker_accent),
        );
        painter.line_segment(
            [box_rect.right_bottom() - egui::vec2(notch_size, 0.0), box_rect.right_bottom() - egui::vec2(0.0, notch_size)],
            egui::Stroke::new(2.5_f32, speaker_accent),
        );

        // Floating Speaker Nameplate Badge
        let badge_w = (speaker.len() as f32 * 11.5 + 40.0).max(110.0);
        let badge_h = 28.0;
        let badge_rect = egui::Rect::from_min_size(
            egui::pos2(box_x + 18.0, box_y - 14.0),
            egui::vec2(badge_w, badge_h),
        );
        painter.rect_filled(badge_rect, 5.0, badge_bg);
        painter.rect_stroke(badge_rect, 5.0, egui::Stroke::new(1.8_f32, speaker_accent), egui::StrokeKind::Inside);
        painter.text(
            badge_rect.center(),
            egui::Align2::CENTER_CENTER,
            speaker.to_uppercase(),
            egui::FontId::proportional(14.0),
            speaker_accent,
        );

        // Dialogue Body Text
        let text_rect = egui::Rect::from_min_max(
            egui::pos2(box_x + 24.0, box_y + 24.0),
            egui::pos2(box_x + box_w - 45.0, box_y + box_h - 18.0),
        );
        let job = markup_job(&text, egui::FontId::proportional(16.5), egui::Color32::from_rgb(245, 248, 255), text_rect.width());
        let galley = ctx.fonts(|f| f.layout_job(job));
        painter.galley(text_rect.left_top(), galley, egui::Color32::WHITE);

        // Pulsing Advance Chevron Indicator (Bottom-right of banner)
        let time = ctx.input(|i| i.time);
        let pulse = ((time * 5.0).sin() * 0.5 + 0.5) as f32;
        let chevron_color = egui::Color32::from_rgba_unmultiplied(
            255,
            (140.0 + 80.0 * pulse) as u8,
            50,
            (160.0 + 95.0 * pulse) as u8,
        );
        let chevron_pos = box_rect.right_bottom() - egui::vec2(22.0, 18.0);
        painter.text(
            chevron_pos,
            egui::Align2::RIGHT_BOTTOM,
            "▼",
            egui::FontId::proportional(17.0),
            chevron_color,
        );

        // Controls footer prompt below dialog
        let prompt_y = box_y + box_h + 6.0;
        let prompt_text = if is_cutscene {
            "Ⓢ / Space: Advance   |   Ⓑ / Esc: Skip Cutscene"
        } else {
            "Ⓢ / Space: Close"
        };
        painter.text(
            egui::pos2(box_rect.center().x, prompt_y),
            egui::Align2::CENTER_TOP,
            prompt_text,
            egui::FontId::proportional(11.5),
            egui::Color32::from_rgba_unmultiplied(160, 180, 210, 190),
        );

        // Clickable interactive buttons below dialogue
        egui::Area::new(egui::Id::new("dialogue_click_prompt"))
            .fixed_pos(egui::pos2(box_rect.center().x - 110.0, prompt_y + 16.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if is_cutscene {
                        if ui.button(egui::RichText::new("▶ Advance (Space)").strong().color(egui::Color32::from_rgb(100, 255, 140))).clicked() {
                            self.pending_advance_cutscene = true;
                        }
                        if ui.button(egui::RichText::new("⏩ Skip Cutscene (Esc)").color(egui::Color32::from_rgb(255, 180, 100))).clicked() {
                            self.game_runtime.skip_cutscene();
                        }
                    } else {
                        if ui.button(egui::RichText::new("Close (Space / Esc)").strong()).clicked() {
                            self.game_runtime.close_popup();
                        }
                    }
                });
            });
    }

    /// Render authentic 1:1 .hack//G.U. Vol. 1//Rebirth Title Screen & Main Menu
    pub fn draw_title_screen(&mut self, ctx: &egui::Context) {
        let screen_rect = ctx.screen_rect();
        let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Middle, egui::Id::new("title_bg")));

        // Dark atmospheric backdrop overlay with soft vignette
        painter.rect_filled(
            screen_rect,
            0.0,
            egui::Color32::from_rgba_unmultiplied(8, 12, 22, 235),
        );

        // Tech grid lines reminiscent of The World R:2 / ALTIMIT OS
        let line_color = egui::Color32::from_rgba_unmultiplied(50, 90, 160, 30);
        let mut y = 30.0;
        while y < screen_rect.height() {
            painter.line_segment(
                [egui::pos2(0.0, y), egui::pos2(screen_rect.width(), y)],
                egui::Stroke::new(1.0_f32, line_color),
            );
            y += 40.0;
        }

        // Center Title & Menu Area
        egui::Area::new(egui::Id::new("title_menu_area"))
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, -10.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new(".hack//G.U.")
                            .size(54.0)
                            .strong()
                            .color(egui::Color32::from_rgb(255, 205, 70)),
                    );
                    ui.add_space(-4.0);
                    ui.label(
                        egui::RichText::new("Vol. 1 // Rebirth")
                            .size(26.0)
                            .strong()
                            .color(egui::Color32::from_rgb(130, 220, 255)),
                    );
                    ui.label(
                        egui::RichText::new("The World R:2 — Native Rust Recompilation Engine")
                            .size(13.0)
                            .italics()
                            .color(egui::Color32::from_rgb(160, 180, 210)),
                    );

                    ui.add_space(28.0);

                    // Authentic CC2 Menu Buttons Frame
                    egui::Frame::NONE
                        .fill(egui::Color32::from_rgba_unmultiplied(16, 22, 36, 220))
                        .stroke(egui::Stroke::new(1.5_f32, egui::Color32::from_rgb(70, 130, 200)))
                        .corner_radius(8.0)
                        .inner_margin(egui::Margin::symmetric(36, 20))
                        .show(ui, |ui| {
                            ui.set_width(320.0);
                            ui.spacing_mut().item_spacing.y = 12.0;

                            // NEW GAME
                            let new_game_btn = egui::Button::new(
                                egui::RichText::new("▶  NEW GAME")
                                    .size(17.0)
                                    .strong()
                                    .color(egui::Color32::from_rgb(255, 215, 80)),
                            )
                            .min_size(egui::vec2(280.0, 36.0));
                            if ui.add(new_game_btn).on_hover_text("Begin Haseo's journey from the opening prologue movie & Mac Anu awakening").clicked() {
                                self.pending_play_movie = Some("OP10000.usm".to_string());
                                self.game_runtime.start_prologue_movie();
                                self.pending_explore = Some(("target/vol1/data/data/town/town01.ccs", "VRChat/work/haseo/raw/2hsw01.ccs"));
                                self.player_mode = true;
                                self.pending_equip = Some("twin_blades".into());
                                self.show_sidebar = false;
                            }

                            // CONTINUE
                            let cont_btn = egui::Button::new(
                                egui::RichText::new("▶  CONTINUE")
                                    .size(17.0)
                                    .strong()
                                    .color(egui::Color32::from_rgb(120, 225, 255)),
                            )
                            .min_size(egui::vec2(280.0, 36.0));
                            if ui.add(cont_btn).on_hover_text("Load character data and enter Mac Anu Central Plaza directly").clicked() {
                                self.game_runtime.login_to_the_world();
                                self.pending_explore = Some(("target/vol1/data/data/town/town01_00h.ccs", "VRChat/work/haseo/raw/2hsw01.ccs"));
                                self.player_mode = true;
                                self.pending_equip = Some("twin_blades".into());
                                self.show_sidebar = false;
                            }

                            // ALTIMIT OS
                            let altimit_btn = egui::Button::new(
                                egui::RichText::new("▶  ALTIMIT OS")
                                    .size(17.0)
                                    .strong()
                                    .color(egui::Color32::from_rgb(140, 190, 255)),
                            )
                            .min_size(egui::vec2(280.0, 36.0));
                            if ui.add(altimit_btn).on_hover_text("Open ALTIMIT OS desktop (Mailer, Forums, News)").clicked() {
                                self.game_runtime.to_desktop();
                                self.show_sidebar = false;
                            }

                            // PROLOGUE MOVIE THEATER
                            let theater_btn = egui::Button::new(
                                egui::RichText::new("▶  PROLOGUE MOVIE THEATER")
                                    .size(15.0)
                                    .strong()
                                    .color(egui::Color32::from_rgb(220, 160, 255)),
                            )
                            .min_size(egui::vec2(280.0, 34.0));
                            if ui.add(theater_btn).on_hover_text("Stream authentic high-definition CRI Sofdec USM full-motion video cutscenes").clicked() {
                                self.show_movie_theater = !self.show_movie_theater;
                            }

                            // QUIT
                            let quit_btn = egui::Button::new(
                                egui::RichText::new("▶  QUIT")
                                    .size(15.0)
                                    .color(egui::Color32::from_rgb(200, 120, 120)),
                            )
                            .min_size(egui::vec2(280.0, 32.0));
                            if ui.add(quit_btn).clicked() {
                                std::process::exit(0);
                            }
                        });

                    ui.add_space(24.0);
                    ui.label(
                        egui::RichText::new("Press [F1] for Developer / Model Inspection Tools")
                            .size(12.0)
                            .color(egui::Color32::from_rgba_unmultiplied(160, 180, 210, 180)),
                    );
                    ui.label(
                        egui::RichText::new("© BANDAI NAMCO Entertainment Inc. / CyberConnect2 Co., Ltd.")
                            .size(11.0)
                            .color(egui::Color32::from_rgba_unmultiplied(130, 150, 180, 140)),
                    );
                });
            });

        // Prologue Movie Theater Modal Window
        if self.show_movie_theater {
            let mut open = true;
            egui::Window::new("🎬 Prologue Movie Theater (Full-Motion CRI USM Cutscenes)")
                .open(&mut open)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .resizable(false)
                .show(ctx, |ui| {
                    ui.set_width(480.0);
                    ui.label("Play authentic pre-rendered USM movies with clean, distortion-free stereo audio:");
                    ui.add_space(8.0);

                    let movies = [
                        ("OP10000.usm", "1. Opening Movie (Haseo begins 'The World R:2')", "1m 11s", egui::Color32::from_rgb(255, 200, 80)),
                        ("MM10145.usm", "2. Ovan's Intervention (Rescue from PKers & AIDA)", "3m 30s", egui::Color32::from_rgb(255, 140, 80)),
                        ("MM10210.usm", "3. Six Months Later: The Terror of Death", "4m 20s", egui::Color32::from_rgb(255, 80, 80)),
                        ("MM10220.usm", "4. Mac Anu PK Confrontation", "3m 03s", egui::Color32::from_rgb(180, 140, 255)),
                        ("MM10230.usm", "5. The Cathedral at Hulle Granz", "3m 41s", egui::Color32::from_rgb(120, 200, 255)),
                        ("MM10240.usm", "6. Tri-Edge Showdown & Data Drain", "4m 53s", egui::Color32::from_rgb(255, 60, 60)),
                    ];

                    for (file, title, dur, color) in movies {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(title).strong().color(color));
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(egui::RichText::new(dur).small());
                                });
                            });
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(format!("File: {}", file)).small().italics());
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.button(egui::RichText::new("▶ Play Movie").strong().color(egui::Color32::from_rgb(80, 255, 120))).clicked() {
                                        self.pending_play_movie = Some(file.to_string());
                                    }
                                });
                            });
                        });
                    }

                    ui.add_space(8.0);
                    if ui.button("✖ Close Theater").clicked() {
                        self.show_movie_theater = false;
                    }
                });
            if !open {
                self.show_movie_theater = false;
            }
        }
    }
}

/// Lay out game message markup: `<colorRRGGBB>` switches colour (FFFFFF = default), `<ruby base|reading>`
/// shows the base text, `<iconNAME>` becomes the PC (Xbox-layout) button glyph the PC version shows.
pub fn markup_job(text: &str, font: egui::FontId, default: egui::Color32, wrap: f32) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    job.wrap.max_width = wrap;
    let mut color = default;
    let mut push = |job: &mut egui::text::LayoutJob, s: &str, c: egui::Color32| {
        if !s.is_empty() {
            job.append(s, 0.0, egui::TextFormat { font_id: font.clone(), color: c, ..Default::default() });
        }
    };
    let mut rest = text;
    while let Some(start) = rest.find('<') {
        push(&mut job, &rest[..start], color);
        let after = &rest[start + 1..];
        let Some(end) = after.find('>') else {
            push(&mut job, &rest[start..], color);
            rest = "";
            break;
        };
        let tag = &after[..end];
        if let Some(hex) = tag.strip_prefix("color") {
            color = match u32::from_str_radix(hex.trim(), 16) {
                Ok(0xFFFFFF) | Err(_) => default,
                Ok(v) => egui::Color32::from_rgb((v >> 16) as u8, (v >> 8) as u8, v as u8),
            };
        } else if let Some(body) = tag.strip_prefix("ruby") {
            push(&mut job, body.split('|').next().unwrap_or(""), color);
        } else if let Some(name) = tag.strip_prefix("icon") {
            let (glyph, c) = match name {
                "CROSS" => ("(A)", egui::Color32::from_rgb(110, 200, 90)),
                "CIRCLE" => ("(B)", egui::Color32::from_rgb(230, 80, 70)),
                "SQUARE" => ("(X)", egui::Color32::from_rgb(80, 140, 240)),
                "TRIANGLE" => ("(Y)", egui::Color32::from_rgb(240, 200, 60)),
                _ => ("", color),
            };
            if glyph.is_empty() {
                push(&mut job, &format!("[{}]", name), color);
            } else {
                push(&mut job, glyph, c);
            }
        } else {
            push(&mut job, &rest[start..start + end + 2], color);
        }
        rest = &after[end + 1..];
    }
    push(&mut job, rest, color);
    job
}

/// Event text, measured on reference_captures/run2_field at 960x540.
/// Spoken line (f00104): centred white text with a dark outline, capitals 9.5 px, last line's baseline at y 479.
/// Notice (op 6, f00128 (game text)): pale gold (225,196,152) text centred in a dark band
/// x 229-731, y 245-297. The speaker's name is not shown.
pub fn draw_event_text(ctx: &egui::Context, text: &str, notice: bool) {
    let screen = ctx.screen_rect();
    let (w, h) = if screen.width() / screen.height() > 16.0 / 9.0 { (screen.height() * 16.0 / 9.0, screen.height()) } else { (screen.width(), screen.width() * 9.0 / 16.0) };
    let o = egui::pos2(screen.center().x - w * 0.5, screen.center().y - h * 0.5);
    let px = h / 540.0;
    let at = |x: f32, y: f32| egui::pos2(o.x + x * px, o.y + y * px);
    let p = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("event_text")));
    if notice {
        let plain = crate::altimit::strip_tags(text);
        p.rect_filled(egui::Rect::from_min_max(at(229.0, 245.0), at(731.0, 297.0)), 0.0, egui::Color32::from_black_alpha(130));
        p.text(at(480.0, 271.0), egui::Align2::CENTER_CENTER, plain, egui::FontId::proportional(11.0 * px / 0.70), egui::Color32::from_rgb(225, 196, 152));
        return;
    }
    let font = egui::FontId::proportional(9.5 * px / 0.70);
    let job = markup_job(text, font.clone(), egui::Color32::WHITE, 820.0 * px);
    let mut job_centered = job.clone();
    job_centered.halign = egui::Align::Center;
    let g = ctx.fonts(|f| f.layout_job(job_centered));
    // Baseline of the last line at y 479: the galley's bottom sits about a quarter of a line below it.
    let line = g.rows.last().map(|r| r.rect.height()).unwrap_or(13.0 * px);
    let top = at(0.0, 479.0).y + line * 0.22 - g.size().y;
    let pos = egui::pos2(at(480.0, 0.0).x, top);
    // Outline: the plain text in black, offset around the glyphs.
    // Same layout as the text (icon glyphs included), all black: a stripped copy was shorter than "(Y)" lines and sat
    // misaligned behind them.
    let mut ojob = markup_job(text, font, egui::Color32::from_black_alpha(200), 820.0 * px);
    for sec in &mut ojob.sections {
        sec.format.color = egui::Color32::from_black_alpha(200);
    }
    ojob.halign = egui::Align::Center;
    let og = ctx.fonts(|f| f.layout_job(ojob));
    for (dx, dy) in [(-1.0, 0.0), (1.0, 0.0), (0.0, -1.0), (0.0, 1.0), (-1.0, -1.0), (1.0, 1.0), (-1.0, 1.0), (1.0, -1.0)] {
        p.galley(pos + egui::vec2(dx, dy) * px.max(1.0), og.clone(), egui::Color32::BLACK);
    }
    p.galley(pos, g, egui::Color32::WHITE);
}
