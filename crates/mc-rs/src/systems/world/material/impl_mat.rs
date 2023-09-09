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

impl BlockMaterial {
    pub fn new(textures: Vec<Handle<Image>>) -> Self {
        Self {
            textures,
            ..Default::default()
        }
    }

    pub fn new_blended(textures: Vec<Handle<Image>>) -> Self {
        Self {
            textures,
            alpha_mode: AlphaMode::Blend,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct BlockAnimation {
    pub block_id: u32,
    pub frame_time: f32,
    pub frame_count: u32,
    pub frame_order: Vec<u32>,
}

pub const ATTRIBUTE_BLOCK_ID: MeshVertexAttribute =
    MeshVertexAttribute::new("BlockId", 978122767, VertexFormat::Uint32);

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
            ATTRIBUTE_BLOCK_ID.at_shader_location(3),
            ATTRIBUTE_TEXTURE_INDEX.at_shader_location(4),
        ])?];

        Ok(())
    }

    fn alpha_mode(&self) -> AlphaMode { self.alpha_mode }
}
