//! CyberConnect2 Animation (0x0700) parser and keyframe sampling for .hack//G.U. Last Recode.

use std::collections::HashMap;
use std::fmt;

use crate::clump::{CcsClump, Mat4};
use crate::objects::CcsObjectNode;
use crate::parser::CcsFile;

#[derive(Debug)]
pub enum AnimError {
    TooShort,
    InvalidData(String),
}

impl fmt::Display for AnimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnimError::TooShort => write!(f, "Payload too short for animation"),
            AnimError::InvalidData(s) => write!(f, "Invalid animation data: {}", s),
        }
    }
}

impl std::error::Error for AnimError {}

#[derive(Clone, Copy, Debug)]
pub struct TranslationKey {
    pub frame: u32,
    pub translation: [f32; 3], // in meters
}

#[derive(Clone, Copy, Debug)]
pub struct RotationKey {
    pub frame: u32,
    pub quat: [f32; 4], // unit quaternion [x, y, z, w]
}

#[derive(Clone, Debug)]
pub struct AnimBoneChannel {
    pub bone_id: u32,
    pub bone_name: String,
    pub static_translation: Option<[f32; 3]>,
    pub translation_keys: Vec<TranslationKey>,
    pub static_rotation: Option<[f32; 3]>, // Euler angles in degrees
    pub rotation_keys: Vec<RotationKey>,
    /// Scale track (flag bits 6-8: 1 static, 2 keyed): (frame, [sx, sy, sz]).
    pub scale_keys: Vec<(u32, [f32; 3])>,
    /// Opacity track (flag bits 9-11: 1 static, 2 keyed): (frame, alpha 0..1). Used by 2D/UI scenes.
    pub alpha_keys: Vec<(u32, f32)>,
}

/// Sample a keyed track linearly (holds the end values outside the key range).
fn sample_keys<const N: usize>(keys: &[(u32, [f32; N])], frame: f32) -> Option<[f32; N]> {
    let first = keys.first()?;
    if frame <= first.0 as f32 {
        return Some(first.1);
    }
    for w in keys.windows(2) {
        let (f0, f1) = (w[0].0 as f32, w[1].0 as f32);
        if frame <= f1 {
            let t = if f1 > f0 { (frame - f0) / (f1 - f0) } else { 1.0 };
            let mut out = w[0].1;
            for i in 0..N {
                out[i] += (w[1].1[i] - w[0].1[i]) * t;
            }
            return Some(out);
        }
    }
    Some(keys.last()?.1)
}

impl AnimBoneChannel {
    /// Scale at `frame` (None when the animation has no scale track).
    pub fn sample_scale(&self, frame: f32) -> Option<[f32; 3]> {
        sample_keys(&self.scale_keys, frame)
    }

    /// Opacity at `frame` (None when the animation has no opacity track).
    pub fn sample_alpha(&self, frame: f32) -> Option<f32> {
        let k: Vec<(u32, [f32; 1])> = self.alpha_keys.iter().map(|&(f, a)| (f, [a])).collect();
        sample_keys(&k, frame).map(|v| v[0])
    }
    pub fn sample_translation(&self, frame: f32, bind_pose: [f32; 3]) -> [f32; 3] {
        if !self.translation_keys.is_empty() {
            let keys = &self.translation_keys;
            if frame <= keys[0].frame as f32 {
                return keys[0].translation;
            }
            if frame >= keys[keys.len() - 1].frame as f32 {
                return keys[keys.len() - 1].translation;
            }
            for i in 0..keys.len() - 1 {
                let f0 = keys[i].frame as f32;
                let f1 = keys[i + 1].frame as f32;
                if frame >= f0 && frame <= f1 {
                    let alpha = if f1 > f0 { (frame - f0) / (f1 - f0) } else { 0.0 };
                    let t0 = keys[i].translation;
                    let t1 = keys[i + 1].translation;
                    return [
                        t0[0] + (t1[0] - t0[0]) * alpha,
                        t0[1] + (t1[1] - t0[1]) * alpha,
                        t0[2] + (t1[2] - t0[2]) * alpha,
                    ];
                }
            }
            keys[keys.len() - 1].translation
        } else if let Some(t) = self.static_translation {
            t
        } else {
            bind_pose
        }
    }

