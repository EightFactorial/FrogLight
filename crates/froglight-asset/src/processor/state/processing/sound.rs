use bevy_app::{App, Update};
use bevy_asset::Assets;
use bevy_audio::AudioSource;
use bevy_ecs::{
    prelude::not,
    reflect::ReflectResource,
    schedule::IntoSystemConfigs,
    system::{Res, ResMut, Resource},
};
use bevy_log::{debug, error};
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_state::state::OnEnter;

use crate::{AssetCatalog, AssetProcess, ResourcePack, ResourcePackList};

#[doc(hidden)]
pub(crate) fn build(app: &mut App) {
    app.register_type::<SoundProcessor>();
    app.init_resource::<SoundProcessor>();

    // Reset the `SoundProcessor` state
    app.add_systems(OnEnter(AssetProcess::Processing), SoundProcessor::reset_sound_state);
    // Clear the `AssetCatalog` sounds
    app.add_systems(OnEnter(AssetProcess::Processing), SoundProcessor::clear_catalog_sounds);

    // Catalog sounds
    app.add_systems(
        Update,
        SoundProcessor::catalog_sounds
            .run_if(not(SoundProcessor::is_finished))
            .ambiguous_with_all()
            .in_set(AssetProcess::Processing),
    );
}

/// A processor that catalogs sounds in the [`AssetCatalog`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct SoundProcessor {
    resource_index: usize,
    sound_index: usize,
    finished: bool,
}

impl SoundProcessor {
    /// A [`Condition`](bevy_ecs::schedule::Condition) that checks if the
    /// [`SoundProcessor`] is finished.
    #[must_use]
    pub fn is_finished(res: Res<Self>) -> bool { res.finished }

    /// A [`System`](bevy_ecs::system::System) that adds sounds to the
    /// [`AssetCatalog`] in batches.
    ///
    /// [`ResourcePack`]s are processed in the same order as they are in the
    /// [`ResourcePackList`].
    pub fn catalog_sounds(
        resources: Res<ResourcePackList>,
        mut assets: ResMut<Assets<ResourcePack>>,
        mut catalog: ResMut<AssetCatalog>,
        mut state: ResMut<Self>,
    ) {
        let _ = Self::catalog_sound_batch(&resources, &mut assets, &mut catalog, &mut state);

        // Check if the processor is finished.
        if state.resource_index >= resources.len() {
            #[cfg(debug_assertions)]
            bevy_log::info!("SoundProcessor: Finished");
            debug!("SoundProcessor: Cataloged {} Sounds", catalog.len_of::<AudioSource>());
            // Set the processor to finished.
            *state = Self { finished: true, ..Self::default() };
        }
    }

    /// The number of sounds to process per frame.
    const SOUNDS_PER_FRAME: usize = 50;

    /// Catalogs a batch of sounds.
    fn catalog_sound_batch(
        resources: &ResourcePackList,
        assets: &mut Assets<ResourcePack>,
        catalog: &mut AssetCatalog,
        state: &mut SoundProcessor,
    ) -> Result<(), ()> {
        // Get the current ResourcePack.
        let handle = resources.get(state.resource_index).ok_or(())?;
        let asset = assets.get_mut(handle).ok_or_else(|| {
            error!("SoundProcessor: ResourcePack Asset missing!");
            state.resource_index += 1;
        })?;

        // Iterate over the next `SOUNDS_PER_FRAME` sounds.
        let mut typed_catalog = catalog.typed_mut::<AudioSource>();
        for (sound_key, sound_handle) in
            asset.sounds.iter_mut().skip(state.sound_index).take(Self::SOUNDS_PER_FRAME)
        {
            // Replace the existing strong handle with a weak handle.
            let sound_handle = std::mem::replace(sound_handle, sound_handle.clone_weak());

            // Add the taken strong handle to the catalog, if it doesn't already exist.
            typed_catalog.entry(sound_key.to_owned()).or_insert(sound_handle.untyped());

            // Increment the sound index.
            state.sound_index += 1;
        }

        // If the sound index is at the end of the sounds,
        // increment the resource index.
        if state.sound_index >= asset.sounds.len() {
            state.resource_index += 1;
            state.sound_index = 0;
        }

        Ok(())
    }

    /// Resets the state of the [`SoundProcessor`].
    fn reset_sound_state(mut res: ResMut<Self>) {
        #[cfg(debug_assertions)]
        bevy_log::trace!("SoundProcessor: Resetting state");
        *res = Self::default();
    }

    /// Clears all sounds from the [`AssetCatalog`].
    fn clear_catalog_sounds(mut catalog: ResMut<AssetCatalog>) {
        #[cfg(debug_assertions)]
        bevy_log::trace!("SoundProcessor: Clearing AssetCatalog Sounds");
        catalog.clear_of::<AudioSource>();
    }
}
