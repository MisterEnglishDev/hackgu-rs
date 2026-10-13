use std::path::{Path, PathBuf};
use std::sync::Arc;
use pollster::block_on;
use wgpu::util::DeviceExt;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowAttributes, WindowId};

use crate::camera::Camera;
use crate::mesh::{GpuScene, GpuVertex};

pub fn resolve_ccs_path(path: &Path) -> Option<PathBuf> {
    if path.exists() && path.is_file() {
        return Some(path.to_path_buf());
    }

    let file_name = path.file_name()?.to_str()?;
    let stem = path.file_stem()?.to_str()?;

    let mut candidate_names = vec![file_name.to_string()];
    candidate_names.push(format!("{}.ccs", stem));
    candidate_names.push(format!("{}.css", stem));
    candidate_names.push(format!("{}.CCS", stem));
    candidate_names.push(format!("{}.CSS", stem));

    let base_prefixes = [
        PathBuf::from("."),
        PathBuf::from(".."),
        PathBuf::from("../.."),
        PathBuf::from("../../.."),
        PathBuf::from("hackgu-rs"),
        PathBuf::from("../hackgu-rs"),
        PathBuf::from("../../hackgu-rs"),
        PathBuf::from("../../../hackgu-rs"),
    ];

    for base in &base_prefixes {
        let p = base.join(path);
        if p.exists() && p.is_file() {
            return Some(p);
        }
        let p_str = p.to_string_lossy();
        if p_str.ends_with(".css") || p_str.ends_with(".CSS") {
            let alt = PathBuf::from(p_str[..p_str.len() - 4].to_string() + ".ccs");
            if alt.exists() && alt.is_file() {
                return Some(alt);
            }
        } else if p_str.ends_with(".ccs") || p_str.ends_with(".CCS") {
            let alt = PathBuf::from(p_str[..p_str.len() - 4].to_string() + ".css");
            if alt.exists() && alt.is_file() {
                return Some(alt);
            }
        }
    }

    let subfolders = [
        "",
        "target",
        "target/cmn/data/data/spc",
        "target/cmn/data/data/spc/ev",
        "target/cmn/data/data/pc",
        "target/cmn/data/data/spc/exc",
        "target/cmn/data/data/spc/ext",
        "target/cmn/data/data/eq",
        "target/cmn/data/data/stage",
        "target/cmn/data/data/npc",
        "target/vol1/data/data/town",
        "target/vol1/data/data/spc",
        "target/vol1/data/data/spc/exc",
        "target/vol1/data/data/spc/ext",
        "target/vol1/data/data/eq",
        "target/vol1/data/data/dungeon",
        "target/vol1/data/data/ava",
        "target/vol1/data/data/ev",
        "target/vol1/data/data/ev/epc",
        "target/vol1/data/data/ev/estr",
        "target/vol1/data/data/field",
        "target/vol1/data/data/en",
        "target/hackGU_vol2_a/vol2/data/data/spc",
        "target/hackGU_vol2_a/vol2/data/data/stage",
        "target/hackGU_vol2_a/vol2/data/data/en",
        "target/hackGU_vol3_a/vol3/data/data/spc",
        "target/hackGU_vol3_a/vol3/data/data/stage",
        "target/hackGU_vol3_a/vol3/data/data/en",
        "target/skeith_full.ccs/cmn/data/data/ava",
        "VRChat/work/haseo/raw",
        "VRChat/work/haseo/wraw",
    ];

    for base in &base_prefixes {
        for sub in &subfolders {
            let root = if sub.is_empty() {
                base.clone()
            } else {
                base.join(sub)
            };
            for name in &candidate_names {
                let p = root.join(name);
                if p.exists() && p.is_file() {
                    return Some(p);
                }
            }
        }
    }

    None
}


/// Follow-camera boom length and aim height (metres). See `reset_follow_camera`.
const FOLLOW_DISTANCE: f32 = 6.0;
const FOLLOW_HEIGHT: f32 = 1.9;

pub struct RenderState {
    pub window: Arc<Window>,
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub depth_texture_view: wgpu::TextureView,
    pub render_pipeline: wgpu::RenderPipeline,
    /// Additive layers (model draw flag 1: water caustics, reflections, glow). No depth write.
    pub additive_pipeline: wgpu::RenderPipeline,
    /// Alpha-blended decals (model draw flag 2: shadows, graffiti). No depth write.
    pub decal_pipeline: wgpu::RenderPipeline,
    /// Background (sky dome, silhouettes, clouds, sun): unlit, no depth test or write, drawn first.
    pub sky_pipeline: wgpu::RenderPipeline,
    pub sky_additive_pipeline: wgpu::RenderPipeline,
    pub camera: Camera,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    pub scene: Option<GpuScene>,
    pub ccs: Option<hackgu_ccs::CcsFile>,
    pub last_instant: std::time::Instant,
    pub show_background: bool,
    pub mouse_pressed: bool,
    pub right_mouse_pressed: bool,
    pub last_mouse_pos: Option<(f64, f64)>,
    pub egui_mgr: crate::gui::EguiManager,
    pub current_ccs_path: Option<PathBuf>,
    pub current_stage_path: Option<PathBuf>,
    pub player: crate::player::PlayerController,
    pub key_w: bool,
    pub key_s: bool,
    pub key_a: bool,
    pub key_d: bool,
    pub equipped_weapon: Option<String>,
    pub gilrs: Option<gilrs::Gilrs>,
    pub spawned_cutscene_id: Option<String>,
    pub movie_player: crate::moviefx::MoviePlayer,
    pub movie_renderer: crate::moviefx::MovieRenderer,
    /// Field collision without the temple seal (swapped in when the temple opens).
    pub field_open_collision: Option<hackgu_world::CollisionMesh>,
    /// "Mission Reward" panel (item name, seconds left), shown when the offering chest opens.
    pub reward_panel: Option<(String, f32)>,
    /// A movie to start after a delay (seconds), e.g. MM10145 once the offering chest has opened.
    pub pending_movie: Option<(String, f32)>,
    /// field_props indices of the altar chests (field) or the offering chest (statue room).
    pub altar_chests: Vec<usize>,
    /// Field tile submeshes over the temple interior (hidden while the camera is inside it).
    pub temple_cover: Vec<usize>,
    /// The tutorial field's Beast Temple interior is loaded instead of the field.
    pub temple_interior: bool,
    /// Story, fights and followers in the tutorial field.
    pub fieldplay: Option<crate::fieldplay::FieldPlay>,
    /// Frame for the next event (set by the story controller).
    pub next_event_frame: Option<crate::cutscene::EvTransform>,
    /// Haseo's last event mark (world position, yaw): play resumes there.
    pub last_haseo_mark: Option<(glam::Vec3, f32)>,
    /// Last event marks of every visible cast member (goblins hand over to the field fight from these).
    pub last_cast_marks: std::collections::HashMap<String, (glam::Vec3, f32)>,
    /// Animated stage props (Platforms ...): scene prop index, place, file, looping animation, time (frames).
    pub field_props: Vec<FieldProp>,
    /// Last Recode post filter (post_filter.bin); the 3D scene renders into its texture.
    pub postfx: crate::postfx::PostFx,
    /// Running event script (camera, cast, fades, messages all come from it).
    pub cutscene: Option<crate::cutscene::Cutscene>,
    /// Screen fade from the event script: 0 = clear, 1 = black.
    pub fade_black: f32,
    /// Localized message database for event text (None = Japanese from the scripts).
    pub lang_db: Option<std::sync::Arc<hackgu_game::msginfo::MessageInfo>>,
    pub sound_system: crate::sound::SoundSystem,
    pub last_spoken_voice: Option<String>,
    /// Frames rendered so far (automation / screenshots).
    pub frame_no: u32,
    /// Mac Anu's locators (town01.ccs, centimetres), e.g. DMY_ev1.. where town events play.
    pub town_locators: std::collections::HashMap<String, [f32; 3]>,
    /// Gamepad buttons pressed while a menu screen is up, handed to egui as keys next frame.
    pub pad_keys: Vec<egui::Key>,
    /// Left-stick menu navigation: the direction held and the time until it repeats.
    pub stick_nav: Option<(egui::Key, f32)>,
    /// The menu button (Y / E / Esc) was pressed while an event line asks for it ("Press <iconTRIANGLE> to open it").
    pub menu_prompt_pressed: bool,
    /// Open the Main Menu once the running event has ended (the prompt was answered).
    pub open_menu_after_event: bool,
    /// The current dungeon floor's floor-change platform (metres), and the floor it leads to.
    pub dungeon_platform: Option<(glam::Vec3, usize)>,
    /// Spawn point for the next dungeon floor load (arriving on another floor's platform).
    pub dungeon_arrival: Option<glam::Vec3>,
    /// The platform only works again once Haseo has stepped off it (he arrives standing on one).
    pub dungeon_platform_armed: bool,
    /// Event locators of the loaded dungeon floor that come from kit pieces (50/51: the last room's DMY_lastroom-ev1/2,
    /// placed by 0x1802E5BF9 / 0x1802E5D53 into the slots the frame resolver reads for 50/51).
    pub dungeon_piece_locators: std::collections::HashMap<i32, glam::Vec3>,
    /// Rotation about z (radians) of the piece that owns each of those locators (the script frame turns with it).
    pub dungeon_piece_yaw: std::collections::HashMap<i32, f32>,
    /// Script frame origin for those locators' events (see build_dungeon).
    pub dungeon_piece_frame: std::collections::HashMap<i32, glam::Vec3>,
    /// The last room's offering chest: position and field_props index.
    pub dungeon_chest: Option<(glam::Vec3, usize)>,
    /// field_props index of the Chaos Gate (held at its first frame).
    pub gate_prop: Option<usize>,
    /// Play the current place's own music when nothing else is playing (set by BGM op 138 and on entering a place).
    pub area_music_wanted: bool,
}

