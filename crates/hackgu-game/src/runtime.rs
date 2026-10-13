//! # CyberConnect2 Game Runtime & Playable Story Orchestrator
//! Ties together ALTIMIT OS Desktop, Town hub exploration, Chaos Gate area selection,
//! Field dungeon exploration, Gimmicks, Combat encounters, and Save progression.

use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::savedata::{SaveData, InventorySlot};
use crate::desktop::AltimitDesktop;
use crate::area::{AreaWordDatabase, GeneratedArea, RootServer};
use crate::context::WorldContext;
use crate::enemy::{EnemyEntity, EnemyArchetype};
use crate::gimmick::{ItemBox, ChestKind, TimBarrel};
use crate::shop::ShopCatalog;
use crate::quest::QuestManager;

/// Interactive dialogue popup box.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DialogueBox {
    pub speaker: String,
    pub portrait: Option<String>,
    pub text: String,
    pub options: Vec<String>,
}

/// Active Combat Encounter state inside the field.
#[derive(Debug, Clone)]
pub struct BattleEncounter {
    pub encounter_id: u32,
    pub enemy: EnemyEntity,
    pub rengeki_meter: f32, // 0.0 to 100.0%
    pub rengeki_ready: bool,
    pub hit_combo: u32,
    pub morale_meter: f32, // 0.0 to 100.0%
    pub battle_timer: f32,
    pub log_messages: Vec<String>,
}

/// Primary Playable Game Lifecycle State Machine.
#[derive(Debug, Clone)]
pub enum GameState {
    /// Authentic .hack//G.U. Vol. 1//Rebirth Title Screen & Main Menu
    TitleScreen,
    /// ALTIMIT OS Desktop (Check emails, read forum threads, click Login)
    Desktop,
    /// Mac Anu City Hub (Explore plaza, talk to NPCs, visit shops, go to Chaos Gate)
    Town {
        district: String,
        active_dialogue: Option<DialogueBox>,
        active_shop: Option<ShopCatalog>,
    },
    /// Chaos Gate 3-Keyword Area Selection Menu
    ChaosGateSelector {
        selected_word_a: usize,
        selected_word_b: usize,
        selected_word_c: usize,
        preview_area: GeneratedArea,
    },
    /// Field / Dungeon Adventure (Explore ruins, open chests, kick chims, battle monsters)
    Field {
        area: GeneratedArea,
        active_encounter: Option<BattleEncounter>,
        active_dialogue: Option<DialogueBox>,
        chests: Vec<ItemBox>,
        barrels: Vec<TimBarrel>,
        enemies: Vec<EnemyEntity>,
        is_cleared: bool,
    },
    /// Authentic Cinematic Cutscene Playback (PM10100 opening, PM10105 Chaos Gate, PM10135 PK betrayal, PM10140 Ovan rescue)
    Cutscene {
        script_id: String,
        scene_title: String,
        current_step: usize,
        total_steps: usize,
        dialogue: Option<DialogueBox>,
        camera_eye: Vec3,
        camera_target: Vec3,
        camera_fov: f32,
        is_finished: bool,
    },
    /// Authentic Full-Motion Video / Movie Playback (OP10000.usm, MM10145.usm)
    Movie {
        movie_file: String,
        movie_title: String,
        next_script_id: Option<String>,
        is_finished: bool,
    },
}

/// Combat action outcome result.
#[derive(Debug, Clone, PartialEq)]
pub enum CombatOutcome {
    Hit { damage: u32, is_combo: bool, is_rengeki: bool },
    EnemyDefeated { exp_earned: u32, gp_earned: u32, item_dropped: Option<String> },
    EnemyAttacked { damage_taken: u32 },
    PlayerDefeated,
    None,
}

/// Top-level Game Engine Runtime.
pub struct GameRuntime {
    pub state: GameState,
    pub save: SaveData,
    pub desktop: AltimitDesktop,
    pub area_db: AreaWordDatabase,
    pub quest_mgr: QuestManager,
    pub world: WorldContext,
    pub message_log: Vec<String>,
    /// A scripted desktop sequence to play when the desktop opens ("reboot").
    pub pending_desktop_sequence: Option<String>,
    /// Haseo was reformatted (after MM10145); the next log-in plays PM10310.
    pub reformatted: bool,
    /// Events the player has seen, in order (drives which town events are armed next).
    pub events_done: Vec<String>,
    /// Play time in seconds (counted outside the title screen; shown in the Data Manager).
    pub play_time: f64,
}

impl GameRuntime {
    /// Initialize a new authentic .hack//G.U. Vol 1 game runtime.
    pub fn new_game(player_name: &str) -> Self {
        let save = SaveData::new_game_vol1(player_name);
        let desktop = AltimitDesktop::new_vol1_defaults();
        let area_db = AreaWordDatabase::new_vol1_defaults();
        let quest_mgr = QuestManager::new_vol1_defaults();
        let world = WorldContext::new();

        Self {
            state: GameState::TitleScreen,
            save,
            desktop,
            area_db,
            quest_mgr,
            world,
            message_log: vec!["Welcome to .hack//G.U. Vol. 1//Rebirth. Press Start or select New Game.".into()],
            pending_desktop_sequence: None,
            reformatted: false,
            events_done: Vec::new(),
            play_time: 0.0,
        }
    }

