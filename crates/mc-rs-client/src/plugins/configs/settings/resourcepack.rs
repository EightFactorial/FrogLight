use bevy::prelude::*;
use compact_str::CompactString;
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
    fn default_paths() -> Vec<CompactString> { vec![CompactString::new_inline("minecraft.jar")] }

    pub(super) fn update_resourcepacks(
        settings: Res<Settings>,
        assets: Res<AssetServer>,

        mut packs: ResMut<ResourcePacks>,
    ) {
        for path in settings.resourcepacks.paths.iter() {
            if !packs.list.iter().any(|pack| pack.path == path) {
                let pack_path = format!("resourcepack://{path}");

                packs.list.push(ResourcePackContainer {
                    handle: assets.load(&pack_path),
                    path: pack_path.into(),
                });
            }
        }
    }
}
