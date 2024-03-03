use bevy::{
    core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    prelude::*,
    render::{
        render_resource::{
            BindGroupLayout, BindGroupLayoutEntry, BindingType, BlendState, BufferBindingType,
            ColorTargetState, ColorWrites, FragmentState, MultisampleState, PrimitiveState,
            RenderPipelineDescriptor, Sampler, SamplerBindingType, ShaderStages, ShaderType,
            SpecializedRenderPipeline, TextureFormat, TextureSampleType, TextureViewDimension,
        },
        renderer::RenderDevice,
        texture::{BevyDefault, ImageSamplerDescriptor},
        view::{ViewTarget, ViewUniform},
    },
};

use super::GaussianNode;

/// The pipeline for the [`GaussianNode`] material.
#[derive(Debug, Resource)]
pub(crate) struct GaussianNodePipeline {
    pub(crate) layout: BindGroupLayout,
    pub(crate) _sampler: Sampler,
    pub(crate) shader: Handle<Shader>,
}

impl FromWorld for GaussianNodePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        // Create the bind group layout
        let layout = render_device.create_bind_group_layout(
            "gaussian_node_pipeline_layout",
            &[
                // View
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX
                        | ShaderStages::FRAGMENT
                        | ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: Some(ViewUniform::min_size()),
                    },
                    count: None,
                },
                // The screen texture
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // The sampler that will be used to sample the screen texture
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                // The settings uniform that will control the effect
                BindGroupLayoutEntry {
                    binding: 3,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: bevy::render::render_resource::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        );

        // Use the default linear sampler
        let sampler = render_device.create_sampler(&ImageSamplerDescriptor::linear().as_wgpu());

        // Load the shader from the asset server
        let shader: Handle<Shader> =
            world.resource::<AssetServer>().load(GaussianNode::SHADER_PATH);

        Self { layout, _sampler: sampler, shader }
    }
}

impl SpecializedRenderPipeline for GaussianNodePipeline {
    type Key = UiMaterialKey<GaussianNode>;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let mut descriptor = RenderPipelineDescriptor {
            vertex: fullscreen_shader_vertex_state(),
            fragment: Some(FragmentState {
                shader: self.shader.clone(),
                shader_defs: Vec::new(),
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format: if key.hdr {
                        ViewTarget::TEXTURE_FORMAT_HDR
                    } else {
                        TextureFormat::bevy_default()
                    },
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            layout: vec![self.layout.clone()],
            depth_stencil: None,
            push_constant_ranges: Vec::new(),
            primitive: PrimitiveState::default(),
            multisample: MultisampleState::default(),
            label: Some("gaussian_node_pipeline".into()),
        };

        GaussianNode::specialize(&mut descriptor, key);

        descriptor
    }
}
