use bevy_app::{App, Update};
use bevy_asset::{Assets, Handle};
use bevy_audio::AudioSource;
use bevy_ecs::{
    prelude::not,
    reflect::ReflectResource,
    schedule::IntoSystemConfigs,
    system::{Res, ResMut, Resource},
};
use bevy_log::error;
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_state::state::OnEnter;
use bevy_utils::Entry;
use froglight_common::ResourceKey;

use super::sound::SoundState;
use crate::{
    assets::{
        processed::{
            sound_event::{SoundEntry, SoundEventStorage, SoundRef, SoundSettings},
            SoundEvent,
        },
        unprocessed::sound_definition::SoundKind,
        SoundDefinitionMap,
    },
    AssetCatalog, AssetLoadState, ResourcePack, ResourcePackList,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<SoundEventState>();

    // Reset the `SoundEventState` when entering `AssetLoadState::Processing`
    app.add_systems(OnEnter(AssetLoadState::Processing), SoundEventState::reset);

    // Generate `SoundEvent`s from the `ResourcePackList`
    app.add_systems(
        Update,
        SoundEventState::create_sound_events
            .ambiguous_with_all()
            .run_if(not(SoundEventState::is_finished))
            .run_if(SoundState::is_finished)
            .after(SoundState::catalog_sounds)
            .in_set(AssetLoadState::Processing),
    );
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Resource)]
#[reflect(Default, Resource)]
pub(super) struct SoundEventState {
    resource_index: usize,
    map_index: usize,
    sound_index: usize,
    finished: bool,
}

impl SoundEventState {
    /// The number of [`SoundEvent`]s to add to the [`AssetCatalog`] per frame.
    const SOUNDEVENTS_PER_FRAME: usize = 8;

    /// Returns `true` if the [`SoundEventState`] has finished.
    pub(super) const fn finished(&self) -> bool { self.finished }

    /// Returns `true` if the [`SoundEventState`] has finished.
    fn is_finished(res: Res<Self>) -> bool { res.finished() }

    /// Create [`SoundEvent`]s from the [`SoundDefinitionMap`]s.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn create_sound_events(
        list: Res<ResourcePackList>,
        pack_assets: Res<Assets<ResourcePack>>,
        def_assets: Res<Assets<SoundDefinitionMap>>,

        mut event_assets: ResMut<Assets<SoundEvent>>,
        mut event_storage: ResMut<SoundEventStorage>,
        mut sound_assets: ResMut<Assets<AudioSource>>,

