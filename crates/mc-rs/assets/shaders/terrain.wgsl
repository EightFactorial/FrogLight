#import bevy_pbr::mesh_bindings   mesh
#import bevy_pbr::mesh_functions  mesh_position_local_to_clip
#import bevy_render::globals  Globals

@group(0) @binding(9)
var<uniform> globals: Globals;

// TODO: I love padding and stride :)
struct StateAnimation {
    frame_time: f32,

    // frame_order: array<u32,32>,
    frame_order0: u32,
    frame_order1: u32,
    frame_order2: u32,
    frame_order3: u32,
    frame_order4: u32,
    frame_order5: u32,
    frame_order6: u32,
    frame_order7: u32,
    frame_order8: u32,
    frame_order9: u32,
    frame_order10: u32,
    frame_order11: u32,
    frame_order12: u32,
    frame_order13: u32,
    frame_order14: u32,
    frame_order15: u32,
    frame_order16: u32,
    frame_order17: u32,
    frame_order18: u32,
    frame_order19: u32,
    frame_order20: u32,
    frame_order21: u32,
    frame_order22: u32,
    frame_order23: u32,
    frame_order24: u32,
    frame_order25: u32,
    frame_order26: u32,
    frame_order27: u32,
    frame_order28: u32,
    frame_order29: u32,
    frame_order30: u32,
    frame_order31: u32,

    _padding0: u32,
    _padding1: u32,
    _padding2: u32,
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

    var uvs: vec2<f32> = in.uvs;
    if in.anim_index != u32(0) {
        // Get the amount of frames
        let dim: vec2<u32> = textureDimensions(tex);
        let frame_count = dim.y / dim.x;

        // Get the animation from the array and the frame index
        let anim = animations[in.anim_index - u32(1)];
        let frame_index = u32(globals.time / anim.frame_time) % frame_count;

        // TODO: Not this...
        // Get the frame
        var frame: u32;
        switch i32(frame_index) {
            case 0: { frame = anim.frame_order0; }
            case 1: { frame = anim.frame_order1; }
            case 2: { frame = anim.frame_order2; }
            case 3: { frame = anim.frame_order3; }
            case 4: { frame = anim.frame_order4; }
            case 5: { frame = anim.frame_order5; }
            case 6: { frame = anim.frame_order6; }
            case 7: { frame = anim.frame_order7; }
            case 8: { frame = anim.frame_order8; }
            case 9: { frame = anim.frame_order9; }
            case 10: { frame = anim.frame_order10; }
            case 11: { frame = anim.frame_order11; }
            case 12: { frame = anim.frame_order12; }
            case 13: { frame = anim.frame_order13; }
            case 14: { frame = anim.frame_order14; }
            case 15: { frame = anim.frame_order15; }
            case 16: { frame = anim.frame_order16; }
            case 17: { frame = anim.frame_order17; }
            case 18: { frame = anim.frame_order18; }
            case 19: { frame = anim.frame_order19; }
            case 20: { frame = anim.frame_order20; }
            case 21: { frame = anim.frame_order21; }
            case 22: { frame = anim.frame_order22; }
            case 23: { frame = anim.frame_order23; }
            case 24: { frame = anim.frame_order24; }
            case 25: { frame = anim.frame_order25; }
            case 26: { frame = anim.frame_order26; }
            case 27: { frame = anim.frame_order27; }
            case 28: { frame = anim.frame_order28; }
            case 29: { frame = anim.frame_order29; }
            case 30: { frame = anim.frame_order30; }
            case 31: { frame = anim.frame_order31; }
            default: { frame = u32(0); }
        }

        uvs.y = (uvs.y + f32(frame)) / f32(frame_count);
    }

    return textureSample(tex, nearest_sampler, uvs);
}