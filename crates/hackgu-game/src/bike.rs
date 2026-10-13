//! # CyberConnect2 Steam Bike Engine
//! Reconstructed directly from `ccGobjBike`, `ccBikeFrame`, `ccBikePartsParamDisp`, and `ccEventBike`.
//!
//! Provides high-speed overland transit, boost thrusters, Chim fuel mechanics,
//! and enemy ramming for preemptive combat strikes.

use glam::Vec3;
use serde::{Deserialize, Serialize};
use crate::object::GameObj;
use crate::context::WorldContext;
use crate::event::GameEvent;

/// Customizable Steam Bike component parts affecting performance stats.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BikeParts {
    pub frame_name: String,
    pub engine_tier: u8,
    pub muffler_tier: u8,
    pub tire_tier: u8,
}

impl Default for BikeParts {
    fn default() -> Self {
        Self {
            frame_name: "Basic Frame".into(),
            engine_tier: 1,
            muffler_tier: 1,
            tire_tier: 1,
        }
    }
}

/// Calculated performance attributes from equipped bike parts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BikeStats {
    pub max_cruise_speed: f32, // meters per second
    pub max_boost_speed: f32,
    pub acceleration: f32,
    pub turn_rate: f32,        // radians per second
    pub boost_fuel_burn: f32,  // fuel per second
    pub ram_damage: u32,
}

impl BikeStats {
    pub fn from_parts(parts: &BikeParts) -> Self {
        let cruise = 16.0 + (parts.engine_tier as f32 * 2.5);
        let boost = cruise + 10.0 + (parts.muffler_tier as f32 * 2.0);
        let accel = 12.0 + (parts.engine_tier as f32 * 3.0);
        let turn = 2.4 + (parts.tire_tier as f32 * 0.4);
        let burn = (15.0 - (parts.muffler_tier as f32 * 1.5)).max(5.0);
        let ram = 50 + (parts.engine_tier as u32 * 30);

        Self {
            max_cruise_speed: cruise,
            max_boost_speed: boost,
            acceleration: accel,
            turn_rate: turn,
            boost_fuel_burn: burn,
            ram_damage: ram,
        }
    }
}

/// Dynamic riding state machine.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BikeRideState {
    Dismounted,
    Mounting { timer: f32 },
    Cruising,
    Boosting,
    Drifting { angle: f32 },
    Crash { timer: f32 },
}

/// The Steam Bike Entity in the CyberConnect2 engine (`ccGobjBike`).
#[derive(Debug, Clone)]
pub struct SteamBike {
    pub obj_id: u32,
    pub position: Vec3,
    pub yaw: f32,
    pub current_speed: f32,
    pub fuel: f32, // 0.0 to 100.0%
    pub active: bool,
    pub parts: BikeParts,
    pub stats: BikeStats,
    pub ride_state: BikeRideState,
}

impl SteamBike {
    pub fn new(obj_id: u32, spawn_pos: Vec3, yaw: f32) -> Self {
        let parts = BikeParts::default();
        let stats = BikeStats::from_parts(&parts);
        Self {
            obj_id,
            position: spawn_pos,
            yaw,
            current_speed: 0.0,
            fuel: 100.0,
            active: true,
            parts,
            stats,
            ride_state: BikeRideState::Dismounted,
        }
    }

    /// Mount the bike.
    pub fn mount(&mut self) {
        if self.ride_state == BikeRideState::Dismounted {
            self.ride_state = BikeRideState::Mounting { timer: 0.6 };
        }
    }

    /// Dismount the bike.
    pub fn dismount(&mut self) {
        self.ride_state = BikeRideState::Dismounted;
        self.current_speed = 0.0;
    }

    /// Add Chim fuel to tank.
    pub fn refuel(&mut self, amount: f32) {
        self.fuel = (self.fuel + amount).clamp(0.0, 100.0);
    }

    /// Process player steering inputs (throttle: -1.0 to 1.0, steer: -1.0 to 1.0, boost: bool, drift: bool).
    pub fn process_input(
        &mut self,
        throttle: f32,
        steer: f32,
        boost: bool,
        drift: bool,
        dt: f32,
        world: &WorldContext,
    ) -> Option<GameEvent> {
        match self.ride_state {
            BikeRideState::Dismounted => return None,
            BikeRideState::Mounting { ref mut timer } => {
                *timer -= dt;
                if *timer <= 0.0 {
                    self.ride_state = BikeRideState::Cruising;
                } else {
                    return None;
                }
            }
            BikeRideState::Crash { ref mut timer } => {
                *timer -= dt;
                self.current_speed = (self.current_speed - 20.0 * dt).max(0.0);
                if *timer <= 0.0 {
                    self.ride_state = BikeRideState::Cruising;
                }
                return None;
            }
            BikeRideState::Cruising | BikeRideState::Boosting | BikeRideState::Drifting { .. } => {}
        }

        // Determine if boosting is active
        let wants_boost = boost && self.fuel > 1.0 && throttle > 0.1;
        let target_max_speed = if wants_boost {
            self.stats.max_boost_speed
        } else if throttle < 0.0 {
            -self.stats.max_cruise_speed * 0.3
        } else {
            self.stats.max_cruise_speed * throttle.abs()
        };

        if wants_boost {
            self.fuel = (self.fuel - self.stats.boost_fuel_burn * dt).max(0.0);
            self.ride_state = BikeRideState::Boosting;
        } else if drift && self.current_speed > 8.0 && steer.abs() > 0.2 {
            self.ride_state = BikeRideState::Drifting { angle: steer * 0.4 };
        } else {
            self.ride_state = BikeRideState::Cruising;
        }

        // Acceleration and braking
        if self.current_speed < target_max_speed {
            self.current_speed = (self.current_speed + self.stats.acceleration * dt).min(target_max_speed);
        } else if self.current_speed > target_max_speed {
            self.current_speed = (self.current_speed - self.stats.acceleration * 1.5 * dt).max(target_max_speed);
        }

        // Steering rotation (high-speed cornering)
        if self.current_speed.abs() > 0.5 {
            let speed_ratio = (self.current_speed / self.stats.max_cruise_speed).clamp(0.2, 1.0);
            let drift_mult = if matches!(self.ride_state, BikeRideState::Drifting { .. }) { 1.4 } else { 1.0 };
            self.yaw += steer * self.stats.turn_rate * drift_mult * speed_ratio * dt;
        }

        // Forward motion vector (+Y is forward in CC2 space)
        let forward = Vec3::new(-self.yaw.sin(), self.yaw.cos(), 0.0);
        let move_delta = forward * self.current_speed * dt;
        self.position += move_delta;

        // Ground conformance
        if let Some(ground_z) = world.ground_height(self.position.x, self.position.y, self.position.z) {
            self.position.z = ground_z;
        }

        None
    }

    /// Process collision with an enemy in the field: triggers preemptive surprise attack!
    pub fn trigger_enemy_ram(&mut self, enemy_id: u32) -> GameEvent {
        self.ride_state = BikeRideState::Crash { timer: 0.8 };
        self.current_speed = 0.0;

        GameEvent::BattleTransition {
            encounter_id: enemy_id,
            bgm_override: None,
        }
    }
}

impl GameObj for SteamBike {
    fn obj_id(&self) -> u32 {
        self.obj_id
    }

    fn name(&self) -> &str {
        "SteamBike"
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

    fn update(&mut self, _dt: f32, _world: &mut WorldContext) {}

    fn on_interact(&mut self, _interactor_id: u32, _world: &mut WorldContext) -> Option<GameEvent> {
        if self.ride_state == BikeRideState::Dismounted {
            self.mount();
        } else {
            self.dismount();
        }
        None
    }

    fn on_trigger_enter(&mut self, _other_id: u32, _world: &mut WorldContext) {}
}
