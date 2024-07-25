use bevy_app::{App, Update};
use bevy_ecs::{
    schedule::IntoSystemConfigs,
    system::{Res, ResMut},
};

mod sounds;
use bevy_state::state::NextState;
use sounds::SoundState;

mod textures;
use textures::TextureState;

use super::AssetLoadState;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    sounds::build(app);
    textures::build(app);

    app.add_systems(
        Update,
        finish_processing
            .after(SoundState::catalog_sounds)
            .after(TextureState::catalog_textures)
            .run_if(is_finished)
            .in_set(AssetLoadState::Processing),
    );
}

/// Returns `true` if all assets have been processed.
fn is_finished(sounds: Res<SoundState>, textures: Res<TextureState>) -> bool {
    sounds.finished() && textures.finished()
}

fn finish_processing(mut state: ResMut<NextState<AssetLoadState>>) {
    #[cfg(debug_assertions)]
    bevy_log::info!("AssetLoadState: Entering `AssetLoadState::Spawning`");
    state.set(AssetLoadState::Spawning);
}
