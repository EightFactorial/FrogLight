use bevy::prelude::*;
use compact_str::CompactString;
use mc_rs_gui::menus::state::GuiLoadState;
use mc_rs_resourcepack::assets::resourcepacks::{ResourcePackContainer, ResourcePacks};
use serde::{Deserialize, Serialize};

use super::Settings;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourcePackSettings {
    #[serde(default = "ResourcePackSettings::default_paths")]
    pub paths: Vec<CompactString>,
}

impl Default for ResourcePackSettings {
    fn default() -> Self {
        Self {
            paths: Self::default_paths(),
        }
    }
}

impl ResourcePackSettings {
    /// The default ResourcePack paths.
    fn default_paths() -> Vec<CompactString> { vec![CompactString::new_inline("minecraft.zip")] }

    /// Load the resourcepacks listed in the [Settings],
    /// and add them to the [ResourcePacks] resource.
    ///
    /// Runs on [Startup], and when the list of resourcepacks changes.
    pub(super) fn update_resourcepacks(
        settings: Res<Settings>,
        assets: Res<AssetServer>,

        mut packs: ResMut<ResourcePacks>,
        mut state: ResMut<NextState<GuiLoadState>>,
    ) {
        let should_reload =
        // Reload if the length has changed
        packs.list.len() != settings.resourcepacks.paths.len()
            ||
        // Reload if the order has changed
        settings
            .resourcepacks
            .paths
            .iter()
            .zip(packs.list.iter())
            .any(|(path, pack)| path != &pack.path);

        if should_reload {
            #[cfg(any(debug_assertions, feature = "debug"))]
            {
                trace!(
                    "ResourcePackSettings {}, ResourcePacks {}",
                    settings.resourcepacks.paths.len(),
                    packs.list.len()
                );

                debug!("Reloading ResourcePacks");
            }

            // Remove all of the old resourcepacks
            packs.list.clear();

            // Load the new resourcepacks
            for path in &settings.resourcepacks.paths {
                packs.list.push(ResourcePackContainer {
                    handle: assets.load(format!("resourcepack://{path}")),
                    path: path.clone(),
                });
            }

            // Reload the textures
            state.set(GuiLoadState::LoadingResourcePacks);
        }
    }
}
