#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_clip}

@group(2) @binding(0) var texture_front: texture_2d<f32>;
@group(2) @binding(1) var sampler_front: sampler;

@group(2) @binding(2) var texture_back: texture_2d<f32>;
@group(2) @binding(3) var sampler_back: sampler;

@group(2) @binding(4) var texture_left: texture_2d<f32>;
@group(2) @binding(5) var sampler_left: sampler;

@group(2) @binding(6) var texture_right: texture_2d<f32>;
@group(2) @binding(7) var sampler_right: sampler;

@group(2) @binding(8) var texture_top: texture_2d<f32>;
@group(2) @binding(9) var sampler_top: sampler;

@group(2) @binding(10) var texture_bottom: texture_2d<f32>;
@group(2) @binding(11) var sampler_bottom: sampler;


struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uvs: vec2<f32>,
    @location(3) sampler_index: u32,
};

@vertex
fn vertex(
    vertex: Vertex,
) -> VertexOutput {
    var output: VertexOutput;

    output.clip_position = mesh_position_local_to_clip(
        get_model_matrix(vertex.instance_index),
        vec4<f32>(vertex.position, 1.0),
    );
    output.position = vertex.position;
    output.normal = vertex.normal;
    output.uvs = vertex.uvs;
    output.sampler_index = vertex.sampler_index;

    return output;
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uvs: vec2<f32>,
    @location(3) sampler_index: u32,
};

@fragment
fn fragment(
    vertex: VertexOutput,
) -> @location(0) vec4<f32> {
    var color: vec4<f32>;

    // Sample the texture
    switch vertex.sampler_index {
        case 0u {
            color = textureSample(texture_front, sampler_front, vertex.uvs);
        }
        case 1u {
            color = textureSample(texture_back, sampler_back, vertex.uvs);
        }
        case 2u {
            color = textureSample(texture_left, sampler_left, vertex.uvs);
        }
        case 3u {
            color = textureSample(texture_right, sampler_right, vertex.uvs);
        }
        case 4u {
            color = textureSample(texture_top, sampler_top, vertex.uvs);
        }
        case 5u {
            color = textureSample(texture_bottom, sampler_bottom, vertex.uvs);
        }
        default {
            color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
        }
    }    

    return color;
}

