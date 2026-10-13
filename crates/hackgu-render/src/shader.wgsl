struct CameraUniform {
    view_proj: mat4x4<f32>,
    cam_pos: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) color: vec4<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.world_pos = model.position;
    out.world_normal = model.normal;
    out.uv = model.uv;
    out.color = model.color;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

// Backdrop (sky dome, distant land, clouds, sun): follows the camera horizontally so it stays on the horizon
// however far the player walks.
@vertex
fn vs_sky(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let p = model.position + vec3<f32>(camera.cam_pos.x, camera.cam_pos.y, 0.0);
    out.world_pos = p;
    out.world_normal = model.normal;
    out.uv = model.uv;
    out.color = model.color;
    out.clip_position = camera.view_proj * vec4<f32>(p, 1.0);
    return out;
}

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(t_diffuse, s_diffuse, in.uv) * in.color;
    
    // Anime hair strands, tassel cutouts, transparent cloth
    if (tex_color.a < 0.25) {
        discard;
    }

    let n = normalize(in.world_normal);
    
    // Key light direction (upper right front in CC2 coordinate system: Z is Up, Y is Depth)
    let light_dir = normalize(vec3<f32>(0.577, 0.577, 0.577));
    let view_dir = normalize(camera.cam_pos.xyz - in.world_pos);

    // Two-sided Lambertian diffuse with soft wrap for anime shading
    let ndotl = dot(n, light_dir);
    let diff = max(0.0, ndotl);
    let back_bounce = max(0.0, -ndotl) * 0.3; // backlight for thin ribbons and hair
    
    // Subtle rim lighting
    let vdotn = abs(dot(view_dir, n));
    let rim = pow(max(0.0, 1.0 - vdotn), 3.0) * 0.35;

    let ambient = 0.42;
    let lighting = ambient + diff * 0.58 + back_bounce + rim;

    let final_rgb = tex_color.rgb * lighting;
    return vec4<f32>(final_rgb, tex_color.a);
}

// Additive layer (model draw flag 1): unlit texture colour, scaled by texture/material alpha in the blend state.
@fragment
fn fs_additive(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.uv) * in.color;
}

// Alpha-blended decal (model draw flag 2).
@fragment
fn fs_decal(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(t_diffuse, s_diffuse, in.uv) * in.color;
    if (tex_color.a < 0.02) {
        discard;
    }
    return tex_color;
}
