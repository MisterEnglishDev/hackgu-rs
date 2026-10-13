//! hackgu-export library: glTF 2.0 (.glb) serialization and conversion functions.

pub mod glb;

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use glb::{GlbBuilder, GlbPrimitive};
use hackgu_ccs::parser::CcsFile;
use hackgu_ccs::objects;

pub fn export_ccs_file(
    ccs_path: &Path,
    dest_glb: &Path,
    anim_override: Option<usize>,
    frame_val: f32,
    bind_pose: bool,
) -> Result<usize, String> {
    let data = fs::read(ccs_path).map_err(|e| format!("Failed to read file: {}", e))?;
    let ccs = CcsFile::parse(&data).map_err(|e| format!("Failed to parse CCS: {}", e))?;
    let stem = ccs_path.file_stem().unwrap().to_string_lossy();
    let (glb_bytes, count) = convert_ccs_to_glb(&ccs, &stem, anim_override, frame_val, bind_pose)?;

    if count == 0 {
        return Err("No exportable meshes found in container".into());
    }

    if let Some(parent) = dest_glb.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let mut file = File::create(dest_glb).map_err(|e| format!("Failed to create output file: {}", e))?;
    file.write_all(&glb_bytes).map_err(|e| format!("Failed to write glb bytes: {}", e))?;
    Ok(count)
}

pub fn batch_export_cpk(cpk_path: &Path, out_dir: &Path) -> Result<usize, String> {
    let mut file = File::open(cpk_path).map_err(|e| format!("Failed to open CPK: {}", e))?;
    let archive = hackgu_cpk::CpkArchive::read_from(&mut file).map_err(|e| format!("Failed to parse CPK: {}", e))?;

    let ccs_entries: Vec<_> = archive
        .entries
        .iter()
        .filter(|f| f.path.ends_with(".ccs"))
        .cloned()
        .collect();

    let mut total_exported = 0;
    for entry in &ccs_entries {
        if let Ok(data) = archive.extract_entry(&mut file, entry) {
            if let Ok(ccs) = CcsFile::parse(&data) {
                let model_count = ccs.objects_of_type(objects::SECTION_MODEL).len();
                if model_count > 0 {
                    let file_stem = Path::new(&entry.path).file_stem().unwrap().to_string_lossy();
                    let cat = if entry.path.contains("/eq/") || file_stem.starts_with('w') {
                        "weapons"
                    } else if file_stem.starts_with("town") || file_stem.starts_with("dungeon") || entry.path.contains("/map/") {
                        "stages"
                    } else if file_stem.starts_with("2") || file_stem.starts_with('c') {
                        "characters"
                    } else {
                        "props"
                    };

                    let cat_dir = out_dir.join(cat);
                    let _ = fs::create_dir_all(&cat_dir);
                    let dest_glb = cat_dir.join(format!("{}.glb", file_stem));

                    if let Ok((glb_bytes, submesh_count)) = convert_ccs_to_glb(&ccs, &file_stem, None, 0.0, false) {
                        if submesh_count > 0 && fs::write(&dest_glb, &glb_bytes).is_ok() {
                            total_exported += 1;
                        }
                    }
                }
            }
        }
    }
    Ok(total_exported)
}

