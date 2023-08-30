use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::{
        mesh::{MeshVertexAttribute, MeshVertexBufferLayout},
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
            VertexFormat,
        },
    },
};

pub(super) fn setup(app: &mut App) {
    app.add_plugins((
        MaterialPlugin::<OpagueBlockMaterial>::default(),
        MaterialPlugin::<TransparentBlockMaterial>::default(),
    ));
}

#[derive(Debug, Clone, TypePath, TypeUuid, AsBindGroup)]
#[uuid = "0059fd0b-5b43-46cc-bd77-c89130562e75"]
pub struct OpagueBlockMaterial {
    pub atlas: Handle<Image>,
}

impl OpagueBlockMaterial {
    pub fn new(atlas: Handle<Image>) -> Self { Self { atlas } }
}

pub const ATTRIBUTE_BLOCK_ID: MeshVertexAttribute =
    MeshVertexAttribute::new("TextureIndex", 978122767, VertexFormat::Uint32);

impl Material for OpagueBlockMaterial {
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
            Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(2),
            ATTRIBUTE_BLOCK_ID.at_shader_location(3),
        ])?];

        Ok(())
    }
}

#[derive(Debug, Clone, TypePath, TypeUuid, AsBindGroup)]
#[uuid = "ae6ba47d-5ce3-4fb6-ab9e-efbecf21395a"]
pub struct TransparentBlockMaterial {
    pub atlas: Handle<Image>,
}

impl TransparentBlockMaterial {
    pub fn new(atlas: Handle<Image>) -> Self { Self { atlas } }
}

impl Material for TransparentBlockMaterial {
    fn alpha_mode(&self) -> AlphaMode { AlphaMode::Blend }

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
            Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(2),
            ATTRIBUTE_BLOCK_ID.at_shader_location(3),
        ])?];

        Ok(())
    }
}
