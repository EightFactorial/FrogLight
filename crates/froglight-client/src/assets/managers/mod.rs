use bevy::app::App;

mod asset_manager;
pub use asset_manager::AssetManager;

mod sound_manager;
pub use sound_manager::{SoundEvent, SoundManager};

mod lang_manager;
pub use lang_manager::LanguageManager;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    asset_manager::build(app);
    sound_manager::build(app);
    lang_manager::build(app);
}