    /// Return back to Title Screen.
    pub fn to_title_screen(&mut self) {
        self.state = GameState::TitleScreen;
        self.message_log.push("Returned to Title Screen.".into());
    }

    /// Open ALTIMIT OS Desktop.
    pub fn to_desktop(&mut self) {
        self.state = GameState::Desktop;
        self.message_log.push("Switched to ALTIMIT OS Desktop.".into());
    }

    /// Log in from ALTIMIT OS to Mac Anu in The World.
    pub fn login_to_the_world(&mut self) {
        self.desktop.login_to_world();
        if self.reformatted {
            // First log-in after the reformat: PM10310 at Mac Anu's Chaos Gate ((game text)).
            self.reformatted = false;
            self.start_script("PM10310");
            return;
        }
        self.state = GameState::Town {
            district: "Mac Anu - Central District".into(),
            active_dialogue: Some(DialogueBox {
                speaker: "System".into(),
                portrait: None,
                text: "Welcome to The World R:2. Delta Server - Water Capital Mac Anu.".into(),
                options: vec!["Continue".into()],
            }),
            active_shop: None,
        };
        self.message_log.push("Logged into The World R:2 (Delta Server).".into());
    }

    /// Log out from The World back to ALTIMIT OS desktop.
    pub fn logout_to_desktop(&mut self) {
        self.desktop.logout_to_desktop();
        self.state = GameState::Desktop;
        self.message_log.push("Logged out to ALTIMIT OS Desktop.".into());
    }

    /// Interact with an authentic NPC in town.
    pub fn talk_to_npc(&mut self, npc_name: &str) {
        let dialogue = match npc_name {
            "Atoli" => DialogueBox {
                speaker: "Atoli".into(),
                portrait: Some("2atl01".into()),
                text: "Nice to meet you, Haseo! I'm Atoli from Moon Tree. The city of Mac Anu is so pretty today, isn't it?".into(),
                options: vec!["Say hello".into(), "Ask about Tri-Edge".into()],
            },
            "Silabus" => DialogueBox {
                speaker: "Silabus".into(),
                portrait: Some("2syl01".into()),
                text: "Hey Haseo! Glad to have you with Canard. Whenever you're ready to tackle quests, let me know!".into(),
                options: vec!["Check quests".into(), "Ask for advice".into()],
            },
            "Gaspard" => DialogueBox {
                speaker: "Gaspard".into(),
                portrait: Some("2gas01".into()),
                text: "H-Haseo! Don't forget you can buy healing supplies at the merchant shop before gating out!".into(),
                options: vec!["Thanks Gaspard".into()],
            },
            _ => DialogueBox {
                speaker: npc_name.to_string(),
                portrait: None,
                text: "The rumors about Tri-Edge at the Lost Ground are getting louder every day...".into(),
                options: vec!["Dismiss".into()],
            },
        };

        if let GameState::Town { ref mut active_dialogue, .. } = self.state {
            *active_dialogue = Some(dialogue);
        }
    }

    /// Open merchant shop menu in town.
    pub fn open_shop(&mut self) {
        if let GameState::Town { ref mut active_shop, .. } = self.state {
            *active_shop = Some(ShopCatalog::new_mac_anu_item_shop());
        }
    }

    /// Close dialogue or shop popups.
    pub fn close_popup(&mut self) {
        if let GameState::Town { ref mut active_dialogue, ref mut active_shop, .. } = self.state {
            *active_dialogue = None;
            *active_shop = None;
        } else if let GameState::Field { ref mut active_dialogue, .. } = self.state {
            *active_dialogue = None;
        }
    }

    /// Open the Chaos Gate area selector.
    pub fn open_chaos_gate(&mut self) {
        let word_a = self.area_db.first_words.first().unwrap();
        let word_b = self.area_db.second_words.first().unwrap();
        let word_c = self.area_db.third_words.first().unwrap();
        let preview = self.area_db.synthesize(
            RootServer::Delta,
            word_a.clone(),
            word_b.clone(),
            word_c.clone(),
        );

        self.state = GameState::ChaosGateSelector {
            selected_word_a: 0,
            selected_word_b: 0,
            selected_word_c: 0,
            preview_area: preview,
        };
    }

    /// Select keywords in the Chaos Gate.
    pub fn select_keywords(&mut self, word_a_idx: usize, word_b_idx: usize, word_c_idx: usize) {
        if let GameState::ChaosGateSelector {
            ref mut selected_word_a,
            ref mut selected_word_b,
            ref mut selected_word_c,
            ref mut preview_area,
        } = self.state {
            *selected_word_a = word_a_idx.min(self.area_db.first_words.len() - 1);
            *selected_word_b = word_b_idx.min(self.area_db.second_words.len() - 1);
            *selected_word_c = word_c_idx.min(self.area_db.third_words.len() - 1);

            let a = self.area_db.first_words[*selected_word_a].clone();
            let b = self.area_db.second_words[*selected_word_b].clone();
            let c = self.area_db.third_words[*selected_word_c].clone();

            *preview_area = self.area_db.synthesize(RootServer::Delta, a, b, c);
        }
    }

