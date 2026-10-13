//! # CyberConnect2 Save Data System
//! Reconstructed directly from `ccSaveDataSys@system@gu`, `SaveDataManager::savedata::bl`,
//! and `ProcessSave` / `ProcessLoad`.
//!
//! Manages serialization, player progression, party roster, inventory,
//! quest flags, Chim Chim counts, and area word collections.

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use crate::pc::WeaponType;
use crate::area::AreaWord;

/// Haseo's Adept Rogue job form progression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HaseoForm {
    /// 1st Form: Twin Blades only (Rebirth start)
    FirstForm,
    /// 2nd Form: Twin Blades + Broadsword (Job Extend unlocked in Vol 1)
    SecondForm,
    /// 3rd Form: Twin Blades + Broadsword + Scythe (Vol 2)
    ThirdForm,
    /// B-st Form: Beast Out berserk form (Vol 2 climax)
    BStForm,
    /// 5th Form: Dual Guns (Vol 4 Last Recode)
    FifthForm,
}

impl HaseoForm {
    pub fn allowed_weapons(&self) -> Vec<WeaponType> {
        match self {
            HaseoForm::FirstForm => vec![WeaponType::TwinBlades],
            HaseoForm::SecondForm => vec![WeaponType::TwinBlades, WeaponType::Broadsword],
            HaseoForm::ThirdForm => vec![WeaponType::TwinBlades, WeaponType::Broadsword, WeaponType::Scythe],
            HaseoForm::BStForm => vec![WeaponType::Broadsword, WeaponType::Scythe],
            HaseoForm::FifthForm => vec![
                WeaponType::TwinBlades,
                WeaponType::Broadsword,
                WeaponType::Scythe,
                WeaponType::DualGuns,
            ],
        }
    }

    pub fn ccs_model(&self) -> &'static str {
        match self {
            HaseoForm::FirstForm => "data/pc/2hsw01.ccs",
            HaseoForm::SecondForm => "data/pc/2hsw02.ccs",
            HaseoForm::ThirdForm => "data/pc/2hsw03.ccs",
            HaseoForm::BStForm => "data/pc/2hsw04.ccs",
            HaseoForm::FifthForm => "data/pc/2hsw05.ccs",
        }
    }
}

/// A companion party member eligible for invitations into Haseo's party.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartyMemberData {
    pub id: u32,
    pub name: String,
    pub class_name: String,
    pub level: u32,
    pub hp: u32,
    pub max_hp: u32,
    pub sp: u32,
    pub max_sp: u32,
    pub affection: u32, // 0 to 1000
    pub is_unlocked: bool,
    pub in_active_party: bool,
}

/// An inventory slot containing an item and quantity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventorySlot {
    pub item_id: u32,
    pub name: String,
    pub quantity: u32,
}

/// Complete `.hack//G.U.` Save File representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveData {
    // Header & Meta
    pub volume: u8, // 1 = Rebirth, 2 = Reminisce, 3 = Redemption, 4 = Reconnection
    pub play_time_seconds: u64,
    pub save_title: String,
    pub save_timestamp: u64,

    // Haseo Player Character Data
    pub player_name: String,
    pub form: HaseoForm,
    pub level: u32,
    pub exp: u32,
    pub hp: u32,
    pub max_hp: u32,
    pub sp: u32,
    pub max_sp: u32,
    pub equipped_weapon: WeaponType,
    pub weapon_model: String,
    pub gp: u32,
    pub chim_count: u32,

    // Weapon Proficiencies (Level 1 to 10)
    pub twin_blade_skill_level: u8,
    pub broadsword_skill_level: u8,
    pub scythe_skill_level: u8,
    pub dual_gun_skill_level: u8,

    // Party Roster
    pub party_members: Vec<PartyMemberData>,

    // Inventory
    pub items: Vec<InventorySlot>,
    pub weapons: Vec<InventorySlot>,
    pub armors: Vec<InventorySlot>,
    pub accessories: Vec<InventorySlot>,

    // Chaos Gate Area Words
    pub unlocked_first_words: HashSet<String>,
    pub unlocked_second_words: HashSet<String>,
    pub unlocked_third_words: HashSet<String>,

    // World & Story State
    pub flags: HashMap<u32, i32>,
    pub mecha_grunty_rescues: u32,
    pub lucky_animals_kicked: u32,
    pub ryu_book_combat_wins: u32,
}

