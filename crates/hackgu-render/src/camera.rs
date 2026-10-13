use glam::{Mat4, Vec3};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
    pub cam_pos: [f32; 4], // xyz + padding
}

pub struct Camera {
    pub target: Vec3,
    pub distance: f32,
    /// Distance after collision clipping (None = unclipped). Set each frame by the follow camera.
    pub clip_distance: Option<f32>,
    pub yaw: f32,   // in radians
    pub pitch: f32, // in radians
    pub fov_y: f32,
    pub aspect: f32,
    pub z_near: f32,
    pub z_far: f32,
    /// Explicit eye position (event/script cameras). When set, `eye_position()` returns it and the orbit
    /// parameters are ignored.
    pub eye_override: Option<Vec3>,
    /// Camera roll in radians (event cameras).
    pub roll: f32,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            target: Vec3::new(0.0, 0.0, 1.0),
            distance: 2.5,
            clip_distance: None,
            yaw: 0.35,
            pitch: 0.15,
            fov_y: 45.0f32.to_radians(),
            aspect: (width / height).max(0.1),
            z_near: 0.05,
            z_far: 2000.0,
            eye_override: None,
            roll: 0.0,
        }
    }

    /// Pin the camera to an explicit eye/target pair (metres).
    pub fn look_from(&mut self, eye: Vec3, target: Vec3) {
        self.eye_override = Some(eye);
        self.target = target;
    }

    pub fn eye_position(&self) -> Vec3 {
        if let Some(e) = self.eye_override {
            return e;
        }
        // In CC2: X=Right, Y=Depth (forward/back), Z=Up
        let cos_p = self.pitch.cos();
        let sin_p = self.pitch.sin();
        let cos_y = self.yaw.cos();
        let sin_y = self.yaw.sin();

        let d = self.clip_distance.unwrap_or(self.distance).min(self.distance);
        let offset = Vec3::new(
            d * cos_p * sin_y,
            -d * cos_p * cos_y,
            d * sin_p,
        );

        self.target + offset
    }

    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let eye = self.eye_position();
        let fwd = (self.target - eye).normalize_or_zero();
        let mut up = Vec3::Z;
        if self.roll != 0.0 && fwd != Vec3::ZERO {
            up = glam::Quat::from_axis_angle(fwd, self.roll) * up;
        }
        let view = Mat4::look_at_rh(eye, self.target, up);
        let proj = Mat4::perspective_infinite_reverse_rh(self.fov_y, self.aspect, self.z_near);
        proj * view
    }

    pub fn to_uniform(&self) -> CameraUniform {
        let view_proj = self.build_view_projection_matrix();
        let eye = self.eye_position();
        CameraUniform {
            view_proj: view_proj.to_cols_array_2d(),
            cam_pos: [eye.x, eye.y, eye.z, 1.0],
        }
    }

    pub fn orbit(&mut self, dx: f32, dy: f32) {
        let sensitivity = 0.005;
        self.yaw -= dx * sensitivity;
        self.pitch += dy * sensitivity;
        // Clamp pitch to avoid gimbal flip
        let limit = 89.0f32.to_radians();
        self.pitch = self.pitch.clamp(-limit, limit);
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
        let pan_speed = 0.0015 * self.distance.max(0.1);
        let cos_y = self.yaw.cos();
        let sin_y = self.yaw.sin();

        // Right vector on horizontal plane
        let right = Vec3::new(cos_y, sin_y, 0.0);
        let up = Vec3::Z;

        self.target += (-right * dx + up * dy) * pan_speed;
    }

    pub fn zoom(&mut self, delta: f32) {
        let zoom_factor = if delta > 0.0 { 0.85 } else { 1.15 };
        self.distance = (self.distance * zoom_factor).clamp(0.05, 5000.0);
    }

    pub fn focus_bounds(&mut self, min: Vec3, max: Vec3) {
        self.target = (min + max) * 0.5;
        let span = (max - min).length();
        self.distance = (span * 1.3).max(0.5);
        self.z_near = (self.distance * 0.001).max(0.05);
        self.z_far = (self.distance * 5.0).max(10000.0);
    }
}
