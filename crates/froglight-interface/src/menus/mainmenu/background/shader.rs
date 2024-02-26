use bevy::{
    asset::embedded_asset,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        mesh::{MeshVertexAttribute, MeshVertexBufferLayout},
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
            VertexFormat,
        },
    },
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    embedded_asset!(app, "shader.wgsl");
    app.add_plugins(MaterialPlugin::<MainMenuBackgroundShader>::default());
}

/// A shader that runs the main menu background.
#[derive(Asset, Debug, Clone, PartialEq, Eq, Hash, TypePath, AsBindGroup)]
pub(super) struct MainMenuBackgroundShader {
    #[texture(0)]
    #[sampler(1)]
    pub(super) front: Handle<Image>,
    #[texture(2)]
    #[sampler(3)]
    pub(super) back: Handle<Image>,
    #[texture(4)]
    #[sampler(5)]
    pub(super) left: Handle<Image>,
    #[texture(6)]
    #[sampler(7)]
    pub(super) right: Handle<Image>,
    #[texture(8)]
    #[sampler(9)]
    pub(super) top: Handle<Image>,
    #[texture(10)]
    #[sampler(11)]
    pub(super) bottom: Handle<Image>,
}

impl Material for MainMenuBackgroundShader {
    fn vertex_shader() -> ShaderRef {
        ShaderRef::from("embedded://froglight_interface/menus/mainmenu/background/shader.wgsl")
    }

    fn fragment_shader() -> ShaderRef {
        ShaderRef::from("embedded://froglight_interface/menus/mainmenu/background/shader.wgsl")
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // Set the vertex layout
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
            ATTRIBUTE_INDEX.at_shader_location(3),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];

        Ok(())
    }
}

pub(super) const ATTRIBUTE_INDEX: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Image_Index", 3_127_983_271, VertexFormat::Uint32);
