use bevy_app::App;

mod texture;
pub use texture::TextureProcessor;

#[doc(hidden)]
pub(crate) fn build(app: &mut App) { texture::build(app); }
