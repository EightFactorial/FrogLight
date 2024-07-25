use bevy_app::{App, Update};
use bevy_asset::Assets;
use bevy_audio::AudioSource;
use bevy_ecs::{
    prelude::not,
    reflect::ReflectResource,
    schedule::IntoSystemConfigs,
    system::{Res, ResMut, Resource},
};
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_state::state::OnEnter;

use crate::{AssetCatalog, AssetLoadState, ResourcePack, ResourcePackList};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<SoundState>();

    // Reset the `SoundState` when entering `AssetLoadState::Processing`
    app.add_systems(OnEnter(AssetLoadState::Processing), SoundState::reset);

    // Catalog sounds from the `ResourcePackList`
    app.add_systems(
        Update,
        SoundState::catalog_sounds
            .ambiguous_with_all()
            .run_if(not(SoundState::is_finished))
            .in_set(AssetLoadState::Processing),
    );
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Resource)]
#[reflect(Default, Resource)]
pub(super) struct SoundState {
    resource_index: usize,
    sound_index: usize,
    finished: bool,
}

impl SoundState {
    /// The number of sounds to add to the [`AssetCatalog`] per frame.
    const SOUNDS_PER_FRAME: usize = 8;

    /// Returns `true` if the [`SoundState`] has finished.
    pub(super) const fn finished(&self) -> bool { self.finished }

    /// Returns `true` if the [`SoundState`] has finished.
    pub(super) fn is_finished(res: Res<Self>) -> bool { res.finished() }

    /// Catalogs sounds from the [`ResourcePackList`].
    pub(super) fn catalog_sounds(
        list: Res<ResourcePackList>,
        assets: Res<Assets<ResourcePack>>,

        mut state: ResMut<Self>,
        mut catalog: ResMut<AssetCatalog>,
    ) {
        let handle = list.get(state.resource_index).expect("ResourceIndex out of bounds");
        let resource = assets.get(handle).expect("ResourcePack not found");

        for (key, handle) in
            resource.sounds.iter().skip(state.sound_index).take(Self::SOUNDS_PER_FRAME)
        {
            catalog.entry::<AudioSource>(key.clone()).or_insert(handle.id().untyped());
            state.sound_index += 1;
        }

        match (
            state.resource_index >= list.len() - 1,
            state.sound_index >= resource.sounds.len() - 1,
        ) {
            (true, true) => {
                #[cfg(debug_assertions)]
                {
                    bevy_log::info!("AssetCatalog: Finished Cataloging Sounds");
                    bevy_log::debug!("AssetCatalog: {} Sounds", catalog.len_of::<AudioSource>());
                }

                state.finished = true;
            }
            (false, true) => {
                state.resource_index += 1;
                state.sound_index = 0;
            }
            _ => {}
        }
    }

    /// Resets the [`SoundState`].
    fn reset(mut res: ResMut<Self>) {
        res.resource_index = 0;
        res.sound_index = 0;
        res.finished = false;
    }
}