    /// Warp through the Chaos Gate into the generated area.
    pub fn warp_to_selected_area(&mut self) {
        let area = if let GameState::ChaosGateSelector { ref preview_area, .. } = self.state {
            preview_area.clone()
        } else {
            // Default to Delta Hidden Forbidden Holy Ground
            self.area_db.generate(
                RootServer::Delta,
                "Hidden",
                "Forbidden",
                "Holy Ground",
            ).unwrap()
        };

        // Populate field entities
        let chests = vec![
            ItemBox::new(101, Vec3::new(15.0, 10.0, 0.0), ChestKind::Wood, "Health Drink"),
            ItemBox::new(102, Vec3::new(30.0, -15.0, 0.0), ChestKind::Silver, "Spin GaiGu"),
        ];

        let barrels = vec![
            TimBarrel::new(201, Vec3::new(8.0, 5.0, 0.0)),
            TimBarrel::new(202, Vec3::new(-12.0, 20.0, 0.0)),
        ];

        let enemies = vec![
            EnemyEntity::new(301, "Blade Goblin", EnemyArchetype::SmallGoblin, Vec3::new(25.0, 0.0, 0.0), area.area_level),
            EnemyEntity::new(302, "Armored Ogre", EnemyArchetype::ArmoredGuard, Vec3::new(45.0, 15.0, 0.0), area.area_level + 1),
        ];

        let full_name = area.full_name.clone();
        self.state = GameState::Field {
            area,
            active_encounter: None,
            active_dialogue: Some(DialogueBox {
                speaker: "System".into(),
                portrait: None,
                text: format!("Gated in to [{}]! Locate the sacred Gott Statue.", full_name),
                options: vec!["Explore".into()],
            }),
            chests,
            barrels,
            enemies,
            is_cleared: false,
        };

        self.message_log.push(format!("Warped to Area: {}", full_name));
    }

    /// Trigger field interaction with nearest chest, barrel, or shrine.
    pub fn interact_nearest(&mut self, player_pos: Vec3) -> Option<String> {
        if let GameState::Field { ref mut chests, ref mut barrels, ref mut is_cleared, .. } = self.state {
            // Check chests
            for chest in chests.iter_mut() {
                let dist = (chest.position - player_pos).length();
                if dist <= 3.0 && !chest.is_opened {
                    chest.is_opened = true;
                    let item_name = chest.item_name.clone();
                    // Add item to save inventory
                    if let Some(slot) = self.save.items.iter_mut().find(|s| s.name == item_name) {
                        slot.quantity = (slot.quantity + 1).min(99);
                    } else {
                        self.save.items.push(InventorySlot {
                            item_id: chest.id,
                            name: item_name.clone(),
                            quantity: 1,
                        });
                    }
                    let msg = format!("Opened treasure chest: Obtained [{}]!", item_name);
                    self.message_log.push(msg.clone());
                    return Some(msg);
                }
            }

            // Check chim barrels
            for barrel in barrels.iter_mut() {
                let dist = (barrel.position - player_pos).length();
                if dist <= 3.0 && barrel.remaining_kicks > 0 {
                    barrel.remaining_kicks -= 1;
                    let count = 4;
                    self.save.add_chim(count);
                    let msg = format!("Kicked Chim Barrel! Obtained {} Chim Chims! (Total: {})", count, self.save.chim_count);
                    self.message_log.push(msg.clone());
                    return Some(msg);
                }
            }

            // Check Gott Statue at (50.0, 0.0, 0.0)
            if (Vec3::new(50.0, 0.0, 0.0) - player_pos).length() <= 5.0 && !*is_cleared {
                *is_cleared = true;
                self.save.add_gp(2000);
                self.save.add_chim(10);
                let msg = "Gott Statue Cleared! Earned 2000 GP and Gott Treasure!".to_string();
                self.message_log.push(msg.clone());
                self.trigger_beast_statue_cutscene();
                return Some(msg);
            }
        }
        None
    }

    /// Engage a field enemy in combat.
    pub fn engage_combat(&mut self, enemy_idx: usize) {
        if let GameState::Field { ref mut enemies, ref mut active_encounter, .. } = self.state {
            if let Some(enemy) = enemies.get_mut(enemy_idx) {
                if enemy.stats.current_hp > 0 {
                    let encounter = BattleEncounter {
                        encounter_id: enemy.obj_id,
                        enemy: enemy.clone(),
                        rengeki_meter: 0.0,
                        rengeki_ready: false,
                        hit_combo: 0,
                        morale_meter: 0.0,
                        battle_timer: 0.0,
                        log_messages: vec![format!("Encounter started with {} (Lv {})!", enemy.name, enemy.stats.level)],
                    };
                    *active_encounter = Some(encounter);
                }
            }
        }
    }

