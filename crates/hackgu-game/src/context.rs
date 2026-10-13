//! Simulation World Context.
//! Holds all active game objects, stage collision meshes, story flags, and event queues.

use std::collections::HashMap;
use glam::Vec3;
use hackgu_world::CollisionMesh;
use crate::object::GameObj;
use crate::event::{EventQueue, GameEvent};

pub struct WorldContext {
    pub collision_mesh: CollisionMesh,
    pub entities: Vec<Box<dyn GameObj>>,
    pub flags: HashMap<u32, i32>,
    pub event_queue: EventQueue,
    pub camera_target: Vec3,
    pub player_id: u32,
}

impl WorldContext {
    pub fn new() -> Self {
        Self {
            collision_mesh: CollisionMesh::empty(),
            entities: Vec::new(),
            flags: HashMap::new(),
            event_queue: EventQueue::new(),
            camera_target: Vec3::ZERO,
            player_id: 1,
        }
    }

    /// Add an entity to the world.
    pub fn add_entity(&mut self, entity: Box<dyn GameObj>) {
        self.entities.push(entity);
    }

    /// Set or update the authentic stage collision mesh.
    pub fn set_collision_mesh(&mut self, mesh: CollisionMesh) {
        self.collision_mesh = mesh;
    }

    /// Get ground height at (x, y) near current Z.
    pub fn ground_height(&self, x: f32, y: f32, current_z: f32) -> Option<f32> {
        self.collision_mesh.get_ground_height(x, y, current_z)
    }

    /// Query global story flag.
    pub fn get_flag(&self, flag_id: u32) -> i32 {
        self.flags.get(&flag_id).copied().unwrap_or(0)
    }

    /// Set global story flag (`ccEventFlag`).
    pub fn set_flag(&mut self, flag_id: u32, value: i32) {
        self.flags.insert(flag_id, value);
    }

    /// Queue a script event for execution.
    pub fn queue_event(&mut self, event: GameEvent) {
        self.event_queue.push(event);
    }

    /// Advance game world by `dt` seconds.
    pub fn update(&mut self, dt: f32) {
        // Step active events
        self.event_queue.step(dt);

        // Update entities (swap out entities temporarily to satisfy borrow checker)
        let mut active_entities = std::mem::take(&mut self.entities);
        for entity in &mut active_entities {
            if entity.is_active() {
                entity.update(dt, self);
            }
        }
        self.entities = active_entities;
    }
}
