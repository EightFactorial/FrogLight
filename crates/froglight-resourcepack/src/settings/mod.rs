//! Settings for [`ResourcePack`](super::ResourcePack)s and the
//! [`ResourcePackLoader`](super::ResourcePackLoader).

use bevy::prelude::*;
use serde::{de::Visitor, Deserialize, Serialize};

use crate::manager::ResourcePackManager;

mod audio;
pub use audio::ResourcePackAudioSettings;

/// Settings for the [`ResourcePackLoader`](crate::ResourcePackLoader) to load
/// [`ResourcePack`](crate::ResourcePack)s.
///
/// In order to track assets, use a [`ResourcePackManager`] to create a new
/// [`ResourcePackLoaderSettings`].
#[derive(Debug, Default, Clone, Deref)]
pub struct ResourcePackLoaderSettings(pub(crate) Option<ResourcePackManager>);

impl ResourcePackLoaderSettings {
    /// Creates a new [`ResourcePackLoaderSettings`] with the given
    /// [`ResourcePackManager`]. Required for tracking assets.
    #[must_use]
    pub fn new(manager: ResourcePackManager) -> Self { Self(Some(manager)) }
}

impl PartialEq for ResourcePackLoaderSettings {
    fn eq(&self, other: &Self) -> bool { self.0.is_some() && other.0.is_some() }
}

impl From<ResourcePackManager> for ResourcePackLoaderSettings {
    fn from(manager: ResourcePackManager) -> Self { Self::new(manager) }
}

impl Serialize for ResourcePackLoaderSettings {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_unit()
    }
}

impl<'de> Deserialize<'de> for ResourcePackLoaderSettings {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct UnitVisitor;
        impl<'de> Visitor<'de> for UnitVisitor {
            type Value = ResourcePackLoaderSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ResourcePackLoaderSettings expects no arguments")
            }

            fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
                Ok(ResourcePackLoaderSettings::default())
            }
        }

        deserializer.deserialize_unit(UnitVisitor)
    }
}
