use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use pollster::block_on;
use wgpu::util::DeviceExt;

use crate::camera::Camera;
use crate::mesh::{GpuScene, GpuVertex};

pub fn render_to_png<P: AsRef<Path>>(
    ccs_path: Option<&PathBuf>,
    out_png: P,
    width: u32,
    height: u32,
    camera_override: Option<Camera>,
    anim_override: Option<usize>,
    frame_override: Option<f32>,
) -> Result<(), String> {
    render_with_weapons(ccs_path, None, None, out_png, width, height, camera_override, anim_override, frame_override)
}

pub fn render_with_weapons<P: AsRef<Path>>(
    ccs_path: Option<&PathBuf>,
    weapon_path: Option<&PathBuf>,
    stage_path: Option<&PathBuf>,
    out_png: P,
    width: u32,
    height: u32,
    camera_override: Option<Camera>,
    anim_override: Option<usize>,
    frame_override: Option<f32>,
) -> Result<(), String> {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });

    let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
    }))
    .ok_or_else(|| "Failed to find a suitable GPU adapter".to_string())?;

    println!("Offscreen GPU Adapter: {}", adapter.get_info().name);

    let (device, queue) = block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: Some("headless_device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::default(),
        },
        None,
    ))
    .map_err(|e| format!("Failed to create device: {}", e))?;

    let texture_format = wgpu::TextureFormat::Rgba8UnormSrgb;

    // Color render target
    let render_target = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("render_target"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: texture_format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let render_target_view = render_target.create_view(&wgpu::TextureViewDescriptor::default());

    // Depth render target
    let depth_target = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("depth_target"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let depth_target_view = depth_target.create_view(&wgpu::TextureViewDescriptor::default());

    // Texture bind group layout
    let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("headless_texture_layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });

    // Camera
    let mut camera = camera_override.unwrap_or_else(|| Camera::new(width as f32, height as f32));
    camera.aspect = width as f32 / height as f32;

    // Load scene
    let scene = if let Some(raw_path) = ccs_path {
        let path = crate::app::resolve_ccs_path(raw_path).unwrap_or_else(|| raw_path.clone());
        let data = std::fs::read(&path).map_err(|e| format!("Failed to read CCS ({}): {}", path.display(), e))?;
        let ccs = hackgu_ccs::CcsFile::parse(&data).map_err(|e| format!("Failed to parse CCS: {}", e))?;
        let mut loaded = GpuScene::from_ccs(&device, &queue, &texture_bind_group_layout, &ccs);
        println!(
            "Loaded {} submeshes, bounds min: {:?}, max: {:?}",
            loaded.submeshes.len(),
            loaded.min_bounds,
            loaded.max_bounds
        );

        if let Some(a_idx) = anim_override {
            loaded.select_animation(a_idx, &queue, &ccs);
        }
        if let Some(f) = frame_override {
            loaded.seek_frame(f, &queue, &ccs);
        }

        if let Some(raw_wep) = weapon_path {
            let wep_p = crate::app::resolve_ccs_path(raw_wep).unwrap_or_else(|| raw_wep.clone());
            if let Ok(wep_data) = std::fs::read(&wep_p) {
                if let Ok(wep_ccs) = hackgu_ccs::CcsFile::parse(&wep_data) {
                    println!("Headless: attaching weapon {}", wep_p.display());
                    loaded.attach_weapon(&device, &queue, &texture_bind_group_layout, &wep_ccs, &ccs, "prop1", hackgu_ccs::Mat4::IDENTITY);
                    loaded.attach_weapon(&device, &queue, &texture_bind_group_layout, &wep_ccs, &ccs, "prop2", hackgu_ccs::Mat4::IDENTITY);
                }
            }
        }

        if let Some(raw_stg) = stage_path {
            let stg_p = crate::app::resolve_ccs_path(raw_stg).unwrap_or_else(|| raw_stg.clone());
            let is_mac_anu = stg_p.to_string_lossy().contains("town01");
            if is_mac_anu {
                println!("Headless: Blending Mac Anu districts (00h, 01h, 02h)...");
                for dist_name in ["town01_00h.ccs", "town01_01h.ccs", "town01_02h.ccs"] {
                    if let Some(dist_p) = crate::app::resolve_ccs_path(Path::new(dist_name)) {
                        if let Ok(dist_data) = std::fs::read(&dist_p) {
                            if let Ok(dist_ccs) = hackgu_ccs::CcsFile::parse(&dist_data) {
                                loaded.load_stage_environment(&device, &queue, &texture_bind_group_layout, &dist_ccs);
                            }
                        }
                    }
                }
                loaded.add_ground_floor(&device, 0.0, 60.0, 320.0, 220.0, -0.02);
            } else {
                if let Ok(stg_data) = std::fs::read(&stg_p) {
                    if let Ok(stg_ccs) = hackgu_ccs::CcsFile::parse(&stg_data) {
                        println!("Headless: loading stage environment {}", stg_p.display());
                        loaded.load_stage_environment(&device, &queue, &texture_bind_group_layout, &stg_ccs);
                    }
                }
                loaded.add_ground_floor(&device, 0.0, 0.0, 150.0, 150.0, -0.02);
            }
        }

        if stage_path.is_some() {
            // Spawn player in the plaza at Y = 25.0, facing forward (+Y away from camera)
            let spawn_pos = glam::Vec3::new(0.0, 25.0, 0.0);
            let rot = hackgu_ccs::Mat4::from_rot_z(std::f32::consts::PI);
            let trans = hackgu_ccs::Mat4::from_translation([spawn_pos.x, spawn_pos.y, spawn_pos.z]);
            loaded.root_transform = Some(trans.mul(&rot));
            loaded.apply_current_pose(&queue, &ccs);

            // Third-person camera behind character looking towards Raven Guild
            camera.target = spawn_pos + glam::Vec3::new(0.0, 0.0, 0.95);
            camera.distance = 4.2;
            camera.yaw = 0.0;
            camera.pitch = 0.22;
        } else if loaded.has_background {
            println!("Headless: background sky/silhouette detected. Focusing camera on playable bounds.");
            camera.focus_bounds(loaded.fg_min_bounds, loaded.fg_max_bounds);
        } else {
            camera.focus_bounds(loaded.min_bounds, loaded.max_bounds);
        }
        println!("Camera target: {:?}, distance: {}", camera.target, camera.distance);
        Some(loaded)
    } else {
        None
    };

    let camera_uniform = camera.to_uniform();
    let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("headless_camera_buffer"),
        contents: bytemuck::cast_slice(&[camera_uniform]),
        usage: wgpu::BufferUsages::UNIFORM,
    });

    let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("headless_camera_layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });

    let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("headless_camera_bind_group"),
        layout: &camera_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: camera_buffer.as_entire_binding(),
        }],
    });

    // Pipeline
    let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("headless_pipeline_layout"),
        bind_group_layouts: &[&camera_bind_group_layout, &texture_bind_group_layout],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("headless_render_pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[GpuVertex::desc()],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: texture_format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });

    // Render pass
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("headless_encoder"),
    });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("headless_render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &render_target_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.05,
                        g: 0.08,
                        b: 0.15,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_target_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&render_pipeline);
        render_pass.set_bind_group(0, &camera_bind_group, &[]);

        if let Some(s) = &scene {
            for submesh in &s.submeshes {
                if s.has_background && submesh.is_background {
                    continue;
                }
                let tex = s.materials.get(&submesh.material_index).unwrap_or(&s.default_texture);
                render_pass.set_bind_group(1, &tex.bind_group, &[]);
                render_pass.set_vertex_buffer(0, submesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(submesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..submesh.index_count, 0, 0..1);
            }
        }
    }

    // Readback buffer (row-pitch alignment is 256 bytes)
    let bytes_per_pixel = 4u32;
    let unpadded_bytes_per_row = width * bytes_per_pixel;
    let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let padded_bytes_per_row_padding = (align - unpadded_bytes_per_row % align) % align;
    let padded_bytes_per_row = unpadded_bytes_per_row + padded_bytes_per_row_padding;

    let output_buffer_size = (padded_bytes_per_row * height) as u64;
    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("output_buffer"),
        size: output_buffer_size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: &render_target,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &output_buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded_bytes_per_row),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );

    queue.submit(std::iter::once(encoder.finish()));

    // Map buffer and read RGBA
    let buffer_slice = output_buffer.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
        tx.send(result).unwrap();
    });

    device.poll(wgpu::Maintain::Wait);
    rx.recv()
        .map_err(|e| format!("Buffer map failed: {:?}", e))?
        .map_err(|e| format!("Buffer map error: {:?}", e))?;

    let padded_data = buffer_slice.get_mapped_range();
    let mut unpadded_rgba = Vec::with_capacity((width * height * 4) as usize);

    for chunk in padded_data.chunks(padded_bytes_per_row as usize) {
        unpadded_rgba.extend_from_slice(&chunk[..unpadded_bytes_per_row as usize]);
    }
    drop(padded_data);
    output_buffer.unmap();

    // Write PNG without external dependencies
    write_png(out_png.as_ref(), width, height, &unpadded_rgba)
}

