use bevy_app::{App, Update};
use bevy_ecs::{
    schedule::IntoSystemConfigs,
    system::{Res, ResMut},
};
use bevy_state::state::NextState;

mod block_model;
use block_model::BlockModelState;

mod resource_atlas;
use resource_atlas::ResourceAtlasState;

mod sound;
use sound::SoundState;

mod sound_event;
use sound_event::SoundEventState;

mod texture;
use texture::TextureState;

use super::AssetLoadState;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    texture::build(app);
    resource_atlas::build(app);
    block_model::build(app);

    sound::build(app);
    sound_event::build(app);

    app.add_systems(
        Update,
        finish_processing
            .after(SoundState::catalog_sounds)
            .after(SoundEventState::create_sound_events)
            .after(TextureState::catalog_textures)
            .after(ResourceAtlasState::create_resource_atlases)
            .after(BlockModelState::create_block_models)
            .run_if(is_finished)
            .in_set(AssetLoadState::Processing),
    );
}

/// Returns `true` if all assets have been processed.
fn is_finished(
    textures: Res<TextureState>,
    resource_atlas: Res<ResourceAtlasState>,
    block_models: Res<BlockModelState>,
    sounds: Res<SoundState>,
    sound_events: Res<SoundEventState>,
) -> bool {
    textures.finished()
        && resource_atlas.finished()
        && block_models.finished()
        && sounds.finished()
        && sound_events.finished()
}

fn finish_processing(mut state: ResMut<NextState<AssetLoadState>>) {
    #[cfg(debug_assertions)]
    bevy_log::info!("AssetLoadState: Entering `AssetLoadState::Spawning`");
    state.set(AssetLoadState::Spawning);
}
