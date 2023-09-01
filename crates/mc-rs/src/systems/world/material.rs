use std::num::NonZeroU32;

use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::{
        mesh::{MeshVertexAttribute, MeshVertexBufferLayout},
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroup, AsBindGroupError, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
            PreparedBindGroup, RenderPipelineDescriptor, SamplerBindingType, ShaderRef,
            ShaderStages, SpecializedMeshPipelineError, TextureSampleType, TextureViewDimension,
            VertexFormat,
        },
        renderer::RenderDevice,
        texture::FallbackImage,
    },
};

pub(super) fn setup(app: &mut App) { app.add_plugins(MaterialPlugin::<BlockMaterial>::default()); }

#[derive(Debug, Default, Clone, TypePath, TypeUuid)]
#[uuid = "0059fd0b-5b43-46cc-bd77-c89130562e75"]
pub struct BlockMaterial {
    pub textures: Vec<Handle<Image>>,
    pub animation_info: Vec<BlockAnimation>,
    pub alpha_mode: AlphaMode,
}

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

const MAX_TEXTURE_COUNT: usize = 32;

impl AsBindGroup for BlockMaterial {
    type Data = ();

    fn as_bind_group(
        &self,
        layout: &BindGroupLayout,
        render_device: &RenderDevice,
        image_assets: &RenderAssets<Image>,
        fallback_image: &FallbackImage,
    ) -> Result<PreparedBindGroup<Self::Data>, AsBindGroupError> {
        // retrieve the render resources from handles
        let mut images = vec![];
        for handle in self.textures.iter().take(MAX_TEXTURE_COUNT) {
            match image_assets.get(handle) {
                Some(image) => images.push(image),
                None => return Err(AsBindGroupError::RetryNextUpdate),
            }
        }

        let fallback_image = &fallback_image.d2;

        let textures = vec![&fallback_image.texture_view; MAX_TEXTURE_COUNT];

        // convert bevy's resource types to WGPU's references
        let mut textures: Vec<_> = textures.into_iter().map(|texture| &**texture).collect();

        // fill in up to the first `MAX_TEXTURE_COUNT` textures and samplers to the arrays
        for (id, image) in images.into_iter().enumerate() {
            textures[id] = &*image.texture_view;
        }

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: "bindless_material_bind_group".into(),
            layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureViewArray(&textures[..]),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&fallback_image.sampler),
                },
            ],
        });

        Ok(PreparedBindGroup {
            bindings: vec![],
            bind_group,
            data: (),
        })
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout
    where
        Self: Sized,
    {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: "bindless_material_layout".into(),
            entries: &[
                // @group(1) @binding(0) var textures: binding_array<texture_2d<f32>>;
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: NonZeroU32::new(MAX_TEXTURE_COUNT as u32),
                },
                // @group(1) @binding(1) var nearest_sampler: sampler;
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                    // Note: as textures, multiple samplers can also be bound onto one binding slot.
                    // One may need to pay attention to the limit of sampler binding amount on some
                    // platforms. count: NonZeroU32::new(MAX_TEXTURE_COUNT as
                    // u32),
                },
            ],
        })
    }
}
