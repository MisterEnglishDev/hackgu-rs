use std::collections::HashMap;
use std::sync::Arc;
use glam::Vec3;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    /// Vertex colour (1.0 = neutral). Sky domes, silhouettes and baked shadows are coloured this way.
    pub color: [f32; 4],
}

/// Model vertex colour as a shader multiplier (the parser already doubles PS2 colours, so 255 = 1.0).
pub fn vcolor(c: Option<[u8; 4]>) -> [f32; 4] {
    match c {
        Some(c) => [c[0] as f32 / 255.0, c[1] as f32 / 255.0, c[2] as f32 / 255.0, c[3] as f32 / 255.0],
        None => [1.0; 4],
    }
}

impl GpuVertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<GpuVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: 12,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: 24,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 32,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub struct GpuTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub bind_group: wgpu::BindGroup,
}

impl GpuTexture {
    pub fn create_default(device: &wgpu::Device, queue: &wgpu::Queue, layout: &wgpu::BindGroupLayout) -> Self {
        // 2x2 white: untextured materials (sky dome, silhouettes, baked shadows) are coloured by their vertex colours
        let rgba: [u8; 16] = [255; 16];
        Self::from_rgba(device, queue, layout, 2, 2, &rgba, Some("default_tex"))
    }

    pub fn from_rgba(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        width: u32,
        height: u32,
        rgba: &[u8],
        label: Option<&str>,
    ) -> Self {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("texture_bind_group"),
        });

        Self {
            texture,
            view,
            sampler,
            bind_group,
        }
    }
}

pub struct GpuSubMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
    pub material_index: u32,
    pub is_background: bool,
    /// 0 = opaque, 1 = additive, 2 = alpha-blended decal (from the model's draw flags).
    pub blend: u8,
    /// Background draw order (0 = world geometry). Sky dome, silhouettes, clouds and sun are drawn first,
    /// without depth, in this order.
    pub bg_layer: u8,
    pub visible: bool,
}

pub struct AnimatedModelRef {
    pub model: hackgu_ccs::DecodedModel,
    pub submesh_indices: Vec<usize>,
    /// For rigid models: the Object node whose world matrix places the model.
    pub rigid_node: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct SocketAttachment {
    pub name: String,
    pub socket_bone_id: u32,
    pub models: Vec<hackgu_ccs::DecodedModel>,
    pub submesh_indices: Vec<Vec<usize>>,
    pub local_matrix: hackgu_ccs::Mat4,
}

#[derive(Clone, Debug)]
pub struct ActorSubMeshRef {
    pub name: String,
    pub submesh_indices: Vec<usize>,
    pub local_vertices: Vec<Vec<[f32; 3]>>, // original local vertex positions per submesh
    pub local_normals: Vec<Vec<[f32; 3]>>,
    pub local_uvs: Vec<Vec<[f32; 2]>>,
    pub local_colors: Vec<Vec<[f32; 4]>>,
    pub clump: Option<hackgu_ccs::CcsClump>,
    pub scene_nodes: HashMap<u32, hackgu_ccs::CcsObjectNode>,
    pub models: Vec<hackgu_ccs::DecodedModel>,
    pub anim_frame: f32,
}

/// One clump of a CCS file placed in the world; its models follow the clump's (animated) object nodes.
pub struct PropInstance {
    pub clump: hackgu_ccs::CcsClump,
    pub nodes: HashMap<u32, hackgu_ccs::CcsObjectNode>,
    /// (model, owner node, submesh indices) per model.
    pub models: Vec<(hackgu_ccs::DecodedModel, u32, Vec<usize>)>,
    /// Effect sprites (CCS 0x0E00) of the clump, drawn as camera-facing additive quads.
    pub effects: Vec<PropEffect>,
}

/// One CCS Effect (0x0E00) sprite: word 0 texture object, word 1 parent object, words 3..6 the quad rectangle in cm
/// (x0, y0, x1, y1). The prop's animation moves/spins it through the channel named after the effect.
pub struct PropEffect {
    pub name: String,
    pub parent: u32,
    pub rect: [f32; 4],
    pub sub: usize,
}

pub struct GpuScene {
    pub submeshes: Vec<GpuSubMesh>,
    pub materials: HashMap<u32, Arc<GpuTexture>>,
    pub default_texture: Arc<GpuTexture>,
    pub min_bounds: Vec3,
    pub max_bounds: Vec3,
    pub fg_min_bounds: Vec3,
    pub fg_max_bounds: Vec3,
    pub has_background: bool,
    pub animations: Vec<hackgu_ccs::CcsAnimation>,
    pub active_anim_idx: Option<usize>,
    pub current_frame: f32,
    pub is_playing: bool,
    pub anim_speed: f32,
    pub clump: Option<hackgu_ccs::CcsClump>,
    pub scene_nodes: HashMap<u32, hackgu_ccs::CcsObjectNode>,
    pub animated_models: Vec<AnimatedModelRef>,
    pub socket_attachments: Vec<SocketAttachment>,
    pub base_submesh_count: usize,
    pub root_transform: Option<hackgu_ccs::Mat4>,
    pub spawned_actors: Vec<ActorSubMeshRef>,
    /// Stage props spawned from one clump of a file and posed by that file's animations (Platform ring, ...).
    pub props: Vec<PropInstance>,
    /// Camera right/up (world) for billboards, set by the app each frame.
    pub billboard_axes: (glam::Vec3, glam::Vec3),
}

impl GpuScene {
    pub fn from_ccs(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_layout: &wgpu::BindGroupLayout,
        ccs: &hackgu_ccs::CcsFile,
    ) -> Self {
        let default_texture = Arc::new(GpuTexture::create_default(device, queue, texture_layout));

        // 1. Decode all textures in the CCS file
        let mut textures: HashMap<u32, Arc<GpuTexture>> = HashMap::new();
        for tex_obj in ccs.objects_of_type(hackgu_ccs::objects::SECTION_TEXTURE) {
            let name = ccs.object_name(tex_obj.object_id).unwrap_or("texture");
            let clut_raw = if tex_obj.payload.len() >= 4 {
                let clut_id = i32::from_le_bytes(tex_obj.payload[0..4].try_into().unwrap());
                if clut_id > 0 {
                    ccs.raw_objects
                        .iter()
                        .find(|o| o.object_id == clut_id as u32 && o.section_type == hackgu_ccs::objects::SECTION_CLUT)
                        .and_then(|o| if o.payload.len() >= 16 { Some(&o.payload[16..]) } else { None })
                } else {
                    None
                }
            } else {
                None
            };
            if let Ok(tex) = hackgu_ccs::DecodedTexture::from_payload_with_clut(name, &tex_obj.payload, clut_raw) {
                let gpu_tex = GpuTexture::from_rgba(
                    device,
                    queue,
                    texture_layout,
                    tex.width as u32,
                    tex.height as u32,
                    &tex.rgba,
                    Some(name),
                );
                textures.insert(tex_obj.object_id, Arc::new(gpu_tex));
            }
        }

        // 2. Link Materials (0x0200) to Textures
        let ccs_materials = ccs.materials();
        let mut materials: HashMap<u32, Arc<GpuTexture>> = HashMap::new();
        for (mat_id, mat) in &ccs_materials {
            if let Some(tex) = textures.get(&mat.texture_obj_id) {
                materials.insert(*mat_id, Arc::clone(tex));
            }
        }

        // 3. Resolve skeletal clumps and scene nodes for forward kinematics
        let all_clumps = ccs.clumps();
        let clump = if !all_clumps.is_empty() { Some(all_clumps[0].clone()) } else { None };
        let scene_nodes = ccs.scene_nodes();

        // Map clump world matrices for each clump
        let mut clump_world_cache: HashMap<usize, HashMap<u32, hackgu_ccs::Mat4>> = HashMap::new();
        for (i, c) in all_clumps.iter().enumerate() {
            clump_world_cache.insert(i, c.compute_world_matrices(&scene_nodes));
        }

        // 4. Parse all animations in the file
        let animations = ccs.animations();
        let active_anim_idx = if !animations.is_empty() && clump.is_some() {
            let default_idx = animations
                .iter()
                .position(|a| a.name.contains("nut00") || a.name.contains("rdy00"))
                .unwrap_or(0);
            println!("Loaded {} animations:", animations.len());
            for (idx, a) in animations.iter().enumerate() {
                let mark = if idx == default_idx { " [ACTIVE]" } else { "" };
                println!("  [{}] {} ({} frames){}", idx, a.name, a.frame_count, mark);
            }
            Some(default_idx)
        } else {
            None
        };

        let world_matrices = if let (Some(c), Some(anim_idx)) = (&clump, active_anim_idx) {
            // Apply frame 0 of active animation
            animations[anim_idx].compute_world_matrices(c, &scene_nodes, ccs, 0.0)
        } else if let Some(c) = &clump {
            c.compute_world_matrices(&scene_nodes)
        } else {
            HashMap::new()
        };

        // 5. Decode and upload all Models (0x0800)
        let owners = ccs.model_owner_nodes();
        let mut submeshes = Vec::new();
        let mut animated_models = Vec::new();
        let mut min_bounds = Vec3::splat(f32::INFINITY);
        let mut max_bounds = Vec3::splat(f32::NEG_INFINITY);
        let mut fg_min_bounds = Vec3::splat(f32::INFINITY);
        let mut fg_max_bounds = Vec3::splat(f32::NEG_INFINITY);
        let mut has_background = false;

        for model_obj in ccs.objects_of_type(hackgu_ccs::objects::SECTION_MODEL) {
            let name = ccs.object_name(model_obj.object_id).unwrap_or("model");
            if let Ok(mut model) = hackgu_ccs::DecodedModel::from_payload(name, &model_obj.payload, ccs.header.raw_version) {
                // Skip shadow volume meshes (type 8 and shadow models)
                if model.model_type == 8 || name.to_lowercase().contains("shadow") {
                    continue;
                }

                let is_background = name.starts_with("MDL_bs")
                    || name.starts_with("MDL_sil")
                    || name.starts_with("MDL_sun")
                    || name.starts_with("MDL_bc")
                    || name.starts_with("MDL_bl")
                    || name.to_lowercase().contains("sky")
                    || name.to_lowercase().contains("silhouette");

                if is_background {
                    has_background = true;
                }

                // Apply initial skeleton pose. Rigid models follow the Object node that references them.
                let owner = owners.get(&model_obj.object_id).copied();
                let rigid_node = if model.is_rigid() { owner.map(|(_, n)| n) } else { None };
                if let (true, Some((ci, node))) = (model.is_rigid(), owner) {
                    let m = if ci == 0 && !world_matrices.is_empty() { world_matrices.get(&node) } else { None }
                        .or_else(|| clump_world_cache.get(&ci).and_then(|wm| wm.get(&node)))
                        .copied()
                        .unwrap_or(hackgu_ccs::Mat4::IDENTITY);
                    model.apply_rigid(&m);
                }
                let matching_clump_idx = if model.is_rigid() {
                    None
                } else if let Some((ci, _)) = owner {
                    Some(ci)
                } else if all_clumps.len() == 1 {
                    Some(0)
                } else if all_clumps.len() > 1 {
                    let suffix = name.strip_prefix("MDL_").unwrap_or(name);
                    all_clumps.iter().position(|c| {
                        let cname = ccs.object_name(c.object_id).unwrap_or("");
                        cname == format!("CMP_{}", suffix) || cname.ends_with(suffix)
                    })
                } else {
                    None
                };

                if let Some(ci) = matching_clump_idx {
                    let c = &all_clumps[ci];
                    let matrices = if let Some(wm) = clump_world_cache.get(&ci) {
                        if !world_matrices.is_empty() && ci == 0 && active_anim_idx.is_some() {
                            &world_matrices
                        } else {
                            wm
                        }
                    } else {
                        &world_matrices
                    };
                    if !matrices.is_empty() {
                        model.apply_skeleton(c, matrices);
                    }
                }

                let mut submesh_indices = Vec::new();

                for mesh in &model.meshes {
                    if mesh.vertices.is_empty() || mesh.indices.is_empty() {
                        continue;
                    }

                    let gpu_vertices: Vec<GpuVertex> = mesh
                        .vertices
                        .iter()
                        .map(|v| {
                            let p = Vec3::from(v.position);
                            min_bounds = min_bounds.min(p);
                            max_bounds = max_bounds.max(p);

                            if !is_background {
                                fg_min_bounds = fg_min_bounds.min(p);
                                fg_max_bounds = fg_max_bounds.max(p);
                            }

                            GpuVertex {
                                position: v.position,
                                normal: v.normal,
                                uv: [v.uv[0], 1.0 - v.uv[1]],
                                color: vcolor(v.color),
                            }
                        })
                        .collect();

                    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("submesh_vertex_buffer"),
                        contents: bytemuck::cast_slice(&gpu_vertices),
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    });

                    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("submesh_index_buffer"),
                        contents: bytemuck::cast_slice(&mesh.indices),
                        usage: wgpu::BufferUsages::INDEX,
                    });

