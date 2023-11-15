use bevy::prelude::*;

use crate::resourcepacks::ResourcePackAsset;

use super::ResourcePacks;

/// An event that is sent when the resourcepacks start loading.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct ResourcePacksStartLoadEvent;

/// An event that is sent when the resourcepacks finish loading.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct ResourcePacksFinishLoadEvent;

/// An event that is sent when all processing of resourcepacks is finished.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct ResourcePacksFinishEvent;

/// Process [ResourcePackAsset] events.
pub(super) fn finish_event(
    mut packs: ResMut<ResourcePacks>,
    mut asset_events: EventReader<AssetEvent<ResourcePackAsset>>,
    mut load_event: EventWriter<ResourcePacksFinishLoadEvent>,
) {
    asset_events.read().for_each(|event| match event {
        AssetEvent::Removed { id } => {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Removing ResourcePackAsset: {id}");

            packs.packs.retain(|pack| &pack.handle.id() != id);
        }
        AssetEvent::LoadedWithDependencies { id } => {
            // Mark the pack as loaded
            if let Some(pack) = packs.packs.iter_mut().find(|pack| &pack.handle.id() == id) {
                pack.loaded = true;

                // If all packs are loaded, send the event
                if packs.packs.iter().all(|pack| pack.loaded) {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    debug!("Sending ResourcePacksFinishReloadEvent!");

                    // Send an event to notify that all packs are loaded
                    load_event.send(ResourcePacksFinishLoadEvent);
                }
            }
        }
        _ => {}
    });
}
