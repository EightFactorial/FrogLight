use bevy_app::{App, Update};
use bevy_asset::{AssetEvent, AssetPath, AssetServer};
use bevy_ecs::{
    event::{Event, EventReader, EventWriter},
    reflect::ReflectResource,
    schedule::{IntoSystemConfigs, OnEnter},
    system::{Res, ResMut, Resource},
};
use bevy_log::debug;
use bevy_reflect::Reflect;

use crate::{systemset::resourcepack::ResourcePackState, AssetManager, ResourcePack};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Register and add the `AssetTracker` resource
    app.register_type::<AssetTracker>().init_resource::<AssetTracker>();

    // Add the `ResourcePacksLoadedEvent` event
    app.add_event::<ResourcePackQueueFinished>();

    // Load the first resource pack when entering the `Loading` state
    app.add_systems(
        OnEnter(ResourcePackState::Loading),
        AssetTracker::load_next_pack_in_queue.in_set(ResourcePackState::Loading),
    );

    // Load the next resource pack when the current one is finished
    app.add_systems(
        Update,
        AssetTracker::load_next_pack_in_queue
            .run_if(AssetTracker::on_load_event)
            .in_set(ResourcePackState::Loading),
    );
}

/// A [`Resource`] for managing when to load resource packs
#[derive(Debug, Default, Clone, PartialEq, Eq, Resource, Reflect)]
#[reflect(Resource)]
pub struct AssetTracker {
    /// The list of resource packs to load
    pub(crate) packs: Vec<String>,

    /// The current index of the resource pack being loaded
    index: usize,
}

impl AssetTracker {
    /// Queue a resource pack to be loaded
    pub fn queue<'a>(&mut self, path: impl Into<AssetPath<'a>>) {
        self.packs.push(path.into().to_string());
    }

    /// If there a resource pack was just loaded return [`true`]
    fn on_load_event(mut events: EventReader<AssetEvent<ResourcePack>>) -> bool {
        events.read().any(|event| matches!(event, AssetEvent::LoadedWithDependencies { .. }))
    }

    /// Load the next [`ResourcePack`] in the [`queue`](Self::packs)
    ///
    /// This system runs when either a
    /// [`ResourcePackStartLoadingEvent`](froglight_core::events::ResourcePackStartLoadingEvent)
    /// is sent or a [`ResourcePack`] is loaded.
    fn load_next_pack_in_queue(
        server: Res<AssetServer>,
        manager: Res<AssetManager>,
        mut tracker: ResMut<Self>,

        mut events: EventWriter<ResourcePackQueueFinished>,
    ) {
        if tracker.index < tracker.packs.len() {
            // Get the next pack
            let path = tracker.packs[tracker.index].clone();
            debug!("Loading ResourcePack: {path}");

            // Load and add the pack to the manager
            let handle = server.load(&path);
            manager.handles.write().push(handle);

            // Increment the index
            tracker.index += 1;
        } else {
            debug!("Sending ResourcePacksLoadedEvent");
            events.send(ResourcePackQueueFinished);
        }
    }
}

/// An event that is sent when all queued [`ResourcePack`]s have been loaded
#[derive(Debug, Default, Clone, PartialEq, Eq, Event)]
pub(crate) struct ResourcePackQueueFinished;