    pub fn sample_rotation(&self, frame: f32, bind_euler: [f32; 3]) -> Mat4 {
        if !self.rotation_keys.is_empty() {
            let keys = &self.rotation_keys;
            if frame <= keys[0].frame as f32 {
                return Mat4::from_quat(keys[0].quat);
            }
            if frame >= keys[keys.len() - 1].frame as f32 {
                return Mat4::from_quat(keys[keys.len() - 1].quat);
            }
            for i in 0..keys.len() - 1 {
                let f0 = keys[i].frame as f32;
                let f1 = keys[i + 1].frame as f32;
                if frame >= f0 && frame <= f1 {
                    let alpha = if f1 > f0 { (frame - f0) / (f1 - f0) } else { 0.0 };
                    let q0 = keys[i].quat;
                    let mut q1 = keys[i + 1].quat;
                    // Ensure shortest arc interpolation
                    let dot = q0[0] * q1[0] + q0[1] * q1[1] + q0[2] * q1[2] + q0[3] * q1[3];
                    if dot < 0.0 {
                        q1 = [-q1[0], -q1[1], -q1[2], -q1[3]];
                    }
                    let nx = q0[0] + (q1[0] - q0[0]) * alpha;
                    let ny = q0[1] + (q1[1] - q0[1]) * alpha;
                    let nz = q0[2] + (q1[2] - q0[2]) * alpha;
                    let nw = q0[3] + (q1[3] - q0[3]) * alpha;
                    let len = (nx * nx + ny * ny + nz * nz + nw * nw).sqrt().max(1e-6);
                    return Mat4::from_quat([nx / len, ny / len, nz / len, nw / len]);
                }
            }
            Mat4::from_quat(keys[keys.len() - 1].quat)
        } else if let Some(r) = self.static_rotation {
            Mat4::from_euler_xyz(r)
        } else {
            Mat4::from_euler_xyz(bind_euler)
        }
    }
}

impl AnimBoneChannel {
    /// Rotation at `frame` as a unit quaternion [x, y, z, w] (same convention as `sample_rotation`).
    pub fn sample_rotation_quat(&self, frame: f32, bind_euler: [f32; 3]) -> [f32; 4] {
        if !self.rotation_keys.is_empty() {
            let keys = &self.rotation_keys;
            if frame <= keys[0].frame as f32 {
                return keys[0].quat;
            }
            for i in 0..keys.len() - 1 {
                let (f0, f1) = (keys[i].frame as f32, keys[i + 1].frame as f32);
                if frame >= f0 && frame <= f1 {
                    let alpha = if f1 > f0 { (frame - f0) / (f1 - f0) } else { 0.0 };
                    return nlerp(keys[i].quat, keys[i + 1].quat, alpha);
                }
            }
            keys[keys.len() - 1].quat
        } else if let Some(r) = self.static_rotation {
            Mat4::euler_deg_to_quat(r)
        } else {
            Mat4::euler_deg_to_quat(bind_euler)
        }
    }
}

/// Normalised linear quaternion interpolation along the shortest arc.
pub fn nlerp(q0: [f32; 4], q1: [f32; 4], t: f32) -> [f32; 4] {
    let dot = q0[0] * q1[0] + q0[1] * q1[1] + q0[2] * q1[2] + q0[3] * q1[3];
    let q1 = if dot < 0.0 { [-q1[0], -q1[1], -q1[2], -q1[3]] } else { q1 };
    let q = [
        q0[0] + (q1[0] - q0[0]) * t,
        q0[1] + (q1[1] - q0[1]) * t,
        q0[2] + (q1[2] - q0[2]) * t,
        q0[3] + (q1[3] - q0[3]) * t,
    ];
    let l = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt().max(1e-6);
    [q[0] / l, q[1] / l, q[2] / l, q[3] / l]
}

/// Per-bone local transform (translation in metres, rotation quaternion), indexed like `clump.bone_obj_ids`.
pub type LocalPose = Vec<([f32; 3], [f32; 4])>;

/// Bind pose of a clump as a `LocalPose`.
pub fn bind_local_pose(clump: &CcsClump) -> LocalPose {
    clump.transforms.iter().map(|t| (t.translation, Mat4::euler_deg_to_quat(t.rotation_deg))).collect()
}

