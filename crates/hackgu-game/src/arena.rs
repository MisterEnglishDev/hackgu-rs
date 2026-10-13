//! # CyberConnect2 Holy Palace Arena Tournament System
//! Reconstructed directly from `ArenaBattleInfoMenu`, `ArenaBattleStartTelop`,
//! `ArenaRankingMenu`, `ArenaSurviveMenu`, and `ArenaResultMenuMain`.
//!
//! Simulates 3v3 competitive team matches in the Holy Palace Arena (Demon Palace Tournament in Vol 1),
//! War Points (WP) scoring, ranking ladder from Rank 200 to Rank 1, and special boss teams.

use serde::{Deserialize, Serialize};
use crate::savedata::{PartyMemberData, SaveData};
use crate::battle::{BattlePlayer, CombatStats};
use glam::Vec3;

/// Arena Match Types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArenaMatchType {
    /// Friendly practice / exhibition match
    Exhibition,
    /// Official ladder ranking bout (earns War Points to advance rank)
    TournamentRankMatch,
    /// Multi-wave survival challenge with no rest between rounds
    SurvivalContinuous,
}

/// An Opponent Team in the Arena Ladder.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArenaTeam {
    pub team_name: String,
    pub leader_name: String,
    pub rank: u32,
    pub members: Vec<ArenaCombatantDef>,
}

/// A combatant on an opponent team.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArenaCombatantDef {
    pub name: String,
    pub class_name: String,
    pub level: u32,
    pub max_hp: u32,
    pub max_sp: u32,
    pub attack: u32,
    pub defense: u32,
}

/// War Points (WP) score card calculated at the end of a match.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ArenaScoreCard {
    pub base_win_wp: u32,
    pub first_strike_bonus: u32,
    pub rengeki_count: u32,
    pub rengeki_bonus: u32,
    pub remaining_time_bonus: u32,
    pub no_damage_bonus: u32,
    pub total_wp: u32,
}

impl ArenaScoreCard {
    pub fn calculate(
        won: bool,
        first_strike: bool,
        rengeki_count: u32,
        remaining_seconds: u32,
        took_no_damage: bool,
    ) -> Self {
        if !won {
            return Self::default();
        }

        let base = 100;
        let fs = if first_strike { 25 } else { 0 };
        let rengeki = rengeki_count * 15;
        let time = remaining_seconds * 2;
        let no_dmg = if took_no_damage { 50 } else { 0 };
        let total = base + fs + rengeki + time + no_dmg;

        Self {
            base_win_wp: base,
            first_strike_bonus: fs,
            rengeki_count,
            rengeki_bonus: rengeki,
            remaining_time_bonus: time,
            no_damage_bonus: no_dmg,
            total_wp: total,
        }
    }
}

/// Active 3v3 Arena Battle Simulation (`ccArenaBattleCtrl`).
#[derive(Debug, Clone)]
pub struct ArenaMatch {
    pub match_type: ArenaMatchType,
    pub player_team: Vec<BattlePlayer>,
    pub opponent_team: Vec<BattlePlayer>,
    pub time_remaining: f32, // 180 seconds match timer
    pub first_strike_by_player: bool,
    pub player_rengeki_count: u32,
    pub is_finished: bool,
    pub player_won: bool,
}

impl ArenaMatch {
    pub fn new(
        match_type: ArenaMatchType,
        save: &SaveData,
        opponents: &ArenaTeam,
    ) -> Self {
        // Build player team: Haseo + 2 active party companions
        let mut player_team = Vec::new();
        let haseo_stats = CombatStats::new(save.hp as i32, save.sp as i32, 45, 30, save.level);
        player_team.push(BattlePlayer::new(1, &save.player_name, haseo_stats, Vec3::new(0.0, -10.0, 0.0)));

        let active_companions: Vec<&PartyMemberData> = save.party_members.iter()
            .filter(|m| m.in_active_party)
            .take(2)
            .collect();

        for (idx, comp) in active_companions.iter().enumerate() {
            let offset_x = if idx == 0 { -4.0 } else { 4.0 };
            let stats = CombatStats::new(comp.hp as i32, comp.sp as i32, 35, 25, comp.level);
            player_team.push(BattlePlayer::new((idx + 2) as u32, &comp.name, stats, Vec3::new(offset_x, -12.0, 0.0)));
        }

        // Build opponent team
        let mut opponent_team = Vec::new();
        for (idx, opp) in opponents.members.iter().enumerate() {
            let offset_x = match idx {
                0 => 0.0,
                1 => -4.0,
                _ => 4.0,
            };
            let stats = CombatStats::new(
                opp.max_hp as i32,
                opp.max_sp as i32,
                opp.attack as i32,
                opp.defense as i32,
                opp.level,
            );
            opponent_team.push(BattlePlayer::new(
                (idx + 10) as u32,
                &opp.name,
                stats,
                Vec3::new(offset_x, 10.0, 0.0),
            ));
        }

        Self {
            match_type,
            player_team,
            opponent_team,
            time_remaining: 180.0,
            first_strike_by_player: false,
            player_rengeki_count: 0,
            is_finished: false,
            player_won: false,
        }
    }

