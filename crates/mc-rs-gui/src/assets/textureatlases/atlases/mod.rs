use std::fmt::Debug;

use bevy::math::Rect;
use mc_rs_core::ResourceLocation;

mod icons;
pub use icons::IconAtlas;

pub trait TextureAtlasData: Debug + 'static {
    fn path() -> ResourceLocation;
    fn coords() -> Vec<Rect>;
}