    /// Execute player attack in active combat.
    pub fn combat_attack(&mut self) -> CombatOutcome {
        if let GameState::Field { ref mut active_encounter, ref mut enemies, .. } = self.state {
            if let Some(ref mut enc) = active_encounter {
                enc.hit_combo += 1;
                enc.rengeki_meter = (enc.rengeki_meter + 50.0).min(100.0);
                enc.morale_meter = (enc.morale_meter + 10.0).min(100.0);
                if enc.rengeki_meter >= 100.0 {
                    enc.rengeki_ready = true;
                }

                let base_atk = 35 + (self.save.level * 5);
                let dmg = enc.enemy.take_damage(base_atk, false, None);
                enc.log_messages.push(format!("Haseo slashes {} for {} damage! (Combo {})", enc.enemy.name, dmg, enc.hit_combo));

                if enc.enemy.stats.current_hp == 0 {
                    let exp = enc.enemy.stats.exp_reward;
                    let gp = enc.enemy.stats.gp_reward;
                    let drop = enc.enemy.stats.drop_item.clone();

                    self.save.exp += exp;
                    self.save.add_gp(gp);
                    if let Some(ref item) = drop {
                        if let Some(slot) = self.save.items.iter_mut().find(|s| &s.name == item) {
                            slot.quantity = (slot.quantity + 1).min(99);
                        } else {
                            self.save.items.push(InventorySlot {
                                item_id: 1,
                                name: item.clone(),
                                quantity: 1,
                            });
                        }
                    }

                    // Level up check
                    if self.save.exp >= self.save.level * 100 {
                        self.save.level += 1;
                        self.save.max_hp += 25;
                        self.save.hp = self.save.max_hp;
                        self.message_log.push(format!("LEVEL UP! Haseo is now Level {}!", self.save.level));
                    }

                    // Sync enemy back to field
                    if let Some(field_enemy) = enemies.iter_mut().find(|e| e.obj_id == enc.encounter_id) {
                        field_enemy.stats.current_hp = 0;
                        field_enemy.active = false;
                    }

                    let outcome = CombatOutcome::EnemyDefeated {
                        exp_earned: exp,
                        gp_earned: gp,
                        item_dropped: drop,
                    };

                    *active_encounter = None;
                    return outcome;
                }

                return CombatOutcome::Hit { damage: dmg, is_combo: true, is_rengeki: false };
            }
        }
        CombatOutcome::None
    }

    /// Execute player Rengeki finishing skill.
    pub fn combat_rengeki(&mut self) -> CombatOutcome {
        if let GameState::Field { ref mut active_encounter, ref mut enemies, .. } = self.state {
            if let Some(ref mut enc) = active_encounter {
                if !enc.rengeki_ready {
                    return CombatOutcome::None;
                }

                enc.rengeki_ready = false;
                enc.rengeki_meter = 0.0;
                let rengeki_atk = 80 + (self.save.level * 12);
                let dmg = enc.enemy.take_damage(rengeki_atk, true, Some(Vec3::new(0.0, 5.0, 0.0)));
                enc.log_messages.push(format!("💥 RENGEKI FINISH! Critical strike for {} damage!", dmg));

                if enc.enemy.stats.current_hp == 0 {
                    let exp = enc.enemy.stats.exp_reward * 2; // Rengeki EXP bonus!
                    let gp = enc.enemy.stats.gp_reward;
                    let drop = enc.enemy.stats.drop_item.clone();

                    self.save.exp += exp;
                    self.save.add_gp(gp);

                    if let Some(field_enemy) = enemies.iter_mut().find(|e| e.obj_id == enc.encounter_id) {
                        field_enemy.stats.current_hp = 0;
                        field_enemy.active = false;
                    }

                    let outcome = CombatOutcome::EnemyDefeated {
                        exp_earned: exp,
                        gp_earned: gp,
                        item_dropped: drop,
                    };

                    *active_encounter = None;
                    return outcome;
                }

                return CombatOutcome::Hit { damage: dmg, is_combo: true, is_rengeki: true };
            }
        }
        CombatOutcome::None
    }

    /// Advance active simulation tick (dt seconds).
    pub fn tick(&mut self, dt: f32, player_pos: Vec3) {
        if let GameState::Field {
            ref mut active_encounter,
            ref mut enemies,
            ..
        } = self.state {
            if let Some(ref mut enc) = active_encounter {
                enc.battle_timer += dt;
                // Enemy counter-attacks periodically
                let (enemy_dmg, _) = enc.enemy.update_combat(dt, player_pos);
                if let Some(dmg) = enemy_dmg {
                    let actual = (dmg / 2).max(1);
                    self.save.hp = self.save.hp.saturating_sub(actual);
                    enc.log_messages.push(format!("{} counter-attacks Haseo for {} damage!", enc.enemy.name, actual));
                }
            } else {
                // Roam enemies in field
                let mut triggered_idx = None;
                for (idx, enemy) in enemies.iter_mut().enumerate() {
                    enemy.update_field(dt, player_pos, &self.world);
                    // If enemy gets within 3.0m and is alive, initiate encounter
                    if (enemy.position - player_pos).length() <= 3.0 && enemy.stats.current_hp > 0 {
                        triggered_idx = Some(idx);
                        break;
                    }
                }
                if let Some(idx) = triggered_idx {
                    self.engage_combat(idx);
                }
            }
        }
    }

    /// Initiate a surprise attack against a field enemy (e.g. striking from behind or Steam Bike ramming).
    pub fn surprise_attack(&mut self, enemy_idx: usize) {
        if let GameState::Field { ref mut enemies, ref mut active_encounter, .. } = self.state {
            if let Some(enemy) = enemies.get_mut(enemy_idx) {
                if enemy.stats.current_hp > 0 {
                    let mut surprise_enemy = enemy.clone();
                    let bonus_dmg = 45;
                    surprise_enemy.take_damage(bonus_dmg, false, None);

                    let encounter = BattleEncounter {
                        encounter_id: surprise_enemy.obj_id,
                        enemy: surprise_enemy,
                        rengeki_meter: 60.0,
                        rengeki_ready: false,
                        hit_combo: 1,
                        morale_meter: 30.0,
                        battle_timer: 0.0,
                        log_messages: vec![
                            format!("💥 SURPRISE ATTACK on {}! Dealt {} ambush damage!", enemy.name, bonus_dmg),
                            "Rengeki gauge charged by 60%!".into(),
                        ],
                    };
                    *active_encounter = Some(encounter);
                }
            }
        }
    }

