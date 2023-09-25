use std::num::{NonZeroU32, NonZeroU64};

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroup, AsBindGroupError, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
            BufferBinding, BufferBindingType, BufferInitDescriptor, BufferUsages,
            PreparedBindGroup, SamplerBindingType, ShaderSize, ShaderStages, TextureSampleType,
            TextureViewDimension,
        },
        renderer::RenderDevice,
        texture::FallbackImage,
    },
};

use super::{BlockMaterial, StateAnimation, MAX_ANIMATION_COUNT, MAX_TEXTURE_COUNT};

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

        // fill in up to the first `MAX_TEXTURE_COUNT` textures and samplers to the array
        for (image, texture) in images
            .into_iter()
            .zip(textures.iter_mut())
            .take(MAX_TEXTURE_COUNT)
        {
            *texture = &*image.texture_view;
        }

        // fill in up to the first `MAX_ANIMATION_COUNT` animations to the array
        let mut animations = vec![StateAnimation::default(); MAX_ANIMATION_COUNT];
        for (anim, out_anim) in self
            .animations
            .iter()
            .zip(animations.iter_mut())
            .take(MAX_ANIMATION_COUNT)
        {
            *out_anim = *anim;
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
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &render_device.create_buffer_with_data(&BufferInitDescriptor {
                            label: "animation_buffer".into(),
                            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                            contents: bytemuck::cast_slice(&animations),
                        }),
                        offset: 0,
                        size: NonZeroU64::new(
                            u64::from(StateAnimation::SHADER_SIZE) * MAX_ANIMATION_COUNT as u64,
                        ),
                    }),
                },
                BindGroupEntry {
                    binding: 2,
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
                // @group(1) @binding(1) var animations: array<StateAnimation, 16>;
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform {},
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: NonZeroU32::new(MAX_ANIMATION_COUNT as u32),
                },
                // @group(1) @binding(2) var nearest_sampler: sampler;
                BindGroupLayoutEntry {
                    binding: 2,
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