/// Crossfade two local poses: `w` = 0 gives `a`, 1 gives `b`.
pub fn blend_local_poses(a: &LocalPose, b: &LocalPose, w: f32) -> LocalPose {
    a.iter()
        .zip(b.iter())
        .map(|((ta, qa), (tb, qb))| {
            ([ta[0] + (tb[0] - ta[0]) * w, ta[1] + (tb[1] - ta[1]) * w, ta[2] + (tb[2] - ta[2]) * w], nlerp(*qa, *qb, w))
        })
        .collect()
}

/// Compose a local pose into world matrices through the scene-graph parents (same rules as
/// `CcsAnimation::compute_world_matrices`).
pub fn world_from_local_pose(clump: &CcsClump, objects: &HashMap<u32, CcsObjectNode>, pose: &LocalPose) -> HashMap<u32, Mat4> {
    let idx: HashMap<u32, usize> = clump.bone_obj_ids.iter().enumerate().map(|(i, &o)| (o, i)).collect();
    let mut cache: HashMap<u32, Mat4> = HashMap::new();
    fn resolve(oid: u32, idx: &HashMap<u32, usize>, clump: &CcsClump, objects: &HashMap<u32, CcsObjectNode>, pose: &LocalPose, cache: &mut HashMap<u32, Mat4>) -> Mat4 {
        if let Some(&m) = cache.get(&oid) {
            return m;
        }
        let Some(&i) = idx.get(&oid) else { return Mat4::IDENTITY };
        let (t, q) = pose[i];
        let local = Mat4::from_translation(t).mul(&Mat4::from_quat(q)).mul(&Mat4::from_scale(clump.transforms[i].scale));
        let parent = objects.get(&oid).map(|o| o.parent_obj_id).unwrap_or(0);
        let world = if parent == 0 || !idx.contains_key(&parent) {
            local
        } else {
            resolve(parent, idx, clump, objects, pose, cache).mul(&local)
        };
        cache.insert(oid, world);
        world
    }
    for &oid in &clump.bone_obj_ids {
        resolve(oid, &idx, clump, objects, pose, &mut cache);
    }
    cache
}

/// A parsed CyberConnect2 Animation object (0x0700).
#[derive(Clone, Debug)]
pub struct CcsAnimation {
    pub object_id: u32,
    pub name: String,
    pub frame_count: u32,
    pub channels: Vec<AnimBoneChannel>,
    /// Name-to-channel index lookup
    pub channel_by_name: HashMap<String, usize>,
}

