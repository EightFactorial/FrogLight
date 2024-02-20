use bevy::prelude::*;
use froglight_core::{events::ResourcePackStartLoadingEvent, systemsets::AssetStartupSet};
use serde::{Deserialize, Serialize};

use super::ConfigFile;
use crate::AssetTracker;

/// A list of paths to [`ResourcePack`]s to load.
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Deref,
    DerefMut,
    Reflect,
    Resource,
    Serialize,
    Deserialize,
)]
#[reflect(Resource)]
pub struct ResourcePackSettings {
    /// A list of paths to [`ResourcePack`]s to load.
    pub resourcepacks: Vec<ResourcePackPath>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Reflect, Serialize, Deserialize)]
pub struct ResourcePackPath {
    pub(crate) path: String,
}

impl ConfigFile for ResourcePackSettings {
    const FILEPATH: &'static str = "resourcepacks.toml";

    #[cfg(feature = "asset_manager")]
    fn build(app: &mut App) {
        app.add_systems(
            Startup,
            Self::load_resourcepacks
                .run_if(resource_exists::<Self>)
                .run_if(run_once())
                .in_set(AssetStartupSet),
        );
    }
}

#[cfg(feature = "asset_manager")]
impl ResourcePackSettings {
    fn load_resourcepacks(
        res: Res<Self>,
        mut tracker: ResMut<AssetTracker>,
        mut events: EventWriter<ResourcePackStartLoadingEvent>,
    ) {
        debug!("Queuing {} ResourcePack(s)", res.resourcepacks.len());
        for path in &res.resourcepacks {
            tracker.queue(path.as_ref().to_string());
        }

        debug!("Sending ResourcePackStartLoadingEvent");
        events.send(ResourcePackStartLoadingEvent);
    }
}

impl From<String> for ResourcePackPath {
    fn from(s: String) -> Self { Self { path: s } }
}

impl From<&str> for ResourcePackPath {
    fn from(s: &str) -> Self { Self { path: s.to_string() } }
}

impl From<ResourcePackPath> for String {
    fn from(s: ResourcePackPath) -> Self { s.path }
}

impl AsRef<str> for ResourcePackPath {
    fn as_ref(&self) -> &str { &self.path }
}
