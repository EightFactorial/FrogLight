use bevy::{prelude::*, utils::HashMap};
use strum::{Display, EnumIter};

pub mod atlases;
use atlases::*;

mod traits;
pub use traits::AtlasFromWorld;

pub(super) fn setup(app: &mut App) { app.init_resource::<TextureAtlases>(); }

#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource)]
pub struct TextureAtlases {
    pub atlases: HashMap<TextureAtlasType, Handle<TextureAtlas>>,
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum TextureAtlasType {
    Icons,
}

impl From<TextureAtlasType> for Vec<Rect> {
    fn from(value: TextureAtlasType) -> Self {
        match value {
            TextureAtlasType::Icons => <IconAtlas as TextureAtlasData>::coords(),
        }
    }
}