impl RenderState {
    pub fn new(window: Arc<Window>, ccs_path: Option<PathBuf>) -> Self {
        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .expect("Failed to find a suitable GPU adapter");

        println!("GPU Adapter: {}", adapter.get_info().name);

        let (device, queue) = block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("hackgu_device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
            },
            None,
        ))
        .expect("Failed to create GPU device");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        // COPY_SRC lets --shot read back the real swapchain image.
        let usage = if surface_caps.usages.contains(wgpu::TextureUsages::COPY_SRC) {
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC
        } else {
            wgpu::TextureUsages::RENDER_ATTACHMENT
        };
        let config = wgpu::SurfaceConfiguration {
            usage,
            format: surface_format,
            width,
            height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: surface_caps.alpha_modes[0],
            // egui draws through a non-sRGB view so its blending happens in gamma space, as egui expects and as the
            // original's 2D screens blend (translucent panels, glows and drop shadows match only this way).
            view_formats: if surface_format.is_srgb() { vec![surface_format.remove_srgb_suffix()] } else { vec![] },
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        // Depth buffer
        let depth_texture_view = Self::create_depth_view(&device, &config);

        // Camera setup
        let mut camera = Camera::new(width as f32, height as f32);
        let camera_uniform = camera.to_uniform();

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("camera_buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("camera_bind_group_layout"),
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
            label: Some("camera_bind_group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        // Texture bind group layout
        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("texture_bind_group_layout"),
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

        // Shader and Pipeline
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("render_pipeline_layout"),
            bind_group_layouts: &[&camera_bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let make_pipeline = |label: &str, fs: &str, blend: wgpu::BlendState, depth_write: bool, depth_compare: wgpu::CompareFunction| {
            let vs = if label.starts_with("sky") { "vs_sky" } else { "vs_main" };
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(label),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some(vs),
                    buffers: &[GpuVertex::desc()],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some(fs),
                    targets: &[Some(wgpu::ColorTargetState { format: config.format, blend: Some(blend), write_mask: wgpu::ColorWrites::ALL })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState { cull_mode: None, ..Default::default() },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: depth_write,
                    depth_compare,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            })
        };
        let additive = wgpu::BlendState {
            color: wgpu::BlendComponent { src_factor: wgpu::BlendFactor::SrcAlpha, dst_factor: wgpu::BlendFactor::One, operation: wgpu::BlendOperation::Add },
            alpha: wgpu::BlendComponent { src_factor: wgpu::BlendFactor::Zero, dst_factor: wgpu::BlendFactor::One, operation: wgpu::BlendOperation::Add },
        };
        let ge = wgpu::CompareFunction::GreaterEqual;
        let always = wgpu::CompareFunction::Always;
        let additive_pipeline = make_pipeline("additive_pipeline", "fs_additive", additive, false, ge);
        let decal_pipeline = make_pipeline("decal_pipeline", "fs_decal", wgpu::BlendState::ALPHA_BLENDING, false, ge);
        let sky_pipeline = make_pipeline("sky_pipeline", "fs_decal", wgpu::BlendState::ALPHA_BLENDING, false, always);
        let sky_additive_pipeline = make_pipeline("sky_additive_pipeline", "fs_additive", additive, false, always);

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render_pipeline"),
            layout: Some(&render_pipeline_layout),
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
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, // Two-sided rendering
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Greater,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // Load initial scene if provided
        let mut scene = None;
        let mut loaded_ccs = None;
        let mut show_background = true;
        let mut current_resolved = None;

        if let Some(ref path) = ccs_path {
            if let Some(resolved) = resolve_ccs_path(path) {
                if let Ok(data) = std::fs::read(&resolved) {
                    if let Ok(ccs) = hackgu_ccs::CcsFile::parse(&data) {
                        println!("Loading CCS scene: {} ({} objects)", resolved.display(), ccs.raw_objects.len());
                        let loaded_scene = GpuScene::from_ccs(&device, &queue, &texture_bind_group_layout, &ccs);
                        println!(
                            "Loaded {} submeshes, {} materials, bounds: [{:.2}, {:.2}, {:.2}] to [{:.2}, {:.2}, {:.2}]",
                            loaded_scene.submeshes.len(),
                            loaded_scene.materials.len(),
                            loaded_scene.min_bounds.x, loaded_scene.min_bounds.y, loaded_scene.min_bounds.z,
                            loaded_scene.max_bounds.x, loaded_scene.max_bounds.y, loaded_scene.max_bounds.z,
                        );
                        if loaded_scene.has_background {
                            show_background = false;
                            camera.focus_bounds(loaded_scene.fg_min_bounds, loaded_scene.fg_max_bounds);
                        } else {
                            camera.focus_bounds(loaded_scene.min_bounds, loaded_scene.max_bounds);
                        }
                        scene = Some(loaded_scene);
                        loaded_ccs = Some(ccs);
                        current_resolved = Some(resolved);
                    }
                }
            }
        }

        let egui_mgr = crate::gui::EguiManager::new(&device, config.format.remove_srgb_suffix(), &window);
        let movie_renderer = crate::moviefx::MovieRenderer::new(&device, config.format);
        let postfx = crate::postfx::PostFx::new(&device, config.format, config.width, config.height);

        Self {
            window,
            surface,
            device,
            queue,
            config,
            depth_texture_view,
            render_pipeline,
            additive_pipeline,
            decal_pipeline,
            sky_pipeline,
            sky_additive_pipeline,
            camera,
            camera_buffer,
            camera_bind_group,
            texture_bind_group_layout,
            scene,
            ccs: loaded_ccs,
            last_instant: std::time::Instant::now(),
            show_background,
            mouse_pressed: false,
            right_mouse_pressed: false,
            last_mouse_pos: None,
            egui_mgr,
            current_ccs_path: current_resolved,
            current_stage_path: None,
            player: crate::player::PlayerController::new(),
            key_w: false,
            key_s: false,
            key_a: false,
            key_d: false,
            equipped_weapon: None,
            gilrs: gilrs::Gilrs::new().ok(),
            spawned_cutscene_id: None,
            movie_player: crate::moviefx::MoviePlayer::new(),
            movie_renderer,
            postfx,
            field_props: Vec::new(),
            fieldplay: None,
            temple_interior: false,
            temple_cover: Vec::new(),
            altar_chests: Vec::new(),
            pending_movie: None,
            reward_panel: None,
            field_open_collision: None,
            next_event_frame: None,
            last_haseo_mark: None,
            last_cast_marks: std::collections::HashMap::new(),
            cutscene: None,
            fade_black: 0.0,
            lang_db: None,
            sound_system: crate::sound::SoundSystem::new(),
            last_spoken_voice: None,
            frame_no: 0,
            pad_keys: Vec::new(),
            stick_nav: None,
            menu_prompt_pressed: false,
            open_menu_after_event: false,
            gate_prop: None,
            dungeon_platform: None,
            dungeon_arrival: None,
            dungeon_platform_armed: true,
            dungeon_piece_locators: std::collections::HashMap::new(),
            dungeon_piece_yaw: std::collections::HashMap::new(),
            dungeon_piece_frame: std::collections::HashMap::new(),
            dungeon_chest: None,
            town_locators: std::collections::HashMap::new(),
            area_music_wanted: true,
        }
    }

    /// Third-person follow camera behind the player. No camera parameters for field/town play were found in the
    /// data, so these are tuned by eye: a ~3.3 m boom aimed at head height, slightly above, 45 degree FOV.
    pub fn reset_follow_camera(&mut self) {
        self.camera.eye_override = None;
        self.camera.roll = 0.0;
        self.camera.clip_distance = None;
        self.camera.distance = FOLLOW_DISTANCE;
        self.camera.pitch = 0.20;
        self.camera.fov_y = 45f32.to_radians();
        self.camera.z_near = 0.05;
        // Behind the character: player yaw 0 faces -Y; camera yaw places the eye at (sin(yaw), -cos(yaw)) * d.
        self.camera.yaw = self.player.yaw + std::f32::consts::PI;
        self.camera.target = self.player.position + glam::Vec3::new(0.0, 0.0, FOLLOW_HEIGHT);
    }

    /// Load an event script, spawn its cast and hand the camera to it.
    pub fn start_cutscene(&mut self, id: &str) {
        use crate::cutscene::{cast_model, load_ccs, Cutscene, EvTransform};
        // Events of the current area play in its field (statue event: in the temple's statue room).
        let area = crate::field::area();
        let field_event = crate::field::is_area_event(id);
        let set = area.kit.set.clone();
        let stage_has = |s: &str| self.current_stage_path.as_ref().map(|p| p.to_string_lossy().contains(s.trim_end_matches(".ccs"))).unwrap_or(false);
        if area.events.statue.as_deref() == Some(id) {
            if !self.temple_interior || !stage_has(&set) {
                self.temple_interior = true;
                self.load_world(Path::new(&set), Path::new("2hsw01.ccs"));
            }
        } else if field_event && (!stage_has(&set) || self.temple_interior) {
            self.temple_interior = false;
            self.load_world(Path::new(&set), Path::new("2hsw01.ccs"));
        }
        // Dungeon events (place type 4) play in the current story dungeon.
        let dungeon = crate::dungeon::current().filter(|_| crate::events::find(id).is_some_and(|e| e.place_type == Some(crate::events::PLACE_DUNGEON)));
        if let Some(d) = &dungeon {
            let loaded = self.current_stage_path.as_ref().is_some_and(|p| p.to_string_lossy().contains(d.def.kit.trim_end_matches(".ccs")));
            if !loaded {
                self.temple_interior = false;
                let kit = d.def.kit.clone();
                self.load_world(Path::new(&kit), Path::new("2hsw01.ccs"));
            }
        }
        let Some(mut cs) = Cutscene::load(id) else { return };
        // Events outside the current area that name no stage of their own play in Mac Anu (PM10100, PM10310...).
        if !field_event && dungeon.is_none() && area.events.statue.as_deref() != Some(id) && cs.player.script.stages().is_empty()
            && !self.current_stage_path.as_ref().is_some_and(|p| p.to_string_lossy().contains("town01"))
        {
            self.temple_interior = false;
            self.load_world(Path::new("town01_00h.ccs"), Path::new("2hsw01.ccs"));
        }
        let auto = crate::auto::get();
        if self.lang_db.is_none() && auto.lang != "jpn" {
            self.lang_db = hackgu_game::msginfo::MessageInfo::load(Path::new("target"), &auto.lang).map(std::sync::Arc::new);
            if self.lang_db.is_none() {
                eprintln!("messageInfo.bin ({}) not found; event text stays Japanese", auto.lang);
            }
        }
        cs.player.lang = self.lang_db.clone();
        cs.overrides = auto
            .evframe
            .iter()
            .map(|(c, v)| (c.clone(), EvTransform { origin: glam::Vec3::new(v[0], v[1], v[2]), yaw: v[3].to_radians() }))
            .collect();
        // Town events play at DMY_ev<N> (event table -> location row -> locator, see events.rs), turned by the script
        // header's yaw (PM10100: pi, matching the real footage; EM10320/EM10322/PM10315/PM10320: 0, checked by eye).
        // The header's origin offset is not applied (PM10100's [0|-260|0] would move it 2.6 m off the footage).
        if cs.overrides.is_empty() {
            if let Some(e) = crate::events::find(id).filter(|e| e.place_type == Some(crate::events::PLACE_TOWN)) {
                if let Some(p) = e.locator.and_then(|n| self.town_locators.get(&format!("DMY_ev{}", n))) {
                    let yaw = cs.player.script.header_origin.map(|(_, y)| y).unwrap_or(0.0);
                    cs.overrides.push(("*".into(), EvTransform { origin: glam::Vec3::from(*p) * 0.01, yaw }));
                }
            }
        }
        // Dungeon events play at the dungeon's locator (dungeon.rs), turned by the script header's yaw as in towns.
        if cs.overrides.is_empty() {
            if let (Some(d), Some(n)) = (&dungeon, crate::events::find(id).and_then(|e| e.locator)) {
                if let Some(p) = self.dungeon_piece_locators.get(&n).copied().or_else(|| d.def.locator(n)) {
                    // A locator inside a kit piece turns the script frame with the piece (last room: local +y = world +x).
                    let yaw = cs.player.script.header_origin.map(|(_, y)| y).unwrap_or(0.0) + self.dungeon_piece_yaw.get(&n).copied().unwrap_or(0.0);
                    let origin = self.dungeon_piece_frame.get(&n).copied().unwrap_or(p);
                    cs.overrides.push(("*".into(), EvTransform { origin, yaw }));
                }
            }
        }
        // Field events: frame chosen by the story controller (anchor + script yaw), unless --evframe overrides.
        if cs.overrides.is_empty() {
            if let Some(t) = self.next_event_frame.take().or_else(|| crate::field::event_frame(id, self.player.position)) {
                cs.overrides.push(("*".into(), t));
            }
        }
        self.last_haseo_mark = None;
        self.last_cast_marks.clear();
        if let (Some(fp), Some(scene)) = (&mut self.fieldplay, &mut self.scene) {
            fp.hide_followers(scene);
            fp.event_started(scene, id);
        }
        println!("event {}: {} ops, header origin {:?}", id, cs.player.script.ops.len(), cs.player.script.header_origin);
        if let Some(scene) = &mut self.scene {
            for (actor, display, kind) in cs.cast_list() {
                let Some((model, base)) = cast_model(&actor, &display, kind) else {
                    eprintln!("event {}: no model known for cast {} ({})", id, actor, display);
                    continue;
                };
                let Some(ccs) = load_ccs(model) else {
                    eprintln!("event {}: model {} not found", id, model);
                    continue;
                };
                scene.spawn_actor_model(&self.device, &self.queue, &self.texture_bind_group_layout, &ccs, glam::Vec3::ZERO, 0.0);
                let scene_idx = scene.spawned_actors.len() - 1;
                scene.set_actor_visible(scene_idx, false);
                let mut anims = ccs.animations();
                if let Some(ev) = load_ccs(&format!("{}ev.ccs", base)) {
                    anims.extend(ev.animations());
                }
                for f in cs.motion_files_for(&actor, &display) {
                    match load_ccs(&format!("{}.ccs", f)) {
                        Some(m) => anims.extend(m.animations()),
                        None => eprintln!("event {}: motion file {} not found", id, f),
                    }
                }
                println!("  cast {} ({}) -> {} with {} animations", display, actor, model, anims.len());
                cs.cast.insert(
                    display.clone(),
                    crate::cutscene::CastActor {
                        scene_idx,
                        clump: ccs.clumps().into_iter().next(),
                        nodes: ccs.scene_nodes(),
                        ccs,
                        anims,
                        cur: None,
                        t: 0.0,
                        serial: u32::MAX,
                        shown: false,
                        fade_from: None,
                        last_pose: None,
                    },
                );
            }
            scene.set_main_character_visible(false);
        }
        self.player.active = false;
        self.egui_mgr.gui_state.player_mode = false;
        self.sound_system.stop_bgm();
        self.cutscene = Some(cs);
        if !auto.ev_seek.is_empty() {
            let spec = auto.ev_seek.clone();
            self.seek_cutscene(&spec);
        }
    }

    /// Fast-forward the running event: `<seconds>` or `<cutLabel>[+frames]` (60 Hz frames).
    fn seek_cutscene(&mut self, spec: &str) {
        let Some(cs) = &mut self.cutscene else { return };
        let (label, extra) = match spec.split_once('+') {
            Some((l, f)) => (l.to_string(), f.parse::<u32>().unwrap_or(0)),
            None => (spec.to_string(), 0),
        };
        if let Ok(secs) = label.parse::<f32>() {
            for _ in 0..(secs * 60.0) as u32 {
                cs.player.update(1.0 / 60.0);
            }
        } else {
            let mut guard = 0;
            while cs.player.state.cut_label != label && !cs.player.is_finished() && guard < 60 * 60 * 30 {
                cs.player.update(1.0 / 60.0);
                guard += 1;
            }
            for _ in 0..extra {
                cs.player.update(1.0 / 60.0);
            }
        }
        let _ = cs.take_new_bgm_ops();
        println!("[auto] event seek '{}' -> cut {} frame {}", spec, cs.player.state.cut_label, cs.player.total_frames);
    }

    /// Stop the event: hide the cast, give control back, and route the story (skipped = player pressed skip).
    pub fn end_cutscene(&mut self, skipped: bool) {
        let Some(cs) = self.cutscene.take() else { return };
        {
            let done = &mut self.egui_mgr.gui_state.game_runtime.events_done;
            if !done.iter().any(|d| d == cs.id()) {
                done.push(cs.id().to_string());
            }
        }
        let town_event = crate::events::find(cs.id()).is_some_and(|e| e.place_type == Some(crate::events::PLACE_TOWN));
        let dungeon_event = crate::dungeon::current().is_some() && crate::events::find(cs.id()).is_some_and(|e| e.place_type == Some(crate::events::PLACE_DUNGEON));
        if let Some(scene) = &mut self.scene {
            for a in cs.cast.values() {
                scene.set_actor_visible(a.scene_idx, false);
            }
            scene.set_main_character_visible(true);
        }
        self.sound_system.stop_voice();
        self.fade_black = 0.0;
        if crate::field::is_area_event(cs.id()) || town_event || dungeon_event {
            if let Some((p, yaw)) = self.last_haseo_mark.take() {
                self.player.position = p;
                self.player.yaw = yaw;
            }
            if let (Some(fp), Some(scene)) = (&mut self.fieldplay, &mut self.scene) {
                fp.place_followers(scene, self.player.position, self.player.yaw);
                let goblins: Vec<(glam::Vec3, f32)> = ["ゴブナイトA", "ゴブナイトB", "ゴブナイトC"].iter().filter_map(|n| self.last_cast_marks.get(*n).copied()).collect();
                fp.event_ended(scene, cs.id(), &goblins);
            }
        }
        self.player.active = true;
        self.egui_mgr.gui_state.player_mode = true;
        self.reset_follow_camera();
        let rt = &mut self.egui_mgr.gui_state.game_runtime;
        if let hackgu_game::GameState::Cutscene { ref mut dialogue, .. } = rt.state {
            *dialogue = None;
        }
        if matches!(rt.state, hackgu_game::GameState::Cutscene { .. }) {
            let area = crate::field::area();
            if town_event && cs.id() != "PM10100" {
                // Town events: a proximity event that leads into a story event plays it next (PM10315 -> PM10320);
                // otherwise back to free movement in town.
                match crate::events::follow_up(cs.id(), &rt.events_done) {
                    Some(next) => rt.start_script(&next.event),
                    None => rt.state = hackgu_game::GameState::Town { district: "Mac Anu".into(), active_dialogue: None, active_shop: None },
                }
            } else if dungeon_event {
                // A story event that directly follows (PM10355 -> PM10360) plays next; otherwise free movement.
                match crate::events::dungeon_follow_up(cs.id(), &rt.events_done) {
                    Some(next) => rt.start_script(&next.event),
                    None => rt.resume_field(),
                }
            } else if crate::field::is_area_event(cs.id()) {
                // Area events chain from the area definition: arrival -> after_arrival -> free play.
                match (&area.events.arrival, &area.events.after_arrival) {
                    (Some(a), Some(next)) if a == cs.id() => rt.start_script(next),
                    _ => rt.resume_field(),
                }
            } else if skipped {
                rt.skip_cutscene();
            } else {
                rt.conclude_active_cutscene();
            }
        }
    }

    /// Advance the running event and apply its frame to the scene.
    fn update_cutscene(&mut self, dt: f32) {
        use crate::cutscene::pose_anim;
        let mute = crate::auto::get().mute;
        let Some(cs) = &mut self.cutscene else { return };
        if self.egui_mgr.gui_state.pending_advance_cutscene {
            self.egui_mgr.gui_state.pending_advance_cutscene = false;
            self.sound_system.stop_voice();
            cs.player.press_advance();
        }
        // GUESS (the original's wait is not traced): a line asking for the menu button (PM10100's last line,
        // (game text)) stays up until the player presses it, then the Main Menu opens after the
        // event. Without this the event just faded out.
        let asks_menu = cs.player.state.message.as_ref().is_some_and(|m| m.text.contains("<iconTRIANGLE>") && m.text.to_lowercase().contains("open"));
        if asks_menu && self.menu_prompt_pressed {
            self.sound_system.stop_voice();
            cs.player.press_advance();
            self.open_menu_after_event = true;
        }
        self.menu_prompt_pressed = false;
        let asks_menu = cs.player.state.message.as_ref().is_some_and(|m| m.text.contains("<iconTRIANGLE>") && m.text.to_lowercase().contains("open"));
        cs.player.hold_message = asks_menu || (!mute && self.sound_system.is_voice_playing());
        cs.player.update(dt);
        for (op, args) in cs.take_new_bgm_ops() {
            match op {
                // BGM操作 123 <slot> <cue>: play bgm.acb cue <cue>. 134 <slot>: stop. 135 <slot>: fade out
                // (GUESS: 2 s; the op has no duration). 138: back to the area's own music (it closes PM10100, PM10105,
                // PM10320). 124/139/141: not known yet (ignored).
                123 => {
                    if let Some(t) = args.get(1).and_then(|t| t.trim().parse().ok()) {
                        if !mute {
                            self.sound_system.play_bgm(t);
                        }
                    }
                }
                134 => self.sound_system.stop_bgm(),
                135 => self.sound_system.fade_out_bgm(2.0),
                138 => {
                    self.sound_system.stop_bgm();
                    self.area_music_wanted = true;
                }
                _ => {}
            }
        }
        if cs.player.is_finished() {
            println!("event {} finished", cs.id());
            self.end_cutscene(false);
            return;
        }
        let xf = cs.transform();
        let frame = cs.player.state.clone();
        self.fade_black = frame.black;

        if let Some(cam) = frame.camera {
            // WORKAROUND: an event camera that lands inside our floor is lifted just above it. PM10100's cutA08 eye
            // is 2 cm above the event origin, where our gate-hall floor is higher; the original's floor or frame there
            // differ in a way not traced yet (its locator DMY_ev1 does not exist; app uses a fitted frame).
            let mut eye = xf.point(cam.eye);
            if let Some(h) = self.player.collision_mesh.as_ref().and_then(|c| c.get_ground_height(eye.x, eye.y, eye.z + 1.0)) {
                if h > eye.z - 0.1 && h < eye.z + 1.0 {
                    eye.z = h + 0.2;
                }
            }
            self.camera.look_from(eye, xf.point(cam.target));
            self.camera.fov_y = cam.fov_deg.to_radians();
            self.camera.roll = cam.roll.to_radians();
            self.camera.clip_distance = None;
        }

        // Event actors stand on the floor: script heights are only near it (PM10100 puts Asta at -60 cm and Iyoten at
        // -55 cm where Haseo is at 0, which sank them to the knees). Snap to a floor within 1 m of the scripted height.
        let col = self.player.collision_mesh.as_ref();
        let ground = |p: glam::Vec3| -> glam::Vec3 {
            match col.and_then(|c| c.get_ground_height(p.x, p.y, p.z + 1.0)) {
                Some(h) if (h - p.z).abs() <= 1.0 => glam::Vec3::new(p.x, p.y, h),
                _ => p,
            }
        };
        if let Some(scene) = &mut self.scene {
            for (name, st) in &frame.actors {
                if name == "ハセヲ" && st.visible {
                    self.last_haseo_mark = Some((ground(xf.point(st.smooth_pos)), st.yaw_deg.to_radians() + xf.yaw));
                }
                if st.visible {
                    self.last_cast_marks.insert(name.clone(), (ground(xf.point(st.smooth_pos)), st.yaw_deg.to_radians() + xf.yaw));
                }
                if std::env::var("HACKGU_DEBUG_CAST").is_ok() && st.visible && self.frame_no % 30 == 0 {
                    eprintln!("cast {} cut {} world {:?}", name, frame.cut_label, ground(xf.point(st.smooth_pos)));
                }
                let Some(actor) = cs.cast.get_mut(name) else { continue };
                if st.visible != actor.shown {
                    scene.set_actor_visible(actor.scene_idx, st.visible);
                    actor.shown = st.visible;
                }
                if !st.visible {
                    continue;
                }
                let desired = if let Some(m) = &st.motion {
                    actor.find_anim(|n| n == m).map(|i| (i, false))
                } else if st.moving {
                    actor.find_anim(|n| n.ends_with("wlk00")).map(|i| (i, true))
                } else {
                    pose_anim(&st.pose).and_then(|(suffix, looping)| actor.find_anim(|n| n.ends_with(suffix)).map(|i| (i, looping)))
                }
                .or_else(|| actor.find_anim(|n| n.ends_with("nut00")).map(|i| (i, true)));
                // Re-issuing the looping animation that is already playing keeps it running. Scripts repeat a pose
                // around cuts (PM10100 sets Asta's 怒り１ again every few frames), and restarting it each time replayed
                // the loop's start over and over, which looked sped up. GUESS (not traced): the original does the same.
                let same_loop = desired.is_some() && desired == actor.cur && desired.is_some_and(|d| d.1);
                if same_loop && st.anim_serial != actor.serial {
                    actor.serial = st.anim_serial;
                }
                if desired.map(|d| d.0) != actor.cur.map(|c| c.0) || st.anim_serial != actor.serial {
                    // Crossfade from whatever was on screen: the script's blend column (event ticks) for pose and
                    // motion ops, a short fixed fade when walking starts or stops.
                    let secs = if st.anim_serial != actor.serial { st.blend_frames / hackgu_game::ev::EV_FPS } else { crate::cutscene::WALK_BLEND_SECONDS };
                    actor.fade_from = match (&actor.last_pose, secs > 0.0) {
                        (Some(p), true) => Some((p.clone(), 0.0, secs)),
                        _ => None,
                    };
                    actor.cur = desired;
                    actor.t = 0.0;
                    actor.serial = st.anim_serial;
                }
                actor.t += dt * 30.0;
                let Some((ai, looping)) = actor.cur else {
                    // No animation available (generic PC base bodies): place the bind pose.
                    scene.update_actor_transform(&self.queue, actor.scene_idx, ground(xf.point(st.smooth_pos)), st.yaw_deg.to_radians() + xf.yaw);
                    continue;
                };
                let anim = &actor.anims[ai];
                let last = anim.frame_count.saturating_sub(1).max(1) as f32;
                let f = if looping { actor.t % last } else { actor.t.min(last) };
                let Some(clump) = &actor.clump else { continue };
                let mut pose = anim.local_pose(clump, &actor.ccs, f);
                if let Some((from, elapsed, total)) = &mut actor.fade_from {
                    *elapsed += dt;
                    let w = (*elapsed / *total).clamp(0.0, 1.0);
                    let w = w * w * (3.0 - 2.0 * w);
                    pose = hackgu_ccs::blend_local_poses(from, &pose, w);
                    if w >= 1.0 {
                        actor.fade_from = None;
                    }
                }
                let world = hackgu_ccs::world_from_local_pose(clump, &actor.nodes, &pose);
                actor.last_pose = Some(pose);
                scene.update_actor_with_matrices(&self.queue, actor.scene_idx, ground(xf.point(st.smooth_pos)), st.yaw_deg.to_radians() + xf.yaw, &world);
            }
        }

        // Message box + voice. PM10100's voice cue is the message number - 1 (M101_{N-1:03d}).
        let msg = frame.message.clone();
        if let hackgu_game::GameState::Cutscene { ref mut dialogue, .. } = self.egui_mgr.gui_state.game_runtime.state {
            *dialogue = msg.as_ref().map(|m| hackgu_game::runtime::DialogueBox {
                speaker: m.speaker.clone(),
                portrait: None,
                text: m.text.clone(),
                // "notice" marks an op-6 line (drawn as the original's mid-screen band, gui.rs).
                options: if m.op == 6 { vec!["notice".into()] } else { vec![] },
            });
        }
        let key = msg.as_ref().map(|m| format!("{:?}:{}", m.id, m.text));
        if key != cs.last_message_key {
            if std::env::var("HACKGU_DEBUG_MSG").is_ok() {
                eprintln!("msg t={:.2}s {} {:?}", { static T0: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new(); T0.get_or_init(std::time::Instant::now).elapsed().as_secs_f32() }, cs.id(), msg.as_ref().map(|m| m.text.chars().take(40).collect::<String>()));
            }
            cs.last_message_key = key;
            if let (Some(m), false) = (&msg, mute) {
                if let Some(n) = m.id {
                    self.sound_system.play_cutscene_voice(cs.id(), n);
                }
            }
        }
    }

    /// A story dungeon floor from its DLL-built layout (dungeon.rs): every cell model through its kit clump, placed at
    /// the cell centre with the DLL's rotation; collision from the clump's own HitMeshes (HIT_<name>-hit*). Returns the
    /// collision and the spawn (start room centre on floor 0).
    fn build_dungeon(&mut self, scene: &mut GpuScene, def: &crate::dungeon::DungeonDef, floor: usize) -> (hackgu_world::CollisionMesh, Option<glam::Vec3>) {
        use crate::cutscene::load_ccs;
        let mut col = hackgu_world::CollisionMesh::empty();
        let Some(kit) = load_ccs(&def.kit) else {
            eprintln!("dungeon: {} not found", def.kit);
            return (col, None);
        };
        let off = scene.register_stage_materials(&self.device, &self.queue, &self.texture_bind_group_layout, &kit);
        let pieces = def.pieces(floor);
        println!("dungeon: floor {} with {} models", floor, pieces.len());
        for (clump, m) in &pieces {
            scene.instance_stage_models(&self.device, &kit, off, |c| c == clump.as_str(), m.clone());
            // Some pieces' HitMeshes have no floor (the floor-up hall HIT_floorup-b01-hit: 6 floor triangles at its far
            // end), so the visible floor meshes (textures TEX_fl*) are walkable too.
            col.merge(&visible_floor(&kit, &format!("MDL_{}", clump.trim_start_matches("CMP_")), m));
            let hit_prefix = format!("HIT_{}-hit", clump.trim_start_matches("CMP_"));
            // The yaw only orients the stored normal hints; mirroring does not change which faces are floors.
            let yaw = m.0[4].atan2(m.0[0]);
            col.merge(&hackgu_world::CollisionMesh::from_hit_meshes_placed(
                &kit,
                |n| n.starts_with(&hit_prefix),
                |v| glam::Vec3::from(m.transform_point(v.to_array())),
                yaw,
            ));
        }
        // Floor change: the platform at the bottom of the floor-up hall (DMY_floorup-b01-plat) leads to the other floor's
        // platform. GUESS (not traced): standing on it switches floors; the original's platform model and prompt are
        // not known, so the field Platform (f_common CMP_op00) marks it.
        let plat_cm = kit.locators().into_iter().find(|(n, _)| n == "DMY_floorup-b01-plat").map(|(_, p)| p);
        self.dungeon_platform = plat_cm.and_then(|p| def.floor_platform(floor, p)).map(|p| (p, if floor == 0 { 1 } else { 0 }));
        if let (Some((p, _)), Some(common)) = (self.dungeon_platform, load_ccs("f_common.ccs")) {
            if let Some(idx) = scene.spawn_prop(&self.device, &self.queue, &self.texture_bind_group_layout, &common, "CMP_op00") {
                let anim = common.animations().into_iter().find(|a| a.name == "ANM_op00mov");
                self.field_props.push(FieldProp { once: false, idx, pos: p, yaw: 0.0, ccs: std::sync::Arc::new(common), anim, t: 0.0 });
            }
        }
        self.dungeon_platform_armed = false;
        self.dungeon_piece_locators.clear();
        self.dungeon_piece_frame.clear();
        self.dungeon_chest = None;
        let locs: std::collections::HashMap<String, [f32; 3]> = kit.locators().into_iter().collect();
        if let Some(cell) = def.floors.get(floor).and_then(|f| f.iter().find(|c| c.models.iter().any(|m| m == "MDL_lastroom-a"))) {
            for (n, name) in [(50, "DMY_lastroom-ev1"), (51, "DMY_lastroom-ev2")] {
                if let Some(p) = locs.get(name) {
                    let m = cell.mat();
                    self.dungeon_piece_locators.insert(n, glam::Vec3::from(m.transform_point([p[0] * 0.01, p[1] * 0.01, p[2] * 0.01])));
                    self.dungeon_piece_yaw.insert(n, m.0[4].atan2(m.0[0]));
                    // FITTED (not traced): the last-room scripts (PM10345..PM10360) are laid out on the room's centre
                    // line x = -8 m of the piece, y from the piece origin: characters then walk to just before the
                    // offering chest (DMY_lastroom-a-item, y 47 m) and stand on the dais heights the script gives.
                    self.dungeon_piece_frame.insert(n, glam::Vec3::from(m.transform_point([p[0] * 0.01, 0.0, 0.0])));
                }
            }
        }
        // The last room's Beast Statue and offering chest, at the kit's DMY_lastroom-a-statue / -item locators (the DLL
        // spawns an object at the statue locator, 0x1802E5334). GUESS: the models (dungeon/statue.ccs CMP_statue and
        // dungeon/itembox21.ccs) are not traced; they are the dungeon folder's own statue and first item box.
        if let Some(cell) = def.floors.get(floor).and_then(|f| f.iter().find(|c| c.models.iter().any(|m| m == "MDL_lastroom-a"))) {
            let m = cell.mat();
            let yaw = m.0[4].atan2(m.0[0]);
            for (loc, file, clump) in [("DMY_lastroom-a-statue", "target/vol1/data/data/dungeon/statue.ccs", "CMP_statue"), ("DMY_lastroom-a-item", "target/vol1/data/data/dungeon/itembox21.ccs", "")] {
                let (Some(p), Some(c)) = (locs.get(loc), load_ccs(file)) else { continue };
                let clump = if clump.is_empty() { c.clumps().first().and_then(|k| c.object_name(k.object_id)).unwrap_or("").to_string() } else { clump.to_string() };
                let pos = glam::Vec3::from(m.transform_point([p[0] * 0.01, p[1] * 0.01, p[2] * 0.01]));
                if let Some(idx) = scene.spawn_prop(&self.device, &self.queue, &self.texture_bind_group_layout, &c, &clump) {
                    if loc.ends_with("item") {
                        self.dungeon_chest = Some((pos, self.field_props.len()));
                    }
                    self.field_props.push(FieldProp { once: true, idx, pos, yaw, ccs: std::sync::Arc::new(c), anim: None, t: 0.0 });
                }
            }
        }
        if let Some(a) = self.dungeon_arrival.take() {
            return (col, Some(a));
        }
        let spawn = if floor == 0 { def.start() } else { def.floors.get(floor).and_then(|f| f.first()).map(|c| c.pos()) };
        (col, spawn)
    }

    /// Place every field piece (visuals + collision) and the altar chests. Returns the collision.
    /// The Beast Temple statue room: field/sf3_item.ccs (CMP_mi01 room, inner doors, lights; HIT_mi01hit), the statue
    /// (field/statue.ccs CMP_statue) at its DMY_statue and the offering chest at DMY_item. hackGU_vol1.dll names
    /// field/sf3_item, DMY_statue and field/statue together with the temple door code.
    fn build_temple_interior(&mut self, scene: &mut GpuScene) -> hackgu_world::CollisionMesh {
        use crate::cutscene::load_ccs;
        let mut col = hackgu_world::CollisionMesh::empty();
        let area = crate::field::area();
        let Some(t) = area.temple.clone() else { return col };
        let Some(room) = load_ccs(&t.statue_room) else { return col };
        let off = scene.register_stage_materials(&self.device, &self.queue, &self.texture_bind_group_layout, &room);
        scene.instance_stage_models(&self.device, &room, off, |c| c.starts_with("CMP_mi01") && !c.starts_with("CMP_mi01_l0"), hackgu_ccs::Mat4::IDENTITY);
        // Light shafts under the dome: CMP_mi01_l00/l01 (additive) placed and moved by ANM_mi01shine.
        let room_arc = std::sync::Arc::new(room.clone());
        let shine = room.animations().into_iter().find(|a| a.name == "ANM_mi01shine");
        for c in ["CMP_mi01_l00", "CMP_mi01_l01"] {
            if let Some(idx) = scene.spawn_prop(&self.device, &self.queue, &self.texture_bind_group_layout, &room, c) {
                self.field_props.push(FieldProp { once: false, idx, pos: glam::Vec3::ZERO, yaw: 0.0, ccs: room_arc.clone(), anim: shine.clone(), t: 0.0 });
            }
        }
        col.merge(&hackgu_world::CollisionMesh::from_hit_meshes_placed(&room, |n| n.starts_with("HIT_mi01"), |v| v, 0.0));
        let loc = |name: &str| -> Option<glam::Vec3> {
            room.objects_of_type(hackgu_ccs::objects::SECTION_DUMMYPOS).into_iter().chain(room.objects_of_type(hackgu_ccs::objects::SECTION_DUMMYPOSROT)).find(|o| room.object_name(o.object_id) == Some(name)).map(|o| {
                let f = |i: usize| f32::from_le_bytes(o.payload[i * 4..i * 4 + 4].try_into().unwrap());
                glam::Vec3::new(f(0), f(1), f(2)) * 0.01
            })
        };
        if let (Some(p), Some(st)) = (loc("DMY_statue"), load_ccs(&t.statue)) {
            if let Some(idx) = scene.spawn_prop(&self.device, &self.queue, &self.texture_bind_group_layout, &st, "CMP_statue") {
                self.field_props.push(FieldProp { once: false, idx, pos: p, yaw: 0.0, ccs: std::sync::Arc::new(st), anim: None, t: 0.0 });
            }
        }
        if let (Some(p), Some(chest)) = (loc("DMY_item"), load_ccs("itembox01.ccs")) {
            if let Some(idx) = scene.spawn_prop(&self.device, &self.queue, &self.texture_bind_group_layout, &chest, "CMP_itembox") {
                self.altar_chests = vec![self.field_props.len()];
                self.field_props.push(FieldProp { once: true, idx, pos: p, yaw: 0.0, ccs: std::sync::Arc::new(chest), anim: None, t: 0.0 });
            }
        }
        col
    }

    fn build_holy_ground(&mut self, scene: &mut GpuScene) -> hackgu_world::CollisionMesh {
        use crate::cutscene::load_ccs;
        let mut col = hackgu_world::CollisionMesh::empty();
        let area = crate::field::area();
        let mut libs: std::collections::HashMap<String, (hackgu_ccs::CcsFile, u32)> = std::collections::HashMap::new();
        let mut files: Vec<&String> = vec![&area.kit.tiles, &area.kit.set, &area.kit.common];
        for d in &area.pieces {
            if !files.contains(&&d.file) {
                files.push(&d.file);
            }
        }
        for file in files {
            match load_ccs(file) {
                Some(c) => {
                    let off = scene.register_stage_materials(&self.device, &self.queue, &self.texture_bind_group_layout, &c);
                    libs.insert(file.clone(), (c, off));
                }
                None => eprintln!("field: {} not found", file),
            }
        }
        // Sky dome, clouds, sun and distant land from the tile kit (sky_0n).
        if let Some((sky, off)) = libs.get(&area.kit.tiles) {
            scene.instance_stage_models(&self.device, sky, *off, |c| ["CMP_bs", "CMP_bc", "CMP_sun", "CMP_bl"].iter().any(|p| c.starts_with(p)), hackgu_ccs::Mat4::IDENTITY);
        }
        let pieces = crate::field::layout();
        println!("field: placing {} pieces", pieces.len());
        // Debug: HACKGU_FIELD_SKIP=CMP_oi00,CMP_ob01 leaves pieces out.
        let skip: Vec<String> = std::env::var("HACKGU_FIELD_SKIP").map(|v| v.split(',').map(|s| s.to_string()).collect()).unwrap_or_default();
        for piece in &pieces {
            if skip.iter().any(|s| *s == piece.clump) {
                continue;
            }
            let Some((lib, off)) = libs.get(&piece.file) else { continue };
            let m = hackgu_ccs::Mat4::from_translation(piece.pos.to_array()).mul(&hackgu_ccs::Mat4::from_rot_z(piece.yaw));
            if piece.clump == "CMP_op00" {
                // Platforms are animated (ANM_op00mov turns the ring); spawned as props, updated every frame.
                if let Some(idx) = scene.spawn_prop(&self.device, &self.queue, &self.texture_bind_group_layout, lib, "CMP_op00") {
                    let anim = lib.animations().into_iter().find(|a| a.name == "ANM_op00mov");
                    self.field_props.push(FieldProp { once: false, idx, pos: piece.pos, yaw: piece.yaw, ccs: std::sync::Arc::new(lib.clone()), anim, t: 0.0 });
                }
            } else {
                let first = scene.submeshes.len();
                scene.instance_stage_models(&self.device, lib, *off, |c| c == piece.clump.as_str(), m);
                if crate::field::over_temple_interior(piece) {
                    self.temple_cover.extend(first..scene.submeshes.len());
                }
            }
            if piece.clump == "CMP_sw01" || piece.clump == "CMP_sb01" {
                // Sea collision in the original blocks movement; our controller would treat it as floor.
                continue;
            }
            if crate::field::over_temple_interior(piece) {
                // The temple's own HitMesh covers its slab, stairs and the ground over its interior.
                continue;
            }
            if piece.clump == "CMP_ob00" {
                // The bridge end's ramp (MDL_ob00 mesh 0, the planks rising from ground level to the deck) has no
                // HitMesh (HIT_ob00hit covers only the gateway end), so its own triangles are the floor there.
                col.merge(&bridge_ramp_floor(lib, piece.pos, piece.yaw));
            }
            let stem = piece.clump.trim_start_matches("CMP_").to_string();
            let hit_prefix = format!("HIT_{}", stem);
            let (sy, cy) = piece.yaw.sin_cos();
            let pos = piece.pos;
            col.merge(&hackgu_world::CollisionMesh::from_hit_meshes_placed(
                lib,
                // The temple's doors have their own HitMeshes (HIT_oi00door00hit / HIT_oi00door02_hit); the entrance
                // stands open in the original and the inner door is a trigger, so neither blocks here.
                |n| n.starts_with(&hit_prefix) && !(piece.clump == "CMP_oi00" && (n.contains("door") || n == "HIT_oi00_ahit")),
                |v| pos + glam::Vec3::new(cy * v.x - sy * v.y, sy * v.x + cy * v.y, v.z),
                piece.yaw,
            ));
        }
        // The shore tiles' sea walls run straight across the bridges; open them where a bridge piece (deck 6.6 m wide,
        // 16 m long, rotated by multiples of 90 deg) crosses them.
        for piece in pieces.iter().filter(|p| p.clump == "CMP_ob01" || p.clump == "CMP_ob00") {
            let along_y = (piece.yaw.cos()).abs() > 0.5;
            let (hx, hy) = if along_y { (3.0, 8.0) } else { (8.0, 3.0) };
            col.cut_walls_in_rect(glam::Vec2::new(piece.pos.x - hx, piece.pos.y - hy), glam::Vec2::new(piece.pos.x + hx, piece.pos.y + hy));
        }
        // The temple seal (HIT_oi00_ahit: a wall across the doorway, z -1..16 m) keeps the temple shut until the
        // Symbol Fragments are joined; the open collision is kept to swap in then.
        if let Some((lib, _)) = libs.get(&area.kit.set) {
            if let Some(tp) = pieces.iter().find(|p| p.clump == "CMP_oi00") {
                let (sy, cy) = tp.yaw.sin_cos();
                let pos = tp.pos;
                let seal = hackgu_world::CollisionMesh::from_hit_meshes_placed(lib, |n| n == "HIT_oi00_ahit", |v| pos + glam::Vec3::new(cy * v.x - sy * v.y, sy * v.x + cy * v.y, v.z), tp.yaw);
                self.field_open_collision = Some(col.clone());
                col.merge(&seal);
            }
        }
        // A treasure chest on every altar, at the plinth's own DMY_oc00item locator (0, 0, 1.02 m).
        if let Some(chest) = load_ccs("itembox01.ccs") {
            let chest = std::sync::Arc::new(chest);
            self.altar_chests.clear();
            for k in 0..area.altars.len() {
                if let Some(idx) = scene.spawn_prop(&self.device, &self.queue, &self.texture_bind_group_layout, &chest, "CMP_itembox") {
                    self.altar_chests.push(self.field_props.len());
                    self.field_props.push(FieldProp { once: true, idx, pos: area.altar_world(k) + glam::Vec3::new(0.0, 0.0, 1.02), yaw: 0.0, ccs: chest.clone(), anim: None, t: 0.0 });
                }
            }
        }
        col
    }

    /// The offering chest's reward: "Dry Gear" in the original (f00860), built like the game's item names from a
    /// prefix row (17325 "Dry", id 0xAEB3FAEF) and a base row (14066 "Gear", id 0xEB867660) of messageInfo.
    pub fn show_mission_reward(&mut self) {
        let db = hackgu_game::msginfo::MessageInfo::load(Path::new("target"), &crate::auto::get().lang);
        let name = db
            .as_ref()
            .and_then(|d| Some(format!("{} {}", d.by_id.get(&0xAEB3FAEF)?, d.by_id.get(&0xEB867660)?)))
            .unwrap_or_else(|| "Dry Gear".into());
        // Header "Mission Reward": messageInfo row 41989 (0x70B51A02).
        let header = db.as_ref().and_then(|d| d.by_id.get(&0x70B51A02).cloned()).unwrap_or_else(|| "Mission Reward".into());
        self.reward_panel = Some((format!("{}|{}", header, name), 3.0));
    }

    /// Play chest k's opening animation (ANM_itembox01_open, once).
    pub fn open_chest(&mut self, k: usize) {
        let Some(&pi) = self.altar_chests.get(k) else { return };
        if let Some(p) = self.field_props.get_mut(pi) {
            if p.anim.is_none() {
                p.anim = p.ccs.animations().into_iter().find(|a| a.name == "ANM_itembox01_open");
                p.t = 0.0;
            }
        }
    }

    /// Within reach of the statue room's offering chest (sf3_item DMY_item (0, 32, 1.43) m).
    /// Haseo is at the last room's offering chest and its event (PM10355) is next.
    pub fn dungeon_chest_event_ready(&self) -> bool {
        let (Some((p, _)), Some(cur)) = (self.dungeon_chest, crate::dungeon::current()) else { return false };
        let d = self.player.position - p;
        glam::Vec2::new(d.x, d.y).length() < 2.6 && crate::events::chest_event(cur.def.words, &self.egui_mgr.gui_state.game_runtime.events_done).is_some()
    }

    /// Open the offering chest (its ANM_itembox21_open) and start the event that follows (PM10355).
    pub fn open_dungeon_chest(&mut self) {
        let Some(cur) = crate::dungeon::current() else { return };
        let Some(ev) = crate::events::chest_event(cur.def.words, &self.egui_mgr.gui_state.game_runtime.events_done) else { return };
        if let Some((_, k)) = self.dungeon_chest {
            if let Some(p) = self.field_props.get_mut(k) {
                p.anim = p.ccs.animations().into_iter().find(|a| a.name.ends_with("_open"));
                p.t = 0.0;
            }
        }
        // The table's script-less event before it (PM10350, the chest) counts as played.
        if let Some(i) = crate::events::table().iter().position(|e| e.event == ev.event) {
            if let Some(prev) = crate::events::table().get(i.wrapping_sub(1)).filter(|e| e.start.is_none()) {
                self.egui_mgr.gui_state.game_runtime.events_done.push(prev.event.clone());
            }
        }
        println!("dungeon chest: {}", ev.event);
        self.egui_mgr.gui_state.game_runtime.start_script(&ev.event);
    }

    pub fn near_offering_chest(&self) -> bool {
        (self.player.position - glam::Vec3::new(0.0, 32.0, 1.2)).truncate().length() < 2.6
    }

    /// Enter an event script directly (automation / debugging).
    /// A full-screen menu or movie is up: the player does not move and the pad drives the menu.
    pub fn menu_open(&self) -> bool {
        use hackgu_game::GameState as G;
        self.egui_mgr.gui_state.main_menu_open || matches!(self.egui_mgr.gui_state.game_runtime.state, G::TitleScreen | G::Desktop | G::ChaosGateSelector { .. } | G::Movie { .. })
    }

    /// Open / close the Main Menu (town and field, outside events).
    pub fn toggle_main_menu(&mut self) {
        let g = &mut self.egui_mgr.gui_state;
        if g.main_menu_open {
            g.main_menu_open = false;
        } else if self.cutscene.is_none() && self.player.active && matches!(g.game_runtime.state, hackgu_game::GameState::Town { .. } | hackgu_game::GameState::Field { .. }) {
            g.main_menu_open = true;
        }
    }

    pub fn start_event(&mut self, id: &str) {
        let rt = &mut self.egui_mgr.gui_state.game_runtime;
        match id {
            "PM10105" => rt.start_prologue_field_cutscene(),
            "PM10100" => rt.start_prologue_cutscene(),
            id => rt.start_script(id),
        }
    }

    pub fn spawn_actor_file(&mut self, file_name: &str, pos: glam::Vec3, yaw: f32) {
        if let Some(scene) = &mut self.scene {
            if let Some(path) = resolve_ccs_path(Path::new(file_name)) {
                if let Ok(data) = std::fs::read(&path) {
                    if let Ok(actor_ccs) = hackgu_ccs::CcsFile::parse(&data) {
                        println!("Spawning authentic actor model from {}", path.display());
                        scene.spawn_actor_model(
                            &self.device,
                            &self.queue,
                            &self.texture_bind_group_layout,
                            &actor_ccs,
                            pos,
                            yaw,
                        );
                    }
                }
            }
        }
    }

    fn create_depth_view(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("depth_texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        depth_texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub fn load_weapon_animations(&mut self, idle_file: &str, run_file: &str) {
        if let (Some(scene), Some(ccs)) = (&mut self.scene, &self.ccs) {
            // Remove previous external weapon stance animations if any
            scene.animations.retain(|a| !a.name.contains("rdy00") && !a.name.contains("str00"));

            if let Some(p) = resolve_ccs_path(Path::new(idle_file)) {
                if let Ok(data) = std::fs::read(&p) {
                    if let Ok(anim_ccs) = hackgu_ccs::CcsFile::parse(&data) {
                        for a in anim_ccs.animations() {
                            scene.animations.push(a);
                        }
                    }
                }
            }
            if let Some(p) = resolve_ccs_path(Path::new(run_file)) {
                if let Ok(data) = std::fs::read(&p) {
                    if let Ok(anim_ccs) = hackgu_ccs::CcsFile::parse(&data) {
                        for a in anim_ccs.animations() {
                            scene.animations.push(a);
                        }
                    }
                }
            }

            // Immediately switch to the weapon idle stance
            if let Some(idle_idx) = scene.animations.iter().rposition(|a| a.name.contains("rdy00")) {
                scene.select_animation(idle_idx, &self.queue, ccs);
            }
        }
    }

    pub fn load_ccs(&mut self, raw_path: &Path) {
        let resolved = match resolve_ccs_path(raw_path) {
            Some(p) => p,
            None => {
                self.egui_mgr.gui_state.status_message = format!("File not found: {}", raw_path.display());
                eprintln!("File not found: {}", raw_path.display());
                return;
            }
        };

        if let Ok(data) = std::fs::read(&resolved) {
            if let Ok(ccs) = hackgu_ccs::CcsFile::parse(&data) {
                let file_name = resolved.file_name().unwrap_or_default().to_string_lossy();
                println!("Loading CCS scene: {} ({} objects)", resolved.display(), ccs.raw_objects.len());
                let loaded_scene = GpuScene::from_ccs(&self.device, &self.queue, &self.texture_bind_group_layout, &ccs);
                println!(
                    "Loaded {} submeshes, {} materials, bounds: [{:.2}, {:.2}, {:.2}] to [{:.2}, {:.2}, {:.2}]",
                    loaded_scene.submeshes.len(),
                    loaded_scene.materials.len(),
                    loaded_scene.min_bounds.x, loaded_scene.min_bounds.y, loaded_scene.min_bounds.z,
                    loaded_scene.max_bounds.x, loaded_scene.max_bounds.y, loaded_scene.max_bounds.z,
                );
                if loaded_scene.has_background {
                    self.show_background = false;
                    self.camera.focus_bounds(loaded_scene.fg_min_bounds, loaded_scene.fg_max_bounds);
                } else {
                    self.show_background = true;
                    self.camera.focus_bounds(loaded_scene.min_bounds, loaded_scene.max_bounds);
                }
                self.scene = Some(loaded_scene);
                self.ccs = Some(ccs);
                self.current_ccs_path = Some(resolved.clone());
                self.equipped_weapon = None;
                self.egui_mgr.gui_state.status_message = format!("Loaded {}", file_name);
                self.window.set_title(&format!(".hack//G.U. Studio - {}", file_name));
            } else {
                self.egui_mgr.gui_state.status_message = format!("Failed to parse CCS container: {}", resolved.display());
                eprintln!("Failed to parse CCS container: {}", resolved.display());
            }
        } else {
            self.egui_mgr.gui_state.status_message = format!("Failed to read file: {}", resolved.display());
            eprintln!("Failed to read file: {}", resolved.display());
        }
    }

    pub fn load_world(&mut self, stage_path: &Path, character_path: &Path) {
        let char_resolved = match resolve_ccs_path(character_path) {
            Some(p) => p,
            None => {
                self.egui_mgr.gui_state.status_message = format!("Character not found: {}", character_path.display());
                eprintln!("Character not found: {}", character_path.display());
                return;
            }
        };

        let stage_resolved = match resolve_ccs_path(stage_path) {
            Some(p) => p,
            None => {
                self.egui_mgr.gui_state.status_message = format!("Stage not found: {}", stage_path.display());
                eprintln!("Stage not found: {}", stage_path.display());
                return;
            }
        };

        let char_data = match std::fs::read(&char_resolved) {
            Ok(d) => d,
            Err(e) => {
                self.egui_mgr.gui_state.status_message = format!("Failed to read character: {}", e);
                return;
            }
        };

        let char_ccs = match hackgu_ccs::CcsFile::parse(&char_data) {
            Ok(c) => c,
            Err(e) => {
                self.egui_mgr.gui_state.status_message = format!("Failed to parse character: {}", e);
                return;
            }
        };

        println!("Spawning character scene: {}", char_resolved.display());
        self.field_props.clear();
        self.gate_prop = None;
        self.dungeon_platform = None;
        self.temple_cover.clear();
        let mut loaded_scene = GpuScene::from_ccs(&self.device, &self.queue, &self.texture_bind_group_layout, &char_ccs);

        // Load stage environment and extract authentic collision mesh from stage models
        let mut stage_collision = hackgu_world::CollisionMesh::empty();
        if !stage_resolved.to_string_lossy().contains("sf3_00") {
            self.temple_interior = false;
        }
        let is_mac_anu = stage_resolved.to_string_lossy().contains("town01");
        let is_field = stage_resolved.to_string_lossy().contains(crate::field::area().kit.set.trim_end_matches(".ccs")) && !self.temple_interior;
        // Mac Anu is town 0 -> record 3 of gu::town::s_post_filter_param (DLL table at RVA 0x79b698).
        let pf = std::path::Path::new("target/cmn/post_filter.bin");
        self.postfx.filter = if let Ok(o) = std::env::var("HACKGU_POSTFX") {
            // Debug: kind:index.
            o.split_once(':').and_then(|(k, i)| crate::postfx::load(pf, k, i.parse().ok()?))
        } else if is_mac_anu {
            crate::postfx::load(pf, "town", crate::postfx::town_record(0))
        } else if self.temple_interior && stage_resolved.to_string_lossy().contains(crate::field::area().kit.set.trim_end_matches(".ccs")) {
            // FITTED, not traced: dungeon record 1 (blue screen-add flare) is the closest of the 7 dungeon records to
            // the original statue room (reference f00825 colour statistics). The record-selection rule is not traced.
            crate::field::area().postfx.statue_room.as_ref().and_then(|(k, i)| crate::postfx::load(pf, k, *i))
        } else {
            None
        };
        // Spawn point (metres) chosen by the stage loader; None = fall back to the origin.
        let mut stage_spawn: Option<glam::Vec3> = None;
        if is_mac_anu {
            // Mac Anu is nine district files (town01_00h .. town01_08h) plus the shared town01.ccs. The districts are
            // positioned by the DMY_town01_XXd locators stored (in centimetres) in town01.ccs, and the collision
            // comes from the HitMesh sections in each file (the game's own walls and floors).
            println!("Assembling Mac Anu from town01.ccs + 9 districts (DMY_town01_XXd placement, HitMesh collision)...");
            let mut locators: std::collections::HashMap<String, [f32; 3]> = std::collections::HashMap::new();
            let main_ccs = resolve_ccs_path(Path::new("town01.ccs"))
                .and_then(|p| std::fs::read(p).ok())
                .and_then(|d| hackgu_ccs::CcsFile::parse(&d).ok());
            if let Some(main) = &main_ccs {
                for (name, pos) in main.locators() {
                    locators.insert(name, pos);
                }
                // town01.ccs is a library: besides the sky (bs/bc/sun/sil) every clump (shop doors, gate lights, ships,
                // sea tiles, box01, waves) sits at the origin and is placed by game code we have not decoded yet.
                // Drawing those at the origin piled them into the central plaza, so only the sky is loaded here.
                loaded_scene.load_stage_environment_where(&self.device, &self.queue, &self.texture_bind_group_layout, main, glam::Vec3::ZERO, |clump| {
                    ["CMP_bs", "CMP_bc", "CMP_sun", "CMP_sil"].iter().any(|p| clump.starts_with(p))
                });
                stage_collision.merge(&hackgu_world::CollisionMesh::from_hit_meshes(main, [0.0; 3]));
            } else {
                eprintln!("town01.ccs not found: districts will be placed at the origin");
            }
            self.town_locators = locators.clone();
            // Debug: HACKGU_TOWN_SKIP=3,5 leaves those districts out.
            let town_skip: Vec<u32> = std::env::var("HACKGU_TOWN_SKIP").map(|v| v.split(',').filter_map(|x| x.trim().parse().ok()).collect()).unwrap_or_default();
            for n in 0..=8u32 {
                if town_skip.contains(&n) {
                    continue;
                }
                let dist_name = format!("town01_{:02}h.ccs", n);
                // town01_00h is the gate-hall interior: it shares town01_01h's origin (its door at local y 20.9 meets
                // DMY_town01h_tobirad, and its floor is covered by 01h's HitMesh; 00h has none of its own).
                let loc_n = if n == 0 { 1 } else { n };
                let off_cm = locators.get(&format!("DMY_town01_{:02}d", loc_n)).copied().unwrap_or([0.0; 3]);
                let off_m = glam::Vec3::new(off_cm[0], off_cm[1], off_cm[2]) * 0.01;
                if let Some(dist_path) = resolve_ccs_path(Path::new(&dist_name)) {
                    if let Ok(dist_data) = std::fs::read(&dist_path) {
                        if let Ok(dist_ccs) = hackgu_ccs::CcsFile::parse(&dist_data) {
                            println!("  {} placed at ({:.1}, {:.1}, {:.1}) m", dist_name, off_m.x, off_m.y, off_m.z);
                            stage_collision.merge(&hackgu_world::CollisionMesh::from_hit_meshes(&dist_ccs, off_cm));
                            loaded_scene.load_stage_environment_at(&self.device, &self.queue, &self.texture_bind_group_layout, &dist_ccs, off_m);
                        }
                    }
                } else {
                    eprintln!("  {} not found", dist_name);
                }
            }
            // The Chaos Gate model (xcg.ccs) stands at DMY_gate (the DLL looks DMY_gate up for the gate object).
            // It loops its idle animation ANM_xcg_nut (outer ring: one full turn per 120 frames; the user confirms the
            // gate turns constantly in the original, faster while warping, not traced).
            // Vol.1's own copy is town/xcg.ccs.
            if let (Some(g), Some(xcg)) = (locators.get("DMY_gate"), crate::cutscene::load_ccs("target/vol1/data/data/town/xcg.ccs").or_else(|| crate::cutscene::load_ccs("xcg.ccs"))) {
                let pos = glam::Vec3::new(g[0], g[1], g[2]) * 0.01;
                if let Some(idx) = loaded_scene.spawn_prop(&self.device, &self.queue, &self.texture_bind_group_layout, &xcg, "CMP_xcg") {
                    let anim = xcg.animations().into_iter().find(|a| a.name == "ANM_xcg_nut");
                    self.field_props.push(FieldProp { once: false, idx, pos, yaw: 0.0, ccs: std::sync::Arc::new(xcg), anim, t: 0.0 });
                    self.gate_prop = Some(self.field_props.len() - 1);
                } else {
                    loaded_scene.load_stage_environment_at(&self.device, &self.queue, &self.texture_bind_group_layout, &xcg, pos);
                }
            }
            // Spawn in front of the Chaos Gate: DMY_start (0, 228.5, 12 m) sits just south of DMY_gate. The opening
            // (PM10100) and every log-in in G.U. start at the gate; DMY_pc01 (central plaza) is the fallback.
            if let Some(p) = locators.get("DMY_start").or_else(|| locators.get("DMY_pc01")) {
                stage_spawn = Some(glam::Vec3::new(p[0], p[1], p[2]) * 0.01);
            }
        } else if let Some(cur) = crate::dungeon::current().filter(|c| stage_resolved.to_string_lossy().contains(c.def.kit.trim_end_matches(".ccs"))) {
            let (col, spawn) = self.build_dungeon(&mut loaded_scene, &cur.def, cur.floor);
            stage_collision = col;
            stage_spawn = spawn;
        } else if stage_resolved.to_string_lossy().contains(crate::field::area().kit.set.trim_end_matches(".ccs")) && self.temple_interior {
            stage_collision = self.build_temple_interior(&mut loaded_scene);
            stage_spawn = Some(crate::field::TEMPLE_INTERIOR_SPAWN);
        } else if stage_resolved.to_string_lossy().contains(crate::field::area().kit.set.trim_end_matches(".ccs")) {
            // The current area's field, assembled from its tile kit (assets/areas/*.json, field.rs).
            stage_collision = self.build_holy_ground(&mut loaded_scene);
            stage_spawn = Some(glam::Vec3::from(crate::field::area().spawn));
        } else {
            if let Ok(stage_data) = std::fs::read(&stage_resolved) {
                if let Ok(stage_ccs) = hackgu_ccs::CcsFile::parse(&stage_data) {
                    println!("Loading stage geometry & collision: {}", stage_resolved.display());
                    stage_collision = if stage_ccs.hit_meshes().is_empty() {
                        hackgu_world::CollisionMesh::from_ccs_stage(&stage_ccs)
                    } else {
                        hackgu_world::CollisionMesh::from_hit_meshes(&stage_ccs, [0.0; 3])
                    };
                    loaded_scene.load_stage_environment(&self.device, &self.queue, &self.texture_bind_group_layout, &stage_ccs);
                }
            }
        }

        // Set base submesh count AFTER stage is loaded, so swapping weapons preserves the world!
        loaded_scene.base_submesh_count = loaded_scene.submeshes.len();

        // Equip Twin Blades by default
        if let Some(dsw_path) = resolve_ccs_path(Path::new("w2dsw00l.ccs")) {
            if let Ok(wep_data) = std::fs::read(&dsw_path) {
                if let Ok(wep_ccs) = hackgu_ccs::CcsFile::parse(&wep_data) {
                    println!("Equipping twin blades onto character sockets (prop1, prop2)...");
                    loaded_scene.attach_weapon(&self.device, &self.queue, &self.texture_bind_group_layout, &wep_ccs, &char_ccs, "prop1", hackgu_ccs::Mat4::IDENTITY);
                    loaded_scene.attach_weapon(&self.device, &self.queue, &self.texture_bind_group_layout, &wep_ccs, &char_ccs, "prop2", hackgu_ccs::Mat4::IDENTITY);
                }
            }
        }

        // Select Idle animation (nut00 / rdy00)
        if let Some(idle_idx) = loaded_scene.animations.iter().position(|a| a.name.contains("nut00") || a.name.contains("rdy00")) {
            loaded_scene.select_animation(idle_idx, &self.queue, &char_ccs);
        }

        // Spawn on the stage's own start locator when it has one (metres); otherwise the origin.
        let initial_pos = stage_spawn.unwrap_or(glam::Vec3::ZERO);
        self.player.position = initial_pos;
        self.player.set_collision(stage_collision);
        // yaw 0 faces -Y: away from the gate (north end of Mac Anu), towards the town.
        // Mac Anu: facing the town (yaw 0 faces -Y). Field and temple interior: facing +Y.
        self.player.yaw = if is_mac_anu { 0.0 } else { std::f32::consts::PI };
        self.player.active = true;
        self.egui_mgr.gui_state.player_mode = true;
        loaded_scene.root_transform = Some(self.player.world_matrix());
        loaded_scene.apply_current_pose(&self.queue, &char_ccs);

        self.reset_follow_camera();
        self.show_background = true;

        self.fieldplay = if is_field {
            crate::fieldplay::FieldPlay::new(&mut loaded_scene, &self.device, &self.queue, &self.texture_bind_group_layout, &char_ccs)
        } else {
            None
        };
        self.scene = Some(loaded_scene);
        self.ccs = Some(char_ccs);
        self.current_ccs_path = Some(char_resolved);
        self.current_stage_path = Some(stage_resolved.clone());
        self.equipped_weapon = Some("twin_blades".into());
        // Twin Blades use Haseo's authentic native 2hsw01 animations (nut00/run00/gma10)
        // Boundaries now come from the stage's HitMesh walls (CollisionMesh::resolve_walls). The old hand-guessed
        // rectangular clamps are only kept as a very loose safety net for stages that ship no HitMesh.
        self.player.bounds_min = [-2000.0, -2000.0];
        self.player.bounds_max = [2000.0, 2000.0];
        let stage_name = stage_resolved.file_name().unwrap_or_default().to_string_lossy();
        self.egui_mgr.gui_state.status_message = format!("Exploring {} (WASD Controls Active)", stage_name);
        self.window.set_title(&format!(".hack//G.U. - Exploring {} (WASD)", stage_name));
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.depth_texture_view = Self::create_depth_view(&self.device, &self.config);
            self.camera.aspect = (new_size.width as f32 / new_size.height as f32).max(0.1);
            self.movie_renderer.set_target_aspect(&self.queue, new_size.width, new_size.height);
            self.postfx.resize(&self.device, self.config.format, new_size.width, new_size.height);
        }
    }

    pub fn update(&mut self) {
        let now = std::time::Instant::now();
        let mut dt = (now - self.last_instant).as_secs_f32().min(0.1);
        self.last_instant = now;
        // Recording saves a PNG every frame, which takes far longer than a frame: step the game by exactly one video
        // frame instead of wall time, or the encoded video plays back sped up.
        if crate::auto::get().record.is_some() {
            dt = 1.0 / crate::auto::RECORD_FPS;
        }

        // Process pending movie playback from GUI
        if let Some(movie) = self.egui_mgr.gui_state.pending_play_movie.take() {
            println!("🎬 Starting in-window cutscene playback: {}", movie);
            let _ = self.movie_player.play(&movie, false);
        }

        // Poll if in-window movie completed
        if self.movie_player.poll_finished() {
            println!("🎬 Movie finished. Advancing story...");
            self.movie_renderer.clear();
            self.egui_mgr.gui_state.game_runtime.advance_cutscene();
        }

        // Upload new movie frames to GPU texture
        if let Some((rgba, w, h)) = self.movie_player.latest_frame() {
            self.movie_renderer.upload(&self.device, &self.queue, &rgba, w, h);
        }

        // Auto-play in-window movie when entering GameState::Movie
        if let hackgu_game::GameState::Movie { ref movie_file, .. } = self.egui_mgr.gui_state.game_runtime.state {
            if !self.movie_player.is_playing() {
                if self.spawned_cutscene_id.as_deref() != Some(movie_file.as_str()) {
                    self.spawned_cutscene_id = Some(movie_file.clone());
                    println!("🎬 Auto-triggering in-window cutscene movie: {}", movie_file);
                    let _ = self.movie_player.play(movie_file, false);
                }
            }
        } else if self.movie_player.is_playing() && !self.egui_mgr.gui_state.show_movie_theater {
            // Stopped movie playback outside movie state
            self.movie_player.stop();
            self.movie_renderer.clear();
        }

        // The temple opens once the story heads there (after PM10135).
        if self.cutscene.is_none() && self.fieldplay.as_ref().map(|f| matches!(f.story, crate::fieldplay::Story::ToTemple | crate::fieldplay::Story::Done)).unwrap_or(false) {
            if let Some(open) = self.field_open_collision.take() {
                self.player.collision_mesh = Some(open);
            }
        }
        // Tiles over the temple interior disappear while the camera is inside it.
        if !self.temple_cover.is_empty() {
            let inside = crate::field::inside_temple(self.camera.eye_position()) || crate::field::inside_temple(self.player.position);
            if let Some(scene) = &mut self.scene {
                for &i in &self.temple_cover {
                    if let Some(sm) = scene.submeshes.get_mut(i) {
                        sm.visible = !inside;
                    }
                }
            }
        }
        if let Some((_, t)) = &mut self.reward_panel {
            *t -= dt;
            if *t <= 0.0 {
                self.reward_panel = None;
            }
        }
        if let Some((m, t)) = &mut self.pending_movie {
            *t -= dt;
            if *t <= 0.0 {
                let m = m.clone();
                self.pending_movie = None;
                self.egui_mgr.gui_state.game_runtime.start_movie(&m);
            }
        }
        // Animated stage props.
        if let Some(scene) = &mut self.scene {
            let eye = self.camera.eye_position();
            let fwd = (self.camera.target - eye).normalize_or(glam::Vec3::Y);
            let right = fwd.cross(glam::Vec3::Z).normalize_or(glam::Vec3::X);
            scene.billboard_axes = (right, right.cross(fwd));
            for (pi, p) in self.field_props.iter_mut().enumerate() {
                let _ = pi;
                p.t += dt * 30.0;
                let f = p.anim.as_ref().map(|a| {
                    let last = a.frame_count.saturating_sub(1).max(1) as f32;
                    if p.once { p.t.min(last) } else { p.t % last }
                }).unwrap_or(0.0);
                scene.update_prop(&self.queue, p.idx, p.pos, p.yaw, p.anim.as_ref().map(|a| (a, &*p.ccs)), f);
            }
        }

        // Event scripts: the EvPlayer is the only driver of camera, cast, fades and messages.
        self.update_cutscene(dt);

        // Process pending file load from GUI / Drag & Drop
        if let Some(path) = self.egui_mgr.gui_state.pending_load.take() {
            self.load_ccs(&path);
        }

        // Process pending playable exploration from GUI
        if let Some((stage_path, char_path)) = self.egui_mgr.gui_state.pending_explore.take() {
            self.load_world(Path::new(stage_path), Path::new(char_path));
        }

        // Process pending GLB export from GUI
        if self.egui_mgr.gui_state.pending_export {
            self.egui_mgr.gui_state.pending_export = false;
            if let (Some(ccs), Some(path)) = (&self.ccs, &self.current_ccs_path) {
                let out_dir = std::path::Path::new("exports");
                let _ = std::fs::create_dir_all(out_dir);
                let stem = path.file_stem().unwrap_or_default().to_string_lossy();
                let out_file = out_dir.join(format!("{}.glb", stem));
                let anim_idx = self.scene.as_ref().and_then(|s| s.active_anim_idx);
                let frame = self.scene.as_ref().map(|s| s.current_frame).unwrap_or(0.0);
                match hackgu_export::convert_ccs_to_glb(ccs, &stem, anim_idx, frame, false) {
                    Ok((glb_bytes, count)) => {
                        if std::fs::write(&out_file, &glb_bytes).is_ok() {
                            self.egui_mgr.gui_state.status_message = format!(
                                "Exported {} ({} meshes, {:.1} KB) to {}",
                                stem,
                                count,
                                glb_bytes.len() as f32 / 1024.0,
                                out_file.display()
                            );
                            println!("Saved GLB: {}", out_file.display());
                        } else {
                            self.egui_mgr.gui_state.status_message = "Failed to write GLB file".into();
                        }
                    }
                    Err(e) => {
                        self.egui_mgr.gui_state.status_message = format!("Export error: {}", e);
                    }
                }
            } else {
                self.egui_mgr.gui_state.status_message = "No model loaded to export".into();
            }
        }

        // Process pending CPK unpack/extract from GUI
        if self.egui_mgr.gui_state.pending_cpk_extract {
            self.egui_mgr.gui_state.pending_cpk_extract = false;
            let cpk_path = self.egui_mgr.gui_state.cpk_path.clone();
            let cpk_path_buf = PathBuf::from(&cpk_path);
            let cpk_stem = cpk_path_buf.file_stem().unwrap_or_default().to_string_lossy();
            let out_dir = std::path::PathBuf::from("target").join(cpk_stem.as_ref());
            self.egui_mgr.gui_state.status_message = format!("Extracting {}...", cpk_stem);
            match hackgu_cpk::CpkArchive::open(&cpk_path) {
                Ok((archive, mut file)) => {
                    match archive.extract_all(&mut file, &out_dir) {
                        Ok(count) => {
                            self.egui_mgr.gui_state.status_message = format!("Extracted {} files to {}", count, out_dir.display());
                            println!("Extracted {} files to {}", count, out_dir.display());
                        }
                        Err(e) => {
                            self.egui_mgr.gui_state.status_message = format!("Extraction error: {}", e);
                            eprintln!("Extraction error: {}", e);
                        }
                    }
                }
                Err(e) => {
                    self.egui_mgr.gui_state.status_message = format!("Failed to open CPK: {}", e);
                    eprintln!("Failed to open CPK {}: {}", cpk_path, e);
                }
            }
        }

        // Process pending batch GLB export from GUI
        if self.egui_mgr.gui_state.pending_batch {
            self.egui_mgr.gui_state.pending_batch = false;
            let cpk_path = self.egui_mgr.gui_state.cpk_path.clone();
            let out_dir = std::path::PathBuf::from("exports/batch");
            self.egui_mgr.gui_state.status_message = "Batch exporting... (processing)".into();
            match hackgu_export::batch_export_cpk(std::path::Path::new(&cpk_path), &out_dir) {
                Ok(count) => {
                    self.egui_mgr.gui_state.status_message = format!("Batch exported {} models to {}", count, out_dir.display());
                    println!("Batch export finished: {} models exported", count);
                }
                Err(e) => {
                    self.egui_mgr.gui_state.status_message = format!("Batch failed: {}", e);
                    eprintln!("Batch export error: {}", e);
                }
            }
        }

        // Process pending weapon equipment from GUI
        if let Some(equip_type) = self.egui_mgr.gui_state.pending_equip.take() {
            let mut weapon_stance: Option<(&str, &str, &str)> = None;
            if let (Some(scene), Some(char_ccs)) = (&mut self.scene, &self.ccs) {
                scene.clear_weapon_attachments();
                match equip_type.as_str() {
                    "twin_blades" => {
                        if let Some(dsw_path) = resolve_ccs_path(Path::new("w2dsw00l.ccs")) {
                            if let Ok(data) = std::fs::read(&dsw_path) {
                                if let Ok(wep_ccs) = hackgu_ccs::CcsFile::parse(&data) {
                                    scene.attach_weapon(
                                        &self.device,
                                        &self.queue,
                                        &self.texture_bind_group_layout,
                                        &wep_ccs,
                                        char_ccs,
                                        "prop1",
                                        hackgu_ccs::Mat4::IDENTITY,
                                    );
                                    scene.attach_weapon(
                                        &self.device,
                                        &self.queue,
                                        &self.texture_bind_group_layout,
                                        &wep_ccs,
                                        char_ccs,
                                        "prop2",
                                        hackgu_ccs::Mat4::IDENTITY,
                                    );
                                    self.egui_mgr.gui_state.status_message = "Equipped Haseo Twin Blades (Dual)".into();
                                    self.equipped_weapon = Some("twin_blades".to_string());
                                    // Twin Blades use Haseo's native 2hsw01 animations (nut00/run00/gma10)
                                    scene.animations.retain(|a| !a.name.contains("2hsw20") && !a.name.contains("2hsw34") && !a.name.contains("2hsw54") && !a.name.contains("2tbl"));
                                    if let Some(idle_idx) = scene.animations.iter().position(|a| a.name.contains("nut00")) {
                                        scene.select_animation(idle_idx, &self.queue, char_ccs);
                                    }
                                }
                            }
                        }
                    }
                    "broadsword" => {
                        if let Some(sw_path) = resolve_ccs_path(Path::new("w22sw01.ccs")) {
                            if let Ok(data) = std::fs::read(&sw_path) {
                                if let Ok(wep_ccs) = hackgu_ccs::CcsFile::parse(&data) {
                                    scene.attach_weapon(
                                        &self.device,
                                        &self.queue,
                                        &self.texture_bind_group_layout,
                                        &wep_ccs,
                                        char_ccs,
                                        "prop1",
                                        hackgu_ccs::Mat4::IDENTITY,
                                    );
                                    self.egui_mgr.gui_state.status_message = "Equipped Broadsword".into();
                                    weapon_stance = Some(("broadsword", "2hsw20rdy00.ccs", "2hsw20str00.ccs"));
                                }
                            }
                        }
                    }
                    "scythe" => {
                        if let Some(sc_path) = resolve_ccs_path(Path::new("w22sc01.ccs")) {
                            if let Ok(data) = std::fs::read(&sc_path) {
                                if let Ok(wep_ccs) = hackgu_ccs::CcsFile::parse(&data) {
                                    scene.attach_weapon(
                                        &self.device,
                                        &self.queue,
                                        &self.texture_bind_group_layout,
                                        &wep_ccs,
                                        char_ccs,
                                        "prop1",
                                        hackgu_ccs::Mat4::IDENTITY,
                                    );
                                    self.egui_mgr.gui_state.status_message = "Equipped Reaper Scythe".into();
                                    weapon_stance = Some(("scythe", "2hsw34rdy00.ccs", "2hsw34str00.ccs"));
                                }
                            }
                        }
                    }
                    "guns" => {
                        if let Some(gn_path) = resolve_ccs_path(Path::new("w2dgn00.ccs")) {
                            if let Ok(data) = std::fs::read(&gn_path) {
                                if let Ok(wep_ccs) = hackgu_ccs::CcsFile::parse(&data) {
                                    scene.attach_weapon(
                                        &self.device,
                                        &self.queue,
                                        &self.texture_bind_group_layout,
                                        &wep_ccs,
                                        char_ccs,
                                        "prop1",
                                        hackgu_ccs::Mat4::IDENTITY,
                                    );
                                    scene.attach_weapon(
                                        &self.device,
                                        &self.queue,
                                        &self.texture_bind_group_layout,
                                        &wep_ccs,
                                        char_ccs,
                                        "prop2",
                                        hackgu_ccs::Mat4::IDENTITY,
                                    );
                                    self.egui_mgr.gui_state.status_message = "Equipped Dual Guns (DG-0)".into();
                                    weapon_stance = Some(("guns", "2hsw54rdy00.ccs", "2hsw54str00.ccs"));
                                }
                            }
                        }
                    }
                    _ => {
                        self.egui_mgr.gui_state.status_message = "Unequipped weapons (Unarmed)".into();
                    }
                }
            } else {
                self.egui_mgr.gui_state.status_message = "Cannot equip weapon: No character loaded".into();
            }

            // Equipping attaches fresh (visible) weapon meshes; while an event hides Haseo they must stay hidden, or
            // they float where his hidden skeleton is (the Gate warp queues twin blades just as PM10105 starts).
            if self.cutscene.is_some() {
                if let Some(scene) = &mut self.scene {
                    scene.set_main_character_visible(false);
                }
            }
            if let Some((kind, idle_anim, run_anim)) = weapon_stance {
                self.equipped_weapon = Some(kind.to_string());
                self.load_weapon_animations(idle_anim, run_anim);
            } else if equip_type == "none" || equip_type == "unarmed" {
                self.equipped_weapon = None;
                if let (Some(scene), Some(ccs)) = (&mut self.scene, &self.ccs) {
                    scene.animations.retain(|a| !a.name.contains("rdy00") && !a.name.contains("str00"));
                    if let Some(idle_idx) = scene.animations.iter().position(|a| a.name.contains("nut00")) {
                        scene.select_animation(idle_idx, &self.queue, ccs);
                    }
                }
            }
        }

        if self.egui_mgr.gui_state.player_mode != self.player.active {
            self.player.active = self.egui_mgr.gui_state.player_mode;
            if !self.player.active {
                if let (Some(scene), Some(ccs)) = (&mut self.scene, &self.ccs) {
                    scene.root_transform = None;
                    scene.apply_current_pose(&self.queue, ccs);
                }
            }
        }

        // Gamepad controller input polling via gilrs
        let mut gamepad_analog_x = 0.0f32;
        let mut gamepad_analog_y = 0.0f32;
        let mut gamepad_cam_rx = 0.0f32;
        let mut gamepad_cam_ry = 0.0f32;

        let mut gp_attack = crate::auto::get().press.contains(&self.frame_no);
        let mut gp_rengeki = false;
        let mut gp_interact = false;
        let mut gp_heal = false;
        let mut gp_center_cam = false;
        let mut gp_toggle_hud = false;
        let mut gp_skip = false;
        let gp_equip: Option<&str> = None;

        // Menu screens (title, desktop, Chaos Gate, movies) take the pad as menu keys and lock the player.
        let menu_open = self.menu_open();
        if let Some(gilrs) = &mut self.gilrs {
            while let Some(event) = gilrs.next_event() {
                gilrs.update(&event);
                if let gilrs::EventType::ButtonPressed(btn, _) = event.event {
                    if menu_open {
                        let key = match btn {
                            gilrs::Button::South => Some(egui::Key::Enter),
                            gilrs::Button::East => Some(egui::Key::Escape),
                            gilrs::Button::DPadUp => Some(egui::Key::ArrowUp),
                            gilrs::Button::DPadDown => Some(egui::Key::ArrowDown),
                            gilrs::Button::DPadLeft => Some(egui::Key::ArrowLeft),
                            gilrs::Button::DPadRight => Some(egui::Key::ArrowRight),
                            _ => None,
                        };
                        if let Some(k) = key {
                            self.pad_keys.push(k);
                        }
                        // A or Start skips a playing movie (as Space does); Start also closes the Main Menu.
                        if (btn == gilrs::Button::South || btn == gilrs::Button::Start) && self.movie_player.is_playing() {
                            gp_skip = true;
                        }
                        if btn == gilrs::Button::Start && self.egui_mgr.gui_state.main_menu_open {
                            gp_toggle_hud = true;
                        }
                        continue;
                    }
                    match btn {
                        // A: confirm / talk / attack (advances event text); B: cancel / close a popup.
                        gilrs::Button::South => gp_attack = true,
                        gilrs::Button::West => gp_rengeki = true,
                        gilrs::Button::East => gp_interact = true,
                        gilrs::Button::North => {
                            gp_heal = true;
                            self.menu_prompt_pressed = true;
                        }
                        gilrs::Button::LeftTrigger => gp_center_cam = true,
                        // Start: skip the running event, otherwise the Main Menu.
                        gilrs::Button::Start => {
                            if self.cutscene.is_some() || matches!(self.egui_mgr.gui_state.game_runtime.state, hackgu_game::GameState::Cutscene { .. } | hackgu_game::GameState::Movie { .. }) {
                                gp_skip = true;
                            } else {
                                gp_toggle_hud = true;
                            }
                        }
                        _ => {}
                    }
                }
            }

            for (_id, gamepad) in gilrs.gamepads() {
                if gamepad.is_connected() {
                    gamepad_analog_x = gamepad.value(gilrs::Axis::LeftStickX);
                    gamepad_analog_y = gamepad.value(gilrs::Axis::LeftStickY);
                    gamepad_cam_rx = gamepad.value(gilrs::Axis::RightStickX);
                    gamepad_cam_ry = gamepad.value(gilrs::Axis::RightStickY);
                    break;
                }
            }
        }

        // In menus the left stick moves the cursor like the D-pad: one step on push, then repeats while held.
        if menu_open {
            let dir = if gamepad_analog_y > 0.6 {
                Some(egui::Key::ArrowUp)
            } else if gamepad_analog_y < -0.6 {
                Some(egui::Key::ArrowDown)
            } else if gamepad_analog_x < -0.6 {
                Some(egui::Key::ArrowLeft)
            } else if gamepad_analog_x > 0.6 {
                Some(egui::Key::ArrowRight)
            } else {
                None
            };
            self.stick_nav = match (dir, self.stick_nav) {
                (None, _) => None,
                (Some(d), Some((held, t))) if d == held => {
                    let t = t - dt;
                    if t <= 0.0 {
                        self.pad_keys.push(d);
                        Some((d, 0.12))
                    } else {
                        Some((d, t))
                    }
                }
                (Some(d), _) => {
                    self.pad_keys.push(d);
                    Some((d, 0.4))
                }
            };
            gamepad_analog_x = 0.0;
            gamepad_analog_y = 0.0;
        } else {
            self.stick_nav = None;
        }

        if gp_skip {
            if self.movie_player.is_playing() {
                self.movie_player.stop();
                self.movie_renderer.clear();
                self.egui_mgr.gui_state.game_runtime.skip_cutscene();
            } else if self.cutscene.is_some() {
                self.end_cutscene(true);
            } else if matches!(self.egui_mgr.gui_state.game_runtime.state, hackgu_game::GameState::Cutscene { .. } | hackgu_game::GameState::Movie { .. }) {
                self.sound_system.stop_voice();
                self.egui_mgr.gui_state.game_runtime.skip_cutscene();
            }
        }

        if gp_toggle_hud {
            // Start / menu button: the Main Menu in town and the field.
            self.toggle_main_menu();
        }
        // Party lines for the Main Menu: Haseo, then the area party (field state when in a field).
        {
            let save = &self.egui_mgr.gui_state.game_runtime.save;
            let mut lines = Vec::new();
            let (hp, max_hp, sp) = self.fieldplay.as_ref().map(|f| (f.hp, f.max_hp, f.sp)).unwrap_or((save.hp as i32, save.max_hp as i32, save.sp as i32));
            lines.push(crate::mainmenu::PartyLine { name_key: "spcName_000_A", face: "xfhsw", level: save.level, hp, max_hp, sp, max_sp: save.max_sp as i32 });
            if let Some(fp) = &self.fieldplay {
                let area = crate::field::area();
                for (m, (hp, max_hp, sp)) in area.party.iter().zip(fp.follower_status()) {
                    let (name_key, face) = crate::mainmenu::party_identity(&m.model);
                    // Followers' levels are not known yet (0 = not shown).
                    lines.push(crate::mainmenu::PartyLine { name_key, face, level: 0, hp, max_hp, sp, max_sp: sp });
                }
            }
            self.egui_mgr.gui_state.party_lines = lines;
        }

        if let Some(eq) = gp_equip {
            self.egui_mgr.gui_state.pending_equip = Some(eq.to_string());
        }

        if gp_center_cam {
            self.camera.yaw = self.player.yaw;
        }

        if gamepad_cam_rx.abs() > 0.15 {
            self.camera.yaw += gamepad_cam_rx * dt * 2.8;
        }
        if gamepad_cam_ry.abs() > 0.15 {
            self.camera.pitch = (self.camera.pitch - gamepad_cam_ry * dt * 1.8).clamp(-0.8, 1.2);
        }

        if gp_heal && self.player.active {
            if self.egui_mgr.gui_state.game_runtime.use_item("Health Drink") {
                self.egui_mgr.gui_state.status_message = format!("Used Health Drink! HP: {}/{}", self.egui_mgr.gui_state.game_runtime.save.hp, self.egui_mgr.gui_state.game_runtime.save.max_hp);
            }
        }

        if gp_attack {
            if self.movie_player.is_playing() {
                self.movie_player.stop();
                self.movie_renderer.clear();
                self.egui_mgr.gui_state.game_runtime.advance_cutscene();
            } else if let Some(cs) = &mut self.cutscene {
                cs.player.press_advance();
            } else if matches!(self.egui_mgr.gui_state.game_runtime.state, hackgu_game::GameState::Cutscene { .. } | hackgu_game::GameState::Movie { .. }) {
                self.egui_mgr.gui_state.game_runtime.advance_cutscene();
            } else if self.player.active && self.dungeon_chest_event_ready() {
                self.open_dungeon_chest();
            } else if self.player.active && self.temple_interior && self.near_offering_chest() {
                self.open_chest(0);
                self.show_mission_reward();
                self.pending_movie = crate::field::area().events.statue_movie.clone().map(|m| (format!("{}.usm", m), 3.0));
            } else if self.player.active && self.fieldplay.is_some() && self.egui_mgr.gui_state.game_runtime.is_in_field() {
                let (pos, yaw) = (self.player.position, self.player.yaw);
                match self.fieldplay.as_mut().unwrap().press_a(pos, yaw) {
                    Some(crate::fieldplay::FieldEvent::Message(m)) => self.egui_mgr.gui_state.status_message = m,
                    Some(crate::fieldplay::FieldEvent::ChestOpened(k, m)) => {
                        self.open_chest(k);
                        self.egui_mgr.gui_state.status_message = m;
                    }
                    _ => {}
                }
            } else if self.player.active && matches!(self.egui_mgr.gui_state.game_runtime.state, hackgu_game::GameState::Town { .. }) {
                // A at the Chaos Gate opens it.
                if near_chaos_gate(self.player.position) {
                    self.egui_mgr.gui_state.game_runtime.open_chaos_gate();
                }
            } else if self.player.active {
                if self.egui_mgr.gui_state.game_runtime.is_in_combat() {
                    let outcome = self.egui_mgr.gui_state.game_runtime.combat_attack();
                    println!("⚔ Gamepad Attack: {:?}", outcome);
                    if let (Some(scene), Some(ccs)) = (&mut self.scene, &self.ccs) {
                        if let Some(atk_idx) = scene.animations.iter().position(|a| a.name.contains("str00") || a.name.contains("gma")) {
                            scene.select_animation(atk_idx, &self.queue, ccs);
                        }
                    }
                } else if self.egui_mgr.gui_state.game_runtime.is_in_field() {
                    let player_pos = self.player.position;
                    let mut nearest_enemy = None;
                    if let hackgu_game::GameState::Field { ref enemies, .. } = self.egui_mgr.gui_state.game_runtime.state {
                        for (idx, e) in enemies.iter().enumerate() {
                            if (e.position - player_pos).length() <= 4.5 && e.stats.current_hp > 0 {
                                nearest_enemy = Some(idx);
                                break;
                            }
                        }
                    }
                    if let Some(e_idx) = nearest_enemy {
                        self.egui_mgr.gui_state.game_runtime.surprise_attack(e_idx);
                    } else if let Some(msg) = self.egui_mgr.gui_state.game_runtime.interact_nearest(player_pos) {
                        self.egui_mgr.gui_state.status_message = msg;
                    }
                }
            }
        }

        if gp_rengeki && self.player.active && self.egui_mgr.gui_state.game_runtime.is_in_combat() {
            let outcome = self.egui_mgr.gui_state.game_runtime.combat_rengeki();
            println!("💥 Gamepad RENGEKI FINISH: {:?}", outcome);
            if let (Some(scene), Some(ccs)) = (&mut self.scene, &self.ccs) {
                if let Some(atk_idx) = scene.animations.iter().position(|a| a.name.contains("mag") || a.name.contains("str00")) {
                    scene.select_animation(atk_idx, &self.queue, ccs);
                }
            }
        }

        // B: closes a popup; in town and the field it still talks to / opens what is in front of Haseo (as E does).
        // It no longer skips events (that is Start).
        if gp_interact && self.cutscene.is_none() && !matches!(self.egui_mgr.gui_state.game_runtime.state, hackgu_game::GameState::Cutscene { .. } | hackgu_game::GameState::Movie { .. }) {
            if matches!(self.egui_mgr.gui_state.game_runtime.state, hackgu_game::GameState::Town { active_dialogue: Some(_), .. } | hackgu_game::GameState::Field { active_dialogue: Some(_), .. }) {
                self.egui_mgr.gui_state.game_runtime.close_popup();
            } else if self.player.active {
                let player_pos = self.player.position;
                match &mut self.egui_mgr.gui_state.game_runtime.state {
                    hackgu_game::GameState::Town { .. } => {
                        // Only the Chaos Gate is interactive in Mac Anu so far (the old fallback opened an invented
                        // Atoli conversation anywhere in town).
                        if near_chaos_gate(player_pos) {
                            self.egui_mgr.gui_state.game_runtime.open_chaos_gate();
                        }
                    }
                    hackgu_game::GameState::Field { .. } => {
                        if let Some(msg) = self.egui_mgr.gui_state.game_runtime.interact_nearest(player_pos) {
                            self.egui_mgr.gui_state.status_message = msg;
                        }
                    }
                    _ => {}
                }
            }
        }

        // Fades, and the place's own music: Mac Anu plays bgm.acb "rtown0" (the cue PM10100 starts over the city), the
        // ALTIMIT desktop "TD_desktop". Fields have no known cue yet (the area table's BGM field is not traced).
        self.sound_system.tick(dt);
        if !crate::auto::get().mute && self.cutscene.is_none() && !self.movie_player.is_playing() {
            let in_town = self.current_stage_path.as_ref().is_some_and(|p| p.to_string_lossy().contains("town01"));
            let want = match &self.egui_mgr.gui_state.game_runtime.state {
                hackgu_game::GameState::Desktop => Some("TD_desktop"),
                hackgu_game::GameState::Town { .. } | hackgu_game::GameState::ChaosGateSelector { .. } if in_town => Some("rtown0"),
                _ => None,
            };
            if let Some(name) = want {
                let cue = self.sound_system.cue_index(name);
                if self.area_music_wanted || self.sound_system.current_bgm() != cue || !self.sound_system.is_bgm_playing() {
                    self.sound_system.play_bgm_named(name);
                }
            }
            self.area_music_wanted = false;
        }

        if std::env::var("HACKGU_DEBUG_POS").is_ok() && self.frame_no % 10 == 0 {
            eprintln!("pos f{} {:?}", self.frame_no, self.player.position);
            if let (Some(c), Ok(spec)) = (&self.player.collision_mesh, std::env::var("HACKGU_PROBE")) {
                // HACKGU_PROBE=x0,y0,x1,y1: floor heights along a line (every metre), once.
                let v: Vec<f32> = spec.split(',').filter_map(|x| x.parse().ok()).collect();
                if v.len() == 4 && self.frame_no == 30 {
                    let (a, b) = (glam::Vec2::new(v[0], v[1]), glam::Vec2::new(v[2], v[3]));
                    let n = (b - a).length().ceil() as i32;
                    for k in 0..=n {
                        let q = a + (b - a) * (k as f32 / n.max(1) as f32);
                        let hs: Vec<String> = (-20..20).filter_map(|z| c.get_ground_height(q.x, q.y, z as f32)).map(|h| format!("{:.2}", h)).collect::<std::collections::BTreeSet<_>>().into_iter().collect();
                        eprintln!("probe {:?} floors {:?}", q, hs);
                    }
                }
            }
            if let Some(c) = &self.player.collision_mesh {
                let p = glam::Vec2::new(self.player.position.x, self.player.position.y);
                for w in &c.walls {
                    let ab = w.b - w.a;
                    let t = ((p - w.a).dot(ab) / ab.length_squared().max(1e-6)).clamp(0.0, 1.0);
                    if (w.a + ab * t - p).length() < 1.0 {
                        eprintln!("   wall {:?}-{:?} z {:.2}..{:.2}", w.a, w.b, w.z_min, w.z_max);
                    }
                }
            }
        }
        // Dungeon floor change: stepping onto the platform loads the other floor, arriving 3 m off its platform.
        if self.cutscene.is_none() && self.player.active && !self.menu_open() {
            if let (Some((p, to)), Some(cur)) = (self.dungeon_platform, crate::dungeon::current()) {
                let d = self.player.position - p;
                let on = glam::Vec2::new(d.x, d.y).length() < 4.0 && d.z.abs() < 2.0;
                if !on {
                    self.dungeon_platform_armed = true;
                }
                if on && self.dungeon_platform_armed {
                    self.dungeon_platform_armed = false;
                    let plat = cur.def.floors.get(to).and_then(|_| {
                        crate::cutscene::load_ccs(&cur.def.kit)
                            .and_then(|k| k.locators().into_iter().find(|(n, _)| n == "DMY_floorup-b01-plat").map(|(_, p)| p))
                            .and_then(|pc| cur.def.floor_platform(to, pc))
                    });
                    if let Some(q) = plat {
                        println!("dungeon: floor {} -> {}", cur.floor, to);
                        crate::dungeon::set_floor(to);
                        // Arrive beside the platform, toward the hall's upper end (+x on both floors here).
                        self.dungeon_arrival = Some(q + glam::Vec3::new(3.0, 0.0, 0.0));
                        let kit = cur.def.kit.clone();
                        self.load_world(Path::new(&kit), Path::new("2hsw01.ccs"));
                        self.player.yaw = std::f32::consts::FRAC_PI_2;
                        self.reset_follow_camera();
                    }
                }
            }
        }

        // The event's menu prompt was answered: open the Main Menu as soon as Haseo is back in control.
        if self.open_menu_after_event && self.cutscene.is_none() && self.player.active && matches!(self.egui_mgr.gui_state.game_runtime.state, hackgu_game::GameState::Town { .. } | hackgu_game::GameState::Field { .. }) {
            self.open_menu_after_event = false;
            if !self.egui_mgr.gui_state.main_menu_open {
                self.toggle_main_menu();
            }
        }

        // Town proximity events (start condition 79): the armed ones play when Haseo comes within their radius of their
        // DMY_ev locator.
        if self.cutscene.is_none() && self.player.active && !self.menu_open() && matches!(self.egui_mgr.gui_state.game_runtime.state, hackgu_game::GameState::Town { .. }) {
            let pos = self.player.position;
            let armed = crate::events::armed_town_events(&self.egui_mgr.gui_state.game_runtime.events_done);
            let hit = armed.into_iter().find(|e| {
                let (Some(n), Some(r)) = (e.locator, e.trigger_radius_cm) else { return false };
                self.town_locators.get(&format!("DMY_ev{}", n)).is_some_and(|p| {
                    let c = glam::Vec3::from(*p) * 0.01;
                    glam::Vec2::new(pos.x - c.x, pos.y - c.y).length() <= r as f32 * 0.01
                })
            });
            if let Some(e) = hit {
                println!("town trigger: {}", e.event);
                self.egui_mgr.gui_state.game_runtime.start_script(&e.event);
            }
        }

        // Dungeon events started by game code (start condition 80): GUESS, not traced: the next one plays when Haseo
        // enters the 16 m cell of its locator (the blob record that fills it, dungeon.rs).
        if self.cutscene.is_none() && self.player.active && !self.menu_open() && matches!(self.egui_mgr.gui_state.game_runtime.state, hackgu_game::GameState::Field { .. }) {
            if let Some(cur) = crate::dungeon::current() {
                let pos = self.player.position;
                let next = crate::events::next_dungeon_event(cur.def.words, &self.egui_mgr.gui_state.game_runtime.events_done);
                if std::env::var("HACKGU_DEBUG_TRIGGER").is_ok() {
                    eprintln!("dungeon trigger check: pos {:?} next {:?} at {:?}", pos, next.map(|e| &e.event), next.and_then(|e| e.locator).and_then(|n| self.dungeon_piece_locators.get(&n).copied().or_else(|| cur.def.locator(n))));
                }
                if let Some(e) = next {
                    let inside = e.locator.and_then(|n| self.dungeon_piece_locators.get(&n).copied().or_else(|| cur.def.locator(n))).is_some_and(|c| (pos.x / 16.0).floor() == (c.x / 16.0).floor() && (pos.y / 16.0).floor() == (c.y / 16.0).floor());
                    if inside {
                        println!("dungeon trigger: {}", e.event);
                        self.egui_mgr.gui_state.game_runtime.start_script(&e.event);
                    }
                }
            }
        }

        // Start / stop event playback when the game state enters or leaves a cutscene.
        let wanted = if let hackgu_game::GameState::Cutscene { ref script_id, .. } = self.egui_mgr.gui_state.game_runtime.state {
            Some(script_id.clone())
        } else {
            None
        };
        let running = self.cutscene.as_ref().map(|c| c.id().to_string());
        if wanted != running {
            if running.is_some() {
                self.end_cutscene(false);
            }
            if let Some(id) = wanted {
                self.start_cutscene(&id);
                // Apply the event's first frame (camera, fade) now: this frame's event update has already run, and
                // the player's view would show for one frame.
                self.update_cutscene(0.0);
            }
        }

        // Automatic stage management matching GameState
        match &self.egui_mgr.gui_state.game_runtime.state {
            hackgu_game::GameState::Field { .. } => {
                // A story dungeon (dungeon.rs) also runs in the Field state, on its own kit.
                let dungeon_kit = crate::dungeon::current().map(|c| c.def.kit.trim_end_matches(".ccs").to_string());
                let is_field_loaded = self.current_stage_path.as_ref()
                    .map(|p| p.to_string_lossy().contains("sf3_00") || dungeon_kit.as_ref().is_some_and(|k| p.to_string_lossy().contains(k.as_str())))
                    .unwrap_or(false);
                if !is_field_loaded {
                    let stage = dungeon_kit.map(|k| format!("{}.ccs", k)).unwrap_or_else(|| crate::field::area().kit.set.clone());
                    println!("Entering the Area: loading {}...", stage);
                    self.load_world(Path::new(&stage), Path::new("2hsw01.ccs"));
                }
            }
            hackgu_game::GameState::Town { .. } => {
                let is_town_loaded = self.current_stage_path.as_ref()
                    .map(|p| p.to_string_lossy().contains("town01"))
                    .unwrap_or(false);
                if !is_town_loaded && self.current_stage_path.is_some() {
                    println!("Returning to Mac Anu: loading town01_00h.ccs...");
                    self.load_world(Path::new("town01_00h.ccs"), Path::new("2hsw01.ccs"));
                }
            }
            _ => {}
        }

        if self.player.active && !matches!(self.egui_mgr.gui_state.game_runtime.state, hackgu_game::GameState::TitleScreen) {
            let locked = (self.fieldplay.as_ref().map(|f| f.locks_player()).unwrap_or(false) && self.cutscene.is_none()) || self.menu_open();
            let fp_anim = self.fieldplay.as_ref().map(|f| f.drives_player_anim()).unwrap_or(false) && self.cutscene.is_none();
            if locked {
                self.player.update(dt, false, false, false, false, 0.0, 0.0, self.camera.yaw);
            } else {
                self.player.update(dt, self.key_w, self.key_s, self.key_a, self.key_d, gamepad_analog_x, gamepad_analog_y, self.camera.yaw);
            }
            if let (Some(scene), Some(ccs)) = (&mut self.scene, &self.ccs) {
                scene.root_transform = Some(self.player.world_matrix());
                if fp_anim {
                    // Battle stance and swings come from fieldplay (2hsw20 set).
                } else if self.player.is_moving {
                    // Prefer weapon run stance (str00) if equipped, else standard run/walk (run00 / wlk00)
                    let run_idx = scene.animations.iter().position(|a| a.name.contains("str00"))
                        .or_else(|| scene.animations.iter().position(|a| a.name.contains("run00") || a.name.contains("wlk00")));
                    if let Some(idx) = run_idx {
                        if scene.active_anim_idx != Some(idx) {
                            scene.select_animation(idx, &self.queue, ccs);
                        }
                    }
                } else {
                    // Prefer weapon ready stance (rdy00) if equipped, else standard neutral idle (nut00 / idl00)
                    let idle_idx = scene.animations.iter().position(|a| a.name.contains("rdy00"))
                        .or_else(|| scene.animations.iter().position(|a| a.name.contains("nut00") || a.name.contains("idl00")));
                    if let Some(idx) = idle_idx {
                        if scene.active_anim_idx != Some(idx) {
                            let should_switch = scene.active_anim_idx.map(|i| {
                                let n = &scene.animations[i].name;
                                n.contains("run00") || n.contains("wlk00") || n.contains("str00") || n.contains("nut00")
                            }).unwrap_or(true);
                            if should_switch {
                                scene.select_animation(idx, &self.queue, ccs);
                            }
                        }
                    }
                }
            }
            // Field story, fights and followers.
            if let (Some(fp), Some(scene), Some(ccs)) = (&mut self.fieldplay, &mut self.scene, &self.ccs) {
                let in_event = self.cutscene.is_some() || !matches!(self.egui_mgr.gui_state.game_runtime.state, hackgu_game::GameState::Field { .. });
                let mut yaw = self.player.yaw;
                let evs = fp.update(dt, scene, &self.queue, ccs, self.player.position, &mut yaw, self.player.is_moving, self.player.collision_mesh.as_ref(), in_event);
                if yaw != self.player.yaw {
                    self.player.yaw = yaw;
                    scene.root_transform = Some(self.player.world_matrix());
                }
                for ev in evs {
                    match ev {
                        crate::fieldplay::FieldEvent::StartScript(id, frame) => {
                            self.next_event_frame = frame;
                            self.egui_mgr.gui_state.game_runtime.start_script(&id);
                        }
                        crate::fieldplay::FieldEvent::Message(m) => self.egui_mgr.gui_state.status_message = m,
                        crate::fieldplay::FieldEvent::ChestOpened(_, m) => self.egui_mgr.gui_state.status_message = m,
                    }
                }
            }
            // Smoothly track character focus point
            self.camera.target = self.player.position + glam::Vec3::new(0.0, 0.0, FOLLOW_HEIGHT);
            // Keep the camera inside the level: pull the eye in along the boom when geometry is in the way.
            self.camera.clip_distance = None;
            if let Some(col) = &self.player.collision_mesh {
                let full = self.camera.eye_position();
                let clipped = col.clip_camera(self.camera.target, full, 0.3);
                let d = (clipped - self.camera.target).length();
                if d < self.camera.distance - 0.01 {
                    self.camera.clip_distance = Some(d.max(0.4));
                }
            }

            // Tick authentic game simulation (field enemy AI, roaming, encounter proximity)
            self.egui_mgr.gui_state.game_runtime.tick(dt, self.player.position);

            // (Gemini's fake enemy runtime used to move "the last N actors" here; removed: the field's actors
            // are the real chests now, and enemies are not implemented yet.)
            if let hackgu_game::GameState::Field { ref active_encounter, .. } = self.egui_mgr.gui_state.game_runtime.state {
                // Battle BGM switching
                if active_encounter.is_some() {
                    if !self.sound_system.is_bgm_playing() {
                        self.sound_system.play_bgm_named("btl0"); // battle theme (bgm.acb cue "btl0")
                    }
                }
            }
        }

        if let (Some(scene), Some(ccs)) = (&mut self.scene, &self.ccs) {
            scene.update_animation(&self.queue, dt, ccs);
        }

        let auto = crate::auto::get();
        if let Some((eye, target, fov)) = auto.cam {
            self.camera.look_from(glam::Vec3::from(eye), glam::Vec3::from(target));
            if let Some(f) = fov {
                self.camera.fov_y = f.to_radians();
            }
        }

        let uniform = self.camera.to_uniform();
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[uniform]));
    }

    /// Copy the current swapchain texture into a PNG (automation).
    fn capture(&self, tex: &wgpu::Texture, path: &str) {
        let (w, h) = (self.config.width, self.config.height);
        let bpr = (w * 4 + 255) / 256 * 256;
        let buf = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("shot"),
            size: (bpr * h) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let mut enc = self.device.create_command_encoder(&Default::default());
        enc.copy_texture_to_buffer(
            tex.as_image_copy(),
            wgpu::TexelCopyBufferInfo { buffer: &buf, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(bpr), rows_per_image: Some(h) } },
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        );
        self.queue.submit(Some(enc.finish()));
        let slice = buf.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        let _ = self.device.poll(wgpu::Maintain::Wait);
        let data = slice.get_mapped_range();
        let bgra = matches!(self.config.format, wgpu::TextureFormat::Bgra8Unorm | wgpu::TextureFormat::Bgra8UnormSrgb);
        let mut rgba = Vec::with_capacity((w * h * 4) as usize);
        for y in 0..h {
            let row = &data[(y * bpr) as usize..(y * bpr + w * 4) as usize];
            for px in row.chunks_exact(4) {
                if bgra { rgba.extend_from_slice(&[px[2], px[1], px[0], 255]); } else { rgba.extend_from_slice(&[px[0], px[1], px[2], 255]); }
            }
        }
        match crate::auto::write_png(path, w, h, &rgba) {
            Ok(()) => println!("[auto] wrote {} ({}x{})", path, w, h),
            Err(e) => eprintln!("[auto] failed to write {}: {}", path, e),
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // 1. egui UI pass
        let mut raw_input = self.egui_mgr.state.take_egui_input(&self.window);
        for key in std::mem::take(&mut self.pad_keys) {
            for pressed in [true, false] {
                raw_input.events.push(egui::Event::Key { key, physical_key: None, pressed, repeat: false, modifiers: Default::default() });
            }
        }
        for (f, k) in &crate::auto::get().keys {
            if *f == self.frame_no && k == "Menu" {
                // Test hook: the menu button.
                if self.cutscene.is_some() {
                    self.menu_prompt_pressed = true;
                } else {
                    self.toggle_main_menu();
                }
                continue;
            }
            if *f == self.frame_no && k == "Skip" && self.movie_player.is_playing() {
                // Test hook: skip the playing movie as the Space key does.
                self.movie_player.stop();
                self.movie_renderer.clear();
                self.egui_mgr.gui_state.game_runtime.advance_cutscene();
                continue;
            }
            if *f == self.frame_no {
                let key = match k.as_str() {
                    "ArrowDown" | "Down" => Some(egui::Key::ArrowDown),
                    "ArrowUp" | "Up" => Some(egui::Key::ArrowUp),
                    "ArrowLeft" | "Left" => Some(egui::Key::ArrowLeft),
                    "ArrowRight" | "Right" => Some(egui::Key::ArrowRight),
                    other => egui::Key::from_name(other),
                };
                if let Some(key) = key {
                    for pressed in [true, false] {
                        raw_input.events.push(egui::Event::Key { key, physical_key: None, pressed, repeat: false, modifiers: Default::default() });
                    }
                }
            }
        }
        let mut recenter_camera = false;

        let no_ui = crate::auto::get().no_ui;
        let fade = self.fade_black;
        let view_proj = self.camera.build_view_projection_matrix();
        let hud_size = (self.config.width as f32, self.config.height as f32);
        let full_output = self.egui_mgr.context.run(raw_input, |ctx| {
            if fade > 0.001 {
                ctx.layer_painter(egui::LayerId::new(egui::Order::Background, egui::Id::new("ev_fade")))
                    .rect_filled(ctx.screen_rect(), 0.0, egui::Color32::from_black_alpha((fade.clamp(0.0, 1.0) * 255.0) as u8));
            }
            if no_ui {
                return;
            }
            if let Some((both, _)) = &self.reward_panel {
                let (header, item) = both.split_once('|').unwrap_or(("Mission Reward", both));
                // Measured on the original (reference f00860, 960x540).
                let screen = ctx.screen_rect();
                let (w, h) = if screen.width() / screen.height() > 16.0 / 9.0 { (screen.height() * 16.0 / 9.0, screen.height()) } else { (screen.width(), screen.width() * 9.0 / 16.0) };
                let o = egui::pos2(screen.center().x - w * 0.5, screen.center().y - h * 0.5);
                let at = |x: f32, y: f32| egui::pos2(o.x + x / 960.0 * w, o.y + y / 540.0 * h);
                let p = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("reward")));
                let font = egui::FontId::proportional(11.5 / 540.0 * h / 0.70);
                p.rect_filled(egui::Rect::from_min_max(at(380.0, 222.0), at(580.0, 250.0)), 4.0, egui::Color32::from_rgba_unmultiplied(22, 20, 26, 230));
                p.text(at(480.0, 236.0), egui::Align2::CENTER_CENTER, header, font.clone(), egui::Color32::from_rgb(200, 200, 205));
                p.rect_filled(egui::Rect::from_min_max(at(248.0, 256.0), at(712.0, 296.0)), 6.0, egui::Color32::from_rgba_unmultiplied(40, 14, 18, 230));
                p.text(at(480.0, 276.0), egui::Align2::CENTER_CENTER, item, font, egui::Color32::from_rgb(232, 202, 92));
            }
            if let Some(fp) = &mut self.fieldplay {
                // The field HUD is hidden while the Main Menu is open (run1_opening f00064: only the menu shows).
                if self.cutscene.is_none() && !self.egui_mgr.gui_state.main_menu_open && matches!(self.egui_mgr.gui_state.game_runtime.state, hackgu_game::GameState::Field { .. }) {
                    fp.draw_minimap(ctx, self.player.position, self.player.yaw);
                    let ppp = ctx.pixels_per_point();
                    let project = |p: glam::Vec3| -> Option<egui::Pos2> {
                        let c = view_proj * p.extend(1.0);
                        if c.w <= 0.01 {
                            return None;
                        }
                        let n = c.truncate() / c.w;
                        Some(egui::Pos2::new((n.x * 0.5 + 0.5) * hud_size.0 / ppp, (0.5 - n.y * 0.5) * hud_size.1 / ppp))
                    };
                    fp.draw_hud(ctx, &project);
                }
            }
            self.egui_mgr.gui_state.draw_ui(
                ctx,
                self.scene.as_mut(),
                self.ccs.as_ref(),
                &self.queue,
                &mut self.show_background,
                &mut recenter_camera,
                self.current_ccs_path.as_deref(),
            );
        });

        if recenter_camera {
            if let Some(scene) = &self.scene {
                if self.show_background {
                    self.camera.focus_bounds(scene.min_bounds, scene.max_bounds);
                } else {
                    self.camera.focus_bounds(scene.fg_min_bounds, scene.fg_max_bounds);
                }
            }
        }

        self.egui_mgr.state.handle_platform_output(&self.window, full_output.platform_output);

        let clipped_primitives = self.egui_mgr.context.tessellate(full_output.shapes, full_output.pixels_per_point);

        for (id, delta) in &full_output.textures_delta.set {
            self.egui_mgr.renderer.update_texture(&self.device, &self.queue, *id, delta);
        }

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [self.config.width, self.config.height],
            pixels_per_point: full_output.pixels_per_point,
        };

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("render_encoder"),
        });

        self.egui_mgr.renderer.update_buffers(
            &self.device,
            &self.queue,
            &mut encoder,
            &clipped_primitives,
            &screen_descriptor,
        );

        // 2. 3D Scene pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("3d_render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.postfx.scene_view,
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
                    view: &self.depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

            if let Some(scene) = &self.scene {
                // Opaque first, then decals, then additive layers (neither blended pass writes depth, so stacked
                // coplanar layers such as Mac Anu's water no longer fight each other or the ground).
                if self.show_background {
                    let mut bg: Vec<&crate::mesh::GpuSubMesh> = scene.submeshes.iter().filter(|s| s.bg_layer > 0 && s.visible).collect();
                    bg.sort_by_key(|s| s.bg_layer);
                    for submesh in bg {
                        render_pass.set_pipeline(if submesh.blend == 1 { &self.sky_additive_pipeline } else { &self.sky_pipeline });
                        let tex = scene.materials.get(&submesh.material_index).unwrap_or(&scene.default_texture);
                        render_pass.set_bind_group(1, &tex.bind_group, &[]);
                        render_pass.set_vertex_buffer(0, submesh.vertex_buffer.slice(..));
                        render_pass.set_index_buffer(submesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                        render_pass.draw_indexed(0..submesh.index_count, 0, 0..1);
                    }
                }
                for (pass, pipeline) in [(0u8, &self.render_pipeline), (2, &self.decal_pipeline), (1, &self.additive_pipeline)] {
                    render_pass.set_pipeline(pipeline);
                    for submesh in scene.submeshes.iter().filter(|s| s.blend == pass && s.bg_layer == 0 && s.visible) {
                        if submesh.is_background && !self.show_background {
                            continue;
                        }
                        let tex = scene
                            .materials
                            .get(&submesh.material_index)
                            .unwrap_or(&scene.default_texture);
                        render_pass.set_bind_group(1, &tex.bind_group, &[]);
                        render_pass.set_vertex_buffer(0, submesh.vertex_buffer.slice(..));
                        render_pass.set_index_buffer(submesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                        render_pass.draw_indexed(0..submesh.index_count, 0, 0..1);
                    }
                }
            }
        }

        // 2a. Post filter: scene texture -> swapchain (gamma-space view).
        {
            let ui_view = output.texture.create_view(&wgpu::TextureViewDescriptor { format: Some(self.config.format.remove_srgb_suffix()), ..Default::default() });
            self.postfx.draw(&self.queue, &mut encoder, &ui_view);
        }

        // 2b. In-window CRI Sofdec Movie playback pass (fullscreen quad). During a story movie the screen is cleared
        // to black first: the letterbox bands stay black and nothing of the scene behind shows before the first frame.
        // Also black for the frame between a movie's end and its event's start (the event is started in update; the
        // player's own view would flash for one frame otherwise).
        let movie_state = (matches!(self.egui_mgr.gui_state.game_runtime.state, hackgu_game::GameState::Movie { .. }) && !self.egui_mgr.gui_state.show_movie_theater)
            || (matches!(self.egui_mgr.gui_state.game_runtime.state, hackgu_game::GameState::Cutscene { .. }) && self.cutscene.is_none());
        if self.movie_renderer.has_frame() || movie_state {
            let mut movie_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("moviefx_render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: if movie_state { wgpu::LoadOp::Clear(wgpu::Color::BLACK) } else { wgpu::LoadOp::Load },
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            if self.movie_renderer.has_frame() {
                self.movie_renderer.draw(&mut movie_pass);
            }
        }

        // 3. egui UI rendering pass (overlay over 3D), through the gamma-space view.
        {
            let ui_view = output.texture.create_view(&wgpu::TextureViewDescriptor { format: Some(self.config.format.remove_srgb_suffix()), ..Default::default() });
            let mut egui_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui_render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &ui_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            }).forget_lifetime();

            self.egui_mgr.renderer.render(&mut egui_pass, &clipped_primitives, &screen_descriptor);
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        for id in &full_output.textures_delta.free {
            self.egui_mgr.renderer.free_texture(id);
        }

        self.frame_no += 1;
        if std::env::var_os("HACKGU_TRACE_PLAYER").is_some() && self.frame_no % 30 == 1 {
            println!("[trace] frame {} player {:?} yaw {:.2} active {} cam eye {:?}", self.frame_no, self.player.position, self.player.yaw, self.player.active, self.camera.eye_position());
            if let Some(col) = &self.player.collision_mesh {
                let mut c = Camera::new(1.0, 1.0);
                c.target = self.camera.target; c.yaw = self.camera.yaw; c.pitch = self.camera.pitch; c.distance = self.camera.distance;
                println!("[trace] camera ray hit {:?}", col.first_hit(self.camera.target, c.eye_position()));
            }
        }
        let shots = &crate::auto::get().shots;
        for (path, f) in shots {
            if *f == self.frame_no {
                self.capture(&output.texture, path);
            }
        }
        if let Some((dir, count)) = &crate::auto::get().record {
            if self.frame_no <= *count {
                let _ = std::fs::create_dir_all(dir);
                let path = format!("{}/frame_{:05}.png", dir, self.frame_no);
                self.capture(&output.texture, &path);
            }
            if self.frame_no >= *count {
                println!("[record] finished recording {} frames to {}", count, dir);
                std::process::exit(0);
            }
        }
        output.present();
        if let Some(last) = shots.last() {
            if self.frame_no >= last.1 {
                println!("[auto] done");
                std::process::exit(0);
            }
        }
        Ok(())
    }
}