impl SaveData {
    /// Construct default new game save for Vol.1 (Rebirth).
    pub fn new_game_vol1(name: &str) -> Self {
        let mut first_words = HashSet::new();
        first_words.insert("Hidden".into());
        first_words.insert("Graceful".into());
        first_words.insert("Submissive".into());

        let mut second_words = HashSet::new();
        second_words.insert("Forbidden".into());
        second_words.insert("Brave".into());
        second_words.insert("Sun's".into());

        let mut third_words = HashSet::new();
        third_words.insert("Holy Ground".into());
        third_words.insert("Fast Food".into());
        third_words.insert("Fort".into());

        let mut flags = HashMap::new();
        flags.insert(1001, 1); // Game initialized
        flags.insert(1010, 0); // Prologue active

        let party_members = vec![
            PartyMemberData {
                id: 1,
                name: "Silabus".into(),
                class_name: "Blade Brandier".into(),
                level: 1,
                hp: 120,
                max_hp: 120,
                sp: 50,
                max_sp: 50,
                affection: 100,
                is_unlocked: true,
                in_active_party: true,
            },
            PartyMemberData {
                id: 2,
                name: "Gaspard".into(),
                class_name: "Shadow Warlock".into(),
                level: 1,
                hp: 90,
                max_hp: 90,
                sp: 80,
                max_sp: 80,
                affection: 100,
                is_unlocked: true,
                in_active_party: true,
            },
            PartyMemberData {
                id: 3,
                name: "Atoli".into(),
                class_name: "Harvest Cleric".into(),
                level: 3,
                hp: 110,
                max_hp: 110,
                sp: 100,
                max_sp: 100,
                affection: 150,
                is_unlocked: false,
                in_active_party: false,
            },
            PartyMemberData {
                id: 4,
                name: "Kuhn".into(),
                class_name: "Steam Gunner".into(),
                level: 5,
                hp: 200,
                max_hp: 200,
                sp: 70,
                max_sp: 70,
                affection: 120,
                is_unlocked: false,
                in_active_party: false,
            },
        ];

        let items = vec![
            InventorySlot { item_id: 1, name: "Health Drink".into(), quantity: 5 },
            InventorySlot { item_id: 2, name: "Sprite Drop".into(), quantity: 2 },
            InventorySlot { item_id: 3, name: "Smoke Screen".into(), quantity: 1 },
        ];

        Self {
            volume: 1,
            play_time_seconds: 0,
            save_title: "Mac Anu - Central District".into(),
            save_timestamp: 0,
            player_name: name.to_string(),
            form: HaseoForm::FirstForm,
            level: 1,
            exp: 0,
            hp: 140,
            max_hp: 140,
            sp: 60,
            max_sp: 60,
            equipped_weapon: WeaponType::TwinBlades,
            weapon_model: "data/eq/w2dsw00l.ccs".into(),
            // A new game starts with 0 GP (run1_opening f00064 shows 0 GP after the opening; 1000 was invented).
            gp: 0,
            chim_count: 0,
            twin_blade_skill_level: 1,
            broadsword_skill_level: 0,
            scythe_skill_level: 0,
            dual_gun_skill_level: 0,
            party_members,
            items,
            weapons: vec![
                InventorySlot { item_id: 10, name: "Spin GaiGu".into(), quantity: 1 },
            ],
            armors: vec![
                InventorySlot { item_id: 20, name: "Rough Leather".into(), quantity: 1 },
            ],
            accessories: vec![
                InventorySlot { item_id: 30, name: "Simple Wristband".into(), quantity: 1 },
            ],
            unlocked_first_words: first_words,
            unlocked_second_words: second_words,
            unlocked_third_words: third_words,
            flags,
            mecha_grunty_rescues: 0,
            lucky_animals_kicked: 0,
            ryu_book_combat_wins: 0,
        }
    }

    /// Unlock a new Area Word in the player's Chaos Gate dictionary.
    pub fn unlock_word(&mut self, word: &AreaWord) {
        match word.category {
            crate::area::WordCategory::PartA => {
                self.unlocked_first_words.insert(word.name.clone());
            }
            crate::area::WordCategory::PartB => {
                self.unlocked_second_words.insert(word.name.clone());
            }
            crate::area::WordCategory::PartC => {
                self.unlocked_third_words.insert(word.name.clone());
            }
        }
    }

    /// Add Chim Chim or GP reward.
    pub fn add_chim(&mut self, count: u32) {
        self.chim_count = (self.chim_count + count).min(99);
    }

    pub fn add_gp(&mut self, amount: u32) {
        self.gp = self.gp.saturating_add(amount);
    }

    /// Set or update a story flag.
    pub fn set_flag(&mut self, flag_id: u32, value: i32) {
        self.flags.insert(flag_id, value);
    }

    pub fn get_flag(&self, flag_id: u32) -> i32 {
        *self.flags.get(&flag_id).unwrap_or(&0)
    }

    /// Serialize save data to JSON string for readable storage or modding.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize save data from JSON string.
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }
}
