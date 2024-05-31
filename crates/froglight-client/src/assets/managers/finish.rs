use bevy::prelude::*;

use super::{
    AssetManager, BlockManager, FontManager, LanguageManager, ParticleManager, SoundManager,
};
use crate::assets::{AssetLoading, ModelManager};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.add_systems(
        Update,
        finish_processing
            .run_if(AssetManager::is_finished)
            .after(AssetManager::populate_asset_manager)
            .run_if(BlockManager::is_finished)
            .after(BlockManager::process_blockstates)
            .run_if(FontManager::is_finished)
            .after(FontManager::populate_font_manager)
            .run_if(LanguageManager::is_finished)
            .after(LanguageManager::populate_language_manager)
            .run_if(ModelManager::is_finished)
            .after(ModelManager::populate_model_manager)
            .run_if(ParticleManager::is_finished)
            .after(ParticleManager::populate_particle_manager)
            .run_if(SoundManager::is_finished)
            .after(SoundManager::populate_sound_manager)
            .in_set(AssetLoading::Processing),
    );
}

/// Set the [`AssetLoading`] state to [`AssetLoading::Finished`] when all
/// asset managers are finished.
fn finish_processing(mut state: ResMut<NextState<AssetLoading>>) {
    #[cfg(debug_assertions)]
    info!("Finished loading assets");
    state.set(AssetLoading::Finished);
}
