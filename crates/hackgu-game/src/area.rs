//! # Area Keyword Generation System
//! Reconstructed from CC2 `CGAreaWord`, `CGMWordCtrl`, and area generation tables.
//!
//! In `.hack//G.U.`, the Chaos Gate uses a 3-keyword combination to generate instanced
//! adventure zones across root servers (Delta, Theta, Sigma).

use serde::{Deserialize, Serialize};

/// The Root Server governing access to particular level brackets and story chapters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RootServer {
    /// Delta Server: Vol 1 (Rebirth) - Levels 1 to 50 (Mac Anu root)
    Delta,
    /// Theta Server: Vol 2 (Reminisce) - Levels 50 to 100 (Dol Dona root)
    Theta,
    /// Sigma Server: Vol 3 (Redemption) - Levels 100 to 150 (Breg Epona root)
    Sigma,
}

impl RootServer {
    pub fn name(&self) -> &'static str {
        match self {
            RootServer::Delta => "Delta",
            RootServer::Theta => "Theta",
            RootServer::Sigma => "Sigma",
        }
    }

    pub fn level_range(&self) -> (u32, u32) {
        match self {
            RootServer::Delta => (1, 50),
            RootServer::Theta => (50, 100),
            RootServer::Sigma => (100, 150),
        }
    }
}

/// The environmental landscape / stage geometry archetype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FieldKind {
    /// Rolling plains with ruins and water streams (`sf3_00.ccs`)
    Grassland,
    /// Arid desert canyons and stone plateaus
    Wasteland,
    /// Underground caverns with crystal formations and torchlight (`dungeon2.ccs`)
    Cave,
    /// Sacred temple corridors and floating obelisks
    Shrine,
}

impl FieldKind {
    pub fn ccs_asset(&self) -> &'static str {
        match self {
            FieldKind::Grassland => "data/field/sf3_00.ccs",
            FieldKind::Wasteland => "data/field/sf3_01.ccs",
            FieldKind::Cave => "data/dungeon/dungeon2.ccs",
            FieldKind::Shrine => "data/dungeon/d02_obj.ccs",
        }
    }
}

/// Atmospheric lighting and sky dome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeatherKind {
    Clear,
    Cloudy,
    Rain,
    Sunset,
    Night,
    Fog,
}

impl WeatherKind {
    pub fn sky_asset(&self) -> &'static str {
        match self {
            WeatherKind::Clear | WeatherKind::Cloudy => "data/field/sky_00.ccs",
            WeatherKind::Sunset | WeatherKind::Night => "data/field/sky_02.ccs",
            WeatherKind::Rain | WeatherKind::Fog => "data/field/sky_00.ccs",
        }
    }
}

/// Elemental affinity that grants buffs to corresponding monster types and governs hazard pools.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElementKind {
    Neutral,
    Fire,
    Water,
    Wind,
    Earth,
    Light,
    Dark,
}

/// The primary goal/objective for conquering the field or dungeon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MissionType {
    /// Navigate to the final room and open the Gott Statue treasure chest
    GottStatue,
    /// Hunt down and defeat the designated boss monster
    DefeatBoss,
    /// Kick 10 Chim Chims or trigger all steam valves
    ChimChimChallenge,
    /// Story / Lost Grounds sacred encounter (e.g., Hidden Forbidden Holy Ground)
    SpecialQuest,
}

/// A single keyword entry in the Chaos Gate database.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AreaWord {
    pub id: u16,
    pub name: String,
    pub category: WordCategory,
    pub level_mod: i32,
    pub element: ElementKind,
    pub grade: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WordCategory {
    PartA, // First Word (Element & Base Level)
    PartB, // Second Word (Layout & Weather)
    PartC, // Third Word (Field Kind & Mission)
}

/// Fully generated Zone configuration resolved from the 3 keywords.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneratedArea {
    pub server: RootServer,
    pub word_a: AreaWord,
    pub word_b: AreaWord,
    pub word_c: AreaWord,
    pub full_name: String,
    pub area_level: u32,
    pub field_kind: FieldKind,
    pub weather: WeatherKind,
    pub element: ElementKind,
    pub mission: MissionType,
    pub floors: u32,
    pub enemy_spawn_count: u32,
    pub chest_count: u32,
    pub gott_statue_tier: u8,
}

/// The Area Word Registry and Generator engine.
#[derive(Debug, Clone, Default)]
pub struct AreaWordDatabase {
    pub first_words: Vec<AreaWord>,
    pub second_words: Vec<AreaWord>,
    pub third_words: Vec<AreaWord>,
}

