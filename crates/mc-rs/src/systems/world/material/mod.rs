use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::ShaderType,
};

mod impl_bind;
use bytemuck::{Pod, Zeroable};
pub use impl_bind::*;

mod impl_mat;
pub use impl_mat::*;

pub(super) fn setup(app: &mut App) { app.add_plugins(MaterialPlugin::<BlockMaterial>::default()); }

pub const MAX_TEXTURE_COUNT: usize = 48;
pub const MAX_ANIMATION_COUNT: usize = 16;

#[derive(Debug, Default, Clone, TypePath, TypeUuid)]
#[uuid = "0059fd0b-5b43-46cc-bd77-c89130562e75"]
pub struct BlockMaterial {
    pub textures: Vec<Handle<Image>>,
    pub animations: Vec<StateAnimation>,
    pub alpha_mode: AlphaMode,
}

impl BlockMaterial {
    pub fn new_opaque(textures: Vec<Handle<Image>>, animations: Vec<StateAnimation>) -> Self {
        Self {
            textures,
            animations,
            alpha_mode: AlphaMode::Opaque,
        }
    }

    pub fn new_blended(textures: Vec<Handle<Image>>, animations: Vec<StateAnimation>) -> Self {
        Self {
            textures,
            animations,
            alpha_mode: AlphaMode::Blend,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, ShaderType, Pod, Zeroable, Reflect)]
#[repr(C)]
pub struct StateAnimation {
    pub frame_time: f32,
    pub order_length: u32,
    pub frame_order: [u32; MAX_ANIMATION_COUNT],
    _padding: [u32; 2],
}

impl StateAnimation {
    pub fn new(frame_time: f32, frame_order: &[u32]) -> Self {
        let order_length = frame_order.len() as u32;

        if order_length > MAX_ANIMATION_COUNT as u32 {
            error!(
                "Animation order length ({order_length}) exceeds maximum ({MAX_ANIMATION_COUNT})",
            );
        }

        let mut frames = [0u32; MAX_ANIMATION_COUNT];
        for (frame, frame_mut) in frame_order
            .iter()
            .zip(frames.iter_mut())
            .take(MAX_ANIMATION_COUNT)
        {
            *frame_mut = *frame;
        }

        Self {
            frame_time,
            order_length,
            frame_order: frames,
            _padding: [0; 2],
        }
    }
}
