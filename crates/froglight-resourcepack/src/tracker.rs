use bevy::{asset::AssetPath, prelude::*};

use crate::{schedule::ResourcePackState, ResourcePack, ResourcePackManager};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Load the first resource pack when entering the `Loading` state
    app.add_systems(
        OnEnter(ResourcePackState::Loading),
        ResourcePackTracker::load_next_pack_in_queue.in_set(ResourcePackState::Loading),
    );

    // Load the next resource pack when the current one is finished
    app.add_systems(
        Update,
        ResourcePackTracker::load_next_pack_in_queue
            .run_if(ResourcePackTracker::on_load_event)
            .in_set(ResourcePackState::Loading),
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

    /// If there a resource pack was just loaded return [`true`]
    fn on_load_event(mut events: EventReader<AssetEvent<ResourcePack>>) -> bool {
        events.read().any(|event| matches!(event, AssetEvent::LoadedWithDependencies { .. }))
    }

    /// Load the next [`ResourcePack`] in the [queue](Self::packs)
    ///
    /// This system runs when either a [`ResourcePackStartLoadingEvent`] is sent
    /// or a [`ResourcePack`] is loaded.
    fn load_next_pack_in_queue(
        server: Res<AssetServer>,
        mut manager: ResMut<ResourcePackManager>,
        mut tracker: ResMut<Self>,

        mut state: ResMut<NextState<ResourcePackState>>,
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
            // Enter the `Processing` state
            debug!("Entering ResourcePackState::Processing");
            state.set(ResourcePackState::Processing);
        }
    }
}
