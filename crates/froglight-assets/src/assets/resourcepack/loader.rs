use async_zip::{
    base::read::{stream::ZipFileReader, WithEntry, ZipEntryReader},
    ZipEntry,
};
use bevy_asset::{
    io::Reader, Asset, AssetLoader, AsyncReadExt, BoxedFuture, ErasedLoadedAsset, Handle,
    LoadContext,
};
use bevy_log::error;
use froglight_components::resourcekey::ResourceKey;
use futures_lite::io::BufReader;
use hashbrown::HashMap;
use serde::de::DeserializeOwned;
use thiserror::Error;

use super::ResourcePack;
use crate::assets::{ModelDefinition, TextSource};

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
                // Check for supported file extensions in subdirectories
                let ext = filename.split('.').last().unwrap_or_default();
                match filename.split('/').nth(2).unwrap_or_default() {
                    "blockstates" | "font" | "lang" | "models" | "particles" => {
                        matches!(ext, "json")
                    }
                    "sounds" => matches!(ext, "ogg"),
                    "texts" => matches!(ext, "json" | "txt"),
                    "textures" => matches!(ext, "png"),
                    "sounds.json" => true,
                    _ => false,
                }
            } else {
                // Check for "pack.mcmeta" and "pack.png"
                matches!(filename, "pack.mcmeta" | "pack.png")
            }
        } else {
            false
        }
    }

    /// Reads an entry from the ZIP file.
    #[allow(clippy::too_many_lines)]
    async fn read_entry(
        reader: &mut ZipEntryReader<'_, BufReader<&mut Reader<'_>>, WithEntry<'_>>,
        context: &mut LoadContext<'_>,
        resource_pack: &mut ResourcePack,
    ) -> Result<(), ResourcePackError> {
        // Get the filename of the entry.
        let filename = reader.entry().filename().as_str().unwrap().to_string();

        // Check for special files
        match filename.as_str() {
            // Deserialize the "pack.mcmeta" file
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
            // Load the "pack.png" image
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

        // Load the asset.
        match filename.split('/').nth(2).unwrap_or_default() {
            "blockstates" => {
                Self::deserialize_and_insert_asset(
                    reader,
                    &mut resource_pack.blockstates,
                    resource_key,
                )
                .await;
            }
            "font" => {
                Self::deserialize_and_insert_asset(reader, &mut resource_pack.fonts, resource_key)
                    .await;
            }
            "lang" => {
                Self::deserialize_and_insert_asset(reader, &mut resource_pack.lang, resource_key)
                    .await;
            }
            "models" => match filename.split('/').nth(3).unwrap_or_default() {
                "block" => {
                    if let Some(asset) = Self::deserialize_asset(reader, &resource_key).await {
                        resource_pack.models.insert(resource_key, ModelDefinition::Block(asset));
                    }
                }
                "item" => {
                    Self::deserialize_and_insert_asset(
                        reader,
                        &mut resource_pack.models,
                        resource_key,
                    )
                    .await;
                }
                _ => {}
            },
            "particles" => {
                Self::deserialize_and_insert_asset(
                    reader,
                    &mut resource_pack.particles,
                    resource_key,
                )
                .await;
            }
            "sounds" => {
                Self::load_and_insert_asset(
                    reader,
                    context,
                    &mut resource_pack.sounds,
                    resource_key,
                    &filename,
                )
                .await;
            }
            "texts" => {
                // Read the text file.
                let mut contents = String::new();
                if let Err(err) = reader.read_to_string(&mut contents).await {
                    error!("Error reading text file: {err}");
                    return Ok(());
                }

                // If the file is a JSON file, parse it as JSON.
                // Otherwise, store it as raw text.
                match filename.split('.').last().unwrap_or_default() {
                    "json" => match serde_json::from_str(&contents) {
                        Ok(json) => {
                            resource_pack.texts.insert(resource_key, TextSource::Json(json));
                        }
                        Err(err) => {
                            error!("Error parsing JSON file \"{filename}\": {err}");
                        }
                    },
                    _ => {
                        resource_pack.texts.insert(resource_key, TextSource::RawText(contents));
                    }
                }
            }
            "textures" => {
                Self::load_and_insert_asset(
                    reader,
                    context,
                    &mut resource_pack.textures,
                    resource_key,
                    &filename,
                )
                .await;
            }
            "sounds.json" => {
                if let Some(definitions) = Self::deserialize_asset(reader, &resource_key).await {
                    resource_pack.sound_defs.insert(resource_key, definitions);
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Converts a file path to a [`ResourceKey`].
    fn filename_to_resourcekey(filename: &str) -> Option<ResourceKey> {
        // Strip the file extension and the "assets/" prefix.
        let ext = filename.split('.').last()?;
        let stripped = filename.strip_prefix("assets/").unwrap().strip_suffix(ext).unwrap();
        let stripped = stripped[..stripped.len() - 1].replacen('/', ":", 1);

        // If the key's path has a directory, remove the first one.
        //
        // For example, "minecraft:block/stone" becomes "minecraft:stone".
        if let Some((prefixed_domain, path)) = stripped.split_once('/') {
            let domain = prefixed_domain.split_once(':').unwrap().0;
            ResourceKey::try_new(format!("{domain}:{path}")).ok()
        } else {
            ResourceKey::try_new(stripped).ok()
        }
    }

    async fn deserialize_and_insert_asset<T: DeserializeOwned>(
        reader: &mut Reader<'_>,
        map: &mut HashMap<ResourceKey, T>,
        key: ResourceKey,
    ) {
        if let Some(asset) = Self::deserialize_asset(reader, &key).await {
            map.insert(key, asset);
        }
    }

    /// Deserializes an asset and inserts it into a map.
    async fn deserialize_asset<T: DeserializeOwned>(
        reader: &mut Reader<'_>,
        path: &str,
    ) -> Option<T> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).await.unwrap();
        match serde_json::from_slice(&buffer) {
            Ok(asset) => Some(asset),
            #[allow(unused_variables)]
            Err(err) => {
                #[cfg(debug_assertions)]
                error!("Error deserializing asset: \"{path}\", {err}");
                #[cfg(not(debug_assertions))]
                error!("Error deserializing asset: \"{path}\"");
                None
            }
        }
    }

    /// Loads an asset and inserts it into a map.
    async fn load_and_insert_asset<A: Asset>(
        reader: &mut Reader<'_>,
        context: &mut LoadContext<'_>,
        map: &mut HashMap<ResourceKey, Handle<A>>,
        key: ResourceKey,
        path: &str,
    ) {
        if let Some(asset) = Self::load_asset(reader, context, &key, path).await {
            map.insert(key, asset);
        }
    }

    /// Loads an asset from a ZIP entry.
    async fn load_asset<A: Asset>(
        reader: &mut Reader<'_>,
        context: &mut LoadContext<'_>,
        key: &str,
        path: &str,
    ) -> Option<Handle<A>> {
        let loaded_asset = match context.load_direct_with_reader(reader, path.to_string()).await {
            Ok(asset) => asset,
            Err(err) => {
                #[cfg(debug_assertions)]
                error!("{err}");
                #[cfg(not(debug_assertions))]
                error!("Error loading asset: \"{path}\"");

                return None;
            }
        };

        #[cfg(debug_assertions)]
        let type_name = loaded_asset.asset_type_name();

        if let Some(asset) = ErasedLoadedAsset::take::<A>(loaded_asset) {
            Some(context.add_labeled_asset(key.to_string(), asset))
        } else {
            #[cfg(debug_assertions)]
            error!("Asset \"{path}\" is `{type_name}`, not `{}`", A::type_path());
            #[cfg(not(debug_assertions))]
            error!("Asset \"{path}\" was not of type `{}`", A::type_path());
            None
        }
    }
}
