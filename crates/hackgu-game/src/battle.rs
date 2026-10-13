//! Combat Encounter & Battle Entity Simulation (`ccGobjBattle`, `ccGobjBpc`, `ccGobjBattleAI`).
//! Reconstructed directly from CC2 engine decompilation and virtual function tables.

use glam::Vec3;
use crate::object::GameObj;
use crate::context::WorldContext;
use crate::event::GameEvent;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CombatState {
    Idle,
    Attacking { combo_step: u32, timer: f32 },
    SkillCasting { skill_id: u32, timer: f32 },
    Guarding,
    HitStun { duration: f32 },
    Knockback { vel: Vec3, duration: f32 },
    Downed { recovery_timer: f32 },
    RengekiAvailable { window_timer: f32 },
    Defeated,
}

#[derive(Clone, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub current_hp: i32,
    pub max_sp: i32,
    pub current_sp: i32,
    pub p_atk: i32,
    pub m_atk: i32,
    pub p_def: i32,
    pub m_def: i32,
    pub level: u32,
}

impl CombatStats {
    pub fn new(hp: i32, sp: i32, patk: i32, pdef: i32, level: u32) -> Self {
        Self {
            max_hp: hp,
            current_hp: hp,
            max_sp: sp,
            current_sp: sp,
            p_atk: patk,
            m_atk: patk,
            p_def: pdef,
            m_def: pdef,
            level,
        }
    }
}

/// Battle Player Character (`ccGobjBpc`).
/// Represents player combat entity with Rengeki triggers and Awakening meters.
#[derive(Debug, Clone)]
pub struct BattlePlayer {
    pub id: u32,
    pub name: String,
    pub active: bool,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,

    pub stats: CombatStats,
    pub state: CombatState,
    pub target_id: Option<u32>,

    pub morale_gauge: f32, // 0.0 to 100.0%
    pub rengeki_hits: u32,
}

impl BattlePlayer {
    pub fn new(id: u32, name: &str, stats: CombatStats, pos: Vec3) -> Self {
        Self {
            id,
            name: name.to_string(),
            active: true,
            position: pos,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
            stats,
            state: CombatState::Idle,
            target_id: None,
            morale_gauge: 0.0,
            rengeki_hits: 0,
        }
    }

    /// Execute a normal weapon strike (virtual method #3 in `ccGobjBattle`).
    pub fn execute_attack(&mut self) {
        if self.state == CombatState::Idle {
            self.state = CombatState::Attacking {
                combo_step: 1,
                timer: 0.45,
            };
            self.rengeki_hits += 1;
            self.morale_gauge = (self.morale_gauge + 5.0).min(100.0);
        }
    }

    /// Take damage from an incoming attack.
    pub fn take_damage(&mut self, damage: i32) {
        self.stats.current_hp = (self.stats.current_hp - damage).max(0);
        if self.stats.current_hp == 0 {
            self.state = CombatState::Defeated;
        } else {
            self.state = CombatState::HitStun { duration: 0.3 };
        }
    }
}

impl GameObj for BattlePlayer {
    fn obj_id(&self) -> u32 {
        self.id
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
        self.rotation
    }

    fn set_rotation(&mut self, rot: Vec3) {
        self.rotation = rot;
    }

    fn scale(&self) -> Vec3 {
        self.scale
    }

    fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }

    fn update(&mut self, dt: f32, world: &mut WorldContext) {
        // Step combat state machine timers
        match self.state {
            CombatState::Attacking { combo_step: _, ref mut timer } => {
                *timer -= dt;
                if *timer <= 0.0 {
                    self.state = CombatState::Idle;
                }
            }
            CombatState::HitStun { ref mut duration } => {
                *duration -= dt;
                if *duration <= 0.0 {
                    self.state = CombatState::Idle;
                }
            }
            _ => {}
        }

        // Keep grounded to stage collision
        if let Some(gz) = world.ground_height(self.position.x, self.position.y, self.position.z) {
            self.position.z = gz;
        }
    }

    fn on_interact(&mut self, _interactor_id: u32, _world: &mut WorldContext) -> Option<GameEvent> {
        None
    }

    fn on_trigger_enter(&mut self, _other_id: u32, _world: &mut WorldContext) {}
}

/// Circular Combat Barrier (`EffectBattleArea`).
/// CC2 battle ring keeping participants inside until battle resolution.
pub struct BattleAreaBarrier {
    pub center: Vec3,
    pub radius: f32,
    pub is_active: bool,
}

impl BattleAreaBarrier {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self {
            center,
            radius,
            is_active: true,
        }
    }

    /// Clamp a position to stay inside the combat barrier ring.
    pub fn constrain_position(&self, pos: &mut Vec3) {
        if !self.is_active {
            return;
        }
        let delta_2d = Vec3::new(pos.x - self.center.x, pos.y - self.center.y, 0.0);
        let dist = delta_2d.length();
        if dist > self.radius {
            let clamped = self.center + delta_2d.normalize() * self.radius;
            pos.x = clamped.x;
            pos.y = clamped.y;
        }
    }
}
