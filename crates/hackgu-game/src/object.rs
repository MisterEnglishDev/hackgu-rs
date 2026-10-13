//! Core Game Object Base (`ccGameObj` / `ccGobjDraw`).
//! Reconstructed from CC2 reverse-engineered class hierarchy and virtual method tables.

use glam::Vec3;
use hackgu_ccs::Mat4;
use crate::context::WorldContext;
use crate::event::GameEvent;

/// Base trait for all CyberConnect2 engine game objects (`ccGameObj`).
/// Mirrors the 14 virtual functions discovered in the game binary's vftable (e.g. 0x008c0438).
pub trait GameObj: Send + Sync {
    /// Object instance identifier.
    fn obj_id(&self) -> u32;

    /// Object type and debug name.
    fn name(&self) -> &str;

    /// Active status flag (paused / rendered / updating).
    fn is_active(&self) -> bool;
    fn set_active(&mut self, active: bool);

    /// Current 3D position in stage coordinate space (+Z is Up).
    fn position(&self) -> Vec3;
    fn set_position(&mut self, pos: Vec3);

    /// Current rotation in Euler radians (roll, pitch, yaw).
    fn rotation(&self) -> Vec3;
    fn set_rotation(&mut self, rot: Vec3);

    /// Current scale factor.
    fn scale(&self) -> Vec3;
    fn set_scale(&mut self, scale: Vec3);

    /// World matrix transform for model rendering and bone hierarchy placement.
    fn world_transform(&self) -> Mat4 {
        let p = self.position();
        let r = self.rotation();
        let s = self.scale();

        let trans = Mat4::from_translation([p.x, p.y, p.z]);
        let rot_z = Mat4::from_rot_z(r.z);
        let rot_y = Mat4::from_rot_y(r.y);
        let rot_x = Mat4::from_rot_x(r.x);
        let scale = Mat4::from_scale([s.x, s.y, s.z]);

        trans.mul(&rot_z).mul(&rot_y).mul(&rot_x).mul(&scale)
    }

    /// Per-frame simulation update step (virtual method #6).
    fn update(&mut self, dt: f32, world: &mut WorldContext);

    /// Interaction trigger when player presses action button near object (virtual method #11).
    fn on_interact(&mut self, interactor_id: u32, world: &mut WorldContext) -> Option<GameEvent>;

    /// Collision volume trigger when an entity enters object radius (virtual method #12).
    fn on_trigger_enter(&mut self, other_id: u32, world: &mut WorldContext);
}