impl AreaWordDatabase {
    /// Construct the authentic Vol.1 Rebirth keyword dictionary.
    pub fn new_vol1_defaults() -> Self {
        let first_words = vec![
            AreaWord { id: 1, name: "Hidden".into(), category: WordCategory::PartA, level_mod: 1, element: ElementKind::Neutral, grade: 1 },
            AreaWord { id: 2, name: "Courageous (勇み行く)".into(), category: WordCategory::PartA, level_mod: 1, element: ElementKind::Neutral, grade: 1 },
            AreaWord { id: 3, name: "Graceful".into(), category: WordCategory::PartA, level_mod: 3, element: ElementKind::Water, grade: 1 },
            AreaWord { id: 4, name: "Submissive".into(), category: WordCategory::PartA, level_mod: 6, element: ElementKind::Earth, grade: 1 },
            AreaWord { id: 5, name: "Coimhre".into(), category: WordCategory::PartA, level_mod: 14, element: ElementKind::Wind, grade: 2 },
            AreaWord { id: 6, name: "Sneering".into(), category: WordCategory::PartA, level_mod: 10, element: ElementKind::Dark, grade: 1 },
            AreaWord { id: 7, name: "Lazy".into(), category: WordCategory::PartA, level_mod: 18, element: ElementKind::Earth, grade: 2 },
            AreaWord { id: 8, name: "Counting".into(), category: WordCategory::PartA, level_mod: 22, element: ElementKind::Light, grade: 2 },
            AreaWord { id: 9, name: "Unknowing".into(), category: WordCategory::PartA, level_mod: 26, element: ElementKind::Neutral, grade: 3 },
            AreaWord { id: 10, name: "Chasing".into(), category: WordCategory::PartA, level_mod: 31, element: ElementKind::Fire, grade: 3 },
            AreaWord { id: 11, name: "Roaring".into(), category: WordCategory::PartA, level_mod: 36, element: ElementKind::Fire, grade: 3 },
            AreaWord { id: 12, name: "Essential".into(), category: WordCategory::PartA, level_mod: 42, element: ElementKind::Light, grade: 4 },
            AreaWord { id: 13, name: "Plenteous".into(), category: WordCategory::PartA, level_mod: 47, element: ElementKind::Water, grade: 4 },
        ];

        let second_words = vec![
            AreaWord { id: 101, name: "Forbidden".into(), category: WordCategory::PartB, level_mod: 0, element: ElementKind::Neutral, grade: 1 },
            AreaWord { id: 102, name: "Maiden's (初陣の)".into(), category: WordCategory::PartB, level_mod: 0, element: ElementKind::Neutral, grade: 1 },
            AreaWord { id: 103, name: "Brave".into(), category: WordCategory::PartB, level_mod: 1, element: ElementKind::Fire, grade: 1 },
            AreaWord { id: 104, name: "Wrath".into(), category: WordCategory::PartB, level_mod: 2, element: ElementKind::Fire, grade: 1 },
            AreaWord { id: 105, name: "Tragedy's".into(), category: WordCategory::PartB, level_mod: 3, element: ElementKind::Dark, grade: 1 },
            AreaWord { id: 106, name: "Sun's".into(), category: WordCategory::PartB, level_mod: 4, element: ElementKind::Light, grade: 2 },
            AreaWord { id: 107, name: "Treasure".into(), category: WordCategory::PartB, level_mod: 5, element: ElementKind::Earth, grade: 2 },
            AreaWord { id: 108, name: "Faced".into(), category: WordCategory::PartB, level_mod: 6, element: ElementKind::Neutral, grade: 2 },
            AreaWord { id: 109, name: "Geppaku".into(), category: WordCategory::PartB, level_mod: 7, element: ElementKind::Light, grade: 3 },
            AreaWord { id: 110, name: "Hate's".into(), category: WordCategory::PartB, level_mod: 8, element: ElementKind::Dark, grade: 3 },
            AreaWord { id: 111, name: "Vast".into(), category: WordCategory::PartB, level_mod: 9, element: ElementKind::Wind, grade: 3 },
            AreaWord { id: 112, name: "Raging".into(), category: WordCategory::PartB, level_mod: 10, element: ElementKind::Fire, grade: 4 },
        ];

        let third_words = vec![
            AreaWord { id: 201, name: "Holy Ground".into(), category: WordCategory::PartC, level_mod: 0, element: ElementKind::Light, grade: 1 },
            AreaWord { id: 202, name: "Sacred End (夢の果て)".into(), category: WordCategory::PartC, level_mod: 0, element: ElementKind::Light, grade: 1 },
            AreaWord { id: 203, name: "Fast Food".into(), category: WordCategory::PartC, level_mod: 1, element: ElementKind::Earth, grade: 1 },
            AreaWord { id: 203, name: "Corpse".into(), category: WordCategory::PartC, level_mod: 2, element: ElementKind::Dark, grade: 1 },
            AreaWord { id: 204, name: "Fort".into(), category: WordCategory::PartC, level_mod: 3, element: ElementKind::Neutral, grade: 1 },
            AreaWord { id: 205, name: "Paradise".into(), category: WordCategory::PartC, level_mod: 4, element: ElementKind::Water, grade: 2 },
            AreaWord { id: 206, name: "Avidya".into(), category: WordCategory::PartC, level_mod: 5, element: ElementKind::Dark, grade: 2 },
            AreaWord { id: 207, name: "Sea of Clouds".into(), category: WordCategory::PartC, level_mod: 6, element: ElementKind::Wind, grade: 2 },
            AreaWord { id: 208, name: "Remnant".into(), category: WordCategory::PartC, level_mod: 7, element: ElementKind::Earth, grade: 3 },
            AreaWord { id: 209, name: "1000 Oaks".into(), category: WordCategory::PartC, level_mod: 8, element: ElementKind::Earth, grade: 3 },
            AreaWord { id: 210, name: "Blue Cloud".into(), category: WordCategory::PartC, level_mod: 9, element: ElementKind::Water, grade: 3 },
            AreaWord { id: 211, name: "Nemesis".into(), category: WordCategory::PartC, level_mod: 10, element: ElementKind::Fire, grade: 4 },
        ];

        Self {
            first_words,
            second_words,
            third_words,
        }
    }

