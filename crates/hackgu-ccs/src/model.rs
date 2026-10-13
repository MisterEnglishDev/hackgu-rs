//! CyberConnect2 3D Model Decoder for CCS containers.
//!
//! Supports Rigid models and Deformable (skinned) models across Gen 1, Gen 2 (.hack//G.U. PS2),
//! and Gen 3 (.hack//G.U. Last Recode PC). Decodes triangle strips and exports Wavefront OBJ.

use std::fmt;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug)]
pub enum ModelError {
    TooShort,
    InvalidData(String),
    Io(io::Error),
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelError::TooShort => write!(f, "Payload too short for model"),
            ModelError::InvalidData(s) => write!(f, "Invalid model data: {}", s),
            ModelError::Io(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for ModelError {}

impl From<io::Error> for ModelError {
    fn from(e: io::Error) -> Self {
        ModelError::Io(e)
    }
}

/// A decoded vertex with positions, normals, texture coordinates, and skeletal weights.
#[derive(Clone, Debug, Default)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub local_positions: [[f32; 3]; 4],
    pub normal: [f32; 3],
    pub local_normals: [[f32; 3]; 4],
    pub uv: [f32; 2],
    pub color: Option<[u8; 4]>,
    pub bone_indices: [u16; 4],
    pub bone_weights: [f32; 4],
    pub triangle_flag: i8,
}

/// A sub-mesh with an assigned material, vertex buffer, and triangle index buffer.
#[derive(Clone, Debug, Default)]
pub struct SubMesh {
    pub material_index: u32,
    pub vertices: Vec<ModelVertex>,
    pub indices: Vec<u32>,
}

/// A fully decoded CyberConnect2 3D model.
#[derive(Clone, Debug)]
pub struct DecodedModel {
    pub name: String,
    pub vertex_scale: f32,
    pub model_type: u8,
    pub model_flags: u8,
    /// Header byte 8. Observed in Mac Anu: 1 on every water/caustic/reflection/glow layer (black = no
    /// contribution, so additive), 2 on shadow/graffiti decals. 0 on ordinary opaque geometry.
    pub draw_flags: u8,
    pub outline_color: [u8; 4],
    pub outline_width: f32,
    pub lookup_list: Vec<u8>,
    pub meshes: Vec<SubMesh>,
}

/// Convert a PS2-style triangle strip with winding flags into triangle index triplets.
pub fn triangulate_strip(flags: &[i8], num_verts: usize) -> Vec<u32> {
    let mut indices = Vec::new();
    let mut i = 0;
    while i < num_verts && i < flags.len() {
        let flag = flags[i];
        if flag == 1 || flag == 2 {
            let mut direction: i32 = if flag == 1 { 1 } else { -1 };
            let mut strip = vec![i as u32];
            i += 1;
            while i < num_verts && i < flags.len() {
                if strip.len() < 2 {
                    strip.push(i as u32);
                } else if flags[i] == 0 {
                    strip.push(i as u32);
                    let k = strip.len() - 1;
                    if direction == 1 {
                        indices.push(strip[k - 2]);
                        indices.push(strip[k - 1]);
                        indices.push(strip[k]);
                    } else {
                        indices.push(strip[k]);
                        indices.push(strip[k - 1]);
                        indices.push(strip[k - 2]);
                    }
                    direction = -direction;
                } else {
                    break;
                }
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    indices
}

impl DecodedModel {
    /// Parse a Model object's binary payload.
    /// Files flagged `UV_TOP_DOWN` (header bytes 2..4 = 0xCCCC: town/xcg.ccs, town/xw.ccs, the Chaos Gate and warp
    /// points) store V the other way up: with the usual flip the gate's medallion showed the texture's opposite corner
    /// (checked against the clockwork medallion in TEX_xcg and run1_opening f00000).
    pub fn from_payload(name: &str, payload: &[u8], version: u32) -> Result<Self, ModelError> {
        let uv_top_down = version & crate::header::UV_TOP_DOWN != 0;
        let version = version & !crate::header::UV_TOP_DOWN;
        if payload.len() < 16 {
            return Err(ModelError::TooShort);
        }

        let vertex_scale = f32::from_le_bytes(payload[0..4].try_into().unwrap());
        let model_type = payload[4];
        let model_flags = payload[5];
        let mesh_count = u16::from_le_bytes(payload[6..8].try_into().unwrap()) as usize;
        let draw_flags = payload[8];
        let _mat_flags2 = payload[9];
        let _unk_flags = i16::from_le_bytes(payload[10..12].try_into().unwrap());
        let lookup_list_count = payload[12] as usize;
        let _extra_flags = payload[13];
        let tan_bin_flag = u16::from_le_bytes(payload[14..16].try_into().unwrap());

        let mut cursor = 16;
        let mut outline_color = [0u8; 4];
        let mut outline_width = 0.0f32;

        if version > 0x110 {
            if cursor + 8 > payload.len() {
                return Err(ModelError::TooShort);
            }
            outline_color = payload[cursor..cursor + 4].try_into().unwrap();
            outline_width = f32::from_le_bytes(payload[cursor + 4..cursor + 8].try_into().unwrap());
            cursor += 8;
        }

        let mut lookup_list = Vec::new();
        if (model_type & 1 == 0) && (model_type & 4 != 0) && version > 0x111 {
            if cursor + lookup_list_count > payload.len() {
                return Err(ModelError::TooShort);
            }
            lookup_list = payload[cursor..cursor + lookup_list_count].to_vec();
            cursor += lookup_list_count;
            if cursor % 4 != 0 {
                cursor += 4 - (cursor % 4);
            }
        }

        if mesh_count == 0 {
            return Ok(DecodedModel {
                name: name.to_string(),
                vertex_scale,
                model_type,
                model_flags,
                draw_flags,
                outline_color,
                outline_width,
                lookup_list,
                meshes: Vec::new(),
            });
        }

        let final_scale = ((vertex_scale / 256.0) / 16.0) * 0.01;
        let normal_scale = 1.0 / 127.0;

        let mut meshes = Vec::with_capacity(mesh_count);

        if model_type == 8 {
            // Shadow Mesh (bounding volume shadow mesh)
            if cursor + 8 <= payload.len() {
                let vertex_count = u32::from_le_bytes(payload[cursor..cursor + 4].try_into().unwrap()) as usize;
                let tri_count = u32::from_le_bytes(payload[cursor + 4..cursor + 8].try_into().unwrap()) as usize;
                cursor += 8;

                let pos_bytes = vertex_count * 6;
                if cursor + pos_bytes <= payload.len() {
                    let mut vertices = Vec::with_capacity(vertex_count.min(payload.len() / 6));
                    for i in 0..vertex_count {
                        let off = cursor + i * 6;
                        let x = i16::from_le_bytes(payload[off..off + 2].try_into().unwrap()) as f32 * final_scale;
                        let y = i16::from_le_bytes(payload[off + 2..off + 4].try_into().unwrap()) as f32 * final_scale;
                        let z = i16::from_le_bytes(payload[off + 4..off + 6].try_into().unwrap()) as f32 * final_scale;
                        vertices.push(ModelVertex {
                            position: [x, y, z],
                            local_positions: [[x, y, z], [0.0; 3], [0.0; 3], [0.0; 3]],
                            normal: [0.0, 1.0, 0.0],
                            local_normals: [[0.0, 1.0, 0.0], [0.0; 3], [0.0; 3], [0.0; 3]],
                            uv: [0.0, 0.0],
                            color: None,
                            bone_indices: [0, 0, 0, 0],
                            bone_weights: [1.0, 0.0, 0.0, 0.0],
                            triangle_flag: 0,
                        });
                    }
                    cursor += pos_bytes;
                    if cursor % 4 != 0 {
                        cursor += 4 - (cursor % 4);
                    }

                    let tri_bytes = tri_count * 4;
                    let mut indices = Vec::new();
                    if cursor + tri_bytes <= payload.len() {
                        indices.reserve(tri_count);
                        for i in 0..tri_count {
                            let idx = i32::from_le_bytes(payload[cursor + i * 4..cursor + (i + 1) * 4].try_into().unwrap()) as u32;
                            indices.push(idx);
                        }
                    }

                    meshes.push(SubMesh {
                        material_index: 0,
                        vertices,
                        indices,
                    });
                }
            }
        } else if (model_type & 4 != 0) && (model_type & 2 == 0) {
            // Deformable Mesh
            for _ in 0..mesh_count {
                if cursor + 12 > payload.len() {
                    break;
                }
                let material_index = u32::from_le_bytes(payload[cursor..cursor + 4].try_into().unwrap());
                let vertex_count = u32::from_le_bytes(payload[cursor + 4..cursor + 8].try_into().unwrap()) as usize;
                let def_count = u32::from_le_bytes(payload[cursor + 8..cursor + 12].try_into().unwrap()) as usize;
                cursor += 12;

                if vertex_count > payload.len() {
                    return Err(ModelError::TooShort);
                }

                let mut vertices = Vec::with_capacity(vertex_count);
                let mut strip_flags = Vec::with_capacity(vertex_count);

                if def_count == 0 {
                    // Single weight per vertex
                    if cursor + 4 > payload.len() {
                        return Err(ModelError::TooShort);
                    }
                    let bone_id = u32::from_le_bytes(payload[cursor..cursor + 4].try_into().unwrap()) as u16;
                    cursor += 4;

                    // Positions: int16 * 3 * vertex_count
                    let pos_bytes = vertex_count * 6;
                    if cursor + pos_bytes > payload.len() {
                        return Err(ModelError::TooShort);
                    }
                    let mut positions = Vec::with_capacity(vertex_count);
                    for i in 0..vertex_count {
                        let off = cursor + i * 6;
                        let x = i16::from_le_bytes(payload[off..off + 2].try_into().unwrap()) as f32 * final_scale;
                        let y = i16::from_le_bytes(payload[off + 2..off + 4].try_into().unwrap()) as f32 * final_scale;
                        let z = i16::from_le_bytes(payload[off + 4..off + 6].try_into().unwrap()) as f32 * final_scale;
                        positions.push([x, y, z]);
                    }
                    cursor += pos_bytes;
                    if cursor % 4 != 0 {
                        cursor += 4 - (cursor % 4);
                    }

                    // Normals + flags: int8 * 4 * vertex_count
                    let norm_bytes = vertex_count * 4;
                    if cursor + norm_bytes > payload.len() {
                        return Err(ModelError::TooShort);
                    }
                    let mut normals = Vec::with_capacity(vertex_count);
                    for i in 0..vertex_count {
                        let off = cursor + i * 4;
                        let nx = (payload[off] as i8) as f32 * normal_scale;
                        let ny = (payload[off + 1] as i8) as f32 * normal_scale;
                        let nz = (payload[off + 2] as i8) as f32 * normal_scale;
                        let flag = payload[off + 3] as i8;
                        normals.push([nx, ny, nz]);
                        strip_flags.push(flag);
                    }
                    cursor += norm_bytes;

                    // UVs
                    let uv_is_32 = version > 0x125;
                    let uv_stride = if uv_is_32 { 8 } else { 4 };
                    let uv_bytes = vertex_count * uv_stride;
                    if cursor + uv_bytes > payload.len() {
                        return Err(ModelError::TooShort);
                    }
                    let mut uvs = Vec::with_capacity(vertex_count);
                    for i in 0..vertex_count {
                        let off = cursor + i * uv_stride;
                        let (u, v) = if uv_is_32 {
                            let raw_u = i32::from_le_bytes(payload[off..off + 4].try_into().unwrap()) as f32 / 65536.0;
                            let raw_v = i32::from_le_bytes(payload[off + 4..off + 8].try_into().unwrap()) as f32 / 65536.0;
                            (raw_u, raw_v)
                        } else {
                            let raw_u = i16::from_le_bytes(payload[off..off + 2].try_into().unwrap()) as f32 / 256.0;
                            let raw_v = i16::from_le_bytes(payload[off + 2..off + 4].try_into().unwrap()) as f32 / 256.0;
                            (raw_u, raw_v)
                        };
                        uvs.push([u, if uv_top_down { 1.0 - v } else { v }]);
                    }
                    cursor += uv_bytes;

                    if tan_bin_flag != 0 {
                        cursor += vertex_count * 8; // skip tangents/bitangents
                    }

                    for i in 0..vertex_count {
                        vertices.push(ModelVertex {
                            position: positions[i],
                            local_positions: [positions[i], [0.0; 3], [0.0; 3], [0.0; 3]],
                            normal: normals[i],
                            local_normals: [normals[i], [0.0; 3], [0.0; 3], [0.0; 3]],
                            uv: uvs[i],
                            color: None,
                            bone_indices: [bone_id, 0, 0, 0],
                            bone_weights: [1.0, 0.0, 0.0, 0.0],
                            triangle_flag: strip_flags[i],
                        });
                    }
                } else {
                    // Multi-weight deformable vertices
                    if version < 0x125 {
                        let vp_bytes = def_count * 8;
                        let vn_bytes = def_count * 4;
                        let uv_bytes = vertex_count * 4;

                        if cursor + vp_bytes + vn_bytes + uv_bytes > payload.len() {
                            return Err(ModelError::TooShort);
                        }

                        let mut vp = Vec::with_capacity(def_count);
                        for i in 0..def_count {
                            let off = cursor + i * 8;
                            let x = i16::from_le_bytes(payload[off..off + 2].try_into().unwrap()) as f32 * final_scale;
                            let y = i16::from_le_bytes(payload[off + 2..off + 4].try_into().unwrap()) as f32 * final_scale;
                            let z = i16::from_le_bytes(payload[off + 4..off + 6].try_into().unwrap()) as f32 * final_scale;
                            let param = u16::from_le_bytes(payload[off + 6..off + 8].try_into().unwrap());
                            vp.push((x, y, z, param));
                        }
                        cursor += vp_bytes;

                        let mut vn = Vec::with_capacity(def_count);
                        for i in 0..def_count {
                            let off = cursor + i * 4;
                            let nx = (payload[off] as i8) as f32 * normal_scale;
                            let ny = (payload[off + 1] as i8) as f32 * normal_scale;
                            let nz = (payload[off + 2] as i8) as f32 * normal_scale;
                            let flag = payload[off + 3] as i8;
                            vn.push(([nx, ny, nz], flag));
                        }
                        cursor += vn_bytes;

                        let mut uvs = Vec::with_capacity(vertex_count);
                        for i in 0..vertex_count {
                            let off = cursor + i * 4;
                            let u = i16::from_le_bytes(payload[off..off + 2].try_into().unwrap()) as f32 / 256.0;
                            let v = i16::from_le_bytes(payload[off + 2..off + 4].try_into().unwrap()) as f32 / 256.0;
                            uvs.push([u, if uv_top_down { 1.0 - v } else { v }]);
                        }
                        cursor += uv_bytes;

                        let mut d_idx = 0;
                        for v in 0..vertex_count {
                            if d_idx >= vp.len() {
                                break;
                            }
                            let (x, y, z, p0) = vp[d_idx];
                            let (norm0, flag) = vn[d_idx];
                            let b0 = (p0 >> 10) as u16;
                            let w0 = (p0 & 0x1ff) as f32 / 256.0;
                            let has_second = ((p0 >> 9) & 1) == 0;

                            let mut b1 = 0u16;
                            let mut w1 = 0.0f32;
                            let mut pos1 = [0.0f32; 3];
                            let mut norm1 = [0.0f32; 3];
                            if has_second && d_idx + 1 < vp.len() {
                                let (x1, y1, z1, p1) = vp[d_idx + 1];
                                let (n1, _) = vn[d_idx + 1];
                                b1 = (p1 >> 10) as u16;
                                w1 = (p1 & 0x1ff) as f32 / 256.0;
                                pos1 = [x1, y1, z1];
                                norm1 = n1;
                            }
                            d_idx += if has_second { 2 } else { 1 };

                            strip_flags.push(flag);
                            vertices.push(ModelVertex {
                                position: [x, y, z],
                                local_positions: [[x, y, z], pos1, [0.0; 3], [0.0; 3]],
                                normal: norm0,
                                local_normals: [norm0, norm1, [0.0; 3], [0.0; 3]],
                                uv: uvs[v],
                                color: None,
                                bone_indices: [b0, b1, 0, 0],
                                bone_weights: [w0, w1, 0.0, 0.0],
                                triangle_flag: flag,
                            });
                        }
                    } else {
                        // Gen 3 format (version >= 0x125)
                        let vp_bytes = def_count * 12;
                        let vn_bytes = def_count * 4;
                        let uv_bytes = vertex_count * 8;

                        if cursor + vp_bytes + vn_bytes + uv_bytes > payload.len() {
                            return Err(ModelError::TooShort);
                        }

                        let vp_start = cursor;
                        cursor += vp_bytes;
                        let vn_start = cursor;
                        cursor += vn_bytes;
                        let uv_start = cursor;
                        cursor += uv_bytes;

                        if tan_bin_flag != 0 {
                            cursor += def_count * 8;
                        }

                        let mut uvs = Vec::with_capacity(vertex_count);
                        for i in 0..vertex_count {
                            let off = uv_start + i * 8;
                            let u = i32::from_le_bytes(payload[off..off + 4].try_into().unwrap()) as f32 / 65536.0;
                            let v = i32::from_le_bytes(payload[off + 4..off + 8].try_into().unwrap()) as f32 / 65536.0;
                            uvs.push([u, if uv_top_down { 1.0 - v } else { v }]);
                        }

                        let mut d_idx = 0;
                        for v in 0..vertex_count {
                            let mut bone_ids = [0u16; 4];
                            let mut weights = [0.0f32; 4];
                            let mut local_positions = [[0.0f32; 3]; 4];
                            let mut local_normals = [[0.0f32; 3]; 4];
                            let mut flag = 0i8;

                            let mut slot = 0;
                            let mut stop = false;

                            while !stop && slot < 4 && d_idx < def_count {
                                let vp_off = vp_start + d_idx * 12;
                                let vn_off = vn_start + d_idx * 4;

                                let x = i16::from_le_bytes(payload[vp_off..vp_off + 2].try_into().unwrap()) as f32 * final_scale;
                                let y = i16::from_le_bytes(payload[vp_off + 2..vp_off + 4].try_into().unwrap()) as f32 * final_scale;
                                let z = i16::from_le_bytes(payload[vp_off + 4..vp_off + 6].try_into().unwrap()) as f32 * final_scale;
                                let w_i16 = i16::from_le_bytes(payload[vp_off + 6..vp_off + 8].try_into().unwrap());
                                let stop_i16 = i16::from_le_bytes(payload[vp_off + 8..vp_off + 10].try_into().unwrap());
                                let bone_i16 = i16::from_le_bytes(payload[vp_off + 10..vp_off + 12].try_into().unwrap());

                                local_positions[slot] = [x, y, z];
                                bone_ids[slot] = bone_i16 as u16;
                                weights[slot] = w_i16 as f32 / 256.0;

                                let nx = (payload[vn_off] as i8) as f32 * normal_scale;
                                let ny = (payload[vn_off + 1] as i8) as f32 * normal_scale;
                                let nz = (payload[vn_off + 2] as i8) as f32 * normal_scale;
                                local_normals[slot] = [nx, ny, nz];
                                flag = payload[vn_off + 3] as i8;

                                if stop_i16 != 0 {
                                    stop = true;
                                }
                                d_idx += 1;
                                slot += 1;
                            }

                            strip_flags.push(flag);
                            vertices.push(ModelVertex {
                                position: local_positions[0],
                                local_positions,
                                normal: local_normals[0],
                                local_normals,
                                uv: uvs[v],
                                color: None,
                                bone_indices: bone_ids,
                                bone_weights: weights,
                                triangle_flag: flag,
                            });
                        }
                    }
                }

                let indices = triangulate_strip(&strip_flags, vertices.len());
                meshes.push(SubMesh {
                    material_index,
                    vertices,
                    indices,
                });
            }
        } else {
            // Rigid Mesh
            for _ in 0..mesh_count {
                if cursor + 12 > payload.len() {
                    break;
                }
                let parent_index = u32::from_le_bytes(payload[cursor..cursor + 4].try_into().unwrap()) as u16;
                let material_index = u32::from_le_bytes(payload[cursor + 4..cursor + 8].try_into().unwrap());
                let vertex_count = u32::from_le_bytes(payload[cursor + 8..cursor + 12].try_into().unwrap()) as usize;
                cursor += 12;

                if vertex_count > payload.len() {
                    return Err(ModelError::TooShort);
                }

                let pos_bytes = vertex_count * 6;
                if cursor + pos_bytes > payload.len() {
                    return Err(ModelError::TooShort);
                }

                let mut vertices = Vec::with_capacity(vertex_count);
                let mut strip_flags = Vec::with_capacity(vertex_count);
                let mut positions = Vec::with_capacity(vertex_count);
                for i in 0..vertex_count {
                    let off = cursor + i * 6;
                    let x = i16::from_le_bytes(payload[off..off + 2].try_into().unwrap()) as f32 * final_scale;
                    let y = i16::from_le_bytes(payload[off + 2..off + 4].try_into().unwrap()) as f32 * final_scale;
                    let z = i16::from_le_bytes(payload[off + 4..off + 6].try_into().unwrap()) as f32 * final_scale;
                    positions.push([x, y, z]);
                }
                cursor += pos_bytes;
                if cursor % 4 != 0 {
                    cursor += 4 - (cursor % 4);
                }

                let norm_bytes = vertex_count * 4;
                if cursor + norm_bytes > payload.len() {
                    return Err(ModelError::TooShort);
                }
                let mut normals = Vec::with_capacity(vertex_count);
                for i in 0..vertex_count {
                    let off = cursor + i * 4;
                    let nx = (payload[off] as i8) as f32 * normal_scale;
                    let ny = (payload[off + 1] as i8) as f32 * normal_scale;
                    let nz = (payload[off + 2] as i8) as f32 * normal_scale;
                    let flag = payload[off + 3] as i8;
                    normals.push([nx, ny, nz]);
                    strip_flags.push(flag);
                }
                cursor += norm_bytes;

                let mut colors: Option<Vec<[u8; 4]>> = None;
                if (model_flags & 2) == 0 {
                    let col_bytes = vertex_count * 4;
                    if cursor + col_bytes <= payload.len() {
                        let mut cols = Vec::with_capacity(vertex_count);
                        for i in 0..vertex_count {
                            let off = cursor + i * 4;
                            let r = (payload[off] as u16 * 2).min(255) as u8;
                            let g = (payload[off + 1] as u16 * 2).min(255) as u8;
                            let b = (payload[off + 2] as u16 * 2).min(255) as u8;
                            let a = (payload[off + 3] as u16 * 2).min(255) as u8;
                            cols.push([r, g, b, a]);
                        }
                        cursor += col_bytes;
                        colors = Some(cols);
                    }
                }

                let mut uvs = Vec::with_capacity(vertex_count);
                if (model_flags & 4) == 0 {
                    let uv_is_32 = version >= 0x125;
                    let uv_stride = if uv_is_32 { 8 } else { 4 };
                    let uv_bytes = vertex_count * uv_stride;
                    if cursor + uv_bytes <= payload.len() {
                        for i in 0..vertex_count {
                            let off = cursor + i * uv_stride;
                            let (u, v) = if uv_is_32 {
                                let raw_u = i32::from_le_bytes(payload[off..off + 4].try_into().unwrap()) as f32 / 65536.0;
                                let raw_v = i32::from_le_bytes(payload[off + 4..off + 8].try_into().unwrap()) as f32 / 65536.0;
                                (raw_u, raw_v)
                            } else {
                                let raw_u = i16::from_le_bytes(payload[off..off + 2].try_into().unwrap()) as f32 / 256.0;
                                let raw_v = i16::from_le_bytes(payload[off + 2..off + 4].try_into().unwrap()) as f32 / 256.0;
                                (raw_u, raw_v)
                            };
                            uvs.push([u, if uv_top_down { 1.0 - v } else { v }]);
                        }
                        cursor += uv_bytes;
                    }
                }
                while uvs.len() < vertex_count {
                    uvs.push([0.0, 0.0]);
                }

                if tan_bin_flag != 0 {
                    cursor += vertex_count * 8;
                }

                for i in 0..vertex_count {
                    vertices.push(ModelVertex {
                        position: positions[i],
                        local_positions: [positions[i], [0.0; 3], [0.0; 3], [0.0; 3]],
                        normal: normals[i],
                        local_normals: [normals[i], [0.0; 3], [0.0; 3], [0.0; 3]],
                        uv: uvs[i],
                        color: colors.as_ref().map(|c| c[i]),
                        bone_indices: [parent_index, 0, 0, 0],
                        bone_weights: [1.0, 0.0, 0.0, 0.0],
                        triangle_flag: strip_flags[i],
                    });
                }

                let indices = triangulate_strip(&strip_flags, vertices.len());
                meshes.push(SubMesh {
                    material_index,
                    vertices,
                    indices,
                });
            }
        }

        Ok(DecodedModel {
            name: name.to_string(),
            vertex_scale,
            model_type,
            model_flags,
            draw_flags,
            outline_color,
            outline_width,
            lookup_list,
            meshes,
        })
    }

    /// Rigid models (type bit 4 clear) are not skinned: every vertex lives in the space of the Object node that
    /// references the model. (The per-mesh u32 the parser stores in `bone_indices[0]` is a mesh name id, not a bone.)
    pub fn is_rigid(&self) -> bool {
        self.model_type != 8 && self.model_type & 4 == 0
    }

    /// Place a rigid model with its owning node's world matrix.
    pub fn apply_rigid(&mut self, m: &crate::clump::Mat4) {
        for mesh in &mut self.meshes {
            for v in &mut mesh.vertices {
                v.position = m.transform_point(v.local_positions[0]);
                let n = m.transform_vector(v.local_normals[0]);
                let l = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
                if l > 1e-6 {
                    v.normal = [n[0] / l, n[1] / l, n[2] / l];
                }
            }
        }
    }

    /// Apply skeleton bind-pose transformations to all vertices in this model.
    pub fn apply_skeleton(&mut self, clump: &crate::clump::CcsClump, world_matrices: &std::collections::HashMap<u32, crate::clump::Mat4>) {
        for mesh in &mut self.meshes {
            for v in &mut mesh.vertices {
                let mut accum_pos = [0.0f32; 3];
                let mut accum_norm = [0.0f32; 3];
                let mut total_w = 0.0f32;

                for k in 0..4 {
                    let w = v.bone_weights[k];
                    if w <= 0.0 {
                        continue;
                    }
                    let bone_id = v.bone_indices[k];
                    let clump_bone_idx = if !self.lookup_list.is_empty() {
                        self.lookup_list.get(bone_id as usize).copied().unwrap_or(0) as usize
                    } else {
                        bone_id as usize
                    };
                    if let Some(&bone_obj_id) = clump.bone_obj_ids.get(clump_bone_idx) {
                        if let Some(world_m) = world_matrices.get(&bone_obj_id) {
                            let p = world_m.transform_point(v.local_positions[k]);
                            let n = world_m.transform_vector(v.local_normals[k]);
                            accum_pos[0] += p[0] * w;
                            accum_pos[1] += p[1] * w;
                            accum_pos[2] += p[2] * w;
                            accum_norm[0] += n[0] * w;
                            accum_norm[1] += n[1] * w;
                            accum_norm[2] += n[2] * w;
                            total_w += w;
                        }
                    }
                }

                if total_w > 0.0 {
                    v.position = [accum_pos[0] / total_w, accum_pos[1] / total_w, accum_pos[2] / total_w];
                    let n_len = (accum_norm[0] * accum_norm[0] + accum_norm[1] * accum_norm[1] + accum_norm[2] * accum_norm[2]).sqrt();
                    if n_len > 1e-6 {
                        v.normal = [accum_norm[0] / n_len, accum_norm[1] / n_len, accum_norm[2] / n_len];
                    }
                }
            }
        }
    }

    /// Save the complete 3D model as a standard Wavefront OBJ file.
    pub fn save_obj<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error> {
        self.save_obj_with_materials(path, &std::collections::HashMap::new())
    }

    /// Save the complete 3D model with material and texture references (OBJ + MTL).
    pub fn save_obj_with_materials<P: AsRef<Path>>(
        &self,
        path: P,
        mat_to_tex_name: &std::collections::HashMap<u32, String>,
    ) -> Result<(), io::Error> {
        let obj_path = path.as_ref();
        let mut file = File::create(obj_path)?;
        let mtl_name = obj_path.file_stem().unwrap().to_string_lossy().to_string() + ".mtl";
        let mtl_path = obj_path.with_file_name(&mtl_name);

        writeln!(file, "# CC2 CCS Model exported by hackgu-rs")?;
        writeln!(file, "# Model Name: {}", self.name)?;
        writeln!(file, "# Sub-meshes: {}", self.meshes.len())?;

        if !mat_to_tex_name.is_empty() {
            writeln!(file, "mtllib {}", mtl_name)?;

            let mut mtl_file = File::create(&mtl_path)?;
            writeln!(mtl_file, "# CC2 Materials exported by hackgu-rs")?;
            for (&mat_id, tex_fname) in mat_to_tex_name {
                writeln!(mtl_file, "\nnewmtl mat_{}", mat_id)?;
                writeln!(mtl_file, "Ka 1.000 1.000 1.000")?;
                writeln!(mtl_file, "Kd 1.000 1.000 1.000")?;
                writeln!(mtl_file, "Ks 0.000 0.000 0.000")?;
                writeln!(mtl_file, "d 1.0")?;
                writeln!(mtl_file, "illum 2")?;
                writeln!(mtl_file, "map_Kd {}", tex_fname)?;
            }
        }

        let mut vert_base: u32 = 1;
        for (m_idx, mesh) in self.meshes.iter().enumerate() {
            writeln!(file, "\no {}_submesh_{}", self.name, m_idx)?;
            if mat_to_tex_name.contains_key(&mesh.material_index) {
                writeln!(file, "usemtl mat_{}", mesh.material_index)?;
            }
            for v in &mesh.vertices {
                writeln!(file, "v {:.6} {:.6} {:.6}", v.position[0], v.position[1], v.position[2])?;
            }
            for v in &mesh.vertices {
                writeln!(file, "vn {:.6} {:.6} {:.6}", v.normal[0], v.normal[1], v.normal[2])?;
            }
            for v in &mesh.vertices {
                // In OBJ, V=0 is bottom; 1.0 - V is commonly flipped for standard textures
                writeln!(file, "vt {:.6} {:.6}", v.uv[0], 1.0 - v.uv[1])?;
            }
            for chunk in mesh.indices.chunks_exact(3) {
                let i0 = vert_base + chunk[0];
                let i1 = vert_base + chunk[1];
                let i2 = vert_base + chunk[2];
                writeln!(file, "f {0}/{0}/{0} {1}/{1}/{1} {2}/{2}/{2}", i0, i1, i2)?;
            }
            vert_base += mesh.vertices.len() as u32;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangulate_strip_basic() {
        // Strip of 4 vertices in CC2 PS2 format: flags: [1, 1, 0, 0]
        // Should yield 2 triangles: (0, 1, 2) and (3, 2, 1)
        let flags = vec![1, 1, 0, 0];
        let indices = triangulate_strip(&flags, 4);
        assert_eq!(indices, vec![0, 1, 2, 3, 2, 1]);
    }

    #[test]
    fn test_triangulate_strip_restart() {
        // Two independent strips of 3 vertices each
        // Strip 1: [1, 1, 0] -> (0, 1, 2)
        // Strip 2: [1, 1, 0] -> (3, 4, 5)
        let flags = vec![1, 1, 0, 1, 1, 0];
        let indices = triangulate_strip(&flags, 6);
        assert_eq!(indices, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_inspect_atoli_body() {
        let path = "../../target/2atl01.ccs";
        if !std::path::Path::new(path).exists() {
            return;
        }
        let data = std::fs::read(path).unwrap();
        let ccs = crate::parser::CcsFile::parse(&data).unwrap();
        let obj = ccs.raw_objects.iter().find(|o| o.object_id == 530).unwrap();
        let model = DecodedModel::from_payload("MDL_2atlbody", &obj.payload, ccs.header.raw_version).unwrap();
        assert_eq!(model.meshes.len(), 25);
        assert_eq!(model.meshes[24].vertices.len(), 2096);
    }

    #[test]
    fn test_inspect_2hsw02_body() {
        let path = "F:/Projects/Decomp/VRChat/work/haseo/raw/2hsw02.ccs";
        if !std::path::Path::new(path).exists() {
            return;
        }
        let data = std::fs::read(path).unwrap();
        let ccs = crate::parser::CcsFile::parse(&data).unwrap();
        let clumps = ccs.clumps();
        assert_eq!(clumps.len(), 1);
        let clump = &clumps[0];
        let scene_nodes = ccs.scene_nodes();
        let _world_matrices = clump.compute_world_matrices(&scene_nodes);

        let body_obj = ccs.raw_objects.iter().find(|o| {
            o.section_type == crate::objects::SECTION_MODEL && ccs.object_name(o.object_id).unwrap_or("").contains("2hsw02body")
        }).unwrap();
        println!("body_obj id: {}, len: {}, raw_version: 0x{:x}", body_obj.object_id, body_obj.payload.len(), ccs.header.raw_version);
        let model_res = DecodedModel::from_payload("MDL_2hsw02body", &body_obj.payload, ccs.header.raw_version);
        if let Err(e) = &model_res {
            println!("DecodedModel error: {:?}", e);
        }
        let anim_idx = ccs.animations().iter().position(|a| a.name.contains("nut00")).unwrap();
        let anim = &ccs.animations()[anim_idx];
        let anim_matrices = anim.compute_world_matrices(clump, &scene_nodes, &ccs, 0.0);
        println!("anim_matrices count: {}", anim_matrices.len());
        let mut model_body = DecodedModel::from_payload("MDL_2hsw02body", &body_obj.payload, ccs.header.raw_version).unwrap();
        model_body.apply_skeleton(clump, &anim_matrices);
        let mut min_p = [f32::INFINITY; 3];
        let mut max_p = [f32::NEG_INFINITY; 3];
        for m in &model_body.meshes {
            for v in &m.vertices {
                for i in 0..3 {
                    min_p[i] = min_p[i].min(v.position[i]);
                    max_p[i] = max_p[i].max(v.position[i]);
                }
            }
        }
        println!("2hsw02 animated bounds: min={:?}, max={:?}", min_p, max_p);
    }
}
