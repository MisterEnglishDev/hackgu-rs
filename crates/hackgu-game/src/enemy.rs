//! # CyberConnect2 Enemy Entity & Combat AI System
//! Reconstructed directly from `ccGobjEnemy` and `ccGobjBattleAIEnemy`.
//!
//! Controls field perception (wandering, spotting, chasing) and combat behaviors
//! across enemy archetypes (Goblin `ees10`, Guard `egg10`, Giant `ehe10`, Shaman `ems10`).

use glam::Vec3;
use serde::{Deserialize, Serialize};
use crate::object::GameObj;
use crate::context::WorldContext;
use crate::event::GameEvent;
use crate::area::ElementKind;

/// Enemy species / archetype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnemyArchetype {
    /// Small, agile scout (e.g. Goblin `ees10`)
    SmallGoblin,
    /// Armored warrior with shield/sword (e.g. Guard `egg10`)
    ArmoredGuard,
    /// Massive brute with high poise and heavy ground slams (e.g. Behemoth `ehe10`)
    GiantBehemoth,
    /// Ranged elemental spellcaster (e.g. Shaman `ems10`)
    MageShaman,
}

impl EnemyArchetype {
    pub fn ccs_asset(&self) -> &'static str {
        match self {
            EnemyArchetype::SmallGoblin => "data/en/ees10.ccs",
            EnemyArchetype::ArmoredGuard => "data/en/egg10.ccs",
            EnemyArchetype::GiantBehemoth => "data/en/ehe10.ccs",
            EnemyArchetype::MageShaman => "data/en/ems10.ccs",
        }
    }

    pub fn detection_radius(&self) -> f32 {
        match self {
            EnemyArchetype::SmallGoblin => 12.0,
            EnemyArchetype::ArmoredGuard => 10.0,
            EnemyArchetype::GiantBehemoth => 14.0,
            EnemyArchetype::MageShaman => 16.0,
        }
    }

    pub fn move_speed(&self) -> f32 {
        match self {
            EnemyArchetype::SmallGoblin => 5.5,
            EnemyArchetype::ArmoredGuard => 3.8,
            EnemyArchetype::GiantBehemoth => 2.5,
            EnemyArchetype::MageShaman => 4.0,
        }
    }
}

/// AI State when roaming the field or dungeon corridor.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EnemyFieldState {
    /// Stationary or looking around
    Idle { timer: f32 },
    /// Roaming near spawn anchor point
    Patrol { target: Vec3 },
    /// Noticed player, displaying alert exclamation mark
    Alert { timer: f32 },
    /// Actively pursuing player to force combat engagement
    Chasing,
    /// Encounter initiated, locked inside battle arena
    InBattle,
}

/// AI State when engaged in combat encounter (`ccGobjBattleAIEnemy`).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EnemyCombatState {
    /// Circling around target waiting for opening
    Circling { timer: f32, direction: f32 },
    /// Windup and striking animation
    Attacking { attack_step: u8, timer: f32 },
    /// Blocking incoming frontal attacks
    Guarding { timer: f32 },
    /// Flinching from player hit
    HitStun { timer: f32 },
    /// Knocked down on the ground
    KnockedDown { timer: f32 },
    /// Defeated / playing death dissolve
    Defeated { timer: f32 },
}

/// Combat parameters and drop tables.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnemyStats {
    pub level: u32,
    pub max_hp: u32,
    pub current_hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub element: ElementKind,
    pub exp_reward: u32,
    pub gp_reward: u32,
    pub drop_item: Option<String>,
}

impl EnemyStats {
    pub fn new(archetype: EnemyArchetype, level: u32) -> Self {
        let (hp_mult, atk_mult, def_mult, exp_mult, drop) = match archetype {
            EnemyArchetype::SmallGoblin => (12, 4, 3, 15, Some("Health Drink".into())),
            EnemyArchetype::ArmoredGuard => (22, 6, 6, 25, Some("Healing Serum".into())),
            EnemyArchetype::GiantBehemoth => (45, 9, 7, 50, Some("Warrior Tonic".into())),
            EnemyArchetype::MageShaman => (15, 8, 2, 30, Some("Sprite Drop".into())),
        };

        let max_hp = 80 + level * hp_mult;
        let attack = 15 + level * atk_mult;
        let defense = 10 + level * def_mult;
        let exp_reward = 20 + level * exp_mult;
        let gp_reward = 30 + level * 10;

        Self {
            level,
            max_hp,
            current_hp: max_hp,
            attack,
            defense,
            element: ElementKind::Neutral,
            exp_reward,
            gp_reward,
            drop_item: drop,
        }
    }
}

