#import bevy_pbr::mesh_bindings   mesh
#import bevy_pbr::mesh_functions  mesh_position_local_to_clip
#import bevy_render::globals  Globals

@group(0) @binding(9)
var<uniform> globals: Globals;

// TODO: I love padding and stride :)
struct StateAnimation {
    frame_time: f32,
    order_length: u32,
    // frame_order: array<u32,16>,
    _padding0: u32,
    _padding1: u32,
};

@group(1) @binding(0)
var textures: binding_array<texture_2d<f32>>;
@group(1) @binding(1)
var<uniform> animations: array<StateAnimation,16>;
@group(1) @binding(2)
var nearest_sampler: sampler;

struct VertexIn {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uvs: vec2<f32>,
    @location(3) anim_index: u32,
    @location(4) tex_index: u32,
};

struct VertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uvs: vec2<f32>,
    @location(3) anim_index: u32,
    @location(4) tex_index: u32,
};

@vertex
fn vertex(
    in: VertexIn,
) -> VertexOut {
    var out: VertexOut;

    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(in.position, 1.0));
    out.uvs = in.uvs;
    out.normal = in.normal;
    out.anim_index = in.anim_index;
    out.tex_index = in.tex_index;

    return out;
}

struct FragmentIn {
    @location(1) normal: vec3<f32>,
    @location(2) uvs: vec2<f32>,
    @location(3) anim_index: u32,
    @location(4) tex_index: u32,
};


@fragment
fn fragment(
    in: FragmentIn,
) -> @location(0) vec4<f32> {
    let tex = textures[in.tex_index];
    let dim: vec2<u32> = textureDimensions(tex);

    // TODO: Fix frame index and use it instead of texture dimensions
    var uvs: vec2<f32> = in.uvs;
    if in.anim_index != u32(0) {
        let anim = animations[in.anim_index - u32(1)];
        let frame_index = u32(globals.time / anim.frame_time) % dim.y;
        uvs.y = (uvs.y + f32(frame_index)) / f32(dim.y);
    }

    return textureSample(tex, nearest_sampler, uvs);
}