use async_zip::base::read::{stream::ZipFileReader, WithEntry, ZipEntryReader};
use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
};
use froglight_core::data::{ResourceKey, ResourceKeyError};
use futures_lite::{io::Take, AsyncRead};
use thiserror::Error;

use crate::{asset_manager::manager::AssetManager, ResourcePack};

mod functions;

/// The resource pack loader.
#[derive(Debug, Default, Clone)]
#[cfg(not(feature = "asset_manager"))]
pub struct ResourcePackLoader;

/// The resource pack loader.
///
/// Internally holds an [`AssetManager`] to load the resource pack.
#[derive(Debug, Default, Clone, Deref)]
#[cfg(feature = "asset_manager")]
pub struct ResourcePackLoader(pub(crate) AssetManager);

#[cfg(feature = "asset_manager")]
impl From<AssetManager> for ResourcePackLoader {
    fn from(manager: AssetManager) -> Self { Self(manager) }
}

/// An error that occurred while loading a resource pack.
#[derive(Debug, Error)]
pub enum ResourcePackLoaderError {
    /// An I/O error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// A ZIP error occurred.
    #[error(transparent)]
    Zip(#[from] async_zip::error::ZipError),
    /// An image error occurred.
    #[error(transparent)]
    Image(#[from] image::error::ImageError),
    /// A ResourceKey error occurred.
    #[error(transparent)]
    ResourceKey(#[from] ResourceKeyError),
}

// Load all assets into the [`ResourcePack`].
//
// If the `asset_manager` feature is enabled, this will load the assets into the
// [`AssetManager`] and store weak handles in the [`ResourcePack`].
impl AssetLoader for ResourcePackLoader {
    type Asset = ResourcePack;
    type Settings = ();
    type Error = ResourcePackLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        (): &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(Self::load(self, reader, load_context))
    }
}

impl ResourcePackLoader {
    /// Loop over all entries in the ZIP file
    /// and load them into a [`ResourcePack`].
    async fn load<'a>(
        &self,
        reader: &'a mut Reader<'a>,
        context: &mut LoadContext<'_>,
    ) -> Result<ResourcePack, ResourcePackLoaderError> {
        // Create a new ZIP file reader
        let mut zip_reader = ZipFileReader::new(reader);

        // Create a new empty resource pack
        let mut resourcepack = ResourcePack::default();

        // Iterate over all entries in the ZIP file
        while let Ok(Some(mut zip_entry)) = zip_reader.next_with_entry().await {
            let entry_reader = zip_entry.reader_mut();

            // Read the entry name
            let entry_name = entry_reader.entry().filename();
            let Ok(entry_name) = entry_name.as_str().map(String::from) else {
                // Go to the next entry
                if let Ok(reader) = zip_entry.skip().await {
                    zip_reader = reader;
                    continue;
                }
                break;
            };

            // Skip directories
            if !entry_name.ends_with('/') {
                // Load the entry into the resource pack
                Self::load_entry(self, entry_name, entry_reader, &mut resourcepack, context)
                    .await?;
            }

            // Go to the next entry
            if let Ok(reader) = zip_entry.skip().await {
                zip_reader = reader;
            } else {
                break;
            }
        }

        Ok(resourcepack)
    }

    async fn load_entry(
        &self,
        entry_name: String,
        entry_reader: &mut ZipEntryReader<
            '_,
            Take<&'_ mut (dyn AsyncRead + Sync + Send + Unpin)>,
            WithEntry<'_>,
        >,
        resourcepack: &mut ResourcePack,
        context: &mut LoadContext<'_>,
    ) -> Result<(), ResourcePackLoaderError> {
        // TODO: Support for ResourcePack metadata
        if matches!(entry_name.as_str(), "pack.png" | "pack.mcmeta") {
            return Ok(());
        }

        // Skip non-asset entries
        let Some(entry_name) = entry_name.strip_prefix("assets/") else {
            return Ok(());
        };

        // Get the namespace and file name
        let Some((entry_namespace, entry_file)) = entry_name.split_once('/') else {
            return Ok(());
        };

        // Get the asset type and path
        let Some((entry_type, entry_path)) = entry_file.split_once('/') else {
            return Ok(());
        };

        // Split the asset path and extension
        let Some((entry_path, entry_ext)) = entry_path.split_once('.') else {
            return Ok(());
        };

        // Create a new ResourceKey
        let resourcekey = match ResourceKey::try_new(format!("{entry_namespace}:{entry_path}")) {
            Ok(resourcekey) => resourcekey,
            Err(err) => {
                warn!("Failed to create a ResourceKey from: `{entry_name}`: {err}");
                return Ok(());
            }
        };

        if entry_ext == ".mcmeta" {
            // TODO: Support for metadata files
        } else if entry_type.starts_with("textures") && matches!(entry_ext, "png" | "jpg" | "jpeg")
        {
            // Load the texture into the resource pack
            if let Some(texture) =
                functions::load_texture(self, &resourcekey, entry_reader, context).await?
            {
                resourcepack.textures.insert(resourcekey, texture);
            }
        } else if entry_type.starts_with("sounds") && matches!(entry_ext, "ogg" | "wav") {
            // Load the audio into the resource pack
            if let Some(sound) =
                functions::load_audio(self, &resourcekey, entry_reader, context).await?
            {
                resourcepack.audio.insert(resourcekey, sound);
            }
        }

        Ok(())
    }
}
