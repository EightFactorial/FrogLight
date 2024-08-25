use bevy_app::App;

mod blockmodel;
pub use blockmodel::BlockModelProcessor;

mod blockstate;
pub use blockstate::BlockStateProcessor;

mod sound;
pub use sound::SoundProcessor;

mod sound_map;
pub use sound_map::SoundMapProcessor;

mod texture;
pub use texture::TextureProcessor;

#[doc(hidden)]
pub(crate) fn build(app: &mut App) {
    blockmodel::build(app);
    blockstate::build(app);
    sound::build(app);
    sound_map::build(app);
    texture::build(app);
}