    /// Consume a recovery or booster item from inventory.
    pub fn use_item(&mut self, item_name: &str) -> bool {
        let has_item = if let Some(slot) = self.save.items.iter_mut().find(|s| s.name == item_name && s.quantity > 0) {
            slot.quantity -= 1;
            true
        } else {
            false
        };

        if has_item {
            self.save.items.retain(|s| s.quantity > 0);
            match item_name {
                "Health Drink" => {
                    let heal = 150;
                    self.save.hp = (self.save.hp + heal).min(self.save.max_hp);
                    let msg = format!("Used Health Drink: Restored {} HP! (HP: {}/{})", heal, self.save.hp, self.save.max_hp);
                    self.message_log.push(msg);
                }
                "Spin GaiGu" => {
                    self.save.sp = (self.save.sp + 50).min(self.save.max_sp);
                    let msg = format!("Used Spin GaiGu: Restored 50 SP! (SP: {}/{})", self.save.sp, self.save.max_sp);
                    self.message_log.push(msg);
                }
                _ => {
                    let msg = format!("Used [{}]!", item_name);
                    self.message_log.push(msg);
                }
            }
            true
        } else {
            false
        }
    }

    /// Check if currently in an active combat encounter.
    pub fn is_in_combat(&self) -> bool {
        if let GameState::Field { active_encounter: Some(_), .. } = self.state {
            true
        } else {
            false
        }
    }

    /// Check if currently exploring a field/dungeon.
    pub fn is_in_field(&self) -> bool {
        matches!(self.state, GameState::Field { .. })
    }

    /// Return back to Mac Anu city hub from completed or abandoned field.
    pub fn return_to_town(&mut self) {
        // No message box: the original has none (the old "Warp complete..." text was invented).
        self.state = GameState::Town { district: "Mac Anu - Central District".into(), active_dialogue: None, active_shop: None };
        self.message_log.push("Returned to Mac Anu.".into());
    }

    /// Start the authentic opening story with pre-rendered intro movie (OP10000.usm).
    pub fn start_prologue_movie(&mut self) {
        self.state = GameState::Movie {
            movie_file: "OP10000.usm".into(),
            movie_title: ".hack//G.U. Vol. 1//Rebirth Opening Movie (Pre-Rendered Intro FMV)".into(),
            next_script_id: Some("PM10100".into()),
            is_finished: false,
        };
        self.message_log.push("Movie OP10000 Started: Vol 1 Opening FMV".into());
    }

    /// Play a pre-rendered event movie (cmn/movie/vol1/ev/<file>).
    pub fn start_movie(&mut self, file: &str) {
        self.state = GameState::Movie { movie_file: file.into(), movie_title: file.into(), next_script_id: None, is_finished: false };
        self.message_log.push(format!("Movie {} started", file));
    }

    /// Start authentic Beast Statue cutscene (PM10140: Forsete's Altar).
    pub fn trigger_beast_statue_cutscene(&mut self) {
        self.state = GameState::Cutscene {
            script_id: "PM10140".into(),
            scene_title: "The Beast Statue Altar (Forsete's Treasure)".into(),
            current_step: 0,
            total_steps: 7,
            dialogue: Some(DialogueBox {
                speaker: "Ｉｙｏｔｅｎ".into(),
                portrait: Some("2iyo01".into()),
                text: String::new(), // game line (read from the player's copy at run time; not shipped)
                options: vec!["Next".into()],
            }),
            camera_eye: Vec3::new(3.53, -28.51, 0.12),
            camera_target: Vec3::new(-2.36, -28.53, 0.23),
            camera_fov: 45.0,
            is_finished: false,
        };
        self.message_log.push("Cutscene PM10140 Started: Beast Statue Altar".into());
    }

    /// Alias for backwards compatibility with tests.
    pub fn start_ovan_cutscene(&mut self) {
        self.trigger_beast_statue_cutscene();
    }

    /// Start the authentic opening story cutscene (PM10100: Welcome to Mac Anu).
    pub fn start_prologue_cutscene(&mut self) {
        self.desktop.login_to_world();
        self.state = GameState::Cutscene {
            script_id: "PM10100".into(),
            scene_title: "Prologue: Welcome to The World R:2 (Mac Anu)".into(),
            current_step: 0,
            total_steps: 6,
            dialogue: Some(DialogueBox {
                speaker: "System".into(),
                portrait: None,
                text: "Delta Server: Water Capital Mac Anu. Two veteran players spot an Adept Rogue newcomer arriving at the plaza...".into(),
                options: vec!["Next".into()],
            }),
            camera_eye: Vec3::new(-24.0, 133.0, 15.0),
            camera_target: Vec3::new(2.2, 137.0, 16.8),
            camera_fov: 65.0,
            is_finished: false,
        };
        self.message_log.push("Cutscene PM10100 Started: Welcome to Mac Anu".into());
    }

