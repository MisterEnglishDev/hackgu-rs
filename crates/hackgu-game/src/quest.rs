//! # CyberConnect2 Quest & Mission System
//! Reconstructed directly from `QuestTakeMenu`, `QuestConfirmMenu`, `QuestClearMenu`,
//! `QuestListWnd`, and `ccEventQuest`.
//!
//! Manages mercenary guild quests (`xquest_01.ccs`), party prerequisite checks,
//! mission objectives, and reward distribution upon clearance.

use serde::{Deserialize, Serialize};
use crate::savedata::{PartyMemberData, SaveData, InventorySlot};

/// Quest status lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestStatus {
    Available,
    InProgress,
    Completed,
    Failed,
}

/// Mission objective requirements.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestObjective {
    /// Defeat designated boss or bounty monsters
    DefeatMonster {
        monster_name: String,
        target_count: u32,
        current_count: u32,
    },
    /// Collect quest items from dungeon chests or drops
    CollectItems {
        item_name: String,
        required_count: u32,
        current_count: u32,
    },
    /// Find and talk to an NPC in a designated lost ground
    LocateTargetNpc {
        npc_name: String,
        located: bool,
    },
}

impl QuestObjective {
    pub fn is_fulfilled(&self) -> bool {
        match self {
            QuestObjective::DefeatMonster { target_count, current_count, .. } => current_count >= target_count,
            QuestObjective::CollectItems { required_count, current_count, .. } => current_count >= required_count,
            QuestObjective::LocateTargetNpc { located, .. } => *located,
        }
    }
}

/// Completion rewards for successful quest clearing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestRewards {
    pub gp: u32,
    pub items: Vec<InventorySlot>,
    pub bonus_words: Vec<String>,
}

/// A Guild Mission posted on the Mercenary Union job board (`xquest_01.ccs`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuestDef {
    pub id: u32,
    pub title: String,
    pub client: String,
    pub summary: String,
    pub rank: u8, // Rank 1 to 5
    pub recommended_level: u32,
    pub required_party_names: Vec<String>,
    pub area_keywords: (String, String, String),
    pub objective: QuestObjective,
    pub rewards: QuestRewards,
    pub status: QuestStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuestError {
    QuestNotFound(u32),
    QuestAlreadyActive,
    PrerequisitePartyMissing(String),
    LevelTooLow { required: u32, current: u32 },
    ObjectiveNotComplete,
}

/// The Mercenary Union Quest Desk Manager (`ccQuestSys`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuestManager {
    pub quests: Vec<QuestDef>,
    pub active_quest_id: Option<u32>,
}

impl QuestManager {
    /// Initialize with authentic Vol 1 Rebirth starter guild quests.
    pub fn new_vol1_defaults() -> Self {
        let quests = vec![
            QuestDef {
                id: 101,
                title: "Steam Scientist's Invention".into(),
                client: "Doctor P".into(),
                summary: "Collect 3 Steam Cores from the cave geysers to power the new steam engine.".into(),
                rank: 1,
                recommended_level: 3,
                required_party_names: vec!["Silabus".into(), "Gaspard".into()],
                area_keywords: ("Graceful".into(), "Brave".into(), "Fast Food".into()),
                objective: QuestObjective::CollectItems {
                    item_name: "Steam Core".into(),
                    required_count: 3,
                    current_count: 0,
                },
                rewards: QuestRewards {
                    gp: 800,
                    items: vec![
                        InventorySlot { item_id: 1, name: "Health Drink".into(), quantity: 3 },
                        InventorySlot { item_id: 3, name: "Sprite Drop".into(), quantity: 1 },
                    ],
                    bonus_words: vec!["Counting".into()],
                },
                status: QuestStatus::Available,
            },
            QuestDef {
                id: 102,
                title: "The Cursed Blade".into(),
                client: "Wandering Swordsman".into(),
                summary: "Exterminate the rogue Blade Goblin terrorizing travelers in the shrine ruins.".into(),
                rank: 2,
                recommended_level: 8,
                required_party_names: vec![],
                area_keywords: ("Submissive".into(), "Wrath".into(), "Corpse".into()),
                objective: QuestObjective::DefeatMonster {
                    monster_name: "Blade Goblin".into(),
                    target_count: 1,
                    current_count: 0,
                },
                rewards: QuestRewards {
                    gp: 1500,
                    items: vec![
                        InventorySlot { item_id: 10, name: "Spin GaiGu".into(), quantity: 1 },
                    ],
                    bonus_words: vec!["Avidya".into()],
                },
                status: QuestStatus::Available,
            },
        ];

        Self {
            quests,
            active_quest_id: None,
        }
    }