/// The Enemy Entity in the CyberConnect2 engine (`ccGobjEnemy`).
#[derive(Debug, Clone)]
pub struct EnemyEntity {
    pub obj_id: u32,
    pub name: String,
    pub archetype: EnemyArchetype,
    pub spawn_anchor: Vec3,
    pub position: Vec3,
    pub yaw: f32,
    pub active: bool,
    pub stats: EnemyStats,
    pub field_state: EnemyFieldState,
    pub combat_state: EnemyCombatState,
}

impl EnemyEntity {
    pub fn new(obj_id: u32, name: &str, archetype: EnemyArchetype, spawn_pos: Vec3, level: u32) -> Self {
        Self {
            obj_id,
            name: name.to_string(),
            archetype,
            spawn_anchor: spawn_pos,
            position: spawn_pos,
            yaw: 0.0,
            active: true,
            stats: EnemyStats::new(archetype, level),
            field_state: EnemyFieldState::Idle { timer: 2.0 },
            combat_state: EnemyCombatState::Circling { timer: 1.5, direction: 1.0 },
        }
    }

    /// Advance Field AI logic (perception and chase).
    pub fn update_field(&mut self, dt: f32, player_pos: Vec3, world: &WorldContext) -> Option<GameEvent> {
        if !self.active || self.stats.current_hp == 0 {
            return None;
        }

        let to_player = player_pos - self.position;
        let dist = to_player.length();
        let detect_radius = self.archetype.detection_radius();

        match self.field_state {
            EnemyFieldState::Idle { ref mut timer } => {
                *timer -= dt;
                if dist < detect_radius {
                    self.field_state = EnemyFieldState::Alert { timer: 0.8 };
                } else if *timer <= 0.0 {
                    // Pick a random patrol waypoint around spawn anchor
                    let patrol_target = self.spawn_anchor + Vec3::new(
                        (self.obj_id as f32).sin() * 5.0,
                        (self.obj_id as f32).cos() * 5.0,
                        0.0,
                    );
                    self.field_state = EnemyFieldState::Patrol { target: patrol_target };
                }
            }
            EnemyFieldState::Patrol { target } => {
                if dist < detect_radius {
                    self.field_state = EnemyFieldState::Alert { timer: 0.8 };
                } else {
                    let to_target = target - self.position;
                    let target_dist = to_target.length();
                    if target_dist < 1.0 {
                        self.field_state = EnemyFieldState::Idle { timer: 3.0 };
                    } else {
                        let dir = to_target.normalize();
                        self.yaw = (-dir.x).atan2(dir.y);
                        self.position += dir * (self.archetype.move_speed() * 0.4) * dt;
                    }
                }
            }
            EnemyFieldState::Alert { ref mut timer } => {
                *timer -= dt;
                // Face player
                self.yaw = (-to_player.x).atan2(to_player.y);
                if *timer <= 0.0 {
                    self.field_state = EnemyFieldState::Chasing;
                }
            }
            EnemyFieldState::Chasing => {
                if dist > detect_radius * 1.8 {
                    // Player escaped detection perimeter
                    self.field_state = EnemyFieldState::Idle { timer: 2.0 };
                } else if dist < 2.0 {
                    // Trigger Combat Encounter Barrier
                    self.field_state = EnemyFieldState::InBattle;
                    return Some(GameEvent::BattleTransition {
                        encounter_id: self.obj_id,
                        bgm_override: None,
                    });
                } else {
                    // Run towards player
                    let dir = to_player.normalize();
                    self.yaw = (-dir.x).atan2(dir.y);
                    self.position += dir * self.archetype.move_speed() * dt;
                }
            }
            EnemyFieldState::InBattle => {}
        }

        // Clamp to collision floor
        if let Some(ground_z) = world.ground_height(self.position.x, self.position.y, self.position.z) {
            self.position.z = ground_z;
        }

        None
    }

