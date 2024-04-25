use async_zip::{
    base::read::{stream::ZipFileReader, WithEntry, ZipEntryReader},
    ZipEntry,
};
use bevy_asset::{
    io::Reader, Asset, AssetLoader, AsyncReadExt, BoxedFuture, ErasedLoadedAsset, Handle,
    LoadContext,
};
use bevy_log::{error, warn};
use froglight_protocol::common::ResourceKey;
use futures_lite::io::BufReader;
use thiserror::Error;

use super::ResourcePack;

/// An [`AssetLoader`] for [`ResourcePack`]s.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourcePackZipLoader;

/// An error that can occur while loading a [`ResourcePack`].
#[derive(Debug, Error)]
pub enum ResourcePackError {
    /// An error occurred while reading the ZIP file.
    #[error(transparent)]
    Zip(#[from] async_zip::error::ZipError),
    /// An IO error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl AssetLoader for ResourcePackZipLoader {
    type Asset = ResourcePack;
    type Settings = ();
    type Error = ResourcePackError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        (): &'a (),
        context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<ResourcePack, ResourcePackError>> {
        Box::pin(Self::load(reader, context))
    }

    fn extensions(&self) -> &[&str] { &["zip"] }
}

impl ResourcePackZipLoader {
    async fn load(
        reader: &mut Reader<'_>,
        context: &mut LoadContext<'_>,
    ) -> Result<ResourcePack, ResourcePackError> {
        let mut resource_pack = ResourcePack::default();

        #[cfg(debug_assertions)]
        let start = std::time::Instant::now();

        // Read the ZIP file.
        let mut reader = ZipFileReader::new(BufReader::new(reader));

        // Read each entry in the ZIP file.
        while let Ok(Some(mut next_entry)) = reader.next_with_entry().await {
            // Skip entries that should not be read.
            let entry_reader = next_entry.reader_mut();
            if Self::should_read(entry_reader.entry()) {
                Self::read_entry(entry_reader, context, &mut resource_pack).await?;
            }

            // Move to the next entry.
            reader = next_entry.skip().await?;
        }

        #[cfg(debug_assertions)]
        bevy_log::debug!(
            "Loaded `{}` in {} seconds",
            context.path().display(),
            start.elapsed().as_secs_f64()
        );

        // Return the resource pack.
        Ok(resource_pack)
    }

    /// Determines whether an entry should be read.
    fn should_read(entry: &ZipEntry) -> bool {
        if let Ok(filename) = entry.filename().as_str() {
            // Check for files in the "assets" directory
            if filename.starts_with("assets/") {
                // Check for supported file extensions
                matches!(filename.split('.').last(), Some("png" | "ogg" | "json"))
            } else {
                // Check for "pack.mcmeta" and "pack.png"
                matches!(filename, "pack.mcmeta" | "pack.png")
            }
        } else {
            #[cfg(debug_assertions)]
            warn!("Error reading `ZipEntry` filename");
            false
        }
    }

    /// Reads an entry from the ZIP file.
    async fn read_entry(
        reader: &mut ZipEntryReader<'_, BufReader<&mut Reader<'_>>, WithEntry<'_>>,
        context: &mut LoadContext<'_>,
        resource_pack: &mut ResourcePack,
    ) -> Result<(), ResourcePackError> {
        // Get the filename of the entry.
        let filename = reader.entry().filename().as_str().unwrap().to_string();

        // Check for special files
        match filename.as_str() {
            "pack.mcmeta" => {
                let mut buffer = Vec::new();
                reader.read_to_end(&mut buffer).await?;

                match serde_json::from_slice(&buffer) {
                    Ok(meta) => {
                        resource_pack.info.mcmeta = meta;
                    }
                    Err(err) => error!("Error parsing ResourcePack metadata: {err}"),
                }

                return Ok(());
            }
            "pack.png" => {
                if let Some(icon) = Self::load_asset(reader, context, "pack_icon", "pack.png").await
                {
                    resource_pack.info.icon = icon;
                }
                return Ok(());
            }
            _ => {}
        }

        // Create a `ResourceKey` from the filename.
        let Some(resource_key) = Self::filename_to_resourcekey(&filename) else {
            #[cfg(debug_assertions)]
            error!("Error creating `ResourceKey`: \"{filename}\"");
            return Ok(());
        };

        // Load the asset based on the file extension.
        match filename.split('.').last() {
            Some("png") => {
                if let Some(texture) =
                    Self::load_asset(reader, context, &resource_key, &filename).await
                {
                    resource_pack.textures.insert(resource_key, texture);
                }
            }
            Some("ogg") => {
                if let Some(audio) =
                    Self::load_asset(reader, context, &resource_key, &filename).await
                {
                    resource_pack.audio.insert(resource_key, audio);
                }
            }
            Some("json") => {
                let mut buffer = Vec::new();
                reader.read_to_end(&mut buffer).await?;

                match serde_json::from_slice(&buffer) {
                    Ok(json) => {
                        resource_pack.json.insert(resource_key, json);
                    }
                    Err(err) => error!("Error parsing JSON \"{resource_key}\": {err}"),
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Converts a file path to a [`ResourceKey`].
    fn filename_to_resourcekey(filename: &str) -> Option<ResourceKey> {
        let ext = filename.split('.').last()?;
        let stripped = filename.strip_prefix("assets/").unwrap().strip_suffix(ext).unwrap();
        ResourceKey::try_new(stripped[..stripped.len() - 1].replacen('/', ":", 1)).ok()
    }

    /// Loads an asset from a ZIP entry.
    async fn load_asset<A: Asset>(
        reader: &mut Reader<'_>,
        context: &mut LoadContext<'_>,
        key: &str,
        path: &str,
    ) -> Option<Handle<A>> {
        match context
            .load_direct_with_reader(reader, path.to_string())
            .await
            .map(ErasedLoadedAsset::take::<A>)
        {
            Ok(Some(asset)) => Some(context.add_labeled_asset(key.to_string(), asset)),
            Ok(None) => {
                error!("Asset \"{path}\" was not of type `{}`", A::type_path());
                None
            }
            Err(err) => {
                #[cfg(debug_assertions)]
                error!("{err}");
                #[cfg(not(debug_assertions))]
                error!("Error loading asset: \"{path}\"");

                None
            }
        }
    }
}
