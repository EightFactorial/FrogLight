use bevy::{asset::AssetPath, prelude::*};
use froglight_core::{
    events::{ResourcePackEndLoadingEvent, ResourcePackStartLoadingEvent},
    systemsets::ResourcePackUpdateSet,
};

use crate::{ResourcePack, ResourcePackManager};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.add_systems(
        Update,
        ResourcePackTracker::load_next_pack_in_queue
            .run_if(ResourcePackTracker::on_load_event)
            .in_set(ResourcePackUpdateSet),
    );
}

/// A [`Resource`] for managing when to load resource packs
#[derive(Debug, Default, Clone, PartialEq, Eq, Resource, Reflect)]
#[reflect(Resource)]
pub struct ResourcePackTracker {
    /// The list of resource packs to load
    pub packs: Vec<String>,

    /// The current index of the resource pack being loaded
    index: usize,
}

impl ResourcePackTracker {
    /// Queue a resource pack to be loaded
    pub fn queue<'a>(&mut self, path: impl Into<AssetPath<'a>>) {
        self.packs.push(path.into().to_string());
    }

    /// If there is an [`Event`] to start loading, or if a resource pack
    /// was just loaded, return [`true`]
    fn on_load_event(
        mut load_events: EventReader<ResourcePackStartLoadingEvent>,
        mut asset_events: EventReader<AssetEvent<ResourcePack>>,
    ) -> bool {
        load_events.read().next().is_some()
            || asset_events
                .read()
                .any(|event| matches!(event, AssetEvent::LoadedWithDependencies { .. }))
    }

    /// Load the next [`ResourcePack`] in the [queue](Self::packs)
    ///
    /// This system runs when either a [`ResourcePackStartLoadingEvent`] is sent
    /// or a [`ResourcePack`] is loaded.
    fn load_next_pack_in_queue(
        server: Res<AssetServer>,
        mut manager: ResMut<ResourcePackManager>,
        mut tracker: ResMut<Self>,

        mut events: EventWriter<ResourcePackEndLoadingEvent>,
    ) {
        if tracker.index < tracker.packs.len() {
            // Load the next resource pack
            let path = tracker.packs[tracker.index].clone();
            debug!("Loading ResourcePack: {path}");

            let handle = manager.load_resourcepack(path, &server);

            // Add the handle to the manager
            manager.handles.push(handle);
            tracker.index += 1;
        } else {
            debug!("Finished loading queued ResourcePacks");

            // Send the end loading event
            events.send(ResourcePackEndLoadingEvent);
        }
    }
}