impl CcsAnimation {
    pub fn from_payload(object_id: u32, name: &str, payload: &[u8], ccs: &CcsFile) -> Result<Self, AnimError> {
        if payload.len() < 8 {
            return Err(AnimError::TooShort);
        }

        let frame_count = u32::from_le_bytes(payload[0..4].try_into().unwrap());
        let word_count = u32::from_le_bytes(payload[4..8].try_into().unwrap()) as usize;

        let words: Vec<u32> = payload
            .chunks_exact(4)
            .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
            .collect();

        if words.len() < 2 + word_count {
            return Err(AnimError::TooShort);
        }

        // Preamble: skip header (2 words) + 3 words preamble (ff01, 1, 0)
        let mut w = 5usize;
        let mut channels = Vec::new();
        let mut channel_by_name = HashMap::new();

        while w < words.len() {
            if w + 2 >= words.len() {
                break;
            }
            let block_len = words[w + 1] as usize;
            // 0xFF01 blocks are per-frame markers (their third word is the frame number), not object tracks.
            if words[w] & 0xffff == 0xff01 {
                w += 2 + block_len;
                continue;
            }
            let bone_id = words[w + 2];
            let bone_name = ccs.object_name(bone_id).unwrap_or("").to_string();

            let block_end = w + 2 + block_len;
            let mut cur = w + 3;

            let mut static_translation = None;
            let mut translation_keys = Vec::new();
            let mut static_rotation = None;
            let mut rotation_keys = Vec::new();
            let mut scale_keys: Vec<(u32, [f32; 3])> = Vec::new();
            let mut alpha_keys: Vec<(u32, f32)> = Vec::new();

            if cur < block_end {
                let type_flag = words[cur];
                cur += 1;

                // Bit 0 (0x01): Static Translation (3 floats, in centimeters)
                if (type_flag & 0x01) != 0 {
                    if cur + 3 <= block_end {
                        let tx = f32::from_bits(words[cur]) * 0.01;
                        let ty = f32::from_bits(words[cur + 1]) * 0.01;
                        let tz = f32::from_bits(words[cur + 2]) * 0.01;
                        static_translation = Some([tx, ty, tz]);
                        cur += 3;
                    }
                }

                // Bit 1 (0x02): Animated Translation (count + count * [frame, x, y, z])
                if (type_flag & 0x02) != 0 {
                    if cur < block_end {
                        let num_keys = words[cur] as usize;
                        cur += 1;
                        for _ in 0..num_keys {
                            if cur + 4 <= block_end {
                                let frame = words[cur];
                                let tx = f32::from_bits(words[cur + 1]) * 0.01;
                                let ty = f32::from_bits(words[cur + 2]) * 0.01;
                                let tz = f32::from_bits(words[cur + 3]) * 0.01;
                                translation_keys.push(TranslationKey {
                                    frame,
                                    translation: [tx, ty, tz],
                                });
                                cur += 4;
                            }
                        }
                    }
                }

                // Bit 3 (0x08): Static Rotation (3 floats Euler degrees)
                if (type_flag & 0x08) != 0 {
                    if cur + 3 <= block_end {
                        let rx = f32::from_bits(words[cur]);
                        let ry = f32::from_bits(words[cur + 1]);
                        let rz = f32::from_bits(words[cur + 2]);
                        static_rotation = Some([rx, ry, rz]);
                        cur += 3;
                    }
                }

                // Bit 4 (0x10): Animated Euler Rotation (count + count * [frame, rx, ry, rz])
                if (type_flag & 0x10) != 0 {
                    if cur < block_end {
                        let num_keys = words[cur] as usize;
                        cur += 1;
                        for _ in 0..num_keys {
                            if cur + 4 <= block_end {
                                let frame = words[cur];
                                let rx = f32::from_bits(words[cur + 1]);
                                let ry = f32::from_bits(words[cur + 2]);
                                let rz = f32::from_bits(words[cur + 3]);
                                let quat = Mat4::euler_deg_to_quat([rx, ry, rz]);
                                rotation_keys.push(RotationKey {
                                    frame,
                                    quat,
                                });
                                cur += 4;
                            }
                        }
                    }
                }

                // Bit 5 (0x20): Animated Quaternion Rotation (count + count * [frame, qx, qy, qz, qw])
                if (type_flag & 0x20) != 0 {
                    if cur < block_end {
                        let num_keys = words[cur] as usize;
                        cur += 1;
                        for _ in 0..num_keys {
                            if cur + 5 <= block_end {
                                let frame = words[cur];
                                let qx = f32::from_bits(words[cur + 1]);
                                let qy = f32::from_bits(words[cur + 2]);
                                let qz = f32::from_bits(words[cur + 3]);
                                let qw = f32::from_bits(words[cur + 4]);
                                // The stored quaternions are the inverse (conjugate) of the rotation convention
                                // used by the bind-pose Euler angles in the clump (verified: for every bone the
                                // key-0 matrix is the transpose of the bind matrix). Conjugate on load so that
                                // animation and bind pose share one convention. Without this, characters are
                                // posed lying on their side with limbs twisted.
                                rotation_keys.push(RotationKey {
                                    frame,
                                    quat: [-qx, -qy, -qz, qw],
                                });
                                cur += 5;
                            }
                        }
                    }
                }

                let fl = |i: usize| f32::from_bits(words[i]);
                // Bits 6-8: scale. 1 = static [sx, sy, sz], 2 = keyed (count + count * [frame, sx, sy, sz]).
                match (type_flag >> 6) & 7 {
                    1 if cur + 3 <= block_end => {
                        scale_keys.push((0, [fl(cur), fl(cur + 1), fl(cur + 2)]));
                        cur += 3;
                    }
                    2 if cur < block_end => {
                        let n = words[cur] as usize;
                        cur += 1;
                        for _ in 0..n {
                            if cur + 4 <= block_end {
                                scale_keys.push((words[cur], [fl(cur + 1), fl(cur + 2), fl(cur + 3)]));
                                cur += 4;
                            }
                        }
                    }
                    _ => {}
                }
                // Bits 9-11: opacity. 1 = static alpha, 2 = keyed (count + count * [frame, alpha]).
                match (type_flag >> 9) & 7 {
                    1 if cur < block_end => {
                        alpha_keys.push((0, fl(cur)));
                    }
                    2 if cur < block_end => {
                        let n = words[cur] as usize;
                        cur += 1;
                        for _ in 0..n {
                            if cur + 2 <= block_end {
                                alpha_keys.push((words[cur], fl(cur + 1)));
                                cur += 2;
                            }
                        }
                    }
                    _ => {}
                }
            }

            let has_data = static_translation.is_some()
                || !translation_keys.is_empty()
                || static_rotation.is_some()
                || !rotation_keys.is_empty();

            if !bone_name.is_empty() {
                if has_data || !channel_by_name.contains_key(&bone_name) {
                    channel_by_name.insert(bone_name.clone(), channels.len());
                }
                let suffix = if let Some(pos) = bone_name.find(' ') {
                    &bone_name[pos + 1..]
                } else {
                    &bone_name
                };
                if has_data || !channel_by_name.contains_key(suffix) {
                    channel_by_name.insert(suffix.to_string(), channels.len());
                }
            }

            channels.push(AnimBoneChannel {
                bone_id,
                bone_name,
                static_translation,
                translation_keys,
                static_rotation,
                rotation_keys,
                scale_keys,
                alpha_keys,
            });

            w = block_end;
        }

        Ok(Self {
            object_id,
            name: name.to_string(),
            frame_count,
            channels,
            channel_by_name,
        })
    }