    /// Accept a quest from the board, verifying level and party prerequisites.
    pub fn accept_quest(
        &mut self,
        quest_id: u32,
        player_level: u32,
        active_party: &[PartyMemberData],
    ) -> Result<(), QuestError> {
        if self.active_quest_id.is_some() {
            return Err(QuestError::QuestAlreadyActive);
        }

        let quest = self.quests.iter_mut().find(|q| q.id == quest_id)
            .ok_or(QuestError::QuestNotFound(quest_id))?;

        if player_level < quest.recommended_level {
            return Err(QuestError::LevelTooLow {
                required: quest.recommended_level,
                current: player_level,
            });
        }

        // Check required companions
        for req in &quest.required_party_names {
            let in_party = active_party.iter().any(|m| m.in_active_party && m.name.eq_ignore_ascii_case(req));
            if !in_party {
                return Err(QuestError::PrerequisitePartyMissing(req.clone()));
            }
        }

        quest.status = QuestStatus::InProgress;
        self.active_quest_id = Some(quest_id);
        Ok(())
    }

    /// Advance active objective on monster defeat.
    pub fn report_monster_kill(&mut self, monster_name: &str) {
        if let Some(active_id) = self.active_quest_id {
            if let Some(quest) = self.quests.iter_mut().find(|q| q.id == active_id) {
                if let QuestObjective::DefeatMonster { monster_name: ref target, ref mut current_count, target_count } = quest.objective {
                    if target.eq_ignore_ascii_case(monster_name) && *current_count < target_count {
                        *current_count += 1;
                    }
                }
            }
        }
    }

    /// Advance active objective on item collection.
    pub fn report_item_collected(&mut self, item_name: &str, count: u32) {
        if let Some(active_id) = self.active_quest_id {
            if let Some(quest) = self.quests.iter_mut().find(|q| q.id == active_id) {
                if let QuestObjective::CollectItems { item_name: ref target, ref mut current_count, required_count } = quest.objective {
                    if target.eq_ignore_ascii_case(item_name) {
                        *current_count = (*current_count + count).min(required_count);
                    }
                }
            }
        }
    }

    /// Turn in and claim rewards for a completed quest.
    pub fn complete_active_quest(&mut self, save: &mut SaveData) -> Result<QuestRewards, QuestError> {
        let active_id = self.active_quest_id.ok_or(QuestError::QuestNotFound(0))?;
        let quest = self.quests.iter_mut().find(|q| q.id == active_id)
            .ok_or(QuestError::QuestNotFound(active_id))?;

        if !quest.objective.is_fulfilled() {
            return Err(QuestError::ObjectiveNotComplete);
        }

        quest.status = QuestStatus::Completed;
        let rewards = quest.rewards.clone();

        // Grant GP
        save.add_gp(rewards.gp);

        // Grant items
        for item in &rewards.items {
            if let Some(slot) = save.items.iter_mut().find(|s| s.item_id == item.item_id) {
                slot.quantity = (slot.quantity + item.quantity).min(99);
            } else {
                save.items.push(item.clone());
            }
        }

        // Grant bonus keywords to Chaos Gate
        for word in &rewards.bonus_words {
            save.unlocked_first_words.insert(word.clone());
        }

        self.active_quest_id = None;
        Ok(rewards)
    }

    /// Abandon or cancel the active quest.
    pub fn cancel_active_quest(&mut self) {
        if let Some(active_id) = self.active_quest_id.take() {
            if let Some(quest) = self.quests.iter_mut().find(|q| q.id == active_id) {
                quest.status = QuestStatus::Available;
            }
        }
    }
}
