//! Last Recode's screen post filter, from `cmn/post_filter.bin` and the game's own shaders.
//!
//! post_filter.bin is a BLBN symbol blob with one table per scene type (`gu::<type>::s_post_filter_param`, 132-byte
//! records of 33 i32). hackGU_vol1.dll applies a record in FUN_1801820b0 (traced 2026-10-11):
//!   w0..w4   depth effect (enable, w1/100, near w2, far w3, w4/100)        -- not implemented here
//!   w5..w12  screen flare A: enable, blend type (0..4), outer flag, falloff w8/100, colour w9..w12 /255
//!   w13..w20 screen flare B: same layout
//!   w21..w26 screen flare C: enable, blend type, colour (no falloff: full screen)
//!   w27..w30 HSL pass: enable, hue + w28/100, saturation * w29/100, lightness * w30/100
//!   w31      contrastParam.x = w31/100;  w32 -> another factor (/100), not identified
//! Shaders (DXBC in hackGU.exe, disassembled with the game's d3dcompiler_47):
//!   screen flare (0x3813e8): a = colour.a * falloff, falloff = sat(1 - d*k) (inner) or sat(d*k - 1) (outer flag),
//!     d = distance from the flare position with x scaled by 16/9; blend 0 normal, 1 add, 2 subtract,
//!     3 multiply S(1-a)+S*C*a, 4 screen S+C*a-S*C*a.
//!   HSL (0x35c368): RGB->HSL, h+=x, s*=y, l*=z, HSL->RGB; then sat(c*(rgb + 0 - 0.5) + 0.5).
//! Which record: DLL table at RVA 0x79b698 maps town id 0..4 -> record 3,4,2,0,1 (town id 0 with a flag -> 5).
//! Flare positions are written as (0, 0) when a record is applied; the flare's screen coordinate space (assumed
//! -1..1, centre = 0) was not traced.

use wgpu::util::DeviceExt;

#[derive(Clone, Copy, Debug, Default)]
pub struct Flare {
    pub enabled: bool,
    pub blend: i32,
    pub outer: bool,
    pub falloff: f32,
    pub color: [f32; 4],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PostFilter {
    pub flares: [Flare; 3],
    pub hsl_enabled: bool,
    pub hsl: [f32; 3],
    pub contrast: f32,
}

/// Scene types in post_filter.bin, in the DLL's order.
pub const KINDS: [&str; 7] = ["field", "dungeon", "lostground", "town", "avatar", "arena", "puppet"];

/// Read record `index` of `kind` from post_filter.bin.
pub fn load(path: &std::path::Path, kind: &str, index: usize) -> Option<PostFilter> {
    let d = std::fs::read(path).ok()?;
    let rd32 = |o: usize| d.get(o..o + 4).map(|b| u32::from_le_bytes(b.try_into().unwrap()));
    let rd64 = |o: usize| d.get(o..o + 8).map(|b| u64::from_le_bytes(b.try_into().unwrap()));
    let want = format!("gu::{}::s_post_filter_param", kind);
    let want_num = format!("{}_num", want);
    // Symbol records start at 0x0C: "SYM\0", size, 0, data offset (u64), name offset (u64), hash, byte size.
    let (mut table, mut num) = (None, None);
    let mut o = 12;
    while d.get(o..o + 4) == Some(b"SYM\0") {
        let data = rd64(o + 12)? as usize;
        let name_off = rd64(o + 20)? as usize;
        let end = d[name_off..].iter().position(|&b| b == 0)? + name_off;
        let name = std::str::from_utf8(&d[name_off..end]).ok()?;
        if name == want {
            table = Some(data);
        } else if name == want_num {
            num = Some(rd32(data)? as usize);
        }
        o += 36;
    }
    let (table, num) = (table?, num?);
    if index >= num {
        return None;
    }
    let w = |i: usize| rd32(table + index * 132 + i * 4).map(|v| v as i32).unwrap_or(0);
    let col = |i: usize| [w(i) as f32 / 255.0, w(i + 1) as f32 / 255.0, w(i + 2) as f32 / 255.0, w(i + 3) as f32 / 255.0];
    let flare = |b: usize, with_falloff: bool| Flare {
        enabled: w(b) > 0,
        blend: w(b + 1).clamp(0, 4),
        outer: with_falloff && w(b + 2) != 0,
        falloff: if with_falloff { w(b + 3) as f32 / 100.0 } else { 0.0 },
        color: if with_falloff { col(b + 4) } else { col(b + 2) },
    };
    Some(PostFilter {
        flares: [flare(5, true), flare(13, true), flare(21, false)],
        hsl_enabled: w(27) > 0,
        hsl: [w(28) as f32 / 100.0, w(29) as f32 / 100.0, w(30) as f32 / 100.0],
        contrast: w(31) as f32 / 100.0,
    })
}

/// Record index for a town: DLL table (RVA 0x79b698) town id -> record.
pub fn town_record(town_id: usize) -> usize {
    [3, 4, 2, 0, 1].get(town_id).copied().unwrap_or(3)
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniform {
    /// Per flare: (enabled, blend, outer, falloff), colour.
    flare_a: [[f32; 4]; 3],
    flare_c: [[f32; 4]; 3],
    /// (hsl enabled, h, s, l)
    hsl: [f32; 4],
    /// (contrast, active, aspect, 0)
    misc: [f32; 4],
}

pub struct PostFx {
    pub filter: Option<PostFilter>,
    scene_tex: wgpu::Texture,
    pub scene_view: wgpu::TextureView,
    sample_view: wgpu::TextureView,
    sampler: wgpu::Sampler,
    uniform: wgpu::Buffer,
    layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
}

const SHADER: &str = r#"
struct U { fa: array<vec4<f32>, 3>, fc: array<vec4<f32>, 3>, hsl: vec4<f32>, misc: vec4<f32> };
@group(0) @binding(0) var t_scene: texture_2d<f32>;
@group(0) @binding(1) var s_scene: sampler;
@group(0) @binding(2) var<uniform> u: U;
struct VOut { @builtin(position) pos: vec4<f32>, @location(0) uv: vec2<f32> };
@vertex fn vs(@builtin(vertex_index) i: u32) -> VOut {
    let p = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
    var o: VOut;
    o.pos = vec4<f32>(p * 2.0 - 1.0, 0.0, 1.0);
    o.uv = vec2<f32>(p.x, 1.0 - p.y);
    return o;
}
fn hue2rgb(p: f32, q: f32, t0: f32) -> f32 {
    var t = t0;
    if (t < 0.0) { t = t + 1.0; }
    if (t > 1.0) { t = t - 1.0; }
    if (t < 1.0 / 6.0) { return p + (q - p) * 6.0 * t; }
    if (t < 0.5) { return q; }
    if (t < 2.0 / 3.0) { return p + (q - p) * (2.0 / 3.0 - t) * 6.0; }
    return p;
}
@fragment fn fs(in: VOut) -> @location(0) vec4<f32> {
    var s = textureSample(t_scene, s_scene, in.uv).rgb;
    if (u.misc.y < 0.5) { return vec4<f32>(s, 1.0); }
    // Flare space: -1..1 across the screen, x scaled by 16/9 (shader 0x3813e8), position (0, 0).
    let q = in.uv * 2.0 - 1.0;
    let d = sqrt(q.y * q.y + (q.x * 1.777778) * (q.x * 1.777778));
    for (var i = 0; i < 3; i = i + 1) {
        let f = u.fa[i];
        if (f.x < 0.5) { continue; }
        let c = u.fc[i];
        var a = c.a;
        if (f.w > 0.0) {
            if (f.z > 0.5) { a = a * clamp(d * f.w - 1.0, 0.0, 1.0); } else { a = a * clamp(1.0 - d * f.w, 0.0, 1.0); }
        }
        let t = i32(f.y + 0.5);
        if (t == 0) { s = s * (1.0 - a) + c.rgb * a; }
        else if (t == 1) { s = s + c.rgb * a; }
        else if (t == 2) { s = s - c.rgb * a; }
        else if (t == 3) { s = s * (1.0 - a) + s * c.rgb * a; }
        else { s = s + c.rgb * a - s * c.rgb * a; }
        s = clamp(s, vec3<f32>(0.0), vec3<f32>(1.0));
    }
    if (u.hsl.x > 0.5) {
        let mx = max(max(s.r, s.g), s.b);
        let mn = min(min(s.r, s.g), s.b);
        var h = 0.0;
        var sat = 0.0;
        let l = (mx + mn) * 0.5;
        let dl = mx - mn;
        if (dl > 0.0) {
            if (mx + mn < 1.0) { sat = dl / (mx + mn); } else { sat = dl / (2.0 - mx - mn); }
            if (mx == s.r) { h = (s.g - s.b) / dl / 6.0; }
            else if (mx == s.g) { h = ((s.b - s.r) / dl + 2.0) / 6.0; }
            else { h = ((s.r - s.g) / dl + 4.0) / 6.0; }
            if (h < 0.0) { h = h + 1.0; }
        }
        h = clamp(h + u.hsl.y, 0.0, 1.0);
        sat = clamp(sat * u.hsl.z, 0.0, 1.0);
        let l2 = clamp(l * u.hsl.w, 0.0, 1.0);
        if (sat == 0.0) {
            s = vec3<f32>(l2);
        } else {
            var qq = 0.0;
            if (l2 < 0.5) { qq = l2 * (1.0 + sat); } else { qq = l2 + sat - l2 * sat; }
            let pp = 2.0 * l2 - qq;
            s = vec3<f32>(hue2rgb(pp, qq, h + 1.0 / 3.0), hue2rgb(pp, qq, h), hue2rgb(pp, qq, h - 1.0 / 3.0));
        }
    }
    s = clamp(u.misc.x * (s - 0.5) + 0.5, vec3<f32>(0.0), vec3<f32>(1.0));
    return vec4<f32>(s, 1.0);
}
"#;

impl PostFx {
    /// `format` is the swapchain format (sRGB); the scene is rendered into a texture of that format and read back
    /// through a non-sRGB view so the filter works on gamma-encoded values, like the original.
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat, width: u32, height: u32) -> Self {
        let (scene_tex, scene_view, sample_view) = Self::target(device, format, width, height);
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor { mag_filter: wgpu::FilterMode::Nearest, min_filter: wgpu::FilterMode::Nearest, ..Default::default() });
        let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("postfx_uniform"),
            contents: bytemuck::bytes_of(&Uniform::zeroed()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("postfx_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
                wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None },
            ],
        });
        let bind_group = Self::bind(device, &layout, &sample_view, &sampler, &uniform);
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("postfx"), source: wgpu::ShaderSource::Wgsl(SHADER.into()) });
        let pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("postfx_pl"), bind_group_layouts: &[&layout], push_constant_ranges: &[] });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("postfx_pipeline"),
            layout: Some(&pl),
            vertex: wgpu::VertexState { module: &module, entry_point: Some("vs"), buffers: &[], compilation_options: Default::default() },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs"),
                targets: &[Some(wgpu::ColorTargetState { format: format.remove_srgb_suffix(), blend: None, write_mask: wgpu::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
            cache: None,
        });
        Self { filter: None, scene_tex, scene_view, sample_view, sampler, uniform, layout, bind_group, pipeline }
    }

    fn target(device: &wgpu::Device, format: wgpu::TextureFormat, width: u32, height: u32) -> (wgpu::Texture, wgpu::TextureView, wgpu::TextureView) {
        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("postfx_scene"),
            size: wgpu::Extent3d { width: width.max(1), height: height.max(1), depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[format.remove_srgb_suffix()],
        });
        let scene_view = tex.create_view(&Default::default());
        let sample_view = tex.create_view(&wgpu::TextureViewDescriptor { format: Some(format.remove_srgb_suffix()), ..Default::default() });
        (tex, scene_view, sample_view)
    }

    fn bind(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, view: &wgpu::TextureView, sampler: &wgpu::Sampler, uniform: &wgpu::Buffer) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("postfx_bg"),
            layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(sampler) },
                wgpu::BindGroupEntry { binding: 2, resource: uniform.as_entire_binding() },
            ],
        })
    }

    pub fn resize(&mut self, device: &wgpu::Device, format: wgpu::TextureFormat, width: u32, height: u32) {
        let (t, v, s) = Self::target(device, format, width, height);
        self.scene_tex = t;
        self.scene_view = v;
        self.sample_view = s;
        self.bind_group = Self::bind(device, &self.layout, &self.sample_view, &self.sampler, &self.uniform);
    }

    /// Apply the filter (or copy the scene through unchanged) into `target`, a non-sRGB view of the swapchain.
    pub fn draw(&self, queue: &wgpu::Queue, encoder: &mut wgpu::CommandEncoder, target: &wgpu::TextureView) {
        let mut u = Uniform::zeroed();
        if let Some(f) = &self.filter {
            for (i, fl) in f.flares.iter().enumerate() {
                u.flare_a[i] = [fl.enabled as i32 as f32, fl.blend as f32, fl.outer as i32 as f32, fl.falloff];
                u.flare_c[i] = fl.color;
            }
            u.hsl = [f.hsl_enabled as i32 as f32, f.hsl[0], f.hsl[1], f.hsl[2]];
            u.misc = [f.contrast, 1.0, 16.0 / 9.0, 0.0];
        }
        queue.write_buffer(&self.uniform, 0, bytemuck::bytes_of(&u));
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("postfx_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: target, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store } })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.draw(0..3, 0..1);
    }
}

use bytemuck::Zeroable;