    /// Local pose (per clump bone) at `frame`, using the same channel lookup as `compute_world_matrices`.
    pub fn local_pose(&self, clump: &CcsClump, ccs: &CcsFile, frame: f32) -> LocalPose {
        clump
            .bone_obj_ids
            .iter()
            .enumerate()
            .map(|(i, &oid)| {
                let bone_name = ccs.object_name(oid).unwrap_or("");
                let channel = self
                    .channel_by_name
                    .get(bone_name)
                    .or_else(|| {
                        let suffix = bone_name.find(' ').map(|p| &bone_name[p + 1..]).unwrap_or(bone_name);
                        self.channel_by_name.get(suffix)
                    })
                    .map(|&ci| &self.channels[ci])
                    .filter(|ch| !ch.bone_name.starts_with("OBJ_trall"));
                let bind = &clump.transforms[i];
                match channel {
                    Some(ch) => (ch.sample_translation(frame, bind.translation), ch.sample_rotation_quat(frame, bind.rotation_deg)),
                    None => (bind.translation, Mat4::euler_deg_to_quat(bind.rotation_deg)),
                }
            })
            .collect()
    }

    /// Compute animated world transformation matrices for every bone in the clump at a given frame.
    pub fn compute_world_matrices(
        &self,
        clump: &CcsClump,
        objects: &HashMap<u32, CcsObjectNode>,
        ccs: &CcsFile,
        frame: f32,
    ) -> HashMap<u32, Mat4> {
        let obj_to_clump_idx: HashMap<u32, usize> = clump
            .bone_obj_ids
            .iter()
            .enumerate()
            .map(|(idx, &oid)| (oid, idx))
            .collect();

        let mut world_cache: HashMap<u32, Mat4> = HashMap::new();

        for &oid in &clump.bone_obj_ids {
            self.resolve_bone_world_matrix(
                oid,
                &obj_to_clump_idx,
                clump,
                objects,
                ccs,
                frame,
                &mut world_cache,
            );
        }

        world_cache
    }