        mut state: ResMut<Self>,
        mut catalog: ResMut<AssetCatalog>,
    ) {
        if Self::generate_sound_events(
            &list,
            &pack_assets,
            &def_assets,
            &mut event_assets,
            &mut event_storage,
            &mut state,
            &mut catalog,
        ) {
            Self::fix_sound_event_refs(&catalog, &mut state, &mut event_assets, &mut sound_assets);
            state.finished = true;
        }
    }

    /// Generate [`SoundEvent`]s from the [`SoundDefinitionMap`]s.
    ///
    /// Returns `true` if all [`SoundEvent`]s have been generated.
    fn generate_sound_events(
        list: &ResourcePackList,
        pack_assets: &Assets<ResourcePack>,
        def_assets: &Assets<SoundDefinitionMap>,

        event_assets: &mut Assets<SoundEvent>,
        event_storage: &mut SoundEventStorage,

        state: &mut Self,
        catalog: &mut AssetCatalog,
    ) -> bool {
        let handle = list.get(state.resource_index).expect("ResourceIndex out of bounds");
        let resource = pack_assets.get(handle).expect("ResourcePack not found");

        let mut processed_events = 0usize;
        let mut total_sounds = 0usize;
        for (namespace, handle) in resource.sound_definitions.iter().skip(state.map_index) {
            let definition_map = def_assets.get(handle).expect("SoundDefinitionMap not found");
            total_sounds = definition_map.len();

            for (key, definition) in definition_map
                .iter()
                .skip(state.sound_index)
                .take(Self::SOUNDEVENTS_PER_FRAME - processed_events)
            {
                let Ok(sound_key) = ResourceKey::try_new(format!("{namespace}:{key}")) else {
                    error!("SoundEvent: Bad Name \"{namespace}:{key}\"");
                    processed_events += 1;
                    continue;
                };

                // Get the catalog entry for the SoundEvent
                let entry = catalog.entry::<SoundEvent>(sound_key);

                // Skip if the entry already exists, unless it's set to replace
                if let Entry::Occupied(..) = entry {
                    if !definition.replace.unwrap_or_default() {
                        processed_events += 1;
                        continue;
                    }
                }

                // Create the `SoundEvent`
                let event_handle = {
                    let mut total_weight = 0;
                    let mut sound_pool = Vec::new();

                    if let Some(definitions) = definition.sounds.as_ref() {
                        for sound in definitions {
                            let Ok(sound_ref_name) =
                                ResourceKey::try_new(format!("{namespace}:{}", sound.get_name()))
                            else {
                                error!("SoundEvent: Bad Name \"{namespace}:{}\"", sound.get_name());
                                continue;
                            };

                            let weight = sound.get_weight();
                            total_weight += weight;

                            sound_pool.push(SoundEntry {
                                // Note: These are placeholder handles!
                                //
                                // These *must* be replaced with the actual handles,
                                // which is done in the `finish_sound_events` system.
                                sound_ref: match sound.get_kind() {
                                    SoundKind::File => SoundRef::Audio(Handle::default()),
                                    SoundKind::Event => SoundRef::Event(Handle::default()),
                                },
                                sound_ref_name,
                                settings: SoundSettings::from(sound),
                                weight,
                            });
                        }
                    }

                    event_assets.add(SoundEvent {
                        subtitle: definition.subtitle.clone(),
                        total_weight,
                        sound_pool,
                    })
                };

                // Insert the `SoundEvent` into the `AssetCatalog`
                entry.or_insert(event_handle.id().untyped());
                // Add the `SoundEvent` to the `SoundEventStorage`
                event_storage.push(event_handle);

                processed_events += 1;
                state.sound_index += 1;
            }

            // If we've processed all the sounds in the definition map, move to the next map
            if state.sound_index >= definition_map.len() {
                state.sound_index = 0;
                state.map_index += 1;
            }
        }

        match (
            state.resource_index >= list.len().checked_sub(1).unwrap_or_default(),
            state.map_index >= resource.sound_definitions.len().checked_sub(1).unwrap_or_default(),
            state.sound_index >= total_sounds.checked_sub(1).unwrap_or_default(),
        ) {
            (true, true, true) => {
                #[cfg(debug_assertions)]
                {
                    bevy_log::info!("AssetCatalog: Finished Cataloging SoundEvents");
                    bevy_log::debug!(
                        "AssetCatalog: {} SoundEvents",
                        catalog.len_of::<SoundEvent>()
                    );
                }

                // We're done creating SoundEvents, fix the SoundRef handles
                return true;
            }
            (false, true, true) => {
                state.resource_index += 1;
                state.map_index = 0;
                state.sound_index = 0;
            }
            (false, false, true) => {
                state.map_index = 0;
                state.sound_index = 0;
            }
            _ => {}
        }

        // We're not done creating SoundEvents
        false
    }

    /// Fix the [`SoundRef`] handles in the [`SoundEvent`]s.
    fn fix_sound_event_refs(
        catalog: &AssetCatalog,

        state: &mut Self,
        event_assets: &mut Assets<SoundEvent>,
        sound_assets: &mut Assets<AudioSource>,
    ) {
        // Clone all of the SoundEvents so we can modify them
        let events = event_assets.iter().map(|(i, e)| (i, e.clone())).collect::<Vec<_>>();

        // Fix the `SoundRef` handles
        for (id, mut event) in events {
            // Iterate over the `SoundEntries` in the `SoundEvent`
            for entry in &mut event.sound_pool {
                // Update the `SoundRef` handle with the actual handle
                match &mut entry.sound_ref {
                    SoundRef::Event(handle) => {
                        if let Some(new_handle) =
                            catalog.create_handle(&entry.sound_ref_name, event_assets)
                        {
                            *handle = new_handle;
                        } else {
                            error!("SoundEvent: Event not found: \"{}\"", entry.sound_ref_name);
                        }
                    }
                    SoundRef::Audio(handle) => {
                        if let Some(new_handle) =
                            catalog.create_handle(&entry.sound_ref_name, sound_assets)
                        {
                            *handle = new_handle;
                        } else {
                            error!("SoundEvent: Audio not found: \"{}\"", entry.sound_ref_name);
                        }
                    }
                }
            }

            // Update the SoundEvent in the asset storage
            event_assets.insert(id, event);
        }

        #[cfg(debug_assertions)]
        bevy_log::debug!("AssetCatalog: Finished Fixing SoundEvent Handles");
        state.finished = true;
    }

    /// Resets the [`SoundState`].
    fn reset(mut res: ResMut<Self>) {
        res.resource_index = 0;
        res.map_index = 0;
        res.sound_index = 0;
        res.finished = false;
    }
}