pub struct App {
    pub ccs_path: Option<PathBuf>,
    pub explore_mode: bool,
    pub state: Option<RenderState>,
}

impl App {
    pub fn new(ccs_path: Option<PathBuf>, explore_mode: bool) -> Self {
        Self {
            ccs_path,
            explore_mode,
            state: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_none() {
            let window_title = match &self.ccs_path {
                Some(p) => format!(".hack//G.U. Model Viewer - {}", p.file_name().unwrap_or_default().to_string_lossy()),
                None => ".hack//G.U. Vol. 1//Rebirth".to_string(),
            };

            let (win_w, win_h) = crate::auto::get().res.unwrap_or((1280, 800));
            let window = Arc::new(
                event_loop
                    .create_window(
                        WindowAttributes::default()
                            .with_title(window_title)
                            .with_inner_size(winit::dpi::LogicalSize::new(win_w as f64, win_h as f64)),
                    )
                    .unwrap(),
            );

            let mut state = RenderState::new(window.clone(), self.ccs_path.clone());
            if self.explore_mode {
                state.load_world(
                    Path::new("target/vol1/data/data/town/town01_00h.ccs"),
                    Path::new("VRChat/work/haseo/raw/2hsw01.ccs"),
                );
            }
            let auto = crate::auto::get();
            if auto.town {
                state.egui_mgr.gui_state.game_runtime.login_to_the_world();
                if let hackgu_game::GameState::Town { ref mut active_dialogue, .. } = state.egui_mgr.gui_state.game_runtime.state {
                    *active_dialogue = None;
                }
            }
            // Debug: HACKGU_EVENTS_DONE=PM10100,PM10310 seeds the played-events list (arms the town events after them).
            if let Ok(list) = std::env::var("HACKGU_EVENTS_DONE") {
                state.egui_mgr.gui_state.game_runtime.events_done = list.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            }
            if let Some(st) = &auto.stage {
                state.load_world(Path::new(st), Path::new("2hsw01.ccs"));
            }
            if let Some((path, floor)) = &auto.dungeon {
                if let Some(d) = crate::dungeon::load_file(path) {
                    state.egui_mgr.gui_state.game_runtime.warp_to_selected_area();
                    if let hackgu_game::GameState::Field { ref mut enemies, ref mut active_dialogue, .. } = state.egui_mgr.gui_state.game_runtime.state {
                        enemies.clear();
                        *active_dialogue = None;
                    }
                    let kit = d.kit.clone();
                    crate::dungeon::set_current(d, *floor);
                    state.load_world(Path::new(&kit), Path::new("2hsw01.ccs"));
                }
            }
            if !auto.gate.is_empty() {
                state.egui_mgr.gui_state.game_runtime.login_to_the_world();
                state.egui_mgr.gui_state.game_runtime.open_chaos_gate();
            }
            if auto.desktop.is_some() {
                state.egui_mgr.gui_state.game_runtime.to_desktop();
            }
            if auto.reboot {
                state.egui_mgr.gui_state.game_runtime.to_desktop();
                state.egui_mgr.gui_state.game_runtime.pending_desktop_sequence = Some("reboot".into());
            }
            if let Some(ev) = &auto.ev {
                state.start_event(ev);
            }
            if auto.movie.is_some() {
                // The story's New Game path: OP10000.usm, then PM10100.
                state.egui_mgr.gui_state.game_runtime.start_prologue_movie();
            }
            if let Some(a) = auto.area.as_ref().and_then(|f| crate::field::load_file(f)) {
                crate::field::set_area(a);
            }
            if auto.field {
                state.egui_mgr.gui_state.game_runtime.warp_to_selected_area();
                if let hackgu_game::GameState::Field { ref mut enemies, ref mut active_dialogue, .. } = state.egui_mgr.gui_state.game_runtime.state {
                    enemies.clear();
                    *active_dialogue = None;
                }
                { let set = crate::field::area().kit.set.clone(); state.load_world(Path::new(&set), Path::new("2hsw01.ccs")); }
            }
            if let Some((p, yaw)) = auto.pos {
                state.player.position = glam::Vec3::from(p);
                if let Some(y) = yaw {
                    state.player.yaw = y.to_radians();
                }
                state.reset_follow_camera();
            }
            if !auto.hold.is_empty() {
                state.key_w = auto.hold.contains('w');
                state.key_a = auto.hold.contains('a');
                state.key_s = auto.hold.contains('s');
                state.key_d = auto.hold.contains('d');
            }
            window.request_redraw();
            self.state = Some(state);
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = &self.state {
            state.window.request_redraw();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        let state = match &mut self.state {
            Some(s) => s,
            None => return,
        };

        // Directly handle file drop & drag-hover events
        match &event {
            WindowEvent::DroppedFile(path) => {
                println!("Dropped file: {}", path.display());
                state.load_ccs(path);
                state.window.request_redraw();
            }
            WindowEvent::HoveredFile(path) => {
                state.egui_mgr.gui_state.status_message = format!("Hovering: {}", path.file_name().unwrap_or_default().to_string_lossy());
                state.window.request_redraw();
            }
            WindowEvent::HoveredFileCancelled => {
                state.window.request_redraw();
            }
            _ => {}
        }

        // Let egui process events first; if consumed by egui UI, avoid 3D camera controls
        if state.egui_mgr.on_window_event(&state.window, &event) {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                state.resize(physical_size);
            }
            WindowEvent::MouseInput { state: button_state, button, .. } => {
                if button == MouseButton::Left {
                    state.mouse_pressed = button_state == ElementState::Pressed;
                } else if button == MouseButton::Right || button == MouseButton::Middle {
                    state.right_mouse_pressed = button_state == ElementState::Pressed;
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                if let Some((last_x, last_y)) = state.last_mouse_pos {
                    let dx = (position.x - last_x) as f32;
                    let dy = (position.y - last_y) as f32;

                    if state.mouse_pressed {
                        state.camera.orbit(dx, dy);
                        if state.player.active {
                            state.camera.pitch = state.camera.pitch.clamp(-0.35, 1.2);
                        }
                    } else if state.right_mouse_pressed {
                        state.camera.pan(dx, dy);
                    }
                }
                state.last_mouse_pos = Some((position.x, position.y));
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let y = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(pos) => pos.y as f32 * 0.1,
                };
                state.camera.zoom(y);
                if state.player.active {
                    state.camera.distance = state.camera.distance.clamp(1.2, 8.0);
                }
            }
            WindowEvent::KeyboardInput { event: key_event, .. } => {
                let pressed = key_event.state == ElementState::Pressed;
                match key_event.physical_key {
                    PhysicalKey::Code(KeyCode::KeyW) => state.key_w = pressed,
                    PhysicalKey::Code(KeyCode::KeyS) => state.key_s = pressed,
                    PhysicalKey::Code(KeyCode::KeyA) => state.key_a = pressed,
                    PhysicalKey::Code(KeyCode::KeyD) => state.key_d = pressed,
                    _ => {}
                }
                if pressed {
                    match key_event.physical_key {
                        PhysicalKey::Code(KeyCode::Tab) => {
                            state.player.active = !state.player.active;
                            println!(
                                "Player Controller Mode: {}",
                                if state.player.active {
                                    "ENABLED (Use W/A/S/D to run around world)"
                                } else {
                                    "DISABLED (Free Orbit Camera)"
                                }
                            );
                            state.egui_mgr.gui_state.player_mode = state.player.active;
                            if !state.player.active {
                                if let (Some(scene), Some(ccs)) = (&mut state.scene, &state.ccs) {
                                    scene.root_transform = None;
                                    scene.apply_current_pose(&state.queue, ccs);
                                }
                            }
                        }
                        PhysicalKey::Code(KeyCode::KeyB) => {
                            state.show_background = !state.show_background;
                            println!(
                                "Background sky/silhouettes: {}",
                                if state.show_background { "ENABLED" } else { "DISABLED" }
                            );
                            if let Some(scene) = &state.scene {
                                if state.show_background {
                                    state.camera.focus_bounds(scene.min_bounds, scene.max_bounds);
                                } else {
                                    state.camera.focus_bounds(scene.fg_min_bounds, scene.fg_max_bounds);
                                }
                            }
                        }
                        PhysicalKey::Code(KeyCode::Space) | PhysicalKey::Code(KeyCode::KeyF) | PhysicalKey::Code(KeyCode::Enter) => {
                            if state.movie_player.is_playing() {
                                state.movie_player.stop();
                                state.movie_renderer.clear();
                                state.egui_mgr.gui_state.game_runtime.advance_cutscene();
                            } else if let Some(cs) = &mut state.cutscene {
                                cs.player.press_advance();
                            } else if matches!(state.egui_mgr.gui_state.game_runtime.state, hackgu_game::GameState::Cutscene { .. } | hackgu_game::GameState::Movie { .. }) {
                                state.egui_mgr.gui_state.game_runtime.advance_cutscene();
                            } else if state.player.active && state.dungeon_chest_event_ready() {
                                state.open_dungeon_chest();
                            } else if state.player.active && state.temple_interior && state.near_offering_chest() {
                                // The offering chest: in the original, opening it starts the PK ambush (MM10145).
                                state.open_chest(0);
                                state.show_mission_reward();
                                state.pending_movie = crate::field::area().events.statue_movie.clone().map(|m| (format!("{}.usm", m), 3.0));
                            } else if state.player.active && state.fieldplay.is_some() && state.egui_mgr.gui_state.game_runtime.is_in_field() {
                                let (pos, yaw) = (state.player.position, state.player.yaw);
                                match state.fieldplay.as_mut().unwrap().press_a(pos, yaw) {
                                    Some(crate::fieldplay::FieldEvent::Message(m)) => state.egui_mgr.gui_state.status_message = m,
                                    Some(crate::fieldplay::FieldEvent::ChestOpened(k, m)) => {
                                        state.open_chest(k);
                                        state.egui_mgr.gui_state.status_message = m;
                                    }
                                    _ => {}
                                }
                            } else if state.player.active {
                                if state.egui_mgr.gui_state.game_runtime.is_in_combat() {
                                    let outcome = state.egui_mgr.gui_state.game_runtime.combat_attack();
                                    println!("⚔ Combat Attack: {:?}", outcome);
                                    if let (Some(scene), Some(ccs)) = (&mut state.scene, &state.ccs) {
                                        if let Some(atk_idx) = scene.animations.iter().position(|a| a.name.contains("str00") || a.name.contains("gma")) {
                                            scene.select_animation(atk_idx, &state.queue, ccs);
                                        }
                                    }
                                } else if state.egui_mgr.gui_state.game_runtime.is_in_field() {
                                    let player_pos = state.player.position;
                                    let mut nearest_enemy = None;
                                    if let hackgu_game::GameState::Field { ref enemies, .. } = state.egui_mgr.gui_state.game_runtime.state {
                                        for (idx, e) in enemies.iter().enumerate() {
                                            if (e.position - player_pos).length() <= 4.5 && e.stats.current_hp > 0 {
                                                nearest_enemy = Some(idx);
                                                break;
                                            }
                                        }
                                    }
                                    if let Some(e_idx) = nearest_enemy {
                                        state.egui_mgr.gui_state.game_runtime.surprise_attack(e_idx);
                                        println!("💥 Surprise Attack Triggered!");
                                    } else if let Some(msg) = state.egui_mgr.gui_state.game_runtime.interact_nearest(player_pos) {
                                        state.egui_mgr.gui_state.status_message = msg;
                                    }
                                }
                            } else if let Some(scene) = &state.scene {
                                if state.show_background {
                                    state.camera.focus_bounds(scene.min_bounds, scene.max_bounds);
                                } else {
                                    state.camera.focus_bounds(scene.fg_min_bounds, scene.fg_max_bounds);
                                }
                            }
                        }
                        PhysicalKey::Code(KeyCode::KeyQ) => {
                            if state.player.active && state.egui_mgr.gui_state.game_runtime.is_in_combat() {
                                let outcome = state.egui_mgr.gui_state.game_runtime.combat_rengeki();
                                println!("💥 RENGEKI FINISH: {:?}", outcome);
                                if let (Some(scene), Some(ccs)) = (&mut state.scene, &state.ccs) {
                                    if let Some(atk_idx) = scene.animations.iter().position(|a| a.name.contains("mag") || a.name.contains("str00")) {
                                        scene.select_animation(atk_idx, &state.queue, ccs);
                                    }
                                }
                            }
                        }
                        PhysicalKey::Code(KeyCode::KeyE) => {
                            state.menu_prompt_pressed = state.cutscene.is_some();
                            if state.player.active {
                                let player_pos = state.player.position;
                                match &mut state.egui_mgr.gui_state.game_runtime.state {
                                    hackgu_game::GameState::Town { .. } => {
                                        if near_chaos_gate(player_pos) {
                                            state.egui_mgr.gui_state.game_runtime.open_chaos_gate();
                                        }
                                    }
                                    hackgu_game::GameState::Field { .. } => {
                                        if let Some(msg) = state.egui_mgr.gui_state.game_runtime.interact_nearest(player_pos) {
                                            state.egui_mgr.gui_state.status_message = msg;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        PhysicalKey::Code(KeyCode::KeyH) => {
                            if state.player.active && state.egui_mgr.gui_state.game_runtime.use_item("Health Drink") {
                                state.egui_mgr.gui_state.status_message = format!("Used Health Drink! HP: {}/{}", state.egui_mgr.gui_state.game_runtime.save.hp, state.egui_mgr.gui_state.game_runtime.save.max_hp);
                            }
                        }
                        PhysicalKey::Code(KeyCode::KeyM) => {
                            state.egui_mgr.gui_state.show_story_window = !state.egui_mgr.gui_state.show_story_window;
                        }
                        PhysicalKey::Code(KeyCode::F1) => {
                            state.egui_mgr.gui_state.show_sidebar = !state.egui_mgr.gui_state.show_sidebar;
                        }
                        PhysicalKey::Code(KeyCode::Digit1) => {
                            if state.player.active {
                                state.egui_mgr.gui_state.pending_equip = Some("twin_blades".into());
                            } else if let (Some(scene), Some(ccs)) = (&mut state.scene, &state.ccs) {
                                scene.select_animation(0, &state.queue, ccs);
                            }
                        }
                        PhysicalKey::Code(KeyCode::Digit2) => {
                            if state.player.active {
                                state.egui_mgr.gui_state.pending_equip = Some("broadsword".into());
                            } else if let (Some(scene), Some(ccs)) = (&mut state.scene, &state.ccs) {
                                scene.select_animation(1, &state.queue, ccs);
                            }
                        }
                        PhysicalKey::Code(KeyCode::Digit3) => {
                            if state.player.active {
                                state.egui_mgr.gui_state.pending_equip = Some("scythe".into());
                            } else if let (Some(scene), Some(ccs)) = (&mut state.scene, &state.ccs) {
                                scene.select_animation(2, &state.queue, ccs);
                            }
                        }
                        PhysicalKey::Code(KeyCode::Digit4) => {
                            if state.player.active {
                                state.egui_mgr.gui_state.pending_equip = Some("guns".into());
                            } else if let (Some(scene), Some(ccs)) = (&mut state.scene, &state.ccs) {
                                scene.select_animation(3, &state.queue, ccs);
                            }
                        }
                        PhysicalKey::Code(KeyCode::Digit5) => {
                            if let (Some(scene), Some(ccs)) = (&mut state.scene, &state.ccs) {
                                scene.select_animation(4, &state.queue, ccs);
                            }
                        }
                        PhysicalKey::Code(KeyCode::BracketLeft) => {
                            if let (Some(scene), Some(ccs)) = (&mut state.scene, &state.ccs) {
                                scene.cycle_animation(false, &state.queue, ccs);
                            }
                        }
                        PhysicalKey::Code(KeyCode::BracketRight) => {
                            if let (Some(scene), Some(ccs)) = (&mut state.scene, &state.ccs) {
                                scene.cycle_animation(true, &state.queue, ccs);
                            }
                        }
                        PhysicalKey::Code(KeyCode::KeyP) => {
                            if let Some(scene) = &mut state.scene {
                                scene.is_playing = !scene.is_playing;
                                println!(
                                    "Animation playback: {}",
                                    if scene.is_playing { "PLAYING" } else { "PAUSED" }
                                );
                            }
                        }
                        PhysicalKey::Code(KeyCode::Comma) => {
                            if let (Some(scene), Some(ccs)) = (&mut state.scene, &state.ccs) {
                                scene.step_frame(-1.0, &state.queue, ccs);
                            }
                        }
                        PhysicalKey::Code(KeyCode::Period) => {
                            if let (Some(scene), Some(ccs)) = (&mut state.scene, &state.ccs) {
                                scene.step_frame(1.0, &state.queue, ccs);
                            }
                        }
                        PhysicalKey::Code(KeyCode::Escape) if state.cutscene.as_ref().is_some_and(|c| c.player.state.message.as_ref().is_some_and(|m| m.text.contains("<iconTRIANGLE>"))) => {
                            state.menu_prompt_pressed = true;
                        }
                        PhysicalKey::Code(KeyCode::Escape) => {
                            if state.movie_player.is_playing() {
                                state.movie_player.stop();
                                state.movie_renderer.clear();
                                state.egui_mgr.gui_state.game_runtime.skip_cutscene();
                            } else if state.cutscene.is_some() {
                                state.end_cutscene(true);
                            } else if matches!(state.egui_mgr.gui_state.game_runtime.state, hackgu_game::GameState::Cutscene { .. } | hackgu_game::GameState::Movie { .. }) {
                                state.sound_system.stop_voice();
                                state.egui_mgr.gui_state.game_runtime.skip_cutscene();
                            } else if state.egui_mgr.gui_state.show_movie_theater {
                                state.egui_mgr.gui_state.show_movie_theater = false;
                            } else if matches!(state.egui_mgr.gui_state.game_runtime.state, hackgu_game::GameState::Town { active_dialogue: Some(_), .. } | hackgu_game::GameState::Field { active_dialogue: Some(_), .. }) {
                                state.egui_mgr.gui_state.game_runtime.close_popup();
                            } else if state.egui_mgr.gui_state.main_menu_open {
                                // The open Main Menu handles Esc itself (Return).
                            } else if matches!(state.egui_mgr.gui_state.game_runtime.state, hackgu_game::GameState::Town { .. } | hackgu_game::GameState::Field { .. }) {
                                state.toggle_main_menu();
                            } else if !matches!(state.egui_mgr.gui_state.game_runtime.state, hackgu_game::GameState::TitleScreen) {
                                // Desktop, Chaos Gate...: their screens use Esc as "Return" themselves.
                            } else if state
                                .egui_mgr
                                .gui_state
                                .title
                                .as_ref()
                                .map(|t| matches!(t.state, crate::title::TitleState::VolumeSelect { .. }))
                                .unwrap_or(true)
                            {
                                // Top level of the title ("B: Quit Game"); inside its submenus Esc goes back instead.
                                event_loop.exit();
                            }
                        }
                        _ => {}
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = state.window.inner_size();
                        state.resize(size);
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        event_loop.exit();
                    }
                    Err(e) => eprintln!("Render error: {:?}", e),
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gui::PRESETS;

    #[test]
    fn test_all_presets_resolve() {
        for preset in PRESETS {
            let res = resolve_ccs_path(Path::new(preset.path));
            assert!(
                res.is_some(),
                "Preset '{}' path '{}' failed to resolve!",
                preset.name,
                preset.path
            );
        }
    }

    #[test]
    fn test_css_extension_resolution() {
        // Test that requesting a .css file resolves to existing .ccs
        let res = resolve_ccs_path(Path::new("town01_01h.css"));
        assert!(res.is_some(), "Failed to resolve town01_01h.css to town01_01h.ccs");
    }

    #[test]
    fn test_inspect_2hsw02_models() {
        let p = resolve_ccs_path(Path::new("2hsw02.ccs")).expect("2hsw02.ccs should resolve");
        let data = std::fs::read(&p).unwrap();
        let ccs = hackgu_ccs::CcsFile::parse(&data).unwrap();
        let clumps = ccs.clumps();
        let models = ccs.objects_of_type(hackgu_ccs::objects::SECTION_MODEL);
        println!("2hsw02 clumps: {}, models: {}", clumps.len(), models.len());
        let anims = ccs.animations();
        println!("2hsw02 animations count: {}", anims.len());
        for (i, a) in anims.iter().enumerate() {
            println!("  anim[{}]: name={:?}, frames={}", i, a.name, a.frame_count);
        }
        let texs = ccs.objects_of_type(hackgu_ccs::objects::SECTION_TEXTURE);
        println!("2hsw02 textures count: {}", texs.len());
        for t in &texs {
            let tname = ccs.object_name(t.object_id).unwrap_or("unknown");
            println!("  tex id={}, name={:?}, payload_len={}", t.object_id, tname, t.payload.len());
            let clut_raw = if t.payload.len() >= 4 {
                let clut_id = i32::from_le_bytes(t.payload[0..4].try_into().unwrap());
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
            match hackgu_ccs::DecodedTexture::from_payload_with_clut(tname, &t.payload, clut_raw) {
                Ok(dt) => println!("    Decoded texture: {}x{}, format={:?}, rgba_len={}", dt.width, dt.height, dt.format, dt.rgba.len()),
                Err(e) => println!("    FAILED TO DECODE TEXTURE: {:?}", e),
            }
        }
        let mut min_b = glam::Vec3::splat(f32::INFINITY);
        let mut max_b = glam::Vec3::splat(f32::NEG_INFINITY);
        for m in &models {
            let name = ccs.object_name(m.object_id).unwrap_or("unknown");
            if let Ok(mut dm) = hackgu_ccs::DecodedModel::from_payload(name, &m.payload, ccs.header.raw_version) {
                if dm.model_type == 8 || name.to_lowercase().contains("shadow") {
                    continue;
                }
                if let Some(c) = clumps.first() {
                    let wm = c.compute_world_matrices(&ccs.scene_nodes());
                    dm.apply_skeleton(c, &wm);
                }
                for mesh in &dm.meshes {
                    for v in &mesh.vertices {
                        let p = glam::Vec3::from(v.position);
                        min_b = min_b.min(p);
                        max_b = max_b.max(p);
                    }
                }
            }
        }
        println!("2hsw02 overall bounds: min={:?}, max={:?}", min_b, max_b);
    }

    #[test]
    fn test_inspect_dungeon2() {
        if let Some(p) = resolve_ccs_path(Path::new("dungeon2.ccs")) {
            let data = std::fs::read(&p).unwrap();
            let ccs = hackgu_ccs::CcsFile::parse(&data).unwrap();
            let clumps = ccs.clumps();
            let models = ccs.objects_of_type(hackgu_ccs::objects::SECTION_MODEL);
            assert_eq!(clumps.len(), 230);
            assert_eq!(models.len(), 257);
        }
    }

    #[test]
    fn test_prologue_cutscene_actors_resolve() {
        let actors = ["2iyo01.ccs", "2ast01.ccs", "2ova01.ccs"];
        for actor_file in &actors {
            let path = resolve_ccs_path(Path::new(actor_file))
                .unwrap_or_else(|| panic!("Actor model {} must resolve in asset directories", actor_file));
            let data = std::fs::read(&path).expect("Failed to read actor CCS data");
            let ccs = hackgu_ccs::CcsFile::parse(&data).expect("Failed to parse actor CCS");
            assert!(!ccs.clumps().is_empty(), "Actor {} should have skeletal clump", actor_file);
            assert!(!ccs.objects_of_type(hackgu_ccs::objects::SECTION_MODEL).is_empty(), "Actor {} should have 3D models", actor_file);
            println!("Verified Prologue actor {}: {} clumps, {} models", actor_file, ccs.clumps().len(), ccs.objects_of_type(hackgu_ccs::objects::SECTION_MODEL).len());
        }
    }
}

/// Mac Anu's Chaos Gate stands at DMY_gate (0, 228.5, 12) m (town01.ccs); the gate prompt opens within 8 m of it.
fn near_chaos_gate(p: glam::Vec3) -> bool {
    glam::Vec2::new(p.x, p.y - 228.5).length() <= 8.0 && (p.z - 12.0).abs() < 4.0
}

/// A stage prop driven by its file's animation.
pub struct FieldProp {
    /// Play the animation once and hold its last frame (chests); otherwise loop.
    pub once: bool,
    pub idx: usize,
    pub pos: glam::Vec3,
    pub yaw: f32,
    pub ccs: std::sync::Arc<hackgu_ccs::CcsFile>,
    pub anim: Option<hackgu_ccs::CcsAnimation>,
    pub t: f32,
}

/// Walkable floor from the deck mesh of a bridge end piece (sf3_00 MDL_ob00, mesh 0), placed like the piece.
fn bridge_ramp_floor(kit: &hackgu_ccs::CcsFile, pos: glam::Vec3, yaw: f32) -> hackgu_world::CollisionMesh {
    let mut tris = Vec::new();
    let (sy, cy) = yaw.sin_cos();
    for m in kit.objects_of_type(hackgu_ccs::objects::SECTION_MODEL) {
        if kit.object_name(m.object_id) != Some("MDL_ob00") {
            continue;
        }
        let Ok(dm) = hackgu_ccs::DecodedModel::from_payload("MDL_ob00", &m.payload, kit.header.raw_version) else { continue };
        let Some(mesh) = dm.meshes.first() else { continue };
        let place = |v: [f32; 3]| pos + glam::Vec3::new(cy * v[0] - sy * v[1], sy * v[0] + cy * v[1], v[2]);
        for t in mesh.indices.chunks_exact(3) {
            let g = |i: u32| mesh.vertices.get(i as usize).map(|v| place(v.local_positions[0]));
            if let (Some(a), Some(b), Some(c)) = (g(t[0]), g(t[1]), g(t[2])) {
                if let Some(mut tri) = hackgu_world::collision::CollisionTriangle::new(a, b, c) {
                    if tri.normal.z < 0.0 {
                        tri.normal = -tri.normal;
                    }
                    tri.is_walkable = tri.normal.z > 0.5;
                    tris.push(tri);
                }
            }
        }
    }
    hackgu_world::CollisionMesh::from_triangles(tris, 4.0)
}

/// Walkable triangles from a model's floor meshes (material texture TEX_fl*), placed by `m` (our Mat4, metres).
fn visible_floor(kit: &hackgu_ccs::CcsFile, model: &str, m: &hackgu_ccs::Mat4) -> hackgu_world::CollisionMesh {
    let mut tris = Vec::new();
    let mats = kit.materials();
    for obj in kit.objects_of_type(hackgu_ccs::objects::SECTION_MODEL) {
        if kit.object_name(obj.object_id) != Some(model) {
            continue;
        }
        let Ok(dm) = hackgu_ccs::DecodedModel::from_payload(model, &obj.payload, kit.header.raw_version) else { continue };
        for mesh in &dm.meshes {
            let tex = mats.get(&mesh.material_index).and_then(|t| kit.object_name(t.texture_obj_id)).unwrap_or("");
            if !tex.starts_with("TEX_fl") {
                continue;
            }
            for t in mesh.indices.chunks_exact(3) {
                let g = |i: u32| mesh.vertices.get(i as usize).map(|v| glam::Vec3::from(m.transform_point(v.local_positions[0])));
                if let (Some(a), Some(b), Some(c)) = (g(t[0]), g(t[1]), g(t[2])) {
                    if let Some(mut tri) = hackgu_world::collision::CollisionTriangle::new(a, b, c) {
                        if tri.normal.z < 0.0 {
                            tri.normal = -tri.normal;
                        }
                        tri.is_walkable = tri.normal.z > 0.5;
                        if tri.is_walkable {
                            tris.push(tri);
                        }
                    }
                }
            }
        }
    }
    hackgu_world::CollisionMesh::from_triangles(tris, 4.0)
}
