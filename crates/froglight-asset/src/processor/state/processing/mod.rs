use bevy_app::App;

mod sound;
pub use sound::SoundProcessor;

mod sound_map;
pub use sound_map::SoundMapProcessor;

mod texture;
pub use texture::TextureProcessor;

#[doc(hidden)]
pub(crate) fn build(app: &mut App) {
    sound::build(app);
    sound_map::build(app);
    texture::build(app);
}
