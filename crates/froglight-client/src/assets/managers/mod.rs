use bevy::app::App;

mod asset_manager;
pub use asset_manager::AssetManager;

mod atlas_manager;
pub use atlas_manager::AtlasManager;

mod block_manager;
pub use block_manager::BlockManager;

mod font_manager;
pub use font_manager::FontManager;

mod lang_manager;
pub use lang_manager::LanguageManager;

pub mod model_manager;
pub use model_manager::ModelManager;

mod particle_manager;
pub use particle_manager::{ParticleEvent, ParticleManager};

mod sound_manager;
pub use sound_manager::{SoundEvent, SoundManager};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    asset_manager::build(app);
    atlas_manager::build(app);
    block_manager::build(app);
    font_manager::build(app);
    lang_manager::build(app);
    model_manager::build(app);
    particle_manager::build(app);
    sound_manager::build(app);
}
