//! Player Character (`ccGobjPc`).
//! Reconstructed from CyberConnect2 player object decompilation.

use glam::Vec3;
use crate::object::GameObj;
use crate::context::WorldContext;
use crate::event::GameEvent;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PlayerLocomotionState {
    Idle,
    Walking,
    Running,
    Talking,
    Locked,
    Warping,
}

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponType {
    Unarmed,
    TwinBlades,
    Broadsword,
    Scythe,
    DualGuns,
}

impl WeaponType {
    pub fn idle_anim_file(&self) -> &'static str {
        match self {
            Self::Unarmed => "2hsw01nut00.ccs",
            Self::TwinBlades => "2tbl10rdy00.ccs",
            Self::Broadsword => "2hsw20rdy00.ccs",
            Self::Scythe => "2hsw34rdy00.ccs",
            Self::DualGuns => "2hsw54rdy00.ccs",
        }
    }

    pub fn run_anim_file(&self) -> &'static str {
        match self {
            Self::Unarmed => "2hsw01run00.ccs",
            Self::TwinBlades => "2tbl10str00.ccs",
            Self::Broadsword => "2hsw20str00.ccs",
            Self::Scythe => "2hsw34str00.ccs",
            Self::DualGuns => "2hsw54str00.ccs",
        }
    }
}

pub struct PlayerCharacter {
    pub id: u32,
    pub name: String,
    pub active: bool,
    pub position: Vec3,
    pub rotation: Vec3, // roll, pitch, yaw (in radians, yaw around +Z)
    pub scale: Vec3,

    pub velocity: Vec3,
    pub move_speed: f32, // max run speed: 5.5 m/s
    pub walk_speed: f32, // walk speed: 2.2 m/s
    pub state: PlayerLocomotionState,
    pub weapon: WeaponType,

    pub control_locked: bool,
    pub interaction_target_id: Option<u32>,
}

impl PlayerCharacter {
    pub fn new(spawn_pos: Vec3, spawn_yaw: f32) -> Self {
        Self {
            id: 1,
            name: "Haseo".into(),
            active: true,
            position: spawn_pos,
            rotation: Vec3::new(0.0, 0.0, spawn_yaw),
            scale: Vec3::ONE,
            velocity: Vec3::ZERO,
            move_speed: 5.5,
            walk_speed: 2.2,
            state: PlayerLocomotionState::Idle,
            weapon: WeaponType::TwinBlades,
            control_locked: false,
            interaction_target_id: None,
        }
    }

    pub fn set_weapon(&mut self, weapon: WeaponType) {
        self.weapon = weapon;
    }

    /// Process input motion in camera-relative coordinate space.
    pub fn process_input(
        &mut self,
        dt: f32,
        input_fwd: f32,
        input_right: f32,
        camera_yaw: f32,
        world: &WorldContext,
    ) {
        if self.control_locked || self.state == PlayerLocomotionState::Locked || self.state == PlayerLocomotionState::Talking {
            self.velocity = Vec3::ZERO;
            self.state = PlayerLocomotionState::Idle;
            return;
        }

        // Camera direction vectors (+Z Up convention)
        // Camera eye is at target + [dist * sin(yaw), -dist * cos(yaw), ...]
        // Looking towards target means forward is [-sin(yaw), cos(yaw)]
        let cam_fwd = Vec3::new(-camera_yaw.sin(), camera_yaw.cos(), 0.0).normalize_or_zero();
        let cam_right = Vec3::new(camera_yaw.cos(), camera_yaw.sin(), 0.0).normalize_or_zero();

        let move_dir = (cam_fwd * input_fwd + cam_right * input_right).normalize_or_zero();

        if move_dir.length_squared() > 0.01 {
            let target_speed = self.move_speed;
            self.velocity = move_dir * target_speed;
            self.state = PlayerLocomotionState::Running;

            // Turn player smoothly towards movement direction
            // In CC2 model bind space, model faces -Y. So angle is atan2(dx, -dy)
            let target_yaw = move_dir.x.atan2(-move_dir.y);
            let mut diff = target_yaw - self.rotation.z;
            while diff > std::f32::consts::PI {
                diff -= 2.0 * std::f32::consts::PI;
            }
            while diff < -std::f32::consts::PI {
                diff += 2.0 * std::f32::consts::PI;
            }
            self.rotation.z += diff * (18.0 * dt).min(1.0);

            // Compute candidate next position
            let candidate_pos = self.position + self.velocity * dt;

            // Query authentic collision ground height from stage mesh
            if let Some(ground_z) = world.ground_height(candidate_pos.x, candidate_pos.y, self.position.z) {
                // If elevation change is walkable (slope or step <= 0.6m)
                if (ground_z - self.position.z).abs() <= 0.6 {
                    self.position.x = candidate_pos.x;
                    self.position.y = candidate_pos.y;
                    self.position.z = ground_z;
                } else {
                    // Try sliding along X or Y axis
                    if let Some(gz_x) = world.ground_height(candidate_pos.x, self.position.y, self.position.z) {
                        if (gz_x - self.position.z).abs() <= 0.6 {
                            self.position.x = candidate_pos.x;
                            self.position.z = gz_x;
                        }
                    } else if let Some(gz_y) = world.ground_height(self.position.x, candidate_pos.y, self.position.z) {
                        if (gz_y - self.position.z).abs() <= 0.6 {
                            self.position.y = candidate_pos.y;
                            self.position.z = gz_y;
                        }
                    }
                }
            } else {
                // Keep current position if candidate is outside walkable geometry bounds
                self.velocity = Vec3::ZERO;
            }
        } else {
            self.velocity = Vec3::ZERO;
            self.state = PlayerLocomotionState::Idle;

            // Snap accurately to ground when resting
            if let Some(ground_z) = world.ground_height(self.position.x, self.position.y, self.position.z) {
                self.position.z = ground_z;
            }
        }
    }
}

impl GameObj for PlayerCharacter {
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

    fn update(&mut self, _dt: f32, world: &mut WorldContext) {
        // Update world camera target to follow player chest level (+1.2m)
        world.camera_target = self.position + Vec3::new(0.0, 0.0, 1.2);
    }

    fn on_interact(&mut self, _interactor_id: u32, _world: &mut WorldContext) -> Option<GameEvent> {
        None
    }

    fn on_trigger_enter(&mut self, _other_id: u32, _world: &mut WorldContext) {}
}
