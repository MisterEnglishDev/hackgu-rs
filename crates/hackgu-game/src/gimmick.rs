//! Interactive Gimmicks (`ccGobjGim*`).
//! Reconstructed from CyberConnect2 engine decompilation.

use glam::Vec3;
use crate::object::GameObj;
use crate::context::WorldContext;
use crate::event::GameEvent;

/// The Chaos Gate warp device (`ccGobjGimChaosGate`).
/// Central warp hub connecting towns, fields, and dungeons.
pub struct ChaosGate {
    pub id: u32,
    pub name: String,
    pub active: bool,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub is_spinning: bool,
    pub ring_angle: f32,
}

impl ChaosGate {
    pub fn new(id: u32, pos: Vec3) -> Self {
        Self {
            id,
            name: "ChaosGate".into(),
            active: true,
            position: pos,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
            is_spinning: true,
            ring_angle: 0.0,
        }
    }
}

impl GameObj for ChaosGate {
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

    fn update(&mut self, dt: f32, _world: &mut WorldContext) {
        if self.is_spinning {
            self.ring_angle += 1.5 * dt;
            self.rotation.z = self.ring_angle;
        }
    }

    fn on_interact(&mut self, _interactor_id: u32, _world: &mut WorldContext) -> Option<GameEvent> {
        Some(GameEvent::OpenChaosGate)
    }

    fn on_trigger_enter(&mut self, _other_id: u32, _world: &mut WorldContext) {}
}

/// Chim Chim Kick Barrel (`ccGobjGimTimBarrel`).
/// Awards Chim Chims upon being kicked 3 times.
#[derive(Debug, Clone)]
pub struct TimBarrel {
    pub id: u32,
    pub name: String,
    pub active: bool,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub remaining_kicks: u32,
    pub chim_reward: u32,
}

impl TimBarrel {
    pub fn new(id: u32, pos: Vec3) -> Self {
        Self {
            id,
            name: "TimBarrel".into(),
            active: true,
            position: pos,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
            remaining_kicks: 3,
            chim_reward: 5,
        }
    }
}

impl GameObj for TimBarrel {
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

    fn update(&mut self, _dt: f32, _world: &mut WorldContext) {}

    fn on_interact(&mut self, _interactor_id: u32, _world: &mut WorldContext) -> Option<GameEvent> {
        if self.remaining_kicks > 0 {
            self.remaining_kicks -= 1;
            Some(GameEvent::KickChimBarrel {
                barrel_id: self.id,
                chim_count: self.chim_reward,
            })
        } else {
            None
        }
    }

    fn on_trigger_enter(&mut self, _other_id: u32, _world: &mut WorldContext) {}
}

/// Treasure Chest (`ccGobjGimItemBox`).
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ChestKind {
    Wood,
    Silver,
    Gold,
    Beast,
}

#[derive(Debug, Clone)]
pub struct ItemBox {
    pub id: u32,
    pub name: String,
    pub kind: ChestKind,
    pub active: bool,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub is_opened: bool,
    pub item_name: String,
}

impl ItemBox {
    pub fn new(id: u32, pos: Vec3, kind: ChestKind, item_name: &str) -> Self {
        Self {
            id,
            name: "ItemBox".into(),
            kind,
            active: true,
            position: pos,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
            is_opened: false,
            item_name: item_name.to_string(),
        }
    }
}

impl GameObj for ItemBox {
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

    fn update(&mut self, _dt: f32, _world: &mut WorldContext) {}

    fn on_interact(&mut self, _interactor_id: u32, _world: &mut WorldContext) -> Option<GameEvent> {
        if !self.is_opened {
            self.is_opened = true;
            Some(GameEvent::ShowMessage {
                speaker: "Treasure".into(),
                text: format!("Obtained [{}]!", self.item_name),
            })
        } else {
            Some(GameEvent::ShowMessage {
                speaker: "Treasure".into(),
                text: "The treasure chest is empty.".into(),
            })
        }
    }

    fn on_trigger_enter(&mut self, _other_id: u32, _world: &mut WorldContext) {}
}

/// Town Transit Boat & Warp Point (`ccGobjGimTownWarp`).
pub struct TownWarp {
    pub id: u32,
    pub name: String,
    pub active: bool,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub destination_district: String,
    pub target_pos: Vec3,
}

impl TownWarp {
    pub fn new(id: u32, pos: Vec3, dest_name: &str, target_pos: Vec3) -> Self {
        Self {
            id,
            name: "TownWarp".into(),
            active: true,
            position: pos,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
            destination_district: dest_name.to_string(),
            target_pos,
        }
    }
}

impl GameObj for TownWarp {
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

    fn update(&mut self, _dt: f32, _world: &mut WorldContext) {}

    fn on_interact(&mut self, _interactor_id: u32, _world: &mut WorldContext) -> Option<GameEvent> {
        Some(GameEvent::PlayerInit {
            spawn_pos: self.target_pos,
            spawn_yaw: 0.0,
        })
    }

    fn on_trigger_enter(&mut self, _other_id: u32, _world: &mut WorldContext) {}
}