    /// Synthesize an area from three keyword names.
    pub fn generate(
        &self,
        server: RootServer,
        word_a_name: &str,
        word_b_name: &str,
        word_c_name: &str,
    ) -> Option<GeneratedArea> {
        let find_word = |list: &[AreaWord], target: &str| {
            list.iter().find(|w| {
                w.name.eq_ignore_ascii_case(target)
                    || w.name.to_lowercase().starts_with(&target.to_lowercase())
                    || w.name.contains(target)
            }).cloned()
        };

        let a = find_word(&self.first_words, word_a_name)?;
        let b = find_word(&self.second_words, word_b_name)?;
        let c = find_word(&self.third_words, word_c_name)?;

        Some(self.synthesize(server, a, b, c))
    }

    /// Compute authentic attributes, level scaling, and topology.
    pub fn synthesize(
        &self,
        server: RootServer,
        word_a: AreaWord,
        word_b: AreaWord,
        word_c: AreaWord,
    ) -> GeneratedArea {
        let full_name = format!("{} {} {} {}", server.name(), word_a.name, word_b.name, word_c.name);

        // Special Holy Ground check: "Delta Hidden Forbidden Holy Ground" or tutorial field
        let is_holy_ground = server == RootServer::Delta
            && (word_a.name.contains("Hidden") || word_a.name.contains("勇み行く") || word_a.id == 1 || word_a.id == 2)
            && (word_b.name.contains("Forbidden") || word_b.name.contains("初陣の") || word_b.id == 101 || word_b.id == 102)
            && (word_c.name.contains("Holy Ground") || word_c.name.contains("夢の果て") || word_c.id == 201);

        let (base_min, base_max) = server.level_range();
        let raw_level = (word_a.level_mod + word_b.level_mod + word_c.level_mod) as u32;
        let area_level = raw_level.clamp(base_min, base_max);

        // Element determination: Part A leads, unless Part C has stronger grade
        let element = if word_c.grade > word_a.grade && word_c.element != ElementKind::Neutral {
            word_c.element
        } else if word_a.element != ElementKind::Neutral {
            word_a.element
        } else {
            word_b.element
        };

        // Field kind determined by Part C hash / id, or Grassland for Holy Ground
        let field_kind = if is_holy_ground {
            FieldKind::Grassland
        } else {
            match word_c.id % 4 {
                0 => FieldKind::Grassland,
                1 => FieldKind::Wasteland,
                2 => FieldKind::Cave,
                _ => FieldKind::Shrine,
            }
        };

        // Weather determined by Part B
        let weather = match word_b.id % 6 {
            0 => WeatherKind::Clear,
            1 => WeatherKind::Cloudy,
            2 => WeatherKind::Sunset,
            3 => WeatherKind::Rain,
            4 => WeatherKind::Night,
            _ => WeatherKind::Fog,
        };

        // Mission and floor depth
        let (mission, floors) = if is_holy_ground {
            (MissionType::SpecialQuest, 1)
        } else {
            match word_c.id % 3 {
                0 => (MissionType::GottStatue, if field_kind == FieldKind::Grassland { 1 } else { 3 }),
                1 => (MissionType::DefeatBoss, if field_kind == FieldKind::Grassland { 1 } else { 2 }),
                _ => (MissionType::ChimChimChallenge, 1),
            }
        };

        let enemy_spawn_count = 6 + (area_level / 5).min(12);
        let chest_count = 4 + (area_level / 10).min(6);
        let gott_statue_tier = ((word_a.grade + word_b.grade + word_c.grade) / 3).max(1);

        GeneratedArea {
            server,
            word_a,
            word_b,
            word_c,
            full_name,
            area_level,
            field_kind,
            weather,
            element,
            mission,
            floors,
            enemy_spawn_count,
            chest_count,
            gott_statue_tier,
        }
    }
}