                    submesh_indices.push(submeshes.len());
                    submeshes.push(GpuSubMesh {
                        vertex_buffer,
                        index_buffer,
                        index_count: mesh.indices.len() as u32,
                        material_index: mesh.material_index,
                        is_background,
                        blend: crate::mesh::blend_mode(model.draw_flags),
                        bg_layer: if is_background { background_layer(name) } else { 0 },
                        visible: true,
                    });
                }

                if !submesh_indices.is_empty() && clump.is_some() {
                    animated_models.push(AnimatedModelRef {
                        model,
                        submesh_indices,
                        rigid_node,
                    });
                }
            }
        }

        if min_bounds.x == f32::INFINITY {
            min_bounds = Vec3::new(-1.0, -1.0, 0.0);
            max_bounds = Vec3::new(1.0, 1.0, 2.0);
        }

        if fg_min_bounds.x == f32::INFINITY {
            fg_min_bounds = min_bounds;
            fg_max_bounds = max_bounds;
        }

        let is_playing = active_anim_idx.is_some();

        let base_submesh_count = submeshes.len();

        Self {
            submeshes,
            materials,
            default_texture,
            min_bounds,
            max_bounds,
            fg_min_bounds,
            fg_max_bounds,
            has_background,
            animations,
            active_anim_idx,
            current_frame: 0.0,
            is_playing,
            anim_speed: 1.0,
            clump,
            scene_nodes,
            animated_models,
            socket_attachments: Vec::new(),
            base_submesh_count,
            root_transform: None,
            spawned_actors: Vec::new(),
            props: Vec::new(),
            billboard_axes: (glam::Vec3::X, glam::Vec3::Z),
        }
    }

    /// Advance and update animation playback for the current frame.
    pub fn update_animation(&mut self, queue: &wgpu::Queue, dt: f32, ccs: &hackgu_ccs::CcsFile) {
        if !self.is_playing || self.active_anim_idx.is_none() || self.clump.is_none() {
            return;
        }

        let anim_idx = self.active_anim_idx.unwrap();
        let anim = &self.animations[anim_idx];

        let max_frame = (anim.frame_count.saturating_sub(1)).max(1) as f32;
        self.current_frame += dt * 30.0 * self.anim_speed;
        if self.current_frame >= max_frame {
            self.current_frame %= max_frame;
        } else if self.current_frame < 0.0 {
            self.current_frame = max_frame - (-self.current_frame % max_frame);
        }

        self.apply_current_pose(queue, ccs);
    }

    /// Select an animation track by index and reset playback.
    pub fn select_animation(&mut self, index: usize, queue: &wgpu::Queue, ccs: &hackgu_ccs::CcsFile) {
        if index < self.animations.len() {
            self.active_anim_idx = Some(index);
            self.current_frame = 0.0;
            println!("Animation Track [{}]: {} ({} frames)", index, self.animations[index].name, self.animations[index].frame_count);
            self.apply_current_pose(queue, ccs);
        }
    }

    /// Cycle to next or previous animation track.
    pub fn cycle_animation(&mut self, next: bool, queue: &wgpu::Queue, ccs: &hackgu_ccs::CcsFile) {
        if self.animations.is_empty() {
            return;
        }
        let cur = self.active_anim_idx.unwrap_or(0);
        let next_idx = if next {
            (cur + 1) % self.animations.len()
        } else if cur == 0 {
            self.animations.len() - 1
        } else {
            cur - 1
        };
        self.select_animation(next_idx, queue, ccs);
    }

    /// Step animation frame by delta (for manual frame stepping).
    pub fn step_frame(&mut self, delta: f32, queue: &wgpu::Queue, ccs: &hackgu_ccs::CcsFile) {
        if let Some(anim_idx) = self.active_anim_idx {
            let anim = &self.animations[anim_idx];
            let max_frame = (anim.frame_count.saturating_sub(1)).max(1) as f32;
            self.current_frame = (self.current_frame + delta).clamp(0.0, max_frame);
            println!("Frame: {:.1} / {:.1}", self.current_frame, max_frame);
            self.apply_current_pose(queue, ccs);
        }
    }

    /// Seek animation to an absolute frame.
    pub fn seek_frame(&mut self, frame: f32, queue: &wgpu::Queue, ccs: &hackgu_ccs::CcsFile) {
        if let Some(anim_idx) = self.active_anim_idx {
            let anim = &self.animations[anim_idx];
            let max_frame = (anim.frame_count.saturating_sub(1)).max(1) as f32;
            self.current_frame = frame.clamp(0.0, max_frame);
            println!("Seek to Frame: {:.1} / {:.1}", self.current_frame, max_frame);
            self.apply_current_pose(queue, ccs);
        }
    }

    /// Clear all attached weapons and restore base submeshes.
    pub fn clear_weapon_attachments(&mut self) {
        self.submeshes.truncate(self.base_submesh_count);
        self.socket_attachments.clear();
    }

    /// Attach a weapon model container to a named character bone/prop socket.
    pub fn attach_weapon(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_layout: &wgpu::BindGroupLayout,
        weapon_ccs: &hackgu_ccs::CcsFile,
        character_ccs: &hackgu_ccs::CcsFile,
        socket_name: &str,
        local_matrix: hackgu_ccs::Mat4,
    ) {
        // 1. Find the socket bone ID from character_ccs
        let mut target_bone_id = None;
        for &oid in self.scene_nodes.keys() {
            if let Some(bname) = character_ccs.object_name(oid) {
                if bname.contains(socket_name) {
                    target_bone_id = Some(oid);
                    break;
                }
            }
        }

        // Fallback to r hand / l hand if prop socket node was not present
        if target_bone_id.is_none() {
            let fallback_name = if socket_name.contains("1") || socket_name.contains("r") {
                "r hand"
            } else {
                "l hand"
            };
            for &oid in self.scene_nodes.keys() {
                if let Some(bname) = character_ccs.object_name(oid) {
                    if bname.contains(fallback_name) {
                        target_bone_id = Some(oid);
                        break;
                    }
                }
            }
        }

        let socket_bone_id = match target_bone_id {
            Some(id) => id,
            None => {
                eprintln!("Socket bone '{}' not found in character skeleton", socket_name);
                return;
            }
        };

        println!(
            "Attaching weapon to socket '{}' (Bone ID: {}, '{}')",
            socket_name,
            socket_bone_id,
            character_ccs.object_name(socket_bone_id).unwrap_or("?")
        );

        // 2. Decode weapon textures and load into materials
        let mat_offset = 0x100000 + (self.socket_attachments.len() as u32 * 0x10000);
        let mut weapon_textures: HashMap<u32, Arc<GpuTexture>> = HashMap::new();
        for tex_obj in weapon_ccs.objects_of_type(hackgu_ccs::objects::SECTION_TEXTURE) {
            let clut_raw = if tex_obj.payload.len() >= 4 {
                let clut_id = i32::from_le_bytes(tex_obj.payload[0..4].try_into().unwrap());
                if clut_id > 0 {
                    weapon_ccs.raw_objects
                        .iter()
                        .find(|o| o.object_id == clut_id as u32 && o.section_type == hackgu_ccs::objects::SECTION_CLUT)
                        .and_then(|o| if o.payload.len() >= 16 { Some(&o.payload[16..]) } else { None })
                } else {
                    None
                }
            } else {
                None
            };

            let name = weapon_ccs.object_name(tex_obj.object_id).unwrap_or("weapon_tex");
            if let Ok(tex) = hackgu_ccs::DecodedTexture::from_payload_with_clut(name, &tex_obj.payload, clut_raw) {
                let gpu_tex = GpuTexture::from_rgba(
                    device,
                    queue,
                    texture_layout,
                    tex.width as u32,
                    tex.height as u32,
                    &tex.rgba,
                    Some(name),
                );
                weapon_textures.insert(tex_obj.object_id, Arc::new(gpu_tex));
            }
        }

        for (mat_id, mat) in &weapon_ccs.materials() {
            if let Some(tex) = weapon_textures.get(&mat.texture_obj_id) {
                self.materials.insert(mat_id + mat_offset, Arc::clone(tex));
            }
        }

        // 2b. Compute weapon internal forward kinematics if weapon has a skeleton/clump (e.g. Scythe)
        let wep_clump = weapon_ccs.clumps().into_iter().next();
        let wep_scene_nodes = weapon_ccs.scene_nodes();
        let wep_world_matrices = if let Some(ref c) = wep_clump {
            c.compute_world_matrices(&wep_scene_nodes)
        } else {
            HashMap::new()
        };

        // 3. Decode weapon models and upload GPU buffers
        let mut attached_models = Vec::new();
        let mut attached_submesh_indices = Vec::new();

        for model_obj in weapon_ccs.objects_of_type(hackgu_ccs::objects::SECTION_MODEL) {
            let name = weapon_ccs.object_name(model_obj.object_id).unwrap_or("weapon_model");
            if let Ok(mut model) = hackgu_ccs::DecodedModel::from_payload(name, &model_obj.payload, weapon_ccs.header.raw_version) {
                if model.model_type == 8 || name.to_lowercase().contains("shadow") || name.to_lowercase().contains("dummy") {
                    continue; // Skip shadow volumes and dummy effect placeholders
                }

                // If weapon has a clump, apply skeletal pose to assemble weapon pieces (e.g. Scythe blade parts onto shaft)
                if let Some(ref c) = wep_clump {
                    if !wep_world_matrices.is_empty() {
                        model.apply_skeleton(c, &wep_world_matrices);
                    }
                }

                // Filter out empty meshes and effect billboard quads with missing textures (fixes white bars on guns)
                model.meshes.retain(|mesh| {
                    if mesh.vertices.is_empty() || mesh.indices.is_empty() {
                        return false;
                    }
                    if let Some(mat) = weapon_ccs.materials().get(&mesh.material_index) {
                        if !weapon_textures.contains_key(&mat.texture_obj_id) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                    true
                });

                if model.meshes.is_empty() {
                    continue;
                }

                let mut sub_indices = Vec::new();
                for mesh in &mut model.meshes {
                    mesh.material_index += mat_offset;

                    let gpu_vertices: Vec<GpuVertex> = mesh
                        .vertices
                        .iter()
                        .map(|v| GpuVertex {
                            position: v.position,
                            normal: v.normal,
                            uv: [v.uv[0], 1.0 - v.uv[1]],
                            color: vcolor(v.color),
                        })
                        .collect();

                    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("weapon_vertex_buffer"),
                        contents: bytemuck::cast_slice(&gpu_vertices),
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    });

                    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("weapon_index_buffer"),
                        contents: bytemuck::cast_slice(&mesh.indices),
                        usage: wgpu::BufferUsages::INDEX,
                    });

                    sub_indices.push(self.submeshes.len());
                    self.submeshes.push(GpuSubMesh {
                        vertex_buffer,
                        index_buffer,
                        index_count: mesh.indices.len() as u32,
                        material_index: mesh.material_index,
                        is_background: false,
                        blend: 0,
                        bg_layer: 0,
                        visible: true,
                    });
                }

                if !sub_indices.is_empty() {
                    attached_models.push(model);
                    attached_submesh_indices.push(sub_indices);
                }
            }
        }

        self.socket_attachments.push(SocketAttachment {
            name: socket_name.to_string(),
            socket_bone_id,
            models: attached_models,
            submesh_indices: attached_submesh_indices,
            local_matrix,
        });

        self.apply_current_pose(queue, character_ccs);
    }

    /// Load an environmental stage container into the existing scene as static world geometry.
    pub fn load_stage_environment(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_layout: &wgpu::BindGroupLayout,
        stage_ccs: &hackgu_ccs::CcsFile,
    ) {
        self.load_stage_environment_at(device, queue, texture_layout, stage_ccs, Vec3::ZERO);
    }

    /// Like `load_stage_environment`, but places the whole file at `offset` (metres). Mac Anu is split into nine
    /// district files whose placement comes from the `DMY_town01_XXd` locators in `town01.ccs`.
    pub fn load_stage_environment_at(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_layout: &wgpu::BindGroupLayout,
        stage_ccs: &hackgu_ccs::CcsFile,
        offset: Vec3,
    ) {
        self.load_stage_environment_where(device, queue, texture_layout, stage_ccs, offset, |_| true);
    }

    /// Like `load_stage_environment_at`, but only loads models whose owning clump name passes `keep`
    /// (models with no owning clump are passed the empty string).
    pub fn load_stage_environment_where(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_layout: &wgpu::BindGroupLayout,
        stage_ccs: &hackgu_ccs::CcsFile,
        offset: Vec3,
        keep: impl Fn(&str) -> bool,
    ) {
        println!("Loading stage environment into scene ({} objects)...", stage_ccs.raw_objects.len());
        let mat_offset = self.register_stage_materials(device, queue, texture_layout, stage_ccs);
        self.instance_stage_models(device, stage_ccs, mat_offset, keep, hackgu_ccs::Mat4::from_translation(offset.to_array()));
    }

    /// Decode a stage file's textures and register its materials once; returns the material-id offset to
    /// pass to `instance_stage_models`.
    pub fn register_stage_materials(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_layout: &wgpu::BindGroupLayout,
        stage_ccs: &hackgu_ccs::CcsFile,
    ) -> u32 {
        // 1. Decode stage textures and materials
        let mat_offset = self.materials.keys().copied().max().unwrap_or(0x200000) + 1000;
        let mut stage_textures: HashMap<u32, Arc<GpuTexture>> = HashMap::new();
        let mut decoded_rgba: HashMap<u32, (u32, u32, Vec<u8>)> = HashMap::new();
        for tex_obj in stage_ccs.objects_of_type(hackgu_ccs::objects::SECTION_TEXTURE) {
            let clut_raw = if tex_obj.payload.len() >= 4 {
                let clut_id = i32::from_le_bytes(tex_obj.payload[0..4].try_into().unwrap());
                if clut_id > 0 {
                    stage_ccs.raw_objects
                        .iter()
                        .find(|o| o.object_id == clut_id as u32 && o.section_type == hackgu_ccs::objects::SECTION_CLUT)
                        .and_then(|o| if o.payload.len() >= 16 { Some(&o.payload[16..]) } else { None })
                } else {
                    None
                }
            } else {
                None
            };

            let name = stage_ccs.object_name(tex_obj.object_id).unwrap_or("stage_tex");
            if let Ok(tex) = hackgu_ccs::DecodedTexture::from_payload_with_clut(name, &tex_obj.payload, clut_raw) {
                decoded_rgba.insert(tex_obj.object_id, (tex.width as u32, tex.height as u32, tex.rgba.clone()));
                let gpu_tex = GpuTexture::from_rgba(
                    device,
                    queue,
                    texture_layout,
                    tex.width as u32,
                    tex.height as u32,
                    &tex.rgba,
                    Some(name),
                );
                stage_textures.insert(tex_obj.object_id, Arc::new(gpu_tex));
            }
        }

        for (mat_id, mat) in &stage_ccs.materials() {
            if let Some(tex) = stage_textures.get(&mat.texture_obj_id) {
                self.materials.insert(mat_id + mat_offset, Arc::clone(tex));
            }
        }
        // Materials whose float parameter is below 1 (e.g. 0.6 on Mac Anu's reflection layers) are drawn at that
        // opacity: bake it into a per-material copy of the texture's alpha.
        for (mat_id, mat) in &stage_ccs.materials() {
            if mat.float_param > 0.0 && mat.float_param < 0.999 {
                if let Some(d) = decoded_rgba.get(&mat.texture_obj_id) {
                    let mut rgba = d.2.clone();
                    for px in rgba.chunks_exact_mut(4) {
                        px[3] = (px[3] as f32 * mat.float_param).round() as u8;
                    }
                    let t = GpuTexture::from_rgba(device, queue, texture_layout, d.0, d.1, &rgba, Some("mat_alpha"));
                    self.materials.insert(mat_id + mat_offset, Arc::new(t));
                }
            }
        }

        mat_offset
    }

    /// Add the models whose owning clump passes `keep`, transformed by `placement` (metres, applied after the
    /// file's own scene-graph placement). Materials must already be registered with `register_stage_materials`.
    pub fn instance_stage_models(
        &mut self,
        device: &wgpu::Device,
        stage_ccs: &hackgu_ccs::CcsFile,
        mat_offset: u32,
        keep: impl Fn(&str) -> bool,
        placement: hackgu_ccs::Mat4,
    ) {
        self.instance_stage_models_by(device, stage_ccs, mat_offset, |c, _| keep(c), placement)
    }

    /// Like `instance_stage_models`, filtering on (owner clump name, model name).
    pub fn instance_stage_models_by(
        &mut self,
        device: &wgpu::Device,
        stage_ccs: &hackgu_ccs::CcsFile,
        mat_offset: u32,
        keep: impl Fn(&str, &str) -> bool,
        placement: hackgu_ccs::Mat4,
    ) {
        // 2. Resolve stage skeletal clumps and scene nodes for world transforms
        let all_clumps = stage_ccs.clumps();
        let scene_nodes = stage_ccs.scene_nodes();
        let mut clump_world_cache: HashMap<usize, HashMap<u32, hackgu_ccs::Mat4>> = HashMap::new();
        for (i, c) in all_clumps.iter().enumerate() {
            clump_world_cache.insert(i, c.compute_world_matrices(&scene_nodes));
        }

        // 3. Decode stage models and upload static GPU buffers
        let owners = stage_ccs.model_owner_nodes();
        for model_obj in stage_ccs.objects_of_type(hackgu_ccs::objects::SECTION_MODEL) {
            let name = stage_ccs.object_name(model_obj.object_id).unwrap_or("stage_model");
            if let Ok(mut model) = hackgu_ccs::DecodedModel::from_payload(name, &model_obj.payload, stage_ccs.header.raw_version) {
                if model.model_type == 8 || name.to_lowercase().contains("shadow") {
                    continue; // Skip shadow volume meshes
                }

                let is_background = name.starts_with("MDL_bs")
                    || name.starts_with("MDL_sil")
                    || name.starts_with("MDL_sun")
                    || name.starts_with("MDL_bc")
                    || name.starts_with("MDL_bl")
                    || name.to_lowercase().contains("sky")
                    || name.to_lowercase().contains("silhouette");

                if is_background {
                    self.has_background = true;
                }

                let owner_clump_name = owners
                    .get(&model_obj.object_id)
                    .and_then(|(ci, _)| all_clumps.get(*ci))
                    .and_then(|c| stage_ccs.object_name(c.object_id))
                    .unwrap_or("");
                if !keep(owner_clump_name, name) {
                    continue;
                }
                // Place the model. Rigid models (all town/dungeon geometry) use the world matrix of the Object node
                // that references them; skinned models use their owning clump.
                let owner = owners.get(&model_obj.object_id).copied();
                if model.is_rigid() {
                    let m = owner
                        .and_then(|(ci, node)| clump_world_cache.get(&ci).and_then(|wm| wm.get(&node)).copied())
                        .unwrap_or(hackgu_ccs::Mat4::IDENTITY);
                    model.apply_rigid(&m);
                }
                let matching_clump_idx = if model.is_rigid() {
                    None
                } else if let Some((ci, _)) = owner {
                    Some(ci)
                } else if all_clumps.len() == 1 {
                    Some(0)
                } else if all_clumps.len() > 1 {
                    let suffix = name.strip_prefix("MDL_").unwrap_or(name);
                    all_clumps.iter().position(|c| {
                        let cname = stage_ccs.object_name(c.object_id).unwrap_or("");
                        cname == format!("CMP_{}", suffix) || cname.ends_with(suffix)
                    })
                } else {
                    None
                };

                if let Some(ci) = matching_clump_idx {
                    let c = &all_clumps[ci];
                    if let Some(wm) = clump_world_cache.get(&ci) {
                        if !wm.is_empty() {
                            model.apply_skeleton(c, wm);
                        }
                    }
                }

                for mesh in &mut model.meshes {
                    if mesh.vertices.is_empty() || mesh.indices.is_empty() {
                        continue;
                    }
                    mesh.material_index += mat_offset;

                    let gpu_vertices: Vec<GpuVertex> = mesh
                        .vertices
                        .iter()
                        .map(|v| {
                            let p = Vec3::from(placement.transform_point(v.position));
                            self.min_bounds = self.min_bounds.min(p);
                            self.max_bounds = self.max_bounds.max(p);
                            if !is_background {
                                self.fg_min_bounds = self.fg_min_bounds.min(p);
                                self.fg_max_bounds = self.fg_max_bounds.max(p);
                            }
                            GpuVertex {
                                position: p.to_array(),
                                normal: placement.transform_vector(v.normal),
                                uv: [v.uv[0], 1.0 - v.uv[1]],
                                color: vcolor(v.color),
                            }
                        })
                        .collect();

                    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("stage_vertex_buffer"),
                        contents: bytemuck::cast_slice(&gpu_vertices),
                        usage: wgpu::BufferUsages::VERTEX,
                    });

                    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("stage_index_buffer"),
                        contents: bytemuck::cast_slice(&mesh.indices),
                        usage: wgpu::BufferUsages::INDEX,
                    });

                    self.submeshes.push(GpuSubMesh {
                        vertex_buffer,
                        index_buffer,
                        index_count: mesh.indices.len() as u32,
                        material_index: mesh.material_index,
                        is_background,
                        blend: crate::mesh::blend_mode(model.draw_flags),
                        bg_layer: if is_background { background_layer(name) } else { 0 },
                        visible: true,
                    });
                }
            }
        }
    }

    /// Add an extensive walkable ground floor covering the city area.
    pub fn add_ground_floor(
        &mut self,
        device: &wgpu::Device,
        center_x: f32,
        center_y: f32,
        half_w: f32,
        half_h: f32,
        z_height: f32,
    ) {
        let mat_id = self.materials.keys().find(|&&k| k >= 0x200000).copied().unwrap_or(0);
        let uv_scale = 0.5;
        let x0 = center_x - half_w;
        let x1 = center_x + half_w;
        let y0 = center_y - half_h;
        let y1 = center_y + half_h;

        let vertices = [
            GpuVertex { position: [x0, y0, z_height], normal: [0.0, 0.0, 1.0], uv: [0.0, 0.0], color: [1.0; 4] },
            GpuVertex { position: [x1, y0, z_height], normal: [0.0, 0.0, 1.0], uv: [half_w * uv_scale, 0.0], color: [1.0; 4] },
            GpuVertex { position: [x1, y1, z_height], normal: [0.0, 0.0, 1.0], uv: [half_w * uv_scale, half_h * uv_scale], color: [1.0; 4] },
            GpuVertex { position: [x0, y1, z_height], normal: [0.0, 0.0, 1.0], uv: [0.0, half_h * uv_scale], color: [1.0; 4] },
        ];
        let indices: [u32; 6] = [0, 1, 2, 0, 2, 3];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("city_ground_vertex_buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("city_ground_index_buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        self.submeshes.push(GpuSubMesh {
            vertex_buffer,
            index_buffer,
            index_count: 6,
            material_index: mat_id,
            is_background: false,
            blend: 0,
                        bg_layer: 0,
                        visible: true,
        });
    }

    /// Spawn an authentic 3D actor model (e.g. Iyoten, Asta, Ovan, Silabus) at world coordinates.
    pub fn spawn_actor_model(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_layout: &wgpu::BindGroupLayout,
        actor_ccs: &hackgu_ccs::CcsFile,
        position: glam::Vec3,
        yaw: f32,
    ) {
        println!("Spawning world actor at [{:.1}, {:.1}, {:.1}] (yaw: {:.2} rad)...", position.x, position.y, position.z, yaw);

        // 1. Decode textures
        let mat_offset = self.materials.keys().copied().max().unwrap_or(0x300000) + 1000;
        let mut actor_textures: HashMap<u32, Arc<GpuTexture>> = HashMap::new();
        for tex_obj in actor_ccs.objects_of_type(hackgu_ccs::objects::SECTION_TEXTURE) {
            let clut_raw = if tex_obj.payload.len() >= 4 {
                let clut_id = i32::from_le_bytes(tex_obj.payload[0..4].try_into().unwrap());
                if clut_id > 0 {
                    actor_ccs.raw_objects
                        .iter()
                        .find(|o| o.object_id == clut_id as u32 && o.section_type == hackgu_ccs::objects::SECTION_CLUT)
                        .and_then(|o| if o.payload.len() >= 16 { Some(&o.payload[16..]) } else { None })
                } else {
                    None
                }
            } else {
                None
            };

            let name = actor_ccs.object_name(tex_obj.object_id).unwrap_or("actor_tex");
            if let Ok(tex) = hackgu_ccs::DecodedTexture::from_payload_with_clut(name, &tex_obj.payload, clut_raw) {
                let gpu_tex = GpuTexture::from_rgba(
                    device,
                    queue,
                    texture_layout,
                    tex.width as u32,
                    tex.height as u32,
                    &tex.rgba,
                    Some(name),
                );
                actor_textures.insert(tex_obj.object_id, Arc::new(gpu_tex));
            }
        }

        for (mat_id, mat) in &actor_ccs.materials() {
            if let Some(tex) = actor_textures.get(&mat.texture_obj_id) {
                self.materials.insert(mat_id + mat_offset, Arc::clone(tex));
            }
        }

        // 2. Pose skeleton
        let all_clumps = actor_ccs.clumps();
        let clump = all_clumps.into_iter().next();
        let scene_nodes = actor_ccs.scene_nodes();

        let mut world_matrices = if let Some(ref c) = clump {
            // Check for authentic idle/neutral pose animation (e.g. nut10, nut00, idl00, rdy00)
            let anim = actor_ccs.animations().into_iter().find(|a| {
                a.name.contains("nut") || a.name.contains("idl") || a.name.contains("rdy") || a.name.contains("ski")
            });
            if let Some(a) = anim {
                a.compute_world_matrices(c, &scene_nodes, actor_ccs, 0.0)
            } else {
                c.compute_world_matrices(&scene_nodes)
            }
        } else {
            HashMap::new()
        };

        let root_mat = hackgu_ccs::Mat4::from_translation([position.x, position.y, position.z])
            .mul(&hackgu_ccs::Mat4::from_rot_z(yaw));

        for mat in world_matrices.values_mut() {
            *mat = root_mat.mul(mat);
        }

        // Rigid parts are placed by the Object node that owns them (any clump of the file), not by skinning.
        let owners = actor_ccs.model_owner_nodes();
        let all_clumps = actor_ccs.clumps();
        let clump_wm: Vec<HashMap<u32, hackgu_ccs::Mat4>> = all_clumps.iter().map(|c| c.compute_world_matrices(&scene_nodes)).collect();

        // 3. Decode models and push GPU submeshes
        let mut actor_submesh_indices = Vec::new();
        let mut actor_local_vertices = Vec::new();
        let mut actor_local_normals = Vec::new();
        let mut actor_local_uvs = Vec::new();
        let mut actor_local_colors = Vec::new();
        let mut actor_models = Vec::new();

        for model_obj in actor_ccs.objects_of_type(hackgu_ccs::objects::SECTION_MODEL) {
            let name = actor_ccs.object_name(model_obj.object_id).unwrap_or("actor_model");
            if let Ok(mut model) = hackgu_ccs::DecodedModel::from_payload(name, &model_obj.payload, actor_ccs.header.raw_version) {
                if model.model_type == 8 || name.to_lowercase().contains("shadow") {
                    continue;
                }

                // Effect geometry (e.g. itembox01's open sparkle MDL_itembox_fx01/fx02) is only shown when
                // triggered; spawned actors start without it.
                if name.contains("_fx") {
                    continue;
                }
                let rigid = model.is_rigid();
                if rigid {
                    let m = owners
                        .get(&model_obj.object_id)
                        .and_then(|(ci, n)| clump_wm.get(*ci).and_then(|w| w.get(n)).copied())
                        .unwrap_or(hackgu_ccs::Mat4::IDENTITY);
                    model.apply_rigid(&m);
                } else if let Some(ref c) = clump {
                    if !world_matrices.is_empty() {
                        model.apply_skeleton(c, &world_matrices);
                    }
                }

                for mesh in &model.meshes {
                    if mesh.vertices.is_empty() || mesh.indices.is_empty() {
                        continue;
                    }

                    let mut loc_pos = Vec::with_capacity(mesh.vertices.len());
                    let mut loc_nor = Vec::with_capacity(mesh.vertices.len());
                    let mut loc_uv = Vec::with_capacity(mesh.vertices.len());
                    let loc_col: Vec<[f32; 4]> = mesh.vertices.iter().map(|v| vcolor(v.color)).collect();

                    let gpu_vertices: Vec<GpuVertex> = mesh
                        .vertices
                        .iter()
                        .map(|v| {
                            loc_pos.push(v.position);
                            loc_nor.push(v.normal);
                            loc_uv.push([v.uv[0], 1.0 - v.uv[1]]);
                            let p = if clump.is_none() || rigid {
                                root_mat.transform_point(v.position)
                            } else {
                                v.position
                            };
                            let n = if clump.is_none() || rigid {
                                root_mat.transform_vector(v.normal)
                            } else {
                                v.normal
                            };
                            GpuVertex {
                                position: p,
                                normal: n,
                                uv: [v.uv[0], 1.0 - v.uv[1]],
                                color: vcolor(v.color),
                            }
                        })
                        .collect();

                    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("actor_vertex_buffer"),
                        contents: bytemuck::cast_slice(&gpu_vertices),
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    });

                    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("actor_index_buffer"),
                        contents: bytemuck::cast_slice(&mesh.indices),
                        usage: wgpu::BufferUsages::INDEX,
                    });

                    let sub_idx = self.submeshes.len();
                    self.submeshes.push(GpuSubMesh {
                        vertex_buffer,
                        index_buffer,
                        index_count: mesh.indices.len() as u32,
                        material_index: mesh.material_index + mat_offset,
                        is_background: false,
                        blend: 0,
                        bg_layer: 0,
                        visible: true,
                    });

                    actor_submesh_indices.push(sub_idx);
                    actor_local_vertices.push(loc_pos);
                    actor_local_normals.push(loc_nor);
                    actor_local_uvs.push(loc_uv);
                    actor_local_colors.push(loc_col);
                }

                actor_models.push(model);
            }
        }

        self.spawned_actors.push(ActorSubMeshRef {
            name: actor_ccs.raw_objects.first().and_then(|o| actor_ccs.object_name(o.object_id)).unwrap_or("actor").to_string(),
            submesh_indices: actor_submesh_indices,
            local_vertices: actor_local_vertices,
            local_normals: actor_local_normals,
            local_uvs: actor_local_uvs,
            local_colors: actor_local_colors,
            clump,
            scene_nodes,
            models: actor_models,
            anim_frame: 0.0,
        });
    }

    /// Spawn the models owned by clump `clump_name` of `ccs` as a prop; returns its index in `props`.
    pub fn spawn_prop(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, texture_layout: &wgpu::BindGroupLayout, ccs: &hackgu_ccs::CcsFile, clump_name: &str) -> Option<usize> {
        let clumps = ccs.clumps();
        let ci = clumps.iter().position(|c| ccs.object_name(c.object_id) == Some(clump_name))?;
        let clump = clumps[ci].clone();
        let nodes = ccs.scene_nodes();
        let owners = ccs.model_owner_nodes();
        // Textures/materials of the file (shared by every prop spawned from it).
        let mat_offset = self.materials.keys().copied().max().unwrap_or(0x300000) + 1000;
        let mut texs: HashMap<u32, Arc<GpuTexture>> = HashMap::new();
        for t in ccs.objects_of_type(hackgu_ccs::objects::SECTION_TEXTURE) {
            let clut_id = t.payload.get(0..4).map(|b| i32::from_le_bytes(b.try_into().unwrap())).unwrap_or(0);
            let clut = if clut_id > 0 {
                ccs.raw_objects.iter().find(|o| o.object_id == clut_id as u32 && o.section_type == hackgu_ccs::objects::SECTION_CLUT).and_then(|o| o.payload.get(16..))
            } else {
                None
            };
            let name = ccs.object_name(t.object_id).unwrap_or("prop_tex");
            if let Ok(d) = hackgu_ccs::DecodedTexture::from_payload_with_clut(name, &t.payload, clut) {
                texs.insert(t.object_id, Arc::new(GpuTexture::from_rgba(device, queue, texture_layout, d.width as u32, d.height as u32, &d.rgba, Some(name))));
            }
        }
        for (mid, mat) in &ccs.materials() {
            if let Some(t) = texs.get(&mat.texture_obj_id) {
                self.materials.insert(mid + mat_offset, Arc::clone(t));
            }
        }
        let mut models = Vec::new();
        for m in ccs.objects_of_type(hackgu_ccs::objects::SECTION_MODEL) {
            let Some(&(oc, node)) = owners.get(&m.object_id) else { continue };
            if oc != ci {
                continue;
            }
            let name = ccs.object_name(m.object_id).unwrap_or("prop");
            let Ok(model) = hackgu_ccs::DecodedModel::from_payload(name, &m.payload, ccs.header.raw_version) else { continue };
            if model.model_type == 8 || name.to_lowercase().contains("shadow") {
                continue;
            }
            let mut subs = Vec::new();
            for mesh in model.meshes.iter().filter(|m| !m.vertices.is_empty() && !m.indices.is_empty()) {
                let verts: Vec<GpuVertex> = mesh.vertices.iter().map(|v| GpuVertex { position: v.position, normal: v.normal, uv: [v.uv[0], 1.0 - v.uv[1]], color: vcolor(v.color) }).collect();
                let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("prop_vb"), contents: bytemuck::cast_slice(&verts), usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST });
                let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("prop_ib"), contents: bytemuck::cast_slice(&mesh.indices), usage: wgpu::BufferUsages::INDEX });
                subs.push(self.submeshes.len());
                self.submeshes.push(GpuSubMesh {
                    vertex_buffer,
                    index_buffer,
                    index_count: mesh.indices.len() as u32,
                    material_index: mesh.material_index + mat_offset,
                    is_background: false,
                    blend: blend_mode(model.draw_flags),
                    bg_layer: 0,
                    visible: true,
                });
            }
            models.push((model, node, subs));
        }
        // Effect sprites parented to this clump's objects.
        let mut effects = Vec::new();
        for e in ccs.objects_of_type(hackgu_ccs::objects::SECTION_EFFECT) {
            let w = |i: usize| e.payload.get(i * 4..i * 4 + 4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).unwrap_or(0);
            let f = |i: usize| f32::from_bits(w(i));
            // Word 0 = texture and word 1 = parent, both indices into the file's object table (resolved by name:
            // e.g. EFF_op00p2 -> TEX_op00_p00 / OBJ_op00_03). Only effects whose parent is a bone of this clump.
            let (tex_ref, parent_ref) = (w(0), w(1));
            let parent_name = ccs.object_name(parent_ref).unwrap_or("");
            let Some(&parent) = clump.bone_obj_ids.iter().find(|&&b| ccs.object_name(b) == Some(parent_name)) else { continue };
            let tex_name = ccs.object_name(tex_ref).unwrap_or("");
            let Some((&tex, t)) = texs.iter().find(|(id, _)| ccs.object_name(**id) == Some(tex_name)) else { continue };
            let key = mat_offset + 500_000 + tex;
            self.materials.insert(key, Arc::clone(t));
            let verts = [GpuVertex { position: [0.0; 3], normal: [0.0, 0.0, 1.0], uv: [0.0, 0.0], color: [1.0; 4] }; 4];
            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("effect_vb"), contents: bytemuck::cast_slice(&verts), usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST });
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some("effect_ib"), contents: bytemuck::cast_slice(&[0u32, 1, 2, 0, 2, 3]), usage: wgpu::BufferUsages::INDEX });
            effects.push(PropEffect {
                name: ccs.object_name(e.object_id).unwrap_or("").to_string(),
                parent,
                rect: [f(3) * 0.01, f(4) * 0.01, f(5) * 0.01, f(6) * 0.01],
                sub: self.submeshes.len(),
            });
            self.submeshes.push(GpuSubMesh { vertex_buffer, index_buffer, index_count: 6, material_index: key, is_background: false, blend: 1, bg_layer: 0, visible: false });
        }
        self.props.push(PropInstance { clump, nodes, models, effects });
        Some(self.props.len() - 1)
    }

    /// Place a prop with an arbitrary root matrix (e.g. a weapon in a hand bone), bind pose.
    pub fn update_prop_matrix(&mut self, queue: &wgpu::Queue, idx: usize, root: &hackgu_ccs::Mat4) {
        let Some(p) = self.props.get_mut(idx) else { return };
        let world = p.clump.compute_world_matrices(&p.nodes);
        for (model, node, subs) in &mut p.models {
            let m = world.get(node).copied().unwrap_or(hackgu_ccs::Mat4::IDENTITY);
            model.apply_rigid(&root.mul(&m));
            for (mesh, &si) in model.meshes.iter().filter(|m| !m.vertices.is_empty() && !m.indices.is_empty()).zip(subs.iter()) {
                let v: Vec<GpuVertex> = mesh.vertices.iter().map(|v| GpuVertex { position: v.position, normal: v.normal, uv: [v.uv[0], 1.0 - v.uv[1]], color: vcolor(v.color) }).collect();
                if let Some(sm) = self.submeshes.get(si) {
                    queue.write_buffer(&sm.vertex_buffer, 0, bytemuck::cast_slice(&v));
                }
            }
        }
    }

    /// Place a prop at (position, yaw), posed by `anim` at `frame` (bind pose when None).
    pub fn update_prop(&mut self, queue: &wgpu::Queue, idx: usize, position: glam::Vec3, yaw: f32, anim: Option<(&hackgu_ccs::CcsAnimation, &hackgu_ccs::CcsFile)>, frame: f32) {
        let (right, up) = self.billboard_axes;
        let Some(p) = self.props.get_mut(idx) else { return };
        let world = match anim {
            Some((a, ccs)) => a.compute_world_matrices(&p.clump, &p.nodes, ccs, frame),
            None => p.clump.compute_world_matrices(&p.nodes),
        };
        let root = hackgu_ccs::Mat4::from_translation([position.x, position.y, position.z]).mul(&hackgu_ccs::Mat4::from_rot_z(yaw));
        for (model, node, subs) in &mut p.models {
            if model.is_rigid() {
                let m = world.get(node).copied().unwrap_or(hackgu_ccs::Mat4::IDENTITY);
                model.apply_rigid(&root.mul(&m));
            } else {
                let w: HashMap<u32, hackgu_ccs::Mat4> = world.iter().map(|(k, m)| (*k, root.mul(m))).collect();
                model.apply_skeleton(&p.clump, &w);
            }
            for (mesh, &si) in model.meshes.iter().filter(|m| !m.vertices.is_empty() && !m.indices.is_empty()).zip(subs.iter()) {
                let v: Vec<GpuVertex> = mesh.vertices.iter().map(|v| GpuVertex { position: v.position, normal: v.normal, uv: [v.uv[0], 1.0 - v.uv[1]], color: vcolor(v.color) }).collect();
                if let Some(sm) = self.submeshes.get(si) {
                    queue.write_buffer(&sm.vertex_buffer, 0, bytemuck::cast_slice(&v));
                }
            }
        }
        // Effect sprites: parent node world (+ the effect's own animated offset/spin), camera-facing.
        let ccs_anim = anim;
        for e in &p.effects {
            let parent = world.get(&e.parent).copied().unwrap_or(hackgu_ccs::Mat4::IDENTITY);
            let ch = ccs_anim.and_then(|(a, _)| a.channel_by_name.get(&e.name).map(|&c| &a.channels[c]));
            if ccs_anim.is_some() && ch.is_none() {
                // Sprites the playing animation does not drive belong to another variant of the object.
                if let Some(sm) = self.submeshes.get_mut(e.sub) {
                    sm.visible = false;
                }
                continue;
            }
            let off = ch.map(|c| c.sample_translation(frame, [0.0; 3])).unwrap_or([0.0; 3]);
            let spin = ch.map(|c| {
                let q = c.sample_rotation_quat(frame, [0.0; 3]);
                2.0 * q[2].atan2(q[3])
            }).unwrap_or(0.0);
            let c = root.mul(&parent).transform_point(off);
            let c = glam::Vec3::from(c);
            let (s, co) = spin.sin_cos();
            let (r, u) = (right * co + up * s, up * co - right * s);
            let [x0, y0, x1, y1] = e.rect;
            let corner = |x: f32, y: f32| (c + r * x + u * y).to_array();
            let v = [
                GpuVertex { position: corner(x0, y0), normal: [0.0, 0.0, 1.0], uv: [0.0, 1.0], color: [1.0; 4] },
                GpuVertex { position: corner(x1, y0), normal: [0.0, 0.0, 1.0], uv: [1.0, 1.0], color: [1.0; 4] },
                GpuVertex { position: corner(x1, y1), normal: [0.0, 0.0, 1.0], uv: [1.0, 0.0], color: [1.0; 4] },
                GpuVertex { position: corner(x0, y1), normal: [0.0, 0.0, 1.0], uv: [0.0, 0.0], color: [1.0; 4] },
            ];
            if let Some(sm) = self.submeshes.get_mut(e.sub) {
                queue.write_buffer(&sm.vertex_buffer, 0, bytemuck::cast_slice(&v));
                sm.visible = true;
            }
        }
    }

    /// Dynamically update a spawned actor's world transform in real-time.
    pub fn update_actor_transform(&mut self, queue: &wgpu::Queue, actor_idx: usize, position: glam::Vec3, yaw: f32) {
        if let Some(act) = self.spawned_actors.get(actor_idx) {
            let root_mat = hackgu_ccs::Mat4::from_translation([position.x, position.y, position.z])
                .mul(&hackgu_ccs::Mat4::from_rot_z(yaw));

            for (m_idx, &sub_idx) in act.submesh_indices.iter().enumerate() {
                if sub_idx < self.submeshes.len() && m_idx < act.local_vertices.len() {
                    let loc_v = &act.local_vertices[m_idx];
                    let loc_n = &act.local_normals[m_idx];
                    let loc_u = &act.local_uvs[m_idx];

                    let mut gpu_vertices: Vec<GpuVertex> = Vec::with_capacity(loc_v.len());
                    for i in 0..loc_v.len() {
                        gpu_vertices.push(GpuVertex {
                            position: root_mat.transform_point(loc_v[i]),
                            normal: root_mat.transform_vector(loc_n[i]),
                            uv: loc_u[i],
                            color: act.local_colors.get(m_idx).and_then(|c| c.get(i)).copied().unwrap_or([1.0; 4]),
                        });
                    }

                    queue.write_buffer(&self.submeshes[sub_idx].vertex_buffer, 0, bytemuck::cast_slice(&gpu_vertices));
                }
            }
        }
    }

    /// Show or hide every submesh of a spawned actor.
    pub fn set_actor_visible(&mut self, actor_idx: usize, visible: bool) {
        if let Some(act) = self.spawned_actors.get(actor_idx) {
            for &i in &act.submesh_indices {
                if let Some(sm) = self.submeshes.get_mut(i) {
                    sm.visible = visible;
                }
            }
        }
    }

    /// Show or hide the scene's own character (the controllable player model and its weapons).
    pub fn set_main_character_visible(&mut self, visible: bool) {
        let mut idx: Vec<usize> = self.animated_models.iter().flat_map(|m| m.submesh_indices.clone()).collect();
        for att in &self.socket_attachments {
            idx.extend(att.submesh_indices.iter().flatten().copied());
        }
        for i in idx {
            if let Some(sm) = self.submeshes.get_mut(i) {
                sm.visible = visible;
            }
        }
    }

    /// Pose a spawned actor from bone world matrices (e.g. a crossfade of two animations).
    pub fn update_actor_with_matrices(
        &mut self,
        queue: &wgpu::Queue,
        actor_idx: usize,
        position: glam::Vec3,
        yaw: f32,
        world_matrices: &HashMap<u32, hackgu_ccs::Mat4>,
    ) {
        let Some(act) = self.spawned_actors.get_mut(actor_idx) else { return };
        let Some(clump) = &act.clump else { return };
        let root_mat = hackgu_ccs::Mat4::from_translation([position.x, position.y, position.z]).mul(&hackgu_ccs::Mat4::from_rot_z(yaw));
        let mut sub_offset = 0;
        for model in &mut act.models {
            model.apply_skeleton(clump, world_matrices);
            for mesh in model.meshes.iter().filter(|m| !m.vertices.is_empty() && !m.indices.is_empty()) {
                if let Some(&sub_idx) = act.submesh_indices.get(sub_offset) {
                    if let Some(sm) = self.submeshes.get(sub_idx) {
                        let v: Vec<GpuVertex> = mesh
                            .vertices
                            .iter()
                            .map(|v| GpuVertex {
                                position: root_mat.transform_point(v.position),
                                normal: root_mat.transform_vector(v.normal),
                                uv: [v.uv[0], 1.0 - v.uv[1]],
                                color: vcolor(v.color),
                            })
                            .collect();
                        queue.write_buffer(&sm.vertex_buffer, 0, bytemuck::cast_slice(&v));
                    }
                }
                sub_offset += 1;
            }
        }
    }

    /// Dynamically animate a spawned actor using an animation keyframe set.
    pub fn update_actor_animation(
        &mut self,
        queue: &wgpu::Queue,
        actor_idx: usize,
        position: glam::Vec3,
        yaw: f32,
        anim: &hackgu_ccs::CcsAnimation,
        anim_ccs: &hackgu_ccs::CcsFile,
        frame: f32,
    ) {
        if let Some(act) = self.spawned_actors.get_mut(actor_idx) {
            act.anim_frame = frame;
            let root_mat = hackgu_ccs::Mat4::from_translation([position.x, position.y, position.z])
                .mul(&hackgu_ccs::Mat4::from_rot_z(yaw));

            if let Some(clump) = &act.clump {
                let world_matrices = anim.compute_world_matrices(clump, &act.scene_nodes, anim_ccs, frame);

                let mut sub_offset = 0;
                for model in &mut act.models {
                    model.apply_skeleton(clump, &world_matrices);

                    for mesh in model.meshes.iter().filter(|m| !m.vertices.is_empty() && !m.indices.is_empty()) {
                        if sub_offset < act.submesh_indices.len() {
                            let sub_idx = act.submesh_indices[sub_offset];
                            if sub_idx < self.submeshes.len() {
                                let mut gpu_vertices = Vec::with_capacity(mesh.vertices.len());
                                for v in &mesh.vertices {
                                    gpu_vertices.push(GpuVertex {
                                        position: root_mat.transform_point(v.position),
                                        normal: root_mat.transform_vector(v.normal),
                                        uv: [v.uv[0], 1.0 - v.uv[1]],
                                        color: vcolor(v.color),
                                    });
                                }
                                queue.write_buffer(&self.submeshes[sub_idx].vertex_buffer, 0, bytemuck::cast_slice(&gpu_vertices));
                            }
                            sub_offset += 1;
                        }
                    }
                }
            }
        }
    }

    /// Recompute skeletal transforms and stream updated vertex positions to GPU vertex buffers.
    pub fn apply_current_pose(&mut self, queue: &wgpu::Queue, ccs: &hackgu_ccs::CcsFile) {
        if let Some(clump) = &self.clump {
            let mut world_matrices = if let Some(anim_idx) = self.active_anim_idx {
                let anim = &self.animations[anim_idx];
                anim.compute_world_matrices(clump, &self.scene_nodes, ccs, self.current_frame)
            } else {
                clump.compute_world_matrices(&self.scene_nodes)
            };

            if let Some(root) = self.root_transform {
                for mat in world_matrices.values_mut() {
                    *mat = root.mul(mat);
                }
            }

            // 1. Update character skinned meshes
            for anim_ref in &mut self.animated_models {
                if let Some(node) = anim_ref.rigid_node {
                    let m = world_matrices.get(&node).copied().or(self.root_transform).unwrap_or(hackgu_ccs::Mat4::IDENTITY);
                    anim_ref.model.apply_rigid(&m);
                } else {
                    anim_ref.model.apply_skeleton(clump, &world_matrices);
                }

                for (m_idx, &sub_idx) in anim_ref.submesh_indices.iter().enumerate() {
                    if m_idx < anim_ref.model.meshes.len() {
                        let mesh = &anim_ref.model.meshes[m_idx];
                        let gpu_submesh = &self.submeshes[sub_idx];

                        let gpu_vertices: Vec<GpuVertex> = mesh
                            .vertices
                            .iter()
                            .map(|v| GpuVertex {
                                position: v.position,
                                normal: v.normal,
                                uv: [v.uv[0], 1.0 - v.uv[1]],
                                color: vcolor(v.color),
                            })
                            .collect();

                        queue.write_buffer(&gpu_submesh.vertex_buffer, 0, bytemuck::cast_slice(&gpu_vertices));
                    }
                }
            }

            // 2. Update socket-attached weapons
            for att in &self.socket_attachments {
                let socket_mat = world_matrices
                    .get(&att.socket_bone_id)
                    .or_else(|| {
                        // Fallback to r hand / l hand if prop channel has no keyframe in current anim
                        if att.name.contains("1") || att.name.contains("r") {
                            world_matrices.iter().find(|(&oid, _)| {
                                ccs.object_name(oid).map(|n| n.contains("r hand")).unwrap_or(false)
                            }).map(|(_, m)| m)
                        } else {
                            world_matrices.iter().find(|(&oid, _)| {
                                ccs.object_name(oid).map(|n| n.contains("l hand")).unwrap_or(false)
                            }).map(|(_, m)| m)
                        }
                    })
                    .copied()
                    .unwrap_or(hackgu_ccs::Mat4::IDENTITY);

                let final_mat = socket_mat.mul(&att.local_matrix);

                for (m_idx, model) in att.models.iter().enumerate() {
                    if let Some(sub_indices) = att.submesh_indices.get(m_idx) {
                        for (mesh_idx, &sub_idx) in sub_indices.iter().enumerate() {
                            if mesh_idx < model.meshes.len() && sub_idx < self.submeshes.len() {
                                let mesh = &model.meshes[mesh_idx];
                                let gpu_submesh = &self.submeshes[sub_idx];

                                let gpu_vertices: Vec<GpuVertex> = mesh
                                    .vertices
                                    .iter()
                                    .map(|v| {
                                        let p = final_mat.transform_point(v.position);
                                        let n = final_mat.transform_vector(v.normal);
                                        GpuVertex {
                                            position: p,
                                            normal: n,
                                            uv: [v.uv[0], 1.0 - v.uv[1]],
                                            color: vcolor(v.color),
                                        }
                                    })
                                    .collect();

                                queue.write_buffer(&gpu_submesh.vertex_buffer, 0, bytemuck::cast_slice(&gpu_vertices));
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Map a model's draw-flag byte (header byte 8) to a render pass. Bit 0 marks additive layers (Mac Anu water,
/// caustics, window reflections), bit 1 alpha-blended decals (handrail shadows, graffiti). Other bits (0x40 on
/// every character body) do not affect blending.
pub fn blend_mode(draw_flags: u8) -> u8 {
    if draw_flags & 1 != 0 {
        1
    } else if draw_flags & 2 != 0 {
        2
    } else {
        0
    }
}

/// Draw order for background models (all drawn before the world, without depth): dome, silhouettes, clouds, sun.
pub fn background_layer(name: &str) -> u8 {
    if name.starts_with("MDL_bs") {
        1
    } else if name.starts_with("MDL_sil") || name.starts_with("MDL_bl") {
        2
    } else if name.starts_with("MDL_bc") {
        3
    } else {
        4
    }
}
