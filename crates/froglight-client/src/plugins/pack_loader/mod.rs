use bevy::prelude::*;
use froglight_core::events::ResourcePackStartLoadingEvent;
use froglight_resourcepack::ResourcePackTracker;
use froglight_settings::ResourcePackSettings;

/// A [`Plugin`] that takes the list of resource packs from
/// [`ResourcePackSettings`] and loads them.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ResourcePackLoaderPlugin;

impl Plugin for ResourcePackLoaderPlugin {
    fn build(&self, app: &mut App) {
        // Configure the set to run at startup
        app.configure_sets(Startup, ResourcePackLoaderStartupSet.ambiguous_with_all());

        // Add the systems to the systemset
        app.add_systems(Startup, Self::queue_resource_packs.in_set(ResourcePackLoaderStartupSet));
    }
}

impl ResourcePackLoaderPlugin {
    /// Queues resource packs loaded from [`ResourcePackSettings`]
    /// to be loaded by the [`ResourcePackTracker`].
    fn queue_resource_packs(
        settings: Res<ResourcePackSettings>,
        mut tracker: ResMut<ResourcePackTracker>,
        mut events: EventWriter<ResourcePackStartLoadingEvent>,
    ) {
        for pack in &settings.packs {
            debug!("Queueing ResourcePack: {}", pack.path);
            tracker.queue(&pack.path);
        }

        if !settings.packs.is_empty() {
            debug!("Sending ResourcePackStartLoadingEvent");
            events.send(ResourcePackStartLoadingEvent);
        }
    }
}

/// A [`SystemSet`] that runs [`ResourcePackLoaderPlugin`]
/// [`System`]s at [`Startup`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
struct ResourcePackLoaderStartupSet;
