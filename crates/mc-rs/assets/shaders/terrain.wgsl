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
    @location(1) normal: vec3<f32>,
    @location(2) uvs: vec2<f32>,
    @location(3) block_id: u32,
    @location(4) index: u32,
};

struct VertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uvs: vec2<f32>,
    @location(3) block_id: u32,
    @location(4) index: u32,
};

@vertex
fn vertex(
    in: VertexIn,
) -> VertexOut {
    var out: VertexOut;

    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(in.position, 1.0));
    out.uvs = in.uvs;
    out.normal = in.normal;
    out.block_id = in.block_id;
    out.index = in.index;

    return out;
}

struct FragmentIn {
    @location(1) normal: vec3<f32>,
    @location(2) uvs: vec2<f32>,
    @location(3) block_id: u32,
    @location(4) index: u32,
};

@fragment
fn fragment(
    in: FragmentIn,
) -> @location(0) vec4<f32> {
    return textureSample(textures[in.index], nearest_sampler, in.uvs);
}