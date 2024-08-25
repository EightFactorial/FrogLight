use bevy_app::{App, Update};
use bevy_asset::Assets;
use bevy_ecs::{
    prelude::not,
    reflect::ReflectResource,
    schedule::IntoSystemConfigs,
    system::{Res, ResMut, Resource},
};
use bevy_log::debug;
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_state::state::OnEnter;
use bevy_utils::Entry;
use froglight_common::ResourceKey;

use super::SoundProcessor;
use crate::{
    assets::{
        processed::{SoundMap, SoundSet},
        raw::SoundDefinitionMap,
    },
    AssetCatalog, AssetProcess, ResourcePack, ResourcePackList,
};

#[doc(hidden)]
pub(crate) fn build(app: &mut App) {
    app.register_type::<SoundMapProcessor>();
    app.init_resource::<SoundMapProcessor>();

    // Reset the `SoundMapProcessor` state
    app.add_systems(OnEnter(AssetProcess::Processing), SoundMapProcessor::reset_soundmap_state);
    // Clear the `SoundMap`
    app.add_systems(OnEnter(AssetProcess::Processing), SoundMapProcessor::clear_soundmap);

    // Catalog sounds
    app.add_systems(
        Update,
        SoundMapProcessor::map_sounds
            .after(SoundProcessor::catalog_sounds)
            .run_if(SoundProcessor::is_finished)
            .run_if(not(SoundMapProcessor::is_finished))
            .ambiguous_with_all()
            .in_set(AssetProcess::Processing),
    );
}

/// A processor that combines [`SoundDefinitionMap`]s into a [`SoundMap`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct SoundMapProcessor {
    resource_index: usize,
    finished: bool,
}

impl SoundMapProcessor {
    /// A [`Condition`](bevy_ecs::schedule::Condition) that checks if the
    /// [`SoundMapProcessor`] is finished.
    #[must_use]
    pub fn is_finished(res: Res<Self>) -> bool { res.finished }

    /// A [`System`](bevy_ecs::system::System) that combines all
    /// [`SoundDefinitionMap`]s together into a [`SoundMap`].
    pub fn map_sounds(
        resources: Res<ResourcePackList>,
        catalog: Res<AssetCatalog>,

        assets: Res<Assets<ResourcePack>>,
        definitions: Res<Assets<SoundDefinitionMap>>,
        mut soundmap: ResMut<SoundMap>,
        mut state: ResMut<Self>,
    ) {
        if let Some(asset) = resources.get(state.resource_index).and_then(|r| assets.get(r)) {
            for (map_key, def_map) in &asset.sound_maps {
                let namespace = map_key.namespace();
                let Some(def_map) = definitions.get(def_map) else {
                    continue;
                };

                // Iterate over the SoundDefinitions in the SoundDefinitionMap
                for (sound_name, sound_def) in def_map.iter() {
                    // Create a ResourceKey for the SoundSet
                    let Ok(sound_key) = ResourceKey::try_new(format!("{namespace}:{sound_name}"))
                    else {
                        continue;
                    };

                    // Insert the SoundSet into the SoundMap
                    match soundmap.entry(sound_key) {
                        // Insert the SoundSet if it doesn't exist
                        Entry::Vacant(entry) => {
                            entry.insert(SoundSet::from_definition(sound_def, &catalog));
                        }
                        // Replace the existing SoundSet if the SoundDefinition says to
                        Entry::Occupied(mut entry) => {
                            if sound_def.replace() {
                                entry.insert(SoundSet::from_definition(sound_def, &catalog));
                            }
                        }
                    }
                }
            }
        }

        // Increment the ResourcePack index
        state.resource_index += 1;

        // Check if we've finished processing all ResourcePacks
        if state.resource_index >= resources.len() {
            #[cfg(debug_assertions)]
            bevy_log::info!("SoundMapProcessor: Finished");
            debug!("SoundMapProcessor: Created {} SoundSets", soundmap.len());
            *state = Self { finished: true, ..Self::default() };
        }
    }

    /// Resets the state of the [`SoundMapProcessor`].
    fn reset_soundmap_state(mut res: ResMut<Self>) {
        #[cfg(debug_assertions)]
        bevy_log::trace!("SoundMapProcessor: Resetting state");
        *res = Self::default();
    }

    /// Clears the [`SoundMap`].
    fn clear_soundmap(mut soundmap: ResMut<SoundMap>) {
        #[cfg(debug_assertions)]
        bevy_log::info!("SoundMapProcessor: Clearing SoundMap");
        soundmap.clear();
    }
}
