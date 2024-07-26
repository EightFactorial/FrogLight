//! Processed assets that can be used directly in the game.

use bevy_app::App;

pub(crate) mod block_model;
pub use block_model::{BlockDataStorage, BlockModel, ModelTransformIndex};

pub(crate) mod resource_atlas;
pub use resource_atlas::ResourceAtlas;

pub(crate) mod sound_event;
pub use sound_event::SoundEvent;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    block_model::build(app);
    resource_atlas::build(app);
    sound_event::build(app);
}