    /// Advance Combat AI logic (`ccGobjBattleAIEnemy`).
    pub fn update_combat(&mut self, dt: f32, player_pos: Vec3) -> (Option<u32>, Option<GameEvent>) {
        if self.stats.current_hp == 0 {
            if let EnemyCombatState::Defeated { ref mut timer } = self.combat_state {
                *timer -= dt;
                if *timer <= 0.0 {
                    self.active = false;
                }
            }
            return (None, None);
        }

        let to_player = player_pos - self.position;
        let dist = to_player.length();
        let mut dealt_damage = None;

        match self.combat_state {
            EnemyCombatState::Circling { ref mut timer, direction } => {
                *timer -= dt;
                self.yaw = (-to_player.x).atan2(to_player.y);

                // Strafe around player
                let tangent = Vec3::new(-to_player.y, to_player.x, 0.0).normalize() * direction;
                self.position += tangent * 2.0 * dt;

                if *timer <= 0.0 {
                    if dist <= 3.0 {
                        self.combat_state = EnemyCombatState::Attacking {
                            attack_step: 1,
                            timer: 0.6,
                        };
                    } else {
                        // Approach closer
                        let dir = to_player.normalize();
                        self.position += dir * self.archetype.move_speed() * dt;
                        if dist <= 3.0 {
                            self.combat_state = EnemyCombatState::Attacking {
                                attack_step: 1,
                                timer: 0.6,
                            };
                        }
                    }
                }
            }
            EnemyCombatState::Attacking { attack_step: _, ref mut timer } => {
                *timer -= dt;
                if *timer <= 0.2 && *timer + dt > 0.2 {
                    // Strike hitbox active: deal damage if in range
                    if dist <= 3.5 {
                        let dmg = self.stats.attack;
                        dealt_damage = Some(dmg);
                    }
                }
                if *timer <= 0.0 {
                    // Attack finished: reset to circling
                    self.combat_state = EnemyCombatState::Circling {
                        timer: 1.8,
                        direction: -1.0,
                    };
                }
            }
            EnemyCombatState::Guarding { ref mut timer } => {
                *timer -= dt;
                if *timer <= 0.0 {
                    self.combat_state = EnemyCombatState::Circling { timer: 1.0, direction: 1.0 };
                }
            }
            EnemyCombatState::HitStun { ref mut timer } => {
                *timer -= dt;
                if *timer <= 0.0 {
                    self.combat_state = EnemyCombatState::Circling { timer: 1.2, direction: 1.0 };
                }
            }
            EnemyCombatState::KnockedDown { ref mut timer } => {
                *timer -= dt;
                if *timer <= 0.0 {
                    self.combat_state = EnemyCombatState::Circling { timer: 1.5, direction: 1.0 };
                }
            }
            EnemyCombatState::Defeated { .. } => {}
        }

        (dealt_damage, None)
    }

    /// Process damage received from player attack or Skill.
    pub fn take_damage(&mut self, incoming_atk: u32, is_rengeki: bool, knockback: Option<Vec3>) -> u32 {
        if self.stats.current_hp == 0 {
            return 0;
        }

        let def = self.stats.defense;
        let raw = incoming_atk.saturating_sub(def / 2).max(1);
        let final_dmg = if is_rengeki { raw * 2 } else { raw };

        self.stats.current_hp = self.stats.current_hp.saturating_sub(final_dmg);

        if self.stats.current_hp == 0 {
            self.combat_state = EnemyCombatState::Defeated { timer: 2.0 };
        } else if is_rengeki {
            self.combat_state = EnemyCombatState::KnockedDown { timer: 1.8 };
            if let Some(kb) = knockback {
                self.position += kb;
            }
        } else {
            self.combat_state = EnemyCombatState::HitStun { timer: 0.35 };
        }

        final_dmg
    }
}

impl GameObj for EnemyEntity {
    fn obj_id(&self) -> u32 {
        self.obj_id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    fn position(&self) -> Vec3 {
        self.position
    }

    fn set_position(&mut self, pos: Vec3) {
        self.position = pos;
    }

    fn rotation(&self) -> Vec3 {
        Vec3::new(0.0, 0.0, self.yaw)
    }

    fn set_rotation(&mut self, rot: Vec3) {
        self.yaw = rot.z;
    }

    fn scale(&self) -> Vec3 {
        Vec3::ONE
    }

    fn set_scale(&mut self, _scale: Vec3) {}

    fn update(&mut self, dt: f32, world: &mut WorldContext) {
        let p_pos = world.camera_target;
        if let Some(event) = self.update_field(dt, p_pos, world) {
            world.queue_event(event);
        }
    }

    fn on_interact(&mut self, _interactor_id: u32, _world: &mut WorldContext) -> Option<GameEvent> {
        None
    }

    fn on_trigger_enter(&mut self, _other_id: u32, _world: &mut WorldContext) {}
}
