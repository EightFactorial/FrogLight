#import bevy_pbr::mesh_bindings   mesh
#import bevy_pbr::mesh_functions  mesh_position_local_to_clip
#import bevy_render::globals  Globals

@group(0) @binding(9)
var<uniform> globals: Globals;

@group(1) @binding(0)
var textures: binding_array<texture_2d<f32>>;
@group(1) @binding(1)
var nearest_sampler: sampler;

struct VertexIn {
    @location(0) position: vec3<f32>,
    @location(1) uvs: vec2<f32>,
    @location(2) index: u32,
    @location(3) anim: vec2<u32>,
};

struct VertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) uvs: vec2<f32>,
    @location(2) index: u32,
    @location(3) anim: vec2<u32>,
};

@vertex
fn vertex(
    in: VertexIn,
) -> VertexOut {
    var out: VertexOut;

    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(in.position, 1.0));
    out.uvs = in.uvs;
    out.index = in.index;
    out.anim = in.anim;

    return out;
}

struct FragmentIn {
    @location(1) uvs: vec2<f32>,
    @location(2) index: u32,
    @location(3) anim: vec2<u32>,
};

@fragment
fn fragment(
    in: FragmentIn,
) -> @location(0) vec4<f32> {
    var v = in.uvs.y;

    if (in.anim.x != 0u) {
        let frames: f32 = globals.time / f32(in.anim.y) * 20.0;
        let frame = f32(u32(frames) % in.anim.x);

        v = (v + frame) / f32(in.anim.x);
    }

    return textureSample(textures[in.index], nearest_sampler, vec2<f32>(in.uvs.x, v));
}