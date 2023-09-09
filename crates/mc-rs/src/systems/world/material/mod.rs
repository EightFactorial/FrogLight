use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};

mod impl_bind;
pub use impl_bind::*;

mod impl_mat;
pub use impl_mat::*;

pub(super) fn setup(app: &mut App) { app.add_plugins(MaterialPlugin::<BlockMaterial>::default()); }

pub const MAX_TEXTURE_COUNT: usize = 48;

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

#[derive(Debug, Default, Clone)]
pub struct StateAnimation {
    pub frame_time: f32,
    pub frame_order: Vec<u32>,
}
