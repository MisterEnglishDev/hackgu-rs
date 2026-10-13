//! Player character controller for third-person WASD exploration.
//! Integrates authentic 3D stage collision meshes extracted directly from stage CCS files.

/// Tallest ledge the player steps up without stopping (GUESS: fitted so dungeon entrance lips of 0.6-0.65 m, e.g. the
/// floor-up hall in dungeon2, can be climbed; the original's value is not traced).
const STEP_HEIGHT: f32 = 0.7;

use glam::Vec3;
use hackgu_world::CollisionMesh;

#[derive(Clone, Debug)]
pub struct PlayerController {
    pub position: Vec3,
    pub yaw: f32, // in radians around +Z axis
    pub move_speed: f32, // meters per second
    pub is_moving: bool,
    pub active: bool, // when true, WASD controls character movement instead of pure orbit
    pub bounds_min: [f32; 2],
    pub bounds_max: [f32; 2],
    pub collision_mesh: Option<CollisionMesh>,
}

impl PlayerController {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 25.0, 12.0), // Spawn at authentic Mac Anu Central Plaza height (12.0m)
            yaw: std::f32::consts::PI,           // Face forward (+Y) away from camera
            move_speed: 5.5,                     // Responsive 5.5 m/s run speed
            is_moving: false,
            active: false,
            bounds_min: [-350.0, -200.0],
            bounds_max: [350.0, 300.0],
            collision_mesh: None,
        }
    }

    /// Set or update the authentic collision mesh extracted from the stage.
    pub fn set_collision(&mut self, mesh: CollisionMesh) {
        // Query floor height at current spawn
        if let Some(h) = mesh.get_ground_height(self.position.x, self.position.y, self.position.z) {
            self.position.z = h;
        }
        self.collision_mesh = Some(mesh);
    }

    /// Update player position and yaw based on input keys and camera orientation,
    /// conforming smoothly to authentic stage collision triangles.
    pub fn update(
        &mut self,
        dt: f32,
        w_pressed: bool,
        s_pressed: bool,
        a_pressed: bool,
        d_pressed: bool,
        analog_x: f32,
        analog_y: f32,
        camera_yaw: f32,
    ) {
        if !self.active {
            self.is_moving = false;
            return;
        }

        // Camera planar directions (in CC2: X=Right, Y=Depth/Forward, Z=Up)
        // Camera eye is at target + [dist * sin(yaw), -dist * cos(yaw), ...]
        // Looking towards target means forward is [-sin(yaw), cos(yaw)]
        let fwd = Vec3::new(-camera_yaw.sin(), camera_yaw.cos(), 0.0).normalize_or_zero();
        let right = Vec3::new(camera_yaw.cos(), camera_yaw.sin(), 0.0).normalize_or_zero();

        let mut input_dir = Vec3::ZERO;
        if w_pressed {
            input_dir += fwd;
        }
        if s_pressed {
            input_dir -= fwd;
        }
        if d_pressed {
            input_dir += right;
        }
        if a_pressed {
            input_dir -= right;
        }

        // Analog stick input (Right = +X, Up/Forward = +Y)
        let stick_len = (analog_x * analog_x + analog_y * analog_y).sqrt();
        if stick_len > 0.15 {
            input_dir += right * analog_x + fwd * analog_y;
        }

        if input_dir.length_squared() > 0.01 {
            let move_dir = input_dir.normalize();
            self.is_moving = true;
            let speed_mult = if stick_len > 0.15 { stick_len.min(1.0) } else { 1.0 };
            let step = move_dir * (self.move_speed * speed_mult) * dt;
            let candidate_pos = self.position + step;

            // Authentic collision: slide against HitMesh walls, then conform to the floor.
            if let Some(ref col) = self.collision_mesh {
                let from = glam::Vec2::new(self.position.x, self.position.y);
                let to = glam::Vec2::new(candidate_pos.x, candidate_pos.y);
                let resolved = if col.has_walls() {
                    col.resolve_walls(from, to, self.position.z, 0.4, 1.8, STEP_HEIGHT)
                } else {
                    to
                };
                match col.get_walk_height(resolved.x, resolved.y, self.position.z, STEP_HEIGHT) {
                    Some(h) if (h - self.position.z).abs() <= 0.8_f32.max(STEP_HEIGHT + 0.1) => {
                        self.position = Vec3::new(resolved.x, resolved.y, h);
                    }
                    _ => {
                        // No walkable floor ahead: try sliding along each axis.
                        for t in [glam::Vec2::new(resolved.x, from.y), glam::Vec2::new(from.x, resolved.y)] {
                            if let Some(h) = col.get_walk_height(t.x, t.y, self.position.z, STEP_HEIGHT) {
                                if (h - self.position.z).abs() <= 0.8_f32.max(STEP_HEIGHT + 0.1) {
                                    self.position = Vec3::new(t.x, t.y, h);
                                    break;
                                }
                            }
                        }
                    }
                }
            } else {
                self.position += step;
            }

            // Clamp to walkable bounds
            self.position.x = self.position.x.clamp(self.bounds_min[0], self.bounds_max[0]);
            self.position.y = self.position.y.clamp(self.bounds_min[1], self.bounds_max[1]);

            // Turn player towards move_dir
            // In CC2 character bind/anim pose, default orientation faces -Y.
            // Rotating by atan2(dx, -dy) aligns character's facing with move_dir.
            let target_yaw = move_dir.x.atan2(-move_dir.y);

            // Smooth angular rotation
            let mut diff = target_yaw - self.yaw;
            while diff > std::f32::consts::PI {
                diff -= 2.0 * std::f32::consts::PI;
            }
            while diff < -std::f32::consts::PI {
                diff += 2.0 * std::f32::consts::PI;
            }
            self.yaw += diff * (18.0 * dt).min(1.0);
        } else {
            self.is_moving = false;

            // Snap accurately to floor when standing still
            if let Some(ref col) = self.collision_mesh {
                if let Some(ground_h) = col.get_ground_height(self.position.x, self.position.y, self.position.z) {
                    self.position.z = ground_h;
                }
            }
        }
    }

    /// Compute world transform matrix for player root position and rotation.
    pub fn world_matrix(&self) -> hackgu_ccs::Mat4 {
        let rot = hackgu_ccs::Mat4::from_rot_z(self.yaw);
        let trans = hackgu_ccs::Mat4::from_translation([self.position.x, self.position.y, self.position.z]);
        trans.mul(&rot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_analog_stick_movement() {
        let mut controller = PlayerController::new();
        controller.active = true;
        let initial_pos = controller.position;

        // Stick within deadzone (<= 0.15) -> No movement
        controller.update(0.1, false, false, false, false, 0.1, 0.1, 0.0);
        assert!(!controller.is_moving);
        assert_eq!(controller.position, initial_pos);

        // Stick deflected forward (analog_y = 1.0) -> Moves forward
        controller.update(0.1, false, false, false, false, 0.0, 1.0, 0.0);
        assert!(controller.is_moving);
        assert!(controller.position.y > initial_pos.y);

        // Stick deflected right (analog_x = 1.0) -> Moves right
        let mid_pos = controller.position;
        controller.update(0.1, false, false, false, false, 1.0, 0.0, 0.0);
        assert!(controller.is_moving);
        assert!(controller.position.x > mid_pos.x);
    }
}