fn write_png(path: &Path, width: u32, height: u32, rgba: &[u8]) -> Result<(), String> {

    // Standard PNG encoder using minimal flate/zlib from std
    // We can use a fast adler32 + crc32 implementation
    let mut file = File::create(path).map_err(|e| format!("Failed to create PNG: {}", e))?;
    
    // PNG Header
    file.write_all(b"\x89PNG\r\n\x1a\n").map_err(|e| e.to_string())?;

    // IHDR
    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.push(8); // 8-bit depth
    ihdr.push(6); // RGBA color
    ihdr.push(0); // compression deflate
    ihdr.push(0); // filter adaptive
    ihdr.push(0); // non-interlaced
    write_chunk(&mut file, b"IHDR", &ihdr)?;

    // IDAT (filter type 0 None + RGBA per scanline)
    let stride = (width * 4) as usize;
    let mut raw_scanlines = Vec::with_capacity((height as usize) * (stride + 1));
    for y in 0..(height as usize) {
        raw_scanlines.push(0u8); // filter type None
        let row_start = y * stride;
        raw_scanlines.extend_from_slice(&rgba[row_start..row_start + stride]);
    }

    // Compress using standard zlib wrapper
    let compressed = miniz_compress(&raw_scanlines);
    write_chunk(&mut file, b"IDAT", &compressed)?;

    // IEND
    write_chunk(&mut file, b"IEND", &[])?;

    println!("Saved offscreen GPU render: {} ({}x{})", path.display(), width, height);
    Ok(())
}

