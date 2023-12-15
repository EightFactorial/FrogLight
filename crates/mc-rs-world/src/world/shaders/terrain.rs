use std::num::NonZeroU32;

use bevy::{
    asset::embedded_asset,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        mesh::{MeshVertexAttribute, MeshVertexBufferLayout},
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroup, AsBindGroupError, BindGroupEntries, BindGroupLayout, BindGroupLayoutEntry,
            BindingType, PreparedBindGroup, RenderPipelineDescriptor, SamplerBindingType,
            ShaderRef, ShaderStages, SpecializedMeshPipelineError, TextureSampleType,
            TextureViewDimension, UnpreparedBindGroup, VertexFormat,
        },
        renderer::RenderDevice,
        texture::FallbackImage,
    },
};

pub(super) fn setup(app: &mut App) {
    embedded_asset!(app, "embedded/terrain.wgsl");
    app.add_plugins(MaterialPlugin::<TerrainMaterial>::default());
}

pub const TEXTURE_INDEX: MeshVertexAttribute =
    MeshVertexAttribute::new("TextureIndex", 422262077, VertexFormat::Uint32);

#[derive(Debug, Default, Clone, Asset, TypePath)]
pub struct TerrainMaterial {
    textures: Vec<Handle<Image>>,
}

impl TerrainMaterial {
    const MAX_TEXTURE_COUNT: usize = 64;
    const BIND_GROUP_LAYOUT: &'static str = "terrain_bind_group_layout";

    pub const fn new(textures: Vec<Handle<Image>>) -> Self { Self { textures } }
}

impl Material for TerrainMaterial {
    fn vertex_shader() -> ShaderRef {
        ShaderRef::Path("embedded://mc_rs_world/world/shaders/embedded/terrain.wgsl".into())
    }

    fn fragment_shader() -> ShaderRef {
        ShaderRef::Path("embedded://mc_rs_world/world/shaders/embedded/terrain.wgsl".into())
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_latout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
            TEXTURE_INDEX.at_shader_location(2),
        ])?;
        descriptor.vertex.buffers = vec![vertex_latout];

        Ok(())
    }
}

impl AsBindGroup for TerrainMaterial {
    type Data = ();

    fn as_bind_group(
        &self,
        layout: &BindGroupLayout,
        render_device: &RenderDevice,
        images: &RenderAssets<Image>,
        fallback_image: &FallbackImage,
    ) -> Result<PreparedBindGroup<Self::Data>, AsBindGroupError> {
        let mut image_data = vec![];
        for handle in self.textures.iter().take(Self::MAX_TEXTURE_COUNT) {
            match images.get(handle) {
                Some(image) => image_data.push(image),
                None => return Err(AsBindGroupError::RetryNextUpdate),
            }
        }

        let fallback_image = &fallback_image.d2;
        let mut textures = vec![&*fallback_image.texture_view; Self::MAX_TEXTURE_COUNT];

        for (id, image) in image_data.into_iter().enumerate() {
            textures[id] = &image.texture_view;
        }

        Ok(PreparedBindGroup {
            bind_group: render_device.create_bind_group(
                Self::BIND_GROUP_LAYOUT,
                layout,
                &BindGroupEntries::sequential((&textures[..], &fallback_image.sampler)),
            ),
            bindings: Vec::new(),
            data: (),
        })
    }

    fn unprepared_bind_group(
        &self,
        _: &BindGroupLayout,
        _: &RenderDevice,
        _: &RenderAssets<Image>,
        _: &FallbackImage,
    ) -> Result<UnpreparedBindGroup<Self::Data>, AsBindGroupError> {
        panic!("bindless textures arrays can't be owned");
    }

    fn bind_group_layout_entries(_: &RenderDevice) -> Vec<BindGroupLayoutEntry>
    where
        Self: Sized,
    {
        vec![
            // @group(1) @binding(0) var textures: binding_array<texture_2d<f32>>;
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: NonZeroU32::new(Self::MAX_TEXTURE_COUNT as u32),
            },
            // @group(1) @binding(1) var texture_sampler: sampler;
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler(SamplerBindingType::Filtering),
                count: None,
                // Note: as textures, multiple samplers can also be bound onto one binding slot.
                // One may need to pay attention to the limit of sampler binding amount on some
                // platforms. count: NonZeroU32::new(MAX_TEXTURE_COUNT as u32),
            },
        ]
    }
}
