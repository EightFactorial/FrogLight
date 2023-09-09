use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        mesh::{MeshVertexAttribute, MeshVertexBufferLayout},
        render_resource::{
            RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError, VertexFormat,
        },
    },
};

use super::BlockMaterial;

pub const ATTRIBUTE_ANIMATION_INDEX: MeshVertexAttribute =
    MeshVertexAttribute::new("StateId", 978122767, VertexFormat::Uint32);

pub const ATTRIBUTE_TEXTURE_INDEX: MeshVertexAttribute =
    MeshVertexAttribute::new("TextureIndex", 978122784, VertexFormat::Uint32);

impl Material for BlockMaterial {
    fn vertex_shader() -> ShaderRef { "shaders/terrain.wgsl".into() }

    fn fragment_shader() -> ShaderRef { "shaders/terrain.wgsl".into() }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.buffers = vec![layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
            ATTRIBUTE_ANIMATION_INDEX.at_shader_location(3),
            ATTRIBUTE_TEXTURE_INDEX.at_shader_location(4),
        ])?];

        Ok(())
    }

    fn alpha_mode(&self) -> AlphaMode { self.alpha_mode }
}
