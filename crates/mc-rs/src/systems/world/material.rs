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
    app.add_plugins((MaterialPlugin::<BlockMaterial>::default(),));
}

#[derive(Debug, Default, Clone, TypePath, TypeUuid, AsBindGroup)]
#[uuid = "0059fd0b-5b43-46cc-bd77-c89130562e75"]
pub struct BlockMaterial {
    pub atlas: Handle<Image>,
    pub animation_info: Vec<BlockAnimation>,
    pub alpha_mode: AlphaMode,
}

#[derive(Debug, Default, Clone)]
pub struct BlockAnimation {
    pub block_id: u32,
    pub frame_time: f32,
    pub frame_count: u32,
    pub frame_order: Vec<u32>,
}

impl BlockMaterial {
    pub fn new(atlas: Handle<Image>) -> Self {
        Self {
            atlas,
            ..Default::default()
        }
    }
}

pub const ATTRIBUTE_BLOCK_ID: MeshVertexAttribute =
    MeshVertexAttribute::new("TextureIndex", 978122767, VertexFormat::Uint32);

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
            Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(2),
            ATTRIBUTE_BLOCK_ID.at_shader_location(3),
        ])?];

        Ok(())
    }

    fn alpha_mode(&self) -> AlphaMode { self.alpha_mode }
}
