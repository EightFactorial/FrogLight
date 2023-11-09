use bevy::prelude::*;
use compact_str::CompactString;
use mc_rs_core::ResourceLocation;

use crate::configs::settings::Settings;

use super::ResourcePackAsset;

/// A collection of resourcepacks loaded.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource)]
pub struct ResourcePacks {
    pub packs: Vec<ResourcePackInfo>,
}

/// Information about a resourcepack.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourcePackInfo {
    pub path: CompactString,
    pub loaded: bool,

    pub handle: Handle<ResourcePackAsset>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct ResourcePacksStartReloadEvent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct ResourcePacksFinishReloadEvent;

pub(super) fn setup(app: &mut App) {
    app.add_event::<ResourcePacksStartReloadEvent>()
        .add_event::<ResourcePacksFinishReloadEvent>()
        .init_resource::<ResourcePacks>();

    app.add_systems(
        Update,
        (
            ResourcePacks::resourcepack_events,
            ResourcePacks::update.run_if(
                resource_exists_and_changed::<Settings>()
                    .and_then(ResourcePacks::resourcepacks_changed),
            ),
        ),
    );
}

impl ResourcePacks {
    /// Listen for resourcepacks to be loaded.
    fn resourcepack_events(
        mut packs: ResMut<ResourcePacks>,
        mut asset_events: EventReader<AssetEvent<ResourcePackAsset>>,
        mut loaded_event: EventWriter<ResourcePacksFinishReloadEvent>,
    ) {
        asset_events.read().for_each(|event| match event {
            AssetEvent::Removed { id } => {
                // Remove the pack from the list
                packs.packs.retain(|pack| &pack.handle.id() != id);
            }
            AssetEvent::LoadedWithDependencies { id } => {
                // Mark the pack as loaded
                if let Some(pack) = packs.packs.iter_mut().find(|pack| &pack.handle.id() == id) {
                    pack.loaded = true;

                    // If all packs are loaded, send the event
                    if packs.packs.iter().all(|pack| pack.loaded) {
                        loaded_event.send(ResourcePacksFinishReloadEvent);
                    }
                }
            }
            _ => {}
        });
    }

    /// Check if no resourcepacks are loaded or if the resourcepacks list has changed.
    fn resourcepacks_changed(settings: Res<Settings>, packs: Res<ResourcePacks>) -> bool {
        packs.packs.is_empty()
            || settings
                .resourcepacks
                .paths
                .iter()
                .zip(packs.iter())
                .any(|(incoming, existing)| incoming != &existing.path)
    }

    /// Reload the resourcepacks.
    fn update(
        assets: Res<AssetServer>,
        settings: Res<Settings>,
        mut packs: ResMut<ResourcePacks>,
        mut events: EventWriter<ResourcePacksStartReloadEvent>,
    ) {
        packs.packs.clear();

        for path in &settings.resourcepacks.paths {
            let handle: Handle<ResourcePackAsset> = assets.load(format!("resourcepack://{}", path));

            packs.packs.push(ResourcePackInfo {
                path: path.clone(),
                loaded: false,

                handle,
            });
        }

        events.send(ResourcePacksStartReloadEvent);
    }

    /// Get a texture from the resourcepacks.
    pub fn get_texture(
        &self,
        id: &ResourceLocation,
        assets: &Assets<ResourcePackAsset>,
    ) -> Option<Handle<Image>> {
        // Loop through all pack handles
        for pack in self.packs.iter().rev() {
            if let Some(pack) = assets.get(&pack.handle) {
                // If the pack has the texture, return it
                if let Some(texture) = pack.textures.get(id) {
                    return Some(texture.clone());
                }
            }
        }

        None
    }
}
