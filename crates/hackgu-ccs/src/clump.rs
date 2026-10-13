//! CyberConnect2 Skeleton and Clump (0x0900) parser and forward kinematics.

use std::collections::HashMap;
use std::fmt;

use crate::objects::CcsObjectNode;

#[derive(Debug)]
pub enum ClumpError {
    TooShort,
    InvalidData(String),
}

impl fmt::Display for ClumpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClumpError::TooShort => write!(f, "Payload too short for clump"),
            ClumpError::InvalidData(s) => write!(f, "Invalid clump data: {}", s),
        }
    }
}

impl std::error::Error for ClumpError {}

/// A 4x4 row-major transformation matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat4(pub [f32; 16]);

impl Mat4 {
    pub const IDENTITY: Self = Self([
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]);

    pub fn from_translation(t: [f32; 3]) -> Self {
        Self([
            1.0, 0.0, 0.0, t[0],
            0.0, 1.0, 0.0, t[1],
            0.0, 0.0, 1.0, t[2],
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn from_scale(s: [f32; 3]) -> Self {
        Self([
            s[0], 0.0,  0.0,  0.0,
            0.0,  s[1], 0.0,  0.0,
            0.0,  0.0,  s[2], 0.0,
            0.0,  0.0,  0.0,  1.0,
        ])
    }

    pub fn from_rot_x(rad: f32) -> Self {
        let (s, c) = rad.sin_cos();
        Self([
            1.0, 0.0,  0.0, 0.0,
            0.0,   c,   -s, 0.0,
            0.0,   s,    c, 0.0,
            0.0, 0.0,  0.0, 1.0,
        ])
    }

    pub fn from_rot_y(rad: f32) -> Self {
        let (s, c) = rad.sin_cos();
        Self([
              c, 0.0,   s, 0.0,
            0.0, 1.0, 0.0, 0.0,
             -s, 0.0,   c, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn from_rot_z(rad: f32) -> Self {
        let (s, c) = rad.sin_cos();
        Self([
              c,  -s, 0.0, 0.0,
              s,   c, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    /// Construct rotation from Euler angles (in degrees) using standard 3ds Max / CC2 XYZ order.
    pub fn from_euler_xyz(deg: [f32; 3]) -> Self {
        let rx = deg[0].to_radians();
        let ry = deg[1].to_radians();
        let rz = deg[2].to_radians();
        let mx = Self::from_rot_x(rx);
        let my = Self::from_rot_y(ry);
        let mz = Self::from_rot_z(rz);
        mx.mul(&my).mul(&mz)
    }

    /// Construct rotation matrix from unit quaternion [x, y, z, w].
    pub fn from_quat(q: [f32; 4]) -> Self {
        let x = q[0];
        let y = q[1];
        let z = q[2];
        let w = q[3];

        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;

        let xx = x * x2;
        let xy = x * y2;
        let xz = x * z2;
        let yy = y * y2;
        let yz = y * z2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;

        Self([
            1.0 - (yy + zz), xy - wz,         xz + wy,         0.0,
            xy + wz,         1.0 - (xx + zz), yz - wx,         0.0,
            xz - wy,         yz + wx,         1.0 - (xx + yy), 0.0,
            0.0,             0.0,             0.0,             1.0,
        ])
    }

    /// Multiply two quaternions [x, y, z, w].
    pub fn quat_mul(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
        let [x1, y1, z1, w1] = a;
        let [x2, y2, z2, w2] = b;
        [
            w1 * x2 + x1 * w2 + y1 * z2 - z1 * y2,
            w1 * y2 - x1 * z2 + y1 * w2 + z1 * x2,
            w1 * z2 + x1 * y2 - y1 * x2 + z1 * w2,
            w1 * w2 - x1 * x2 - y1 * y2 - z1 * z2,
        ]
    }

    /// Convert Euler angles in degrees (XYZ order) to a normalized quaternion [x, y, z, w].
    pub fn euler_deg_to_quat(deg: [f32; 3]) -> [f32; 4] {
        let hx = deg[0].to_radians() * 0.5;
        let hy = deg[1].to_radians() * 0.5;
        let hz = deg[2].to_radians() * 0.5;

        let (sx, cx) = hx.sin_cos();
        let (sy, cy) = hy.sin_cos();
        let (sz, cz) = hz.sin_cos();

        let qx = [sx, 0.0, 0.0, cx];
        let qy = [0.0, sy, 0.0, cy];
        let qz = [0.0, 0.0, sz, cz];

        Self::quat_mul(Self::quat_mul(qx, qy), qz)
    }

    /// Standard matrix multiplication (C = A * B).
    pub fn mul(&self, rhs: &Self) -> Self {
        let mut out = [0.0f32; 16];
        for row in 0..4 {
            for col in 0..4 {
                let mut sum = 0.0f32;
                for k in 0..4 {
                    sum += self.0[row * 4 + k] * rhs.0[k * 4 + col];
                }
                out[row * 4 + col] = sum;
            }
        }
        Self(out)
    }

    /// Transform an affine 3D point (w = 1.0).
    pub fn transform_point(&self, p: [f32; 3]) -> [f32; 3] {
        [
            self.0[0] * p[0] + self.0[1] * p[1] + self.0[2] * p[2] + self.0[3],
            self.0[4] * p[0] + self.0[5] * p[1] + self.0[6] * p[2] + self.0[7],
            self.0[8] * p[0] + self.0[9] * p[1] + self.0[10] * p[2] + self.0[11],
        ]
    }

    /// Transform a 3D direction vector (w = 0.0, ignoring translation).
    pub fn transform_vector(&self, v: [f32; 3]) -> [f32; 3] {
        [
            self.0[0] * v[0] + self.0[1] * v[1] + self.0[2] * v[2],
            self.0[4] * v[0] + self.0[5] * v[1] + self.0[6] * v[2],
            self.0[8] * v[0] + self.0[9] * v[1] + self.0[10] * v[2],
        ]
    }
}

/// A bone's local transform in the bind pose.
#[derive(Clone, Debug)]
pub struct CcsBoneTransform {
    pub translation: [f32; 3], // in meters
    pub rotation_deg: [f32; 3],
    pub scale: [f32; 3],
}

/// A parsed Clump object (0x0900) representing a skeleton hierarchy.
#[derive(Clone, Debug)]
pub struct CcsClump {
    pub object_id: u32,
    pub bone_count: usize,
    pub bone_obj_ids: Vec<u32>,
    pub transforms: Vec<CcsBoneTransform>,
}

impl CcsClump {
    /// Parse a Clump object from its binary payload.
    pub fn from_payload(object_id: u32, payload: &[u8]) -> Result<Self, ClumpError> {
        if payload.len() < 4 {
            return Err(ClumpError::TooShort);
        }

        let bone_count = u32::from_le_bytes(payload[0..4].try_into().unwrap()) as usize;
        let expected_len = 4 + bone_count * 4 + bone_count * 36;
        if payload.len() < expected_len {
            return Err(ClumpError::TooShort);
        }

        let mut bone_obj_ids = Vec::with_capacity(bone_count);
        for i in 0..bone_count {
            let off = 4 + i * 4;
            let oid = u32::from_le_bytes(payload[off..off + 4].try_into().unwrap());
            bone_obj_ids.push(oid);
        }

        let trans_start = 4 + bone_count * 4;
        let mut transforms = Vec::with_capacity(bone_count);
        for i in 0..bone_count {
            let off = trans_start + i * 36;
            // Binary translation is in centimeters; convert to meters (* 0.01) to match vertex scale
            let tx = f32::from_le_bytes(payload[off..off + 4].try_into().unwrap()) * 0.01;
            let ty = f32::from_le_bytes(payload[off + 4..off + 8].try_into().unwrap()) * 0.01;
            let tz = f32::from_le_bytes(payload[off + 8..off + 12].try_into().unwrap()) * 0.01;

            let rx = f32::from_le_bytes(payload[off + 12..off + 16].try_into().unwrap());
            let ry = f32::from_le_bytes(payload[off + 16..off + 20].try_into().unwrap());
            let rz = f32::from_le_bytes(payload[off + 20..off + 24].try_into().unwrap());

            let sx = f32::from_le_bytes(payload[off + 24..off + 28].try_into().unwrap());
            let sy = f32::from_le_bytes(payload[off + 28..off + 32].try_into().unwrap());
            let sz = f32::from_le_bytes(payload[off + 32..off + 36].try_into().unwrap());

            transforms.push(CcsBoneTransform {
                translation: [tx, ty, tz],
                rotation_deg: [rx, ry, rz],
                scale: [sx, sy, sz],
            });
        }

        Ok(Self {
            object_id,
            bone_count,
            bone_obj_ids,
            transforms,
        })
    }

    /// Compute world transformation matrices for every bone by traversing the scene graph.
    pub fn compute_world_matrices(&self, objects: &HashMap<u32, CcsObjectNode>) -> HashMap<u32, Mat4> {
        let obj_to_clump_idx: HashMap<u32, usize> = self
            .bone_obj_ids
            .iter()
            .enumerate()
            .map(|(idx, &oid)| (oid, idx))
            .collect();

        let mut world_cache: HashMap<u32, Mat4> = HashMap::new();

        for &oid in &self.bone_obj_ids {
            Self::resolve_world_matrix(oid, &obj_to_clump_idx, self, objects, &mut world_cache);
        }

        world_cache
    }

    fn resolve_world_matrix(
        oid: u32,
        obj_to_clump_idx: &HashMap<u32, usize>,
        clump: &Self,
        objects: &HashMap<u32, CcsObjectNode>,
        cache: &mut HashMap<u32, Mat4>,
    ) -> Mat4 {
        if let Some(&m) = cache.get(&oid) {
            return m;
        }

        let idx = match obj_to_clump_idx.get(&oid) {
            Some(&i) => i,
            None => return Mat4::IDENTITY,
        };

        let t = clump.transforms[idx].translation;
        let r = clump.transforms[idx].rotation_deg;
        let s = clump.transforms[idx].scale;

        let local = Mat4::from_translation(t)
            .mul(&Mat4::from_euler_xyz(r))
            .mul(&Mat4::from_scale(s));

        let parent_oid = objects.get(&oid).map(|o| o.parent_obj_id).unwrap_or(0);
        let world = if parent_oid == 0 || !obj_to_clump_idx.contains_key(&parent_oid) {
            local
        } else {
            let parent_world = Self::resolve_world_matrix(parent_oid, obj_to_clump_idx, clump, objects, cache);
            parent_world.mul(&local)
        };

        cache.insert(oid, world);
        world
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler_deg_to_quat() {
        for angles in [
            [0.0, 0.0, 0.0],
            [90.0, 0.0, 0.0],
            [0.0, 90.0, 0.0],
            [0.0, 0.0, 90.0],
            [45.0, -30.0, 60.0],
            [-90.0, 76.0, -90.0],
        ] {
            let m_euler = Mat4::from_euler_xyz(angles);
            let q = Mat4::euler_deg_to_quat(angles);
            let m_quat = Mat4::from_quat(q);
            for i in 0..16 {
                assert!(
                    (m_euler.0[i] - m_quat.0[i]).abs() < 1e-4,
                    "Mismatch at index {} for angles {:?}: euler={}, quat={}",
                    i, angles, m_euler.0[i], m_quat.0[i]
                );
            }
        }
    }
}
