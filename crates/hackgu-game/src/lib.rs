//! # hackgu-game
//! CyberConnect2 Game Engine Architecture & Object Simulation in pure Rust.
//! Reconstructed directly from decompiled `.hack//G.U.` engine binaries.

pub mod object;
pub mod context;
pub mod pc;
pub mod npc;
pub mod gimmick;
pub mod event;
pub mod battle;
pub mod area;
pub mod enemy;
pub mod savedata;
pub mod bike;
pub mod shop;
pub mod quest;
pub mod desktop;
pub mod arena;
pub mod customize;
pub mod runtime;
pub mod ev;
pub mod msginfo;
pub mod msgtable;
pub mod areaword;

pub use object::GameObj;
pub use context::WorldContext;
pub use pc::{PlayerCharacter, PlayerLocomotionState, WeaponType};
pub use npc::{NonPlayerCharacter, SubPlayerCharacter};
pub use gimmick::{ChaosGate, TimBarrel, ItemBox, ChestKind, TownWarp};
pub use event::{GameEvent, EventQueue, EventScript};
pub use battle::{BattlePlayer, CombatStats, CombatState, BattleAreaBarrier};
pub use area::{
    AreaWord, AreaWordDatabase, GeneratedArea, RootServer, FieldKind,
    WeatherKind, ElementKind, MissionType, WordCategory,
};
pub use enemy::{EnemyEntity, EnemyArchetype, EnemyFieldState, EnemyCombatState, EnemyStats};
pub use savedata::{SaveData, HaseoForm, PartyMemberData, InventorySlot};
pub use bike::{SteamBike, BikeParts, BikeStats, BikeRideState};
pub use shop::{ShopCatalog, ShopKind, ShopItemListing, ShopError, NpcTradeDesk};
pub use quest::{QuestManager, QuestDef, QuestStatus, QuestObjective, QuestRewards, QuestError};
pub use desktop::{AltimitDesktop, MailMessage, MailReplyOption, BbsThread, BbsPost, BbsBoardCategory};
pub use arena::{ArenaLadder, ArenaMatch, ArenaMatchType, ArenaTeam, ArenaScoreCard, ArenaCombatantDef};
pub use customize::{CustomizedGear, SlotEffect, CustomizeWorkbench, CustomizeError};
pub use runtime::{GameRuntime, GameState, DialogueBox, BattleEncounter, CombatOutcome};

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec3;
    use hackgu_world::{CollisionMesh, CollisionTriangle};

    #[test]
    fn test_player_locomotion_ground_clamping() {
        let mut world = WorldContext::new();

        // Create an authentic floor at Z = 12.0 (same as Mac Anu Plaza)
        let t1 = CollisionTriangle::new(
            Vec3::new(-10.0, -10.0, 12.0),
            Vec3::new(10.0, -10.0, 12.0),
            Vec3::new(10.0, 10.0, 12.0),
        ).unwrap();
        let t2 = CollisionTriangle::new(
            Vec3::new(-10.0, -10.0, 12.0),
            Vec3::new(10.0, 10.0, 12.0),
            Vec3::new(-10.0, 10.0, 12.0),
        ).unwrap();
        world.set_collision_mesh(CollisionMesh::from_triangles(vec![t1, t2], 4.0));

        // Spawn player at (0.0, 0.0, 12.0)
        let mut player = PlayerCharacter::new(Vec3::new(0.0, 0.0, 12.0), 0.0);
        assert_eq!(player.position.z, 12.0);

        // Advance 1 second with forward movement
        player.process_input(1.0, 1.0, 0.0, 0.0, &world);

        // Player should have moved forward and remain clamped at Z = 12.0
        assert!(player.position.y > 0.0);
        assert_eq!(player.position.z, 12.0);
        assert_eq!(player.state, PlayerLocomotionState::Running);
    }

    #[test]
    fn test_chest_interaction() {
        let mut world = WorldContext::new();
        let mut chest = ItemBox::new(10, Vec3::new(5.0, 0.0, 0.0), ChestKind::Silver, "Health Drink");

        let ev1 = chest.on_interact(1, &mut world).unwrap();
        assert_eq!(ev1, GameEvent::ShowMessage {
            speaker: "Treasure".into(),
            text: "Obtained [Health Drink]!".into(),
        });
        assert!(chest.is_opened);

        let ev2 = chest.on_interact(1, &mut world).unwrap();
        assert_eq!(ev2, GameEvent::ShowMessage {
            speaker: "Treasure".into(),
            text: "The treasure chest is empty.".into(),
        });
    }

    #[test]
    fn test_chim_barrel_kicking() {
        let mut world = WorldContext::new();
        let mut barrel = TimBarrel::new(20, Vec3::new(0.0, 5.0, 0.0));

        let ev1 = barrel.on_interact(1, &mut world).unwrap();
        assert_eq!(ev1, GameEvent::KickChimBarrel {
            barrel_id: 20,
            chim_count: 5,
        });
        assert_eq!(barrel.remaining_kicks, 2);
    }

    #[test]
    fn test_battle_simulation() {
        let stats = CombatStats::new(450, 120, 35, 20, 1);
        let mut bpc = BattlePlayer::new(1, "Haseo", stats, Vec3::new(0.0, 0.0, 0.0));

        // Attack combo step
        bpc.execute_attack();
        assert_eq!(bpc.state, CombatState::Attacking { combo_step: 1, timer: 0.45 });
        assert_eq!(bpc.rengeki_hits, 1);
        assert_eq!(bpc.morale_gauge, 5.0);

        // Take damage
        bpc.take_damage(50);
        assert_eq!(bpc.stats.current_hp, 400);
        assert_eq!(bpc.state, CombatState::HitStun { duration: 0.3 });

        // Barrier constrain
        let barrier = BattleAreaBarrier::new(Vec3::new(0.0, 0.0, 0.0), 10.0);
        let mut out_pos = Vec3::new(15.0, 0.0, 0.0);
        barrier.constrain_position(&mut out_pos);
        assert!((out_pos.x - 10.0).abs() < 1e-4);
    }

    #[test]
    fn test_area_word_synthesis() {
        let db = AreaWordDatabase::new_vol1_defaults();

        // Authentic Vol.1 Opening Lost Ground
        let area = db.generate(
            RootServer::Delta,
            "Hidden",
            "Forbidden",
            "Holy Ground",
        ).expect("Valid keyword combination");

        assert_eq!(area.full_name, "Delta Hidden Forbidden Holy Ground");
        assert_eq!(area.area_level, 1);
        assert_eq!(area.mission, MissionType::SpecialQuest);
        assert_eq!(area.server, RootServer::Delta);

        // High-level dungeon
        let dungeon = db.generate(
            RootServer::Delta,
            "Plenteous",
            "Raging",
            "Nemesis",
        ).expect("Valid high level combo");

        assert!(dungeon.area_level >= 45);
        assert_eq!(dungeon.element, ElementKind::Water);
    }

    #[test]
    fn test_event_csv_parsing() {
        let sample_csv = r#"PM10100,,[0.00|-260.00|0.00],3.141593,0,
,,,,62,,,,,,,,,,,
,,,,63,SPC,2atl01,Atoli,101,,,,,,,
,,,,63,PC,2hsw01,Haseo,1,,,,,,,
cutA01,,,,60,,,,,,Ver0003:2:[-243.74|13348.30|1516.10]:[22.35|13737.46|1682.68]:0.00:65.00:5.00:0:1:,Ver0001:0:
,,,,32,,Haseo,[205.08|21554.81|-1215.00],-163.5,ON,,,,,,
,,,,7,,Haseo,nut00,0,,,,,,,
,,,,4,,60,,,,,,,,,
"#;

        let script = EventScript::parse_csv(sample_csv);
        assert_eq!(script.name, "PM10100");
        assert_eq!(script.spawn_pos, Some(Vec3::new(0.0, -260.0, 0.0)));
        assert!(script.events.len() >= 5);

        // Verify player registration and placement parsed
        let has_reg = script.events.iter().any(|ev| matches!(ev, GameEvent::ActorRegister { instance_name, .. } if instance_name == "Haseo"));
        assert!(has_reg);

        let has_place = script.events.iter().any(|ev| matches!(ev, GameEvent::ActorPlace { instance_name, .. } if instance_name == "Haseo"));
        assert!(has_place);

        let has_wait = script.events.iter().any(|ev| matches!(ev, GameEvent::Wait { duration } if (*duration - 2.0).abs() < 1e-4));
        assert!(has_wait);
    }

    #[test]
    fn test_enemy_field_ai_and_combat() {
        let mut enemy = EnemyEntity::new(
            100,
            "Goblin",
            EnemyArchetype::SmallGoblin,
            Vec3::new(10.0, 0.0, 0.0),
            5,
        );

        let world = WorldContext::new();

        // Player far away -> Idle
        enemy.update_field(0.1, Vec3::new(100.0, 0.0, 0.0), &world);
        assert!(matches!(enemy.field_state, EnemyFieldState::Idle { .. }));

        // Player approaches inside detection radius (12m) -> Alert
        enemy.update_field(0.1, Vec3::new(15.0, 0.0, 0.0), &world);
        assert!(matches!(enemy.field_state, EnemyFieldState::Alert { .. }));

        // Combat damage processing
        let hp_before = enemy.stats.current_hp;
        let dmg = enemy.take_damage(40, false, None);
        assert!(dmg > 0);
        assert_eq!(enemy.stats.current_hp, hp_before - dmg);
        assert!(matches!(enemy.combat_state, EnemyCombatState::HitStun { .. }));

        // Rengeki combo attack triggers knockdown
        enemy.take_damage(60, true, Some(Vec3::new(0.0, 2.0, 0.0)));
        assert!(matches!(enemy.combat_state, EnemyCombatState::KnockedDown { .. }));
    }

    #[test]
    fn test_savedata_progression_and_serialization() {
        let mut save = SaveData::new_game_vol1("Haseo");
        assert_eq!(save.level, 1);
        assert_eq!(save.form, HaseoForm::FirstForm);
        assert_eq!(save.chim_count, 0);

        // Add rewards
        save.add_chim(15);
        save.add_gp(500);
        assert_eq!(save.chim_count, 15);
        assert_eq!(save.gp, 500);

        // Unlock an area word
        let db = AreaWordDatabase::new_vol1_defaults();
        let word = &db.first_words[4]; // "Coimhre"
        save.unlock_word(word);
        assert!(save.unlocked_first_words.contains("Coimhre"));

        // Roundtrip JSON serialization
        let json_str = save.to_json().expect("Save serializes to JSON");
        assert!(json_str.contains("Haseo"));
        assert!(json_str.contains("Coimhre"));

        let loaded = SaveData::from_json(&json_str).expect("Save deserializes from JSON");
        assert_eq!(loaded.player_name, save.player_name);
        assert_eq!(loaded.gp, 500);
        assert_eq!(loaded.chim_count, 15);
        assert_eq!(loaded.unlocked_first_words, save.unlocked_first_words);
    }

    #[test]
    fn test_steam_bike_physics_and_boost() {
        let mut bike = SteamBike::new(50, Vec3::new(0.0, 0.0, 0.0), 0.0);
        let world = WorldContext::new();

        assert_eq!(bike.ride_state, BikeRideState::Dismounted);
        bike.mount();
        assert!(matches!(bike.ride_state, BikeRideState::Mounting { .. }));

        // Finish mounting transition
        bike.process_input(1.0, 0.0, false, false, 0.7, &world);
        assert_eq!(bike.ride_state, BikeRideState::Cruising);
        assert!(bike.current_speed > 0.0);

        // Engage boost
        let fuel_before = bike.fuel;
        bike.process_input(1.0, 0.0, true, false, 0.5, &world);
        assert_eq!(bike.ride_state, BikeRideState::Boosting);
        assert!(bike.fuel < fuel_before);
        assert!(bike.current_speed > 10.0);

        // Preemptive strike ram
        let ev = bike.trigger_enemy_ram(100);
        assert_eq!(ev, GameEvent::BattleTransition {
            encounter_id: 100,
            bgm_override: None,
        });
        assert!(matches!(bike.ride_state, BikeRideState::Crash { .. }));
        assert_eq!(bike.current_speed, 0.0);
    }

    #[test]
    fn test_shop_purchase_and_barter() {
        let mut shop = ShopCatalog::new_mac_anu_item_shop();
        let mut save = SaveData::new_game_vol1("Haseo");
        save.add_gp(1000);
        let initial_gp = save.gp;

        // Buy 2 Health Drinks (100 GP each)
        shop.buy(1, 2, &mut save).expect("Purchase should succeed");
        assert_eq!(save.gp, initial_gp - 200);

        let drink_slot = save.items.iter().find(|s| s.item_id == 1).unwrap();
        assert_eq!(drink_slot.quantity, 7); // 5 initial + 2 bought

        // Sell 1 Health Drink (25 GP)
        let earnings = shop.sell(1, 1, &mut save).expect("Sale should succeed");
        assert_eq!(earnings, 25);
        assert_eq!(save.gp, initial_gp - 175);

        // Barter trade with NPC
        let mut trade = NpcTradeDesk::new(
            "Trader Bob",
            vec![ShopItemListing {
                item_id: 99,
                name: "Ancient Talisman".into(),
                buy_price: 100,
                sell_price: 25,
                stock: Some(1),
                required_story_flag: None,
            }],
        );

        // Trade 2 Health Drinks (50 * 2 = 100 value) for Ancient Talisman
        let result = trade.execute_trade(99, &[(1, 2)], &mut save);
        assert!(result.is_ok());
        let has_talisman = save.items.iter().any(|s| s.item_id == 99 && s.name == "Ancient Talisman");
        assert!(has_talisman);
    }

    #[test]
    fn test_quest_progression_and_turnin() {
        let mut qm = QuestManager::new_vol1_defaults();
        let mut save = SaveData::new_game_vol1("Haseo");

        // Accept starter quest (requires Silabus and Gaspard in active party)
        let res = qm.accept_quest(101, 3, &save.party_members);
        assert!(res.is_ok());
        assert_eq!(qm.active_quest_id, Some(101));

        // Attempt completion before objective is met -> Error
        let premature = qm.complete_active_quest(&mut save);
        assert_eq!(premature, Err(QuestError::ObjectiveNotComplete));

        // Report 3 Steam Cores collected
        qm.report_item_collected("Steam Core", 3);

        // Complete quest and claim rewards
        let gp_before = save.gp;
        let rewards = qm.complete_active_quest(&mut save).expect("Quest should complete");
        assert_eq!(save.gp, gp_before + rewards.gp);
        assert!(save.unlocked_first_words.contains("Counting"));
        assert_eq!(qm.active_quest_id, None);
    }

    #[test]
    fn test_altimit_desktop_mail_and_bbs() {
        let mut dt = AltimitDesktop::new_vol1_defaults();
        let mut save = SaveData::new_game_vol1("Haseo");

        assert_eq!(dt.unread_mail_count, 2);

        // Read Atoli's mail
        let mail = dt.read_email(1).expect("Email 1 exists");
        assert!(mail.is_read);
        assert_eq!(dt.unread_mail_count, 1);

        // Reply to Atoli's email with positive response
        let atoli_before = save.party_members.iter().find(|m| m.name == "Atoli").unwrap().affection;
        let resp = dt.reply_email(1, 0, &mut save).expect("Reply succeeds");
        assert!(resp.contains("Central Dome"));
        let atoli_after = save.party_members.iter().find(|m| m.name == "Atoli").unwrap().affection;
        assert_eq!(atoli_after, atoli_before + 30);

        // Read BBS thread 102 and harvest secret keywords
        let thread = dt.read_bbs_thread(102, &mut save).expect("Thread 102 exists");
        assert_eq!(thread.title, "Strange light in Delta Cavern");
        assert!(save.unlocked_first_words.contains("Submissive"));
        assert!(save.unlocked_second_words.contains("Wrath"));
        assert!(save.unlocked_third_words.contains("Corpse"));
    }

    #[test]
    fn test_arena_tournament_match_and_ladder() {
        let save = SaveData::new_game_vol1("Haseo");
        let mut ladder = ArenaLadder::new_vol1_defaults();
        assert_eq!(ladder.current_rank, 200);

        let team_bordeaux = &ladder.opponent_roster[0];
        let mut match_sim = ArenaMatch::new(
            ArenaMatchType::TournamentRankMatch,
            &save,
            team_bordeaux,
        );

        assert_eq!(match_sim.player_team.len(), 3); // Haseo + Silabus + Gaspard
        assert_eq!(match_sim.opponent_team.len(), 3); // Bordeaux + Negimaru + Grein
        assert_eq!(match_sim.is_finished, false);

        // Defeat opponent team
        for opp in &mut match_sim.opponent_team {
            opp.stats.current_hp = 0;
        }

        let score = match_sim.update(1.0).expect("Match finishes on wipe");
        assert!(match_sim.player_won);
        assert!(score.total_wp >= 100);

        // Record victory on ladder
        ladder.record_victory(&score);
        assert!(ladder.current_rank < 200);
        assert!(ladder.accumulated_wp >= 100);
    }

    #[test]
    fn test_equipment_alchemy_and_slotting() {
        let mut save = SaveData::new_game_vol1("Haseo");
        let mut weapon = CustomizedGear::new(10, "Spin GaiGu", 25, 2);

        assert_eq!(weapon.effective_power(), 25);
        assert_eq!(weapon.display_name(), "Spin GaiGu");

        // Perform alchemy upgrade with duplicate weapon in inventory
        let level = CustomizeWorkbench::fuse_alchemy(&mut weapon, 10, &mut save).expect("Alchemy succeeds");
        assert_eq!(level, 1);
        assert_eq!(weapon.display_name(), "Spin GaiGu +1");
        assert!(weapon.effective_power() > 25);

        // Install a slot effect (Fire Affinity using an item catalyst)
        let slot_res = CustomizeWorkbench::install_slot(
            &mut weapon,
            SlotEffect::ElementalAffinity(ElementKind::Fire),
            1, // Health Drink as catalyst
            &mut save,
        );
        assert!(slot_res.is_ok());
        assert_eq!(weapon.installed_slots.len(), 1);
        assert_eq!(weapon.installed_slots[0], SlotEffect::ElementalAffinity(ElementKind::Fire));
    }

    #[test]
    fn test_first_part_full_gameplay_loop() {
        // 1. Initialize Game at Title Screen & enter Desktop
        let mut runtime = GameRuntime::new_game("Haseo");
        assert!(matches!(runtime.state, GameState::TitleScreen));
        runtime.to_desktop();
        assert!(matches!(runtime.state, GameState::Desktop));

        // 2. Desktop Activities: Check Mail & Read Forum
        let mail = runtime.desktop.read_email(1).expect("Atoli's email exists");
        assert_eq!(mail.sender, "Atoli");
        let reply_res = runtime.desktop.reply_email(1, 0, &mut runtime.save);
        assert!(reply_res.is_ok());

        // Read Forum BBS to learn secret Area Keywords
        runtime.desktop.read_bbs_thread(102, &mut runtime.save);
        assert!(runtime.save.unlocked_first_words.contains("Submissive"));

        // 3. Log In to The World -> Spawns in Mac Anu Central District
        runtime.login_to_the_world();
        assert!(matches!(runtime.state, GameState::Town { .. }));

        // 4. Interact with NPCs in Town
        runtime.talk_to_npc("Atoli");
        if let GameState::Town { ref active_dialogue, .. } = runtime.state {
            assert!(active_dialogue.as_ref().unwrap().text.contains("Mac Anu"));
        }
        runtime.close_popup();

        // 5. Open Chaos Gate & Select Area Keywords
        runtime.open_chaos_gate();
        assert!(matches!(runtime.state, GameState::ChaosGateSelector { .. }));

        // Select "Delta Hidden Forbidden Holy Ground" (keywords 0, 0, 0)
        runtime.select_keywords(0, 0, 0);

        // 6. Gate In to Field
        runtime.warp_to_selected_area();
        assert!(matches!(runtime.state, GameState::Field { .. }));

        // 7. Field Gimmicks: Open Chest at (15.0, 10.0, 0.0) & Kick Chim Barrel at (8.0, 5.0, 0.0)
        let chest_res = runtime.interact_nearest(Vec3::new(15.0, 10.0, 0.0));
        assert!(chest_res.is_some());
        assert!(chest_res.unwrap().contains("Health Drink"));

        let chim_res = runtime.interact_nearest(Vec3::new(8.0, 5.0, 0.0));
        assert!(chim_res.is_some());
        assert!(runtime.save.chim_count > 0);

        // 8. Engage Blade Goblin in Combat
        runtime.engage_combat(0);
        if let GameState::Field { ref active_encounter, .. } = runtime.state {
            assert!(active_encounter.is_some());
        }

        // Execute Attacks to build Combo and Rengeki meter
        let hit1 = runtime.combat_attack();
        assert!(matches!(hit1, CombatOutcome::Hit { is_combo: true, is_rengeki: false, .. }));
        let hit2 = runtime.combat_attack(); // Fills Rengeki meter to 100%
        assert!(matches!(hit2, CombatOutcome::Hit { is_combo: true, is_rengeki: false, .. }));

        // Execute Rengeki Finish -> Defeats monster and rewards EXP/GP!
        let rengeki_res = runtime.combat_rengeki();
        assert!(matches!(rengeki_res, CombatOutcome::EnemyDefeated { .. }));

        // 9. Reach Gott Statue at (50.0, 0.0, 0.0) and Clear Field
        let statue_res = runtime.interact_nearest(Vec3::new(50.0, 0.0, 0.0));
        assert!(statue_res.is_some());
        assert!(statue_res.unwrap().contains("Gott Statue Cleared"));

        // 10. Warp back to Mac Anu & Log Out to Desktop
        runtime.return_to_town();
        assert!(matches!(runtime.state, GameState::Town { .. }));

        runtime.logout_to_desktop();
        assert!(matches!(runtime.state, GameState::Desktop));
    }

    #[test]
    fn test_surprise_attack_and_item_usage() {
        let mut runtime = GameRuntime::new_game("Haseo");
        runtime.login_to_the_world();
        runtime.open_chaos_gate();
        runtime.select_keywords(0, 0, 0);
        runtime.warp_to_selected_area();
        assert!(runtime.is_in_field());

        // Surprise attack on enemy 0
        runtime.surprise_attack(0);
        assert!(runtime.is_in_combat());
        if let GameState::Field { ref active_encounter, .. } = runtime.state {
            let enc = active_encounter.as_ref().unwrap();
            assert_eq!(enc.rengeki_meter, 60.0);
        }

        // Hit once to charge remaining meter to 100%
        let hit = runtime.combat_attack();
        assert!(matches!(hit, CombatOutcome::Hit { .. }));

        // Finish with Rengeki
        let outcome = runtime.combat_rengeki();
        assert!(matches!(outcome, CombatOutcome::EnemyDefeated { .. }));
        assert!(!runtime.is_in_combat());

        // Use Health Drink obtained earlier
        runtime.save.hp = 100;
        runtime.save.items.push(crate::savedata::InventorySlot {
            item_id: 1,
            name: "Health Drink".into(),
            quantity: 2,
        });
        assert!(runtime.use_item("Health Drink"));
        assert_eq!(runtime.save.hp, 140); // Clamped to max_hp (140)
        let count = runtime.save.items.iter().find(|s| s.name == "Health Drink").unwrap().quantity;
        assert_eq!(count, 4); // 3 (starter) + 2 (added) - 1 (used) = 4
    }

    #[test]
    fn test_initial_cutscenes_prologue_chain() {
        let mut runtime = GameRuntime::new_game("Haseo");
        assert!(matches!(runtime.state, GameState::TitleScreen));

        // 1. Start Opening Cutscene PM10100 (Welcome to Mac Anu)
        runtime.start_prologue_cutscene();
        if let GameState::Cutscene { ref script_id, current_step, .. } = runtime.state {
            assert_eq!(script_id, "PM10100");
            assert_eq!(current_step, 0);
        } else {
            panic!("Expected Cutscene PM10100");
        }

        // Advance through steps of PM10100 (meeting Iyoten & Asta)
        for _ in 0..6 {
            runtime.advance_cutscene();
        }

        // 2. Back in Mac Anu (the Main Menu prompt, then the player walks to the Chaos Gate)
        assert!(matches!(runtime.state, GameState::Town { .. }));
        runtime.open_chaos_gate();
        assert!(matches!(runtime.state, GameState::ChaosGateSelector { .. }));

        // Warp through Chaos Gate into tutorial Field
        runtime.warp_to_selected_area();

        // 3. Successfully gates into Field
        assert!(runtime.is_in_field());

        // 4. Reach Gott Statue -> Triggers PM10140 (Forsete's Beast Statue Altar)
        let statue_res = runtime.interact_nearest(Vec3::new(50.0, 0.0, 0.0));
        assert!(statue_res.is_some());
        if let GameState::Cutscene { ref script_id, .. } = runtime.state {
            assert_eq!(script_id, "PM10140");
        } else {
            panic!("Expected Cutscene PM10140 upon Gott Statue clear");
        }

        // Advance through steps of PM10140 (Iyoten & Asta encouraging Haseo to open chest)
        for _ in 0..7 {
            runtime.advance_cutscene();
        }

        // 5. Concludes PM10140 and transitions to PM10145 (Ovan's Savior Arrival)
        runtime.start_script("PM10145");
        runtime.advance_cutscene();

        // 6. Automatically transitions to Pre-rendered Movie MM10145 (Ovan's Savior Arrival)
        assert!(matches!(runtime.state, GameState::Movie { ref movie_file, .. } if movie_file == "MM10145.usm"));

        // 7. The original follows the movie with the ALTIMIT desktop reboot (reference_captures/run2_field f01040+).
        runtime.advance_cutscene();
        assert!(matches!(runtime.state, GameState::Desktop));
        assert_eq!(runtime.pending_desktop_sequence.as_deref(), Some("reboot"));
        assert!(runtime.reformatted);

        // 8. The next log-in plays PM10310 at Mac Anu's gate ((game text)), then free movement.
        runtime.login_to_the_world();
        assert!(matches!(runtime.state, GameState::Cutscene { ref script_id, .. } if script_id == "PM10310"));
        runtime.conclude_active_cutscene();
        assert!(matches!(runtime.state, GameState::Town { active_dialogue: None, .. }));
    }
}
