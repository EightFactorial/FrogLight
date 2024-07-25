use bevy_app::{App, Update};
use bevy_ecs::{
    schedule::IntoSystemConfigs,
    system::{Res, ResMut},
};
use bevy_state::state::NextState;

mod sound;
use sound::SoundState;

mod sound_event;
use sound_event::SoundEventState;

mod texture;
use texture::TextureState;

use super::AssetLoadState;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    sound::build(app);
    texture::build(app);
    sound_event::build(app);

    app.add_systems(
        Update,
        finish_processing
            .after(SoundState::catalog_sounds)
            .after(TextureState::catalog_textures)
            .after(SoundEventState::finish_sound_events)
            .run_if(is_finished)
            .in_set(AssetLoadState::Processing),
    );
}

/// Returns `true` if all assets have been processed.
fn is_finished(
    sounds: Res<SoundState>,
    textures: Res<TextureState>,
    sound_events: Res<SoundEventState>,
) -> bool {
    sounds.finished() && textures.finished() && sound_events.finished()
}

fn finish_processing(mut state: ResMut<NextState<AssetLoadState>>) {
    #[cfg(debug_assertions)]
    bevy_log::info!("AssetLoadState: Entering `AssetLoadState::Spawning`");
    state.set(AssetLoadState::Spawning);
}
