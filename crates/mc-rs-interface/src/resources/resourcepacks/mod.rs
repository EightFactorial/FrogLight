use bevy::{prelude::*, utils::HashMap};
use mc_rs_core::ResourceLocation;

use crate::{configs::settings::Settings, resourcepacks::ResourcePackAsset};

mod atlases;
pub use atlases::{gui_icons::GuiIcons, AtlasKind};

pub mod events;
use events::{ResourcePacksFinishEvent, ResourcePacksFinishLoadEvent, ResourcePacksStartLoadEvent};

mod info;
pub use info::ResourcePackInfo;

/// A collection of resourcepacks loaded.
///
/// Use this to get textures loaded from resourcepacks.
#[derive(Debug, Clone, PartialEq, Eq, Resource)]
pub struct ResourcePacks {
    packs: Vec<ResourcePackInfo>,
    atlases: HashMap<AtlasKind, Handle<TextureAtlas>>,

    pub fallback: Handle<Image>,
}

pub(super) fn setup(app: &mut App) {
    app.add_systems(Startup, ResourcePacks::default);

    app.add_event::<ResourcePacksStartLoadEvent>()
        .add_event::<ResourcePacksFinishLoadEvent>()
        .add_event::<ResourcePacksFinishEvent>();

    app.add_systems(
        Update,
        (
            events::finish_event.run_if(on_event::<AssetEvent<ResourcePackAsset>>()),
            ResourcePacks::update.run_if(
                resource_exists_and_changed::<Settings>()
                    .and_then(ResourcePacks::resourcepack_check),
            ),
            AtlasKind::update_atlases.run_if(on_event::<ResourcePacksFinishLoadEvent>()),
        ),
    );
}

impl ResourcePacks {
    /// Insert the default [ResourcePacks] resource.
    fn default(assets: Res<AssetServer>, mut commands: Commands) {
        commands.insert_resource(Self {
            packs: Vec::new(),
            atlases: HashMap::new(),

            // TODO: Use embedded_asset!() macro
            fallback: assets.load("textures/fallback.png"),
        })
    }

    /// Check if no [ResourcePackAsset]s are loaded or if [Settings]'s
    /// ResourcePack list doesn't match.
    fn resourcepack_check(settings: Res<Settings>, packs: Res<ResourcePacks>) -> bool {
        packs.packs.is_empty()
            || settings.resourcepacks.paths.len() != packs.packs.len()
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
        mut start_events: EventWriter<ResourcePacksStartLoadEvent>,
        mut finish_events: EventWriter<ResourcePacksFinishEvent>,
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

        start_events.send(ResourcePacksStartLoadEvent);

        if packs.packs.is_empty() {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("No ResourcePackAssets, sending ResourcePacksFinishReloadEvent!");

            finish_events.send(ResourcePacksFinishEvent);
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

    /// Try to get a texture atlas from the resourcepacks.
    pub fn try_get_atlas(&self, kind: &AtlasKind) -> Option<&Handle<TextureAtlas>> {
        self.atlases.get(kind)
    }
}