    /// Advance active arena combat tick.
    pub fn update(&mut self, dt: f32) -> Option<ArenaScoreCard> {
        if self.is_finished {
            return None;
        }

        self.time_remaining = (self.time_remaining - dt).max(0.0);

        // Check if all opponents defeated
        let opponents_defeated = self.opponent_team.iter().all(|o| o.stats.current_hp == 0);
        let players_defeated = self.player_team.iter().all(|p| p.stats.current_hp == 0);

        if opponents_defeated {
            self.is_finished = true;
            self.player_won = true;
            let score = ArenaScoreCard::calculate(
                true,
                self.first_strike_by_player,
                self.player_rengeki_count,
                self.time_remaining as u32,
                false,
            );
            return Some(score);
        } else if players_defeated || self.time_remaining <= 0.0 {
            self.is_finished = true;
            self.player_won = false;
            return Some(ArenaScoreCard::calculate(false, false, 0, 0, false));
        }

        None
    }
}

/// The Arena Ranking Ladder and Tournament Manager (`ArenaRankingMenu`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArenaLadder {
    pub current_rank: u32, // Starts at 200, Rank 1 is Emperor
    pub accumulated_wp: u32,
    pub opponent_roster: Vec<ArenaTeam>,
}

impl ArenaLadder {
    pub fn new_vol1_defaults() -> Self {
        let opponent_roster = vec![
            ArenaTeam {
                team_name: "Team Bordeaux".into(),
                leader_name: "Bordeaux".into(),
                rank: 16,
                members: vec![
                    ArenaCombatantDef { name: "Bordeaux".into(), class_name: "Blade Brandier".into(), level: 25, max_hp: 650, max_sp: 180, attack: 75, defense: 45 },
                    ArenaCombatantDef { name: "Negimaru".into(), class_name: "Twin Blade".into(), level: 24, max_hp: 550, max_sp: 140, attack: 68, defense: 38 },
                    ArenaCombatantDef { name: "Grein".into(), class_name: "Tribal Grappler".into(), level: 24, max_hp: 700, max_sp: 120, attack: 72, defense: 50 },
                ],
            },
            ArenaTeam {
                team_name: "Team Moon Tree".into(),
                leader_name: "Matsu".into(),
                rank: 6,
                members: vec![
                    ArenaCombatantDef { name: "Matsu".into(), class_name: "Adept Rogue".into(), level: 35, max_hp: 950, max_sp: 250, attack: 110, defense: 70 },
                    ArenaCombatantDef { name: "Moon Cleric".into(), class_name: "Harvest Cleric".into(), level: 33, max_hp: 680, max_sp: 320, attack: 60, defense: 55 },
                    ArenaCombatantDef { name: "Moon Guard".into(), class_name: "Edge Punisher".into(), level: 34, max_hp: 1100, max_sp: 180, attack: 95, defense: 80 },
                ],
            },
            ArenaTeam {
                team_name: "The Emperor's Guard".into(),
                leader_name: "Endrance".into(),
                rank: 1,
                members: vec![
                    ArenaCombatantDef { name: "Endrance".into(), class_name: "Blade Brandier".into(), level: 50, max_hp: 2200, max_sp: 600, attack: 185, defense: 120 },
                    ArenaCombatantDef { name: "Aida Servant 1".into(), class_name: "Shadow Warlock".into(), level: 48, max_hp: 1400, max_sp: 500, attack: 140, defense: 95 },
                    ArenaCombatantDef { name: "Aida Servant 2".into(), class_name: "Lord Partizan".into(), level: 48, max_hp: 1800, max_sp: 400, attack: 160, defense: 110 },
                ],
            },
        ];

        Self {
            current_rank: 200,
            accumulated_wp: 0,
            opponent_roster,
        }
    }

    /// Record a match victory and promote ranking tier.
    pub fn record_victory(&mut self, score: &ArenaScoreCard) {
        self.accumulated_wp = self.accumulated_wp.saturating_add(score.total_wp);

        // Advance rank for every 200 WP earned
        let rank_gain = (score.total_wp / 100).max(1);
        self.current_rank = self.current_rank.saturating_sub(rank_gain).max(1);
    }
}
