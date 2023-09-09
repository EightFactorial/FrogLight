use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};

use self::impl_mat::BlockAnimation;

mod impl_bind;
pub use impl_bind::*;

mod impl_mat;
pub use impl_mat::*;

pub(super) fn setup(app: &mut App) { app.add_plugins(MaterialPlugin::<BlockMaterial>::default()); }

#[derive(Debug, Default, Clone, TypePath, TypeUuid)]
#[uuid = "0059fd0b-5b43-46cc-bd77-c89130562e75"]
pub struct BlockMaterial {
    pub textures: Vec<Handle<Image>>,
    pub animation_info: Vec<BlockAnimation>,
    pub alpha_mode: AlphaMode,
}

pub const MAX_TEXTURE_COUNT: usize = 48;
