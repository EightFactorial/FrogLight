#import bevy_pbr::mesh_bindings   mesh
#import bevy_pbr::mesh_functions  mesh_position_local_to_clip
#import bevy_render::globals  Globals

@group(0) @binding(9)
var<uniform> globals: Globals;

struct StateAnimation {
    frame_time: f32,

    // Compressed frame indices
    frame_data_0: u32,
    frame_data_1: u32,
    frame_data_2: u32,
    frame_data_3: u32,
    frame_data_4: u32,
    frame_data_5: u32,
    frame_data_6: u32,
    frame_data_7: u32,  

    _padding_0: u32,
    _padding_1: u32,
    _padding_2: u32,
};

// Extract the frame index from the animation data
fn get_frame(state: StateAnimation, index: u32) -> u32 {
    var dat: u32;
    switch (index / 4u) {
        case 0u: { dat = state.frame_data_0; }
        case 1u: { dat = state.frame_data_1; }
        case 2u: { dat = state.frame_data_2; }
        case 3u: { dat = state.frame_data_3; }
        case 4u: { dat = state.frame_data_4; }
        case 5u: { dat = state.frame_data_5; }
        case 6u: { dat = state.frame_data_6; }
        case 7u: { dat = state.frame_data_7; }
        default: { return 0u; }
    }

    return extractBits(dat, (index % 4u) * 8u, 8u);
}


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

    var uvs: vec2<f32> = in.uvs;
    if in.anim_index != u32(0) {
        // Get the amount of frames
        let dim: vec2<u32> = textureDimensions(tex);
        let frame_count = dim.y / dim.x;

        // Get the frame from the animation and global time
        let anim = animations[in.anim_index - u32(1)];
        let frame_index = u32(globals.time / anim.frame_time) % frame_count;
        let frame = get_frame(anim, frame_index);

        // Update the uvs
        uvs.y = (uvs.y + f32(frame)) / f32(frame_count);
    }

    // Sample the texture
    return textureSample(tex, nearest_sampler, uvs);
}