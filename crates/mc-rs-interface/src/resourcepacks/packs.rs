use bevy::prelude::*;
use compact_str::CompactString;
use mc_rs_core::ResourceLocation;

use crate::configs::settings::Settings;

use super::ResourcePackAsset;

/// A collection of resourcepacks loaded.
///
/// Use this to get textures loaded from resourcepacks.
#[derive(Debug, Clone, PartialEq, Eq, Resource)]
pub struct ResourcePacks {
    packs: Vec<ResourcePackInfo>,

    pub fallback: Handle<Image>,
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
    app.add_systems(Startup, ResourcePacks::default);

    app.add_event::<ResourcePacksStartReloadEvent>()
        .add_event::<ResourcePacksFinishReloadEvent>();

    app.add_systems(
        Update,
        (
            ResourcePacks::resourcepack_events.run_if(on_event::<AssetEvent<ResourcePackAsset>>()),
            ResourcePacks::update.run_if(
                resource_exists_and_changed::<Settings>()
                    .and_then(ResourcePacks::resourcepack_check),
            ),
        ),
    );
}

impl ResourcePacks {
    fn default(assets: Res<AssetServer>, mut commands: Commands) {
        commands.insert_resource(Self {
            packs: Vec::new(),
            // TODO: Use embedded_asset!() macro
            fallback: assets.load("textures/fallback.png"),
        })
    }

    /// Process [ResourcePackAsset] events.
    fn resourcepack_events(
        mut packs: ResMut<ResourcePacks>,
        mut asset_events: EventReader<AssetEvent<ResourcePackAsset>>,
        mut loaded_event: EventWriter<ResourcePacksFinishReloadEvent>,
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

                        loaded_event.send(ResourcePacksFinishReloadEvent);
                    }
                }
            }
            _ => {}
        });
    }

    /// Check if no [ResourcePackAsset]s are loaded or if the [Settings] have changed.
    fn resourcepack_check(settings: Res<Settings>, packs: Res<ResourcePacks>) -> bool {
        packs.packs.is_empty()
            || settings
                .resourcepacks
                .paths
                .iter()
                .zip(packs.packs.iter())
                .any(|(incoming, existing)| incoming != &existing.path)
    }

    /// Reload the resourcepacks.
    fn update(
        assets: Res<AssetServer>,
        settings: Res<Settings>,
        mut packs: ResMut<ResourcePacks>,
        mut start_events: EventWriter<ResourcePacksStartReloadEvent>,
        mut finish_events: EventWriter<ResourcePacksFinishReloadEvent>,
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

        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Sending ResourcePacksStartReloadEvent!");

        start_events.send(ResourcePacksStartReloadEvent);

        if packs.packs.is_empty() {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("No ResourcePackAssets, sending ResourcePacksFinishReloadEvent!");

            finish_events.send(ResourcePacksFinishReloadEvent);
        }
    }

    /// Get a texture from the resourcepacks.
    ///
    /// If no resourcepack has the texture, the fallback texture is returned.
    pub fn get_texture<'a>(
        &'a self,
        resource: &ResourceLocation,
        assets: &'a Assets<ResourcePackAsset>,
    ) -> &'a Handle<Image> {
        // Loop through all pack handles
        for pack in self.packs.iter().rev() {
            if let Some(pack) = assets.get(&pack.handle) {
                // If the pack has the texture, return it
                if let Some(texture) = pack.textures.get(resource) {
                    return texture;
                }
            }
        }

        #[cfg(any(debug_assertions, feature = "debug"))]
        warn!("No resource pack has the texture: {}", resource);

        // If no pack has the texture, return the fallback
        &self.fallback
    }

    /// Try to get a texture from the resourcepacks.
    pub fn try_get_texture<'a>(
        &'a self,
        resource: &ResourceLocation,
        assets: &'a Assets<ResourcePackAsset>,
    ) -> Option<&'a Handle<Image>> {
        // Loop through all pack handles
        for pack in self.packs.iter().rev() {
            if let Some(pack) = assets.get(&pack.handle) {
                // If the pack has the texture, return it
                if let Some(texture) = pack.textures.get(resource) {
                    return Some(texture);
                }
            }
        }

        #[cfg(any(debug_assertions, feature = "debug"))]
        warn!("No resource pack has the texture: {}", resource);

        // If no pack has the texture, return the fallback
        None
    }
}