pub fn convert_ccs_to_glb(
    ccs: &CcsFile,
    model_name: &str,
    anim_override: Option<usize>,
    frame_val: f32,
    bind_pose: bool,
) -> Result<(Vec<u8>, usize), String> {
    let mut builder = GlbBuilder::new();

    // 1. Textures & CLUTs
    let mut tex_to_glb_idx: HashMap<u32, usize> = HashMap::new();
    for tex_obj in ccs.objects_of_type(objects::SECTION_TEXTURE) {
        let name = ccs.object_name(tex_obj.object_id).unwrap_or("tex");
        let clut_raw = if tex_obj.payload.len() >= 4 {
            let clut_id = i32::from_le_bytes(tex_obj.payload[0..4].try_into().unwrap());
            if clut_id > 0 {
                ccs.raw_objects
                    .iter()
                    .find(|o| o.object_id == clut_id as u32 && o.section_type == objects::SECTION_CLUT)
                    .and_then(|o| if o.payload.len() >= 16 { Some(&o.payload[16..]) } else { None })
            } else {
                None
            }
        } else {
            None
        };

        if let Ok(tex) = hackgu_ccs::DecodedTexture::from_payload_with_clut(name, &tex_obj.payload, clut_raw) {
            if let Ok(glb_tex_idx) = builder.add_image_rgba(tex.width as u32, tex.height as u32, &tex.rgba) {
                tex_to_glb_idx.insert(tex_obj.object_id, glb_tex_idx);
            }
        }
    }

    // 2. Materials
    let ccs_materials = ccs.materials();
    let mut mat_to_glb_idx: HashMap<u32, usize> = HashMap::new();
    for (mat_id, mat) in &ccs_materials {
        let name = ccs.object_name(*mat_id).unwrap_or("material");
        let glb_tex = tex_to_glb_idx.get(&mat.texture_obj_id).copied();
        let glb_mat = builder.add_material(name, glb_tex);
        mat_to_glb_idx.insert(*mat_id, glb_mat);
    }

    // Default white fallback material
    let fallback_mat = builder.add_material("default_white", None);

    // 3. Skeletons & Animation Evaluation
    let mut clumps = ccs.clumps();
    let clump = clumps.pop();
    let scene_nodes = ccs.scene_nodes();

    let mut animations = Vec::new();
    for anim_obj in ccs.objects_of_type(objects::SECTION_ANIME) {
        let name = ccs.object_name(anim_obj.object_id).unwrap_or("anim");
        if let Ok(anim) = hackgu_ccs::CcsAnimation::from_payload(anim_obj.object_id, name, &anim_obj.payload, ccs) {
            animations.push(anim);
        }
    }

    let active_anim = anim_override
        .and_then(|idx| animations.get(idx))
        .or_else(|| {
            // Default to neutral/idle or first animation
            animations.iter().find(|a| a.name.contains("nut00")).or_else(|| animations.first())
        });

    let world_matrices = if bind_pose || active_anim.is_none() {
        clump.as_ref().map(|c| c.compute_world_matrices(&scene_nodes)).unwrap_or_default()
    } else if let (Some(anim), Some(c)) = (active_anim, &clump) {
        anim.compute_world_matrices(c, &scene_nodes, ccs, frame_val)
    } else {
        HashMap::new()
    };

    // 4. Decode all Models & Submeshes
    let mut primitives = Vec::new();
    let mut total_submeshes = 0;

    for model_obj in ccs.objects_of_type(objects::SECTION_MODEL) {
        let name = ccs.object_name(model_obj.object_id).unwrap_or("model");
        if let Ok(mut model) = hackgu_ccs::DecodedModel::from_payload(name, &model_obj.payload, ccs.header.raw_version) {
            // Skip shadow volumes
            if model.model_type == 8 {
                continue;
            }

            // Apply skeleton if available
            if let Some(c) = &clump {
                if !world_matrices.is_empty() {
                    model.apply_skeleton(c, &world_matrices);
                }
            }

            for mesh in &model.meshes {
                if mesh.vertices.is_empty() || mesh.indices.is_empty() {
                    continue;
                }

                // Coordinate conversion: CC2 (+Z Up, +Y Forward) -> glTF (+Y Up, -Z Forward)
                let mut pos_bytes = Vec::with_capacity(mesh.vertices.len() * 12);
                let mut norm_bytes = Vec::with_capacity(mesh.vertices.len() * 12);
                let mut uv_bytes = Vec::with_capacity(mesh.vertices.len() * 8);

                let mut min_pos = [f32::INFINITY; 3];
                let mut max_pos = [f32::NEG_INFINITY; 3];

                for v in &mesh.vertices {
                    // glTF: x = x, y = z, z = -y
                    let gx = v.position[0];
                    let gy = v.position[2];
                    let gz = -v.position[1];

                    min_pos[0] = min_pos[0].min(gx);
                    min_pos[1] = min_pos[1].min(gy);
                    min_pos[2] = min_pos[2].min(gz);

                    max_pos[0] = max_pos[0].max(gx);
                    max_pos[1] = max_pos[1].max(gy);
                    max_pos[2] = max_pos[2].max(gz);

                    pos_bytes.extend_from_slice(&gx.to_le_bytes());
                    pos_bytes.extend_from_slice(&gy.to_le_bytes());
                    pos_bytes.extend_from_slice(&gz.to_le_bytes());

                    let nx = v.normal[0];
                    let ny = v.normal[2];
                    let nz = -v.normal[1];
                    norm_bytes.extend_from_slice(&nx.to_le_bytes());
                    norm_bytes.extend_from_slice(&ny.to_le_bytes());
                    norm_bytes.extend_from_slice(&nz.to_le_bytes());

                    uv_bytes.extend_from_slice(&v.uv[0].to_le_bytes());
                    uv_bytes.extend_from_slice(&(1.0 - v.uv[1]).to_le_bytes());
                }

                let pos_bv = builder.add_buffer_view(&pos_bytes, Some(34962));
                let pos_acc = builder.add_accessor(pos_bv, 5126, mesh.vertices.len(), "VEC3", Some(&min_pos), Some(&max_pos));

                let norm_bv = builder.add_buffer_view(&norm_bytes, Some(34962));
                let norm_acc = builder.add_accessor(norm_bv, 5126, mesh.vertices.len(), "VEC3", None, None);

                let uv_bv = builder.add_buffer_view(&uv_bytes, Some(34962));
                let uv_acc = builder.add_accessor(uv_bv, 5126, mesh.vertices.len(), "VEC2", None, None);

                // Indices: unsigned short (u16)
                let mut idx_bytes = Vec::with_capacity(mesh.indices.len() * 2);
                for &idx in &mesh.indices {
                    idx_bytes.extend_from_slice(&(idx as u16).to_le_bytes());
                }
                let idx_bv = builder.add_buffer_view(&idx_bytes, Some(34963));
                let idx_acc = builder.add_accessor(idx_bv, 5123, mesh.indices.len(), "SCALAR", None, None);

                let mat_idx = mat_to_glb_idx.get(&mesh.material_index).copied().unwrap_or(fallback_mat);

                primitives.push(GlbPrimitive {
                    position_accessor: pos_acc,
                    normal_accessor: Some(norm_acc),
                    uv_accessor: Some(uv_acc),
                    index_accessor: idx_acc,
                    material_index: Some(mat_idx),
                });

                total_submeshes += 1;
            }
        }
    }

    if total_submeshes == 0 {
        return Ok((Vec::new(), 0));
    }

    let mesh_idx = builder.add_mesh(model_name, &primitives);
    let root_node = builder.add_node_mesh(model_name, mesh_idx);
    let glb_bytes = builder.build(&[root_node]);

    Ok((glb_bytes, total_submeshes))
}
