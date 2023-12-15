@group(1) @binding(0) var textures: binding_array<texture_2d<f32>>;
@group(1) @binding(1) var textures_sampler: sampler;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) tex_index: u32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) tex_index: u32,
};

@vertex
fn vertex(
    in: VertexInput
) -> VertexOutput {
    var out: VertexOutput;

    out.position = vec4<f32>(in.position, 0.0, 1.0);
    out.tex_coords = in.tex_coords;
    out.tex_index = in.tex_index;

    return out;
}

@fragment
fn fragment(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    textureSample(textures[in.tex_index], textures_sampler, in.tex_coords);
}