    /// Start the authentic arrival cutscene in the tutorial field (PM10105: Arrival at Delta Hidden Forbidden Holy Ground).
    pub fn start_prologue_field_cutscene(&mut self) {
        self.state = GameState::Cutscene {
            script_id: "PM10105".into(),
            scene_title: "Arrival at the Holy Ground (Tutorial Field)".into(),
            current_step: 0,
            total_steps: 4,
            dialogue: Some(DialogueBox {
                speaker: "Ｉｙｏｔｅｎ".into(),
                portrait: Some("2iyo01".into()),
                text: String::new(), // game line (read from the player's copy at run time; not shipped)
                options: vec!["Next".into()],
            }),
            camera_eye: Vec3::new(-70.45, -87.08, 5.60),
            camera_target: Vec3::new(-71.85, -82.52, 7.10),
            camera_fov: 45.0,
            is_finished: false,
        };
        self.message_log.push("Cutscene PM10105 Started: Arrival at the Holy Ground".into());
    }

    /// Start a retail event script by id; the renderer's EvPlayer drives it (camera, cast, text all from the script).
    pub fn start_script(&mut self, id: &str) {
        self.state = GameState::Cutscene {
            script_id: id.into(),
            scene_title: id.into(),
            current_step: 0,
            total_steps: 1,
            dialogue: None,
            camera_eye: Vec3::ZERO,
            camera_target: Vec3::Y,
            camera_fov: 45.0,
            is_finished: false,
        };
        self.message_log.push(format!("Event {} started", id));
    }

    /// Back to free movement in the current field (after a field event).
    pub fn resume_field(&mut self) {
        self.warp_to_selected_area();
        if let GameState::Field { ref mut enemies, ref mut active_dialogue, .. } = self.state {
            // Enemies in the tutorial field are spawned by the story controller (renderer), not Gemini's list.
            enemies.clear();
            *active_dialogue = None;
        }
    }

    pub fn trigger_betrayal_cutscene(&mut self) {
        self.state = GameState::Cutscene {
            script_id: "PM10135".into(),
            scene_title: "The Betrayal of the PKs (Gott Statue Altar)".into(),
            current_step: 0,
            total_steps: 7,
            dialogue: Some(DialogueBox {
                speaker: "Ｉｙｏｔｅｎ".into(),
                portrait: Some("2iyo01".into()),
                text: String::new(), // game line (read from the player's copy at run time; not shipped)
                options: vec!["Next".into()],
            }),
            camera_eye: Vec3::new(-1.31, 1.04, 1.38),
            camera_target: Vec3::new(0.38, -0.77, 1.20),
            camera_fov: 50.0,
            is_finished: false,
        };
        self.message_log.push("Cutscene PM10135 Started: The PK Betrayal".into());
    }

    /// Advance active cutscene by one dialogue step or transition to next movie in saga.
    pub fn advance_cutscene(&mut self) {
        if let GameState::Movie { ref movie_file, ref next_script_id, .. } = self.state {
            match movie_file.as_str() {
                "OP10000.usm" => {
                    self.start_prologue_cutscene();
                }
                "MM10145.usm" => {
                    // In the original (reference_captures/run2_field f01040-f01060) the ambush movie is followed by
                    // the ALTIMIT desktop and its "Reboot Complete" sequence; MM10210 (Atoli, Kuhn) comes later.
                    self.state = GameState::Desktop;
                    self.pending_desktop_sequence = Some("reboot".into());
                    self.reformatted = true;
                    self.message_log.push("Desktop: reboot after the PK ambush".into());
                }
                "MM10210.usm" => {
                    self.state = GameState::Movie {
                        movie_file: "MM10220.usm".into(),
                        movie_title: "Prologue: Confrontation with Iyoten & Asta in Mac Anu (Event FMV)".into(),
                        next_script_id: Some("MM10230.usm".into()),
                        is_finished: false,
                    };
                    self.message_log.push("Movie MM10220 Started: PK Interrogation in Mac Anu".into());
                }
                "MM10220.usm" => {
                    self.state = GameState::Movie {
                        movie_file: "MM10230.usm".into(),
                        movie_title: "Prologue: The Cathedral of Hulle Granz - Tri-Edge Appears (Event FMV)".into(),
                        next_script_id: Some("MM10240.usm".into()),
                        is_finished: false,
                    };
                    self.message_log.push("Movie MM10230 Started: The Cathedral at Hulle Granz".into());
                }
                "MM10230.usm" => {
                    self.state = GameState::Movie {
                        movie_file: "MM10240.usm".into(),
                        movie_title: "Prologue: Battle with Tri-Edge & The Data Drain (Event FMV)".into(),
                        next_script_id: Some("PM10140".into()),
                        is_finished: false,
                    };
                    self.message_log.push("Movie MM10240 Started: Tri-Edge Showdown & Data Drain".into());
                }
                "MM10240.usm" => {
                    // Data drained to Level 1, awakens in Mac Anu!
                    self.save.level = 1;
                    self.save.hp = 120;
                    self.save.max_hp = 120;
                    self.save.sp = 50;
                    self.save.max_sp = 50;
                    self.save.exp = 0;
                    self.return_to_town();
                    if let GameState::Town { ref mut active_dialogue, .. } = self.state {
                        *active_dialogue = Some(DialogueBox {
                            speaker: "Silabus".into(),
                            portrait: Some("2syl01".into()),
                            text: "Hey! You're awake! We found you collapsed by the gate. I'm Silabus, and this is Gaspard from guild Canard. Are you okay?".into(),
                            options: vec!["I'm alright".into(), "Who was that guy?".into()],
                        });
                    }
                    self.message_log.push("Data Drained to Level 1! Awakened in Mac Anu with Silabus & Gaspard.".into());
                }
                _ => {
                    let next = next_script_id.clone();
                    match next.as_deref() {
                        Some("PM10100") => self.start_prologue_cutscene(),
                        Some("PM10140") => self.start_ovan_cutscene(),
                        _ => self.return_to_town(),
                    }
                }
            }
            return;
        }

        if let GameState::Cutscene {
            ref script_id,
            ref mut current_step,
            total_steps,
            ref mut dialogue,
            ref mut camera_eye,
            ref mut camera_target,
            ref mut camera_fov,
            ref mut is_finished,
            ..
        } = self.state {
            *current_step += 1;
            if *current_step >= total_steps {
                *is_finished = true;
            } else {
                match (script_id.as_str(), *current_step) {
                    // PM10100: Mac Anu meeting with Iyoten & Asta
                    ("PM10100", 1) => {
                        *camera_eye = Vec3::new(-3.0, 137.0, 3.1);
                        *camera_target = Vec3::new(23.5, 141.0, 4.8);
                        *camera_fov = 55.0;
                        *dialogue = Some(DialogueBox {
                            speaker: "Iyoten".into(),
                            portrait: Some("2iyo01".into()),
                            text: "Hey there! First time in The World R:2? You look like you just spawned in!".into(),
                            options: vec!["Next".into()],
                        });
                    }
                    ("PM10100", 2) => {
                        *dialogue = Some(DialogueBox {
                            speaker: "Haseo".into(),
                            portrait: Some("2hsw01".into()),
                            text: "Yeah... I just finalized my character data. I'm Haseo. An Adept Rogue.".into(),
                            options: vec!["Next".into()],
                        });
                    }
                    ("PM10100", 3) => {
                        *dialogue = Some(DialogueBox {
                            speaker: "Asta".into(),
                            portrait: Some("2ast01".into()),
                            text: "Adept Rogue is tough to level, but versatile! We're experienced players—we can teach you the ropes!".into(),
                            options: vec!["Next".into()],
                        });
                    }
                    ("PM10100", 4) => {
                        *dialogue = Some(DialogueBox {
                            speaker: "Iyoten".into(),
                            portrait: Some("2iyo01".into()),
                            text: "How about we head to an easy beginner dungeon through the Chaos Gate? We'll watch your back and help you level up!".into(),
                            options: vec!["Next".into()],
                        });
                    }
                    ("PM10100", 5) => {
                        *dialogue = Some(DialogueBox {
                            speaker: "Haseo".into(),
                            portrait: Some("2hsw01".into()),
                            text: "Sounds great to me. Thanks, I appreciate the help. Lead the way!".into(),
                            options: vec!["Proceed to Chaos Gate".into()],
                        });
                    }

                    // PM10105: Tutorial field arrival with Iyoten & Asta
                    ("PM10105", 1) => {
                        *dialogue = Some(DialogueBox {
                            speaker: "アスタ".into(),
                            portrait: Some("2ast01".into()),
                            text: String::new(), // game line (read from the player's copy at run time; not shipped)
                            options: vec!["Next".into()],
                        });
                    }
                    ("PM10105", 2) => {
                        *dialogue = Some(DialogueBox {
                            speaker: "Ｉｙｏｔｅｎ".into(),
                            portrait: Some("2iyo01".into()),
                            text: String::new(), // game line (read from the player's copy at run time; not shipped)
                            options: vec!["Next".into()],
                        });
                    }
                    ("PM10105", 3) => {
                        *dialogue = Some(DialogueBox {
                            speaker: "ハセヲ".into(),
                            portrait: Some("2hsw01".into()),
                            text: String::new(), // game line (read from the player's copy at run time; not shipped)
                            options: vec!["Explore Field".into()],
                        });
                    }

                    // PM10140: Beast Statue Altar (Forsete's Treasure)
                    ("PM10140", 1) => {
                        *dialogue = Some(DialogueBox {
                            speaker: "Ｉｙｏｔｅｎ".into(),
                            portrait: Some("2iyo01".into()),
                            text: String::new(), // game line (read from the player's copy at run time; not shipped)
                            options: vec!["Next".into()],
                        });
                    }
                    ("PM10140", 2) => {
                        *dialogue = Some(DialogueBox {
                            speaker: "Ｉｙｏｔｅｎ".into(),
                            portrait: Some("2iyo01".into()),
                            text: String::new(), // game line (read from the player's copy at run time; not shipped)
                            options: vec!["Next".into()],
                        });
                    }
                    ("PM10140", 3) => {
                        *dialogue = Some(DialogueBox {
                            speaker: "Ｉｙｏｔｅｎ".into(),
                            portrait: Some("2iyo01".into()),
                            text: String::new(), // game line (read from the player's copy at run time; not shipped)
                            options: vec!["Next".into()],
                        });
                    }
                    ("PM10140", 4) => {
                        *dialogue = Some(DialogueBox {
                            speaker: "アスタ".into(),
                            portrait: Some("2ast01".into()),
                            text: String::new(), // game line (read from the player's copy at run time; not shipped)
                            options: vec!["Next".into()],
                        });
                    }
                    ("PM10140", 5) => {
                        *dialogue = Some(DialogueBox {
                            speaker: "ハセヲ".into(),
                            portrait: Some("2hsw01".into()),
                            text: String::new(), // game line (read from the player's copy at run time; not shipped)
                            options: vec!["Next".into()],
                        });
                    }
                    ("PM10140", 6) => {
                        *dialogue = Some(DialogueBox {
                            speaker: "Ｉｙｏｔｅｎ".into(),
                            portrait: Some("2iyo01".into()),
                            text: String::new(), // game line (read from the player's copy at run time; not shipped)
                            options: vec!["Open Chest".into()],
                        });
                    }

                    // PM10135: The PK Betrayal
                    ("PM10135", 1) => {
                        *dialogue = Some(DialogueBox {
                            speaker: "Ｉｙｏｔｅｎ".into(),
                            portrait: Some("2iyo01".into()),
                            text: String::new(), // game line (read from the player's copy at run time; not shipped)
                            options: vec!["Next".into()],
                        });
                    }
                    ("PM10135", 2) => {
                        *dialogue = Some(DialogueBox {
                            speaker: "アスタ".into(),
                            portrait: Some("2ast01".into()),
                            text: String::new(), // game line (read from the player's copy at run time; not shipped)
                            options: vec!["Next".into()],
                        });
                    }
                    ("PM10135", 3) => {
                        *dialogue = Some(DialogueBox {
                            speaker: "Ｉｙｏｔｅｎ".into(),
                            portrait: Some("2iyo01".into()),
                            text: String::new(), // game line (read from the player's copy at run time; not shipped)
                            options: vec!["Next".into()],
                        });
                    }
                    ("PM10135", 4) => {
                        *dialogue = Some(DialogueBox {
                            speaker: "ハセヲ".into(),
                            portrait: Some("2hsw01".into()),
                            text: String::new(), // game line (read from the player's copy at run time; not shipped)
                            options: vec!["Next".into()],
                        });
                    }
                    ("PM10135", 5) => {
                        *dialogue = Some(DialogueBox {
                            speaker: "Ｉｙｏｔｅｎ".into(),
                            portrait: Some("2iyo01".into()),
                            text: String::new(), // game line (read from the player's copy at run time; not shipped)
                            options: vec!["Next".into()],
                        });
                    }
                    ("PM10135", 6) => {
                        *dialogue = Some(DialogueBox {
                            speaker: "アスタ".into(),
                            portrait: Some("2ast01".into()),
                            text: String::new(), // game line (read from the player's copy at run time; not shipped)
                            options: vec!["Next".into()],
                        });
                    }
                    _ => {}
                }
            }
        }

        // Check if cutscene reached its conclusion to transition state
        let should_transition = if let GameState::Cutscene { is_finished, .. } = self.state {
            is_finished
        } else {
            false
        };

        if should_transition {
            self.conclude_active_cutscene();
        }
    }

    /// Skip active cutscene or movie immediately.
    pub fn skip_cutscene(&mut self) {
        if let GameState::Movie { ref movie_file, ref next_script_id, .. } = self.state {
            if movie_file == "MM10145.usm" {
                // Skipping the movie leads to the same place as watching it (the desktop reboot).
                self.advance_cutscene();
                return;
            }
            let next = next_script_id.clone();
            match next.as_deref() {
                Some("PM10100") => self.start_prologue_cutscene(),
                Some("PM10140") => self.trigger_beast_statue_cutscene(),
                _ => self.return_to_town(),
            }
            return;
        }
        if matches!(self.state, GameState::Cutscene { .. }) {
            self.conclude_active_cutscene();
        }
    }

    /// Complete current cutscene and route to appropriate next game state.
    pub fn conclude_active_cutscene(&mut self) {
        let script = if let GameState::Cutscene { ref script_id, .. } = self.state {
            script_id.clone()
        } else {
            return;
        };

        match script.as_str() {
            "PM10100" => {
                // The event ends on Iyoten's (game text); the renderer waits for that press and
                // opens the Main Menu, and the player then walks to the Chaos Gate (A opens it; its tutorial runs
                // there). GUESS: the original's exact order after the menu is not traced.
                self.state = GameState::Town { district: "Mac Anu".into(), active_dialogue: None, active_shop: None };
            }
            // Δ Courageous Engaging Daydream, in the original's order (reference_captures/run2_field):
            // PM10105 arrival -> PM10110 Platform talk -> free play; PM10120 / PM10125 open the altar fights;
            // PM10135 after the Symbol Fragments join; PM10140 at the Beast Statue -> chest -> MM10145.
            "PM10105" => self.start_script("PM10110"),
            "PM10110" | "PM10120" | "PM10125" | "PM10135" => self.resume_field(),
            // Mac Anu after the reformat: free movement (no dialogue box; the next lines come from walking about).
            "PM10310" => {
                self.state = GameState::Town { district: "Mac Anu".into(), active_dialogue: None, active_shop: None };
            }
            "PM10140" => {
                // Opening the offering chest plays the PK ambush movie (MM10145); the renderer starts it.
                self.resume_field();
            }
            "PM10145" => {
                self.state = GameState::Movie {
                    movie_file: "MM10145.usm".into(),
                    movie_title: "Ovan's Intervention - The Relic Arm (Pre-Rendered Event FMV)".into(),
                    next_script_id: None,
                    is_finished: false,
                };
                self.message_log.push("Movie MM10145: Ovan's Intervention FMV Started.".into());
            }
            _ => {
                self.return_to_town();
            }
        }
    }
}
