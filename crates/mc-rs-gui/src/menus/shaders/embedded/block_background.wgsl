#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0)
var<uniform> scale_x: f32;
@group(1) @binding(1)
var<uniform> scale_y: f32;

@group(1) @binding(2) 
var block_texture: texture_2d<f32>;
@group(1) @binding(3)
var block_sampler: sampler;


@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    return textureSample(block_texture, block_sampler, in.uv * vec2<f32>(scale_x, scale_y));
}
