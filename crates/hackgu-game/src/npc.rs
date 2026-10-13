//! Non-Player Character (`ccGobjNpc`) and Sub-Player Companion (`ccGobjSpc`).
//! Reconstructed from CyberConnect2 engine decompilation.

use glam::Vec3;
use crate::object::GameObj;
use crate::context::WorldContext;
use crate::event::GameEvent;

pub struct NonPlayerCharacter {
    pub id: u32,
    pub name: String,
    pub model_file: String,
    pub active: bool,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,

    pub dialogue_lines: Vec<String>,
    pub dialogue_index: usize,
    pub interaction_radius: f32,
}

impl NonPlayerCharacter {
    pub fn new(id: u32, name: &str, model_file: &str, pos: Vec3, yaw: f32, dialogue: Vec<String>) -> Self {
        Self {
            id,
            name: name.to_string(),
            model_file: model_file.to_string(),
            active: true,
            position: pos,
            rotation: Vec3::new(0.0, 0.0, yaw),
            scale: Vec3::ONE,
            dialogue_lines: dialogue,
            dialogue_index: 0,
            interaction_radius: 2.5,
        }
    }
}

impl GameObj for NonPlayerCharacter {
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
        // Keep grounded to authentic stage collision floor
        if let Some(gz) = world.ground_height(self.position.x, self.position.y, self.position.z) {
            self.position.z = gz;
        }
    }

    fn on_interact(&mut self, _interactor_id: u32, _world: &mut WorldContext) -> Option<GameEvent> {
        if self.dialogue_lines.is_empty() {
            return None;
        }

        let text = self.dialogue_lines[self.dialogue_index].clone();
        self.dialogue_index = (self.dialogue_index + 1) % self.dialogue_lines.len();

        Some(GameEvent::ShowMessage {
            speaker: self.name.clone(),
            text,
        })
    }

    fn on_trigger_enter(&mut self, _other_id: u32, _world: &mut WorldContext) {}
}

/// Companion Sub-Player Character (`ccGobjSpc`, e.g. Atoli, Kuhn).
/// Follows the player with authentic party formation offset.
pub struct SubPlayerCharacter {
    pub id: u32,
    pub name: String,
    pub model_file: String,
    pub active: bool,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,

    pub formation_offset: Vec3,
    pub follow_speed: f32,
}

impl SubPlayerCharacter {
    pub fn new(id: u32, name: &str, model_file: &str, offset: Vec3) -> Self {
        Self {
            id,
            name: name.to_string(),
            model_file: model_file.to_string(),
            active: true,
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
            formation_offset: offset,
            follow_speed: 5.5,
        }
    }
}

impl GameObj for SubPlayerCharacter {
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
        // Find player position in world
        let player_pos = world.entities.iter().find(|e| e.obj_id() == world.player_id).map(|e| e.position());

        if let Some(target_base) = player_pos {
            let target = target_base + self.formation_offset;
            let dist = self.position.distance(target);

            if dist > 1.2 {
                let dir = (target - self.position).normalize_or_zero();
                let step = (dir * self.follow_speed * dt).min(target - self.position);
                self.position += step;

                // Turn towards walk direction
                self.rotation.z = dir.x.atan2(-dir.y);
            }

            // Snap to authentic ground height
            if let Some(gz) = world.ground_height(self.position.x, self.position.y, self.position.z) {
                self.position.z = gz;
            }
        }
    }

    fn on_interact(&mut self, _interactor_id: u32, _world: &mut WorldContext) -> Option<GameEvent> {
        Some(GameEvent::ShowMessage {
            speaker: self.name.clone(),
            text: format!("Ready when you are, Haseo!"),
        })
    }

    fn on_trigger_enter(&mut self, _other_id: u32, _world: &mut WorldContext) {}
}
