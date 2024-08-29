//! Assets processed into usable forms.

use bevy_app::App;

mod atlas;
pub use atlas::BlockAtlas;

pub mod model;
pub use model::{
    BlockModel,
    BlockModelCache,
    // ItemModel
};

pub mod sound;
pub use sound::{SoundMap, SoundSet};

mod texture;
pub use texture::FallbackTexture;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    atlas::build(app);
    model::build(app);
    sound::build(app);
    texture::build(app);
}