fn write_chunk<W: Write>(w: &mut W, tag: &[u8; 4], data: &[u8]) -> Result<(), String> {
    let len = (data.len() as u32).to_be_bytes();
    w.write_all(&len).map_err(|e| e.to_string())?;
    w.write_all(tag).map_err(|e| e.to_string())?;
    w.write_all(data).map_err(|e| e.to_string())?;

    let mut crc = crc32_init();
    crc = crc32_update(crc, tag);
    crc = crc32_update(crc, data);
    let crc_bytes = crc.to_be_bytes();
    w.write_all(&crc_bytes).map_err(|e| e.to_string())?;
    Ok(())
}

fn crc32_init() -> u32 {
    0xFFFFFFFF
}

fn crc32_update(mut crc: u32, data: &[u8]) -> u32 {
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

fn miniz_compress(data: &[u8]) -> Vec<u8> {
    // Standard zlib envelope: header 0x78, 0x9C (default compression) + raw deflate + adler32
    // For fast uncompressed deflate block (BTYPE=00):
    let mut out = Vec::new();
    out.push(0x78);
    out.push(0x01); // No compression zlib header

    let mut offset = 0;
    while offset < data.len() {
        let chunk_len = (data.len() - offset).min(65535);
        let is_last = (offset + chunk_len) == data.len();

        let header = if is_last { 0x01u8 } else { 0x00u8 };
        out.push(header);

        let len_u16 = chunk_len as u16;
        let nlen_u16 = !len_u16;

        out.extend_from_slice(&len_u16.to_le_bytes());
        out.extend_from_slice(&nlen_u16.to_le_bytes());
        out.extend_from_slice(&data[offset..offset + chunk_len]);

        offset += chunk_len;
    }

    // Adler32
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for &byte in data {
        a = (a + byte as u32) % 65521;
        b = (b + a) % 65521;
    }
    let adler = (b << 16) | a;
    out.extend_from_slice(&adler.to_be_bytes());

    out
}