    fn resolve_bone_world_matrix(
        &self,
        oid: u32,
        obj_to_clump_idx: &HashMap<u32, usize>,
        clump: &CcsClump,
        objects: &HashMap<u32, CcsObjectNode>,
        ccs: &CcsFile,
        frame: f32,
        cache: &mut HashMap<u32, Mat4>,
    ) -> Mat4 {
        if let Some(&m) = cache.get(&oid) {
            return m;
        }

        let idx = match obj_to_clump_idx.get(&oid) {
            Some(&i) => i,
            None => return Mat4::IDENTITY,
        };

        let bone_name = ccs.object_name(oid).unwrap_or("");
        let channel = self
            .channel_by_name
            .get(bone_name)
            .or_else(|| {
                let suffix = if let Some(pos) = bone_name.find(' ') {
                    &bone_name[pos + 1..]
                } else {
                    bone_name
                };
                self.channel_by_name.get(suffix)
            })
            .map(|&ci| &self.channels[ci]);

        let bind_trans = clump.transforms[idx].translation;
        let bind_rot = clump.transforms[idx].rotation_deg;
        let scale = clump.transforms[idx].scale;

        let trans = match channel {
            Some(ch) if !ch.bone_name.starts_with("OBJ_trall") => ch.sample_translation(frame, bind_trans),
            _ => bind_trans,
        };

        let rot_mat = match channel {
            Some(ch) if !ch.bone_name.starts_with("OBJ_trall") => ch.sample_rotation(frame, bind_rot),
            _ => Mat4::from_euler_xyz(bind_rot),
        };

        let local = Mat4::from_translation(trans)
            .mul(&rot_mat)
            .mul(&Mat4::from_scale(scale));

        let parent_oid = objects.get(&oid).map(|o| o.parent_obj_id).unwrap_or(0);
        let world = if parent_oid == 0 || !obj_to_clump_idx.contains_key(&parent_oid) {
            local
        } else {
            let parent_world = self.resolve_bone_world_matrix(
                parent_oid,
                obj_to_clump_idx,
                clump,
                objects,
                ccs,
                frame,
                cache,
            );
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
    fn test_parse_haseo_idle() {
        let path = "../../target/2hsw01.ccs";
        if !std::path::Path::new(path).exists() {
            return;
        }
        let data = std::fs::read(path).unwrap();
        let ccs = CcsFile::parse(&data).unwrap();

        let anim_obj = ccs
            .raw_objects
            .iter()
            .find(|o| o.section_type == crate::objects::SECTION_ANIME && ccs.object_name(o.object_id).unwrap_or("").contains("nut00"))
            .unwrap();

        let name = ccs.object_name(anim_obj.object_id).unwrap();
        let anim = CcsAnimation::from_payload(anim_obj.object_id, name, &anim_obj.payload, &ccs).unwrap();

        assert_eq!(anim.frame_count, 91);
        assert!(anim.channels.len() >= 25);

        let clump = ccs.clumps().remove(0);
        let scene_nodes = ccs.scene_nodes();

        // Sample frame 0, 45, 90
        for f in [0.0, 45.0, 90.0] {
            let matrices = anim.compute_world_matrices(&clump, &scene_nodes, &ccs, f);
            assert_eq!(matrices.len(), clump.bone_count);
            for (oid, mat) in matrices {
                for val in mat.0 {
                    assert!(!val.is_nan(), "NaN found in matrix for bone {}", oid);
                    assert!(!val.is_infinite(), "Inf found in matrix for bone {}", oid);
                }
            }
        }
    }

    #[test]
    fn test_parse_weapon_stance_anim() {
        let char_path = "../../target/2hsw01.ccs";
        let wep_anim_path = "../../target/cmn/data/data/spc/2tbl10rdy00.ccs";
        if !std::path::Path::new(char_path).exists() || !std::path::Path::new(wep_anim_path).exists() {
            return;
        }
        let char_ccs = CcsFile::parse(&std::fs::read(char_path).unwrap()).unwrap();
        let anim_ccs = CcsFile::parse(&std::fs::read(wep_anim_path).unwrap()).unwrap();

        let anim = anim_ccs.animations().remove(0);
        println!("Weapon anim name: {}, channels: {}", anim.name, anim.channels.len());
        for ch in &anim.channels {
            println!("  Channel bone: '{}'", ch.bone_name);
        }

        let clump = char_ccs.clumps().remove(0);
        let scene_nodes = char_ccs.scene_nodes();
        for &oid in &clump.bone_obj_ids {
            let bname = char_ccs.object_name(oid).unwrap_or("?");
            if bname.contains("hand") || bname.contains("pelvis") || bname.contains("trall") {
                let idx = clump.bone_obj_ids.iter().position(|&x| x == oid).unwrap();
                println!("Bone '{}' bind_trans: {:?}, rot: {:?}", bname, clump.transforms[idx].translation, clump.transforms[idx].rotation_deg);
            }
        }
        let matrices = anim.compute_world_matrices(&clump, &scene_nodes, &char_ccs, 0.0);
        for &oid in &clump.bone_obj_ids {
            let bname = char_ccs.object_name(oid).unwrap_or("?");
            if bname.contains("hand") || bname.contains("pelvis") || bname.contains("trall") {
                if let Some(m) = matrices.get(&oid) {
                    println!("Bone '{}' world pos: [{:.3}, {:.3}, {:.3}]", bname, m.0[3], m.0[7], m.0[11]);
                }
            }
        }
    }

    #[test]
    fn test_haseo_walk_vertex_deformation() {
        let path = "../../target/2hsw01.ccs";
        if !std::path::Path::new(path).exists() {
            return;
        }
        let data = std::fs::read(path).unwrap();
        let ccs = CcsFile::parse(&data).unwrap();

        let anim_obj = ccs
            .raw_objects
            .iter()
            .find(|o| o.section_type == crate::objects::SECTION_ANIME && ccs.object_name(o.object_id).unwrap_or("").contains("wlk00"))
            .unwrap();

        let name = ccs.object_name(anim_obj.object_id).unwrap();
        let anim = CcsAnimation::from_payload(anim_obj.object_id, name, &anim_obj.payload, &ccs).unwrap();
        let clump = ccs.clumps().remove(0);
        let scene_nodes = ccs.scene_nodes();

        // Find main body model
        let model_obj = ccs
            .raw_objects
            .iter()
            .find(|o| o.section_type == crate::objects::SECTION_MODEL && ccs.object_name(o.object_id).unwrap_or("").contains("body"))
            .unwrap();
        let mut model = crate::model::DecodedModel::from_payload("MDL_2hswbody", &model_obj.payload, ccs.header.raw_version).unwrap();

        // Deform at frame 0
        let m0 = anim.compute_world_matrices(&clump, &scene_nodes, &ccs, 0.0);
        model.apply_skeleton(&clump, &m0);
        let _p0 = model.meshes[0].vertices[0].position;

        // Deform at frame 20 (mid-step of walk cycle)
        let m20 = anim.compute_world_matrices(&clump, &scene_nodes, &ccs, 20.0);
        model.apply_skeleton(&clump, &m20);
        let _p20 = model.meshes[0].vertices[0].position;

        let _bind_matrices = clump.compute_world_matrices(&scene_nodes);
        println!("OBJ_2hsw01body node: {:?}", scene_nodes.get(&58));
        let head_oid = 16;
        let pelvis_oid = 8;
        let foot_l_oid = 42;
        let foot_r_oid = 50;
        for (name, oid) in [("Pelvis", pelvis_oid), ("Head", head_oid), ("L Foot", foot_l_oid), ("R Foot", foot_r_oid)] {
            let p_bind = clump.compute_world_matrices(&scene_nodes)[&oid].transform_point([0.0, 0.0, 0.0]);
            let p_anim = m0[&oid].transform_point([0.0, 0.0, 0.0]);
            println!("{:8}: Bind = {:?}, Anim = {:?}", name, p_bind, p_anim);
        }
        for (i, &oid) in clump.bone_obj_ids.iter().enumerate() {
            let name = ccs.object_name(oid).unwrap_or("");
            let parent_id = scene_nodes.get(&oid).map(|o| o.parent_obj_id).unwrap_or(0);
            let parent_name = ccs.object_name(parent_id).unwrap_or("ROOT");
            println!("  [{:2}] oid={:3} parent={:3} ('{}') -> '{}'", i, oid, parent_id, parent_name, name);
        }

        println!("=== Clump Bones ===");
        for (i, &oid) in clump.bone_obj_ids.iter().enumerate() {
            let name = ccs.object_name(oid).unwrap_or("");
            let has_ch = anim.channels.iter().any(|c| c.bone_name == name);
            println!("  Clump bone #{:2}: oid={}, name='{}', has_channel={}", i, oid, name, has_ch);
        }

        println!("=== Animation Channels ===");
        for (i, ch) in anim.channels.iter().enumerate() {
            let in_clump = clump.bone_obj_ids.iter().any(|&oid| ccs.object_name(oid).unwrap_or("") == ch.bone_name);
            println!(
                "  Anim ch #{:2}: bone_id={}, name='{}', in_clump={}, trans_keys={}, rot_keys={}, static_t={:?}, static_r={:?}",
                i, ch.bone_id, ch.bone_name, in_clump, ch.translation_keys.len(), ch.rotation_keys.len(), ch.static_translation.is_some(), ch.static_rotation.is_some()
            );
        }

        // Check a vertex attached to a moving bone (e.g. foot, calf, thigh)
        let mut max_vert_diff = 0.0f32;
        for mesh in &model.meshes {
            for (_idx, v) in mesh.vertices.iter().enumerate() {
                // compute p0 and p20 manually
                let mut accum0 = [0.0f32; 3];
                let mut accum20 = [0.0f32; 3];
                let mut tw = 0.0f32;
                for k in 0..4 {
                    let w = v.bone_weights[k];
                    if w <= 0.0 { continue; }
                    let b_idx = if !model.lookup_list.is_empty() {
                        model.lookup_list.get(v.bone_indices[k] as usize).copied().unwrap_or(0) as usize
                    } else {
                        v.bone_indices[k] as usize
                    };
                    if let Some(&oid) = clump.bone_obj_ids.get(b_idx) {
                        let p0 = m0[&oid].transform_point(v.local_positions[k]);
                        let p20 = m20[&oid].transform_point(v.local_positions[k]);
                        for c in 0..3 {
                            accum0[c] += p0[c] * w;
                            accum20[c] += p20[c] * w;
                        }
                        tw += w;
                    }
                }
                if tw > 0.0 {
                    let d = ((accum20[0]/tw - accum0[0]/tw).powi(2) + (accum20[1]/tw - accum0[1]/tw).powi(2) + (accum20[2]/tw - accum0[2]/tw).powi(2)).sqrt();
                    if d > max_vert_diff {
                        max_vert_diff = d;
                    }
                }
            }
        }
        println!("Max vertex movement across all meshes: {:.4} meters", max_vert_diff);
        assert!(max_vert_diff > 0.01, "No vertices moved!");
    }

    #[test]
    fn test_inspect_prop() {
        let p = std::path::Path::new("../../target/2hsw01.ccs");
        assert!(p.exists(), "target/2hsw01.ccs does not exist");
        let bytes = std::fs::read(p).unwrap();
        let ccs = CcsFile::parse(&bytes).unwrap();
        let clump = &ccs.clumps()[0];
        let scene_nodes = ccs.scene_nodes();

        println!("=== Sockets in 2hsw01 scene_nodes ===");
        for (&oid, node) in &scene_nodes {
            let name = ccs.object_name(oid).unwrap_or("");
            if name.contains("prop") || name.contains("hand") {
                let parent_name = ccs.object_name(node.parent_obj_id).unwrap_or("ROOT");
                let in_clump = clump.bone_obj_ids.contains(&oid);
                println!("Node oid={:3} in_clump={} '{}', parent={:3} '{}'", oid, in_clump, name, node.parent_obj_id, parent_name);
            }
        }
        for (i, &oid) in clump.bone_obj_ids.iter().enumerate() {
            let name = ccs.object_name(oid).unwrap_or("");
            if name.contains("prop") || name.contains("hand") {
                println!("Clump bone [{}] oid={} '{}', transform={:?}", i, oid, name, clump.transforms[i]);
            }
        }

        let anims = ccs.animations();
        for a in &anims {
            println!("Animation '{}':", a.name);
            for ch in &a.channels {
                if ch.bone_name.contains("prop") || ch.bone_name.contains("hand") {
                    println!("  ch bone='{}', trans_keys={}, rot_keys={}, static_t={:?}, static_r={:?}",
                        ch.bone_name, ch.translation_keys.len(), ch.rotation_keys.len(), ch.static_translation, ch.static_rotation);
                }
            }
        }
    }
}

#[cfg(test)]
mod local_pose_tests {
    use super::*;

    #[test]
    fn local_pose_matches_world_matrices() {
        let base = std::path::Path::new("../../target");
        let (Ok(chr), Ok(mot)) = (std::fs::read(base.join("cmn/data/data/spc/2ast01.ccs")), std::fs::read(base.join("cmn/data/data/spc/ev/2astev.ccs"))) else { return };
        let chr = CcsFile::parse(&chr).unwrap();
        let mot = CcsFile::parse(&mot).unwrap();
        let clump = &chr.clumps()[0];
        let nodes = chr.scene_nodes();
        for anim in mot.animations().iter().take(3) {
            for frame in [0.0, 7.5, 30.0] {
                let a = anim.compute_world_matrices(clump, &nodes, &chr, frame);
                let b = world_from_local_pose(clump, &nodes, &anim.local_pose(clump, &chr, frame));
                for (k, m) in &a {
                    let n = b[k];
                    for i in 0..16 {
                        assert!((m.0[i] - n.0[i]).abs() < 1e-3, "{} frame {} bone {} elem {}: {} vs {}", anim.name, frame, k, i, m.0[i], n.0[i]);
                    }
                }
            }
        }
    }
}
