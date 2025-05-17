use std::path::PathBuf;

use bevy::prelude::*;
use bevy_platform::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A [`Plugin`] that reads the [`PlayerPermissions`]
/// from the disk and provides it as a [`Resource`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PermissionsPlugin(PathBuf);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource, Reflect, Deref, DerefMut)]
#[reflect(Debug, Clone, PartialEq, Hash, Resource)]
struct FilePath(PathBuf);

impl PermissionsPlugin {
    /// Create a new [`AdminPlugin`] that reads from the given path.
    #[must_use]
    pub fn new(path: impl Into<PathBuf>) -> Self { Self(path.into()) }
}

impl Plugin for PermissionsPlugin {
    fn build(&self, app: &mut App) {
        // Setup the `AdminPath` resource to read and write from the file.
        app.register_type::<FilePath>().insert_resource(FilePath(self.0.clone()));

        // Read the file and create the `PlayerPermissions` resource.
        let permissions = PlayerPermissions::from_world(app.world_mut());
        info!("{permissions:#?}");
        app.register_type::<PlayerPermissions>().insert_resource(permissions);
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Resource, Reflect, Deref, DerefMut)]
#[reflect(Debug, Clone, PartialEq, Resource)]
pub struct PlayerPermissions(HashMap<Uuid, PlayerRole>);

impl PlayerPermissions {
    /// Create a new [`PlayerPermissions`] resource from a [`PermissionsFile`].
    #[must_use]
    pub fn from_file(file: &PermissionsFile) -> Self {
        Self(file.entries.iter().map(|entry| (entry.uuid, entry.role)).collect())
    }

    /// Create a new [`PermissionsFile`] from this [`PlayerPermissions`].
    #[must_use]
    pub fn as_file(&self) -> PermissionsFile {
        let entries =
            self.iter().map(|(uuid, role)| PermissionsEntry { uuid: *uuid, role: *role }).collect();
        PermissionsFile { entries }
    }
}

impl FromWorld for PlayerPermissions {
    fn from_world(world: &mut World) -> Self {
        if let Some(path) = world.get_resource::<FilePath>() {
            // Read the file from the given path
            let contents = match std::fs::read_to_string(&**path) {
                Ok(contents) => contents,
                Err(err) => {
                    panic!("Failed to read file \"{}\": {err}", path.display());
                }
            };

            // Parse the contents as TOML
            match toml::from_str::<PermissionsFile>(&contents) {
                Ok(file) => Self::from_file(&file),
                Err(err) => panic!("Failed to parse file \"{}\": {err}", path.display()),
            }
        } else {
            panic!("App is missing `AdminPath` resource, has the plugin not been set up yet?");
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PermissionsFile {
    #[serde(default, rename = "player")]
    pub entries: Vec<PermissionsEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PermissionsEntry {
    pub uuid: Uuid,
    pub role: PlayerRole,
}

#[repr(u8)]
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Reflect,
    Serialize,
    Deserialize,
)]
#[reflect(Debug, Default, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum PlayerRole {
    #[default]
    Default = 0,
    Moderator = 1,
    Admin = 2,
    Owner = 255,
}
