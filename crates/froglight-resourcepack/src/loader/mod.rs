use async_zip::base::read::{mem::ZipFileReader, WithEntry, ZipEntryReader};
use bevy::{
    asset::{AssetLoader, LoadContext},
    log::warn,
    prelude::*,
    utils::BoxedFuture,
};
use froglight_core::data::ResourceKey;
use futures_lite::AsyncReadExt;
use image::io::Reader as ImageReader;
use thiserror::Error;

use crate::{
    meta::PackMcMeta,
    resourcepack::ResourcePack,
    settings::{ResourcePackAudioSettings, ResourcePackLoaderSettings},
};

mod audio;
mod textures;

/// A loader for resource packs.
///
/// Use [`AssetServer::load_with_settings`](bevy::asset::AssetServer::load_with_settings)
/// and a [`ResourcePackLoaderSettings`] to load resource packs and
/// automatically track their assets.
///
/// Supports loading `.zip` and `.jar` files.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourcePackLoader;

#[derive(Debug, Error)]
pub enum ResourcePackLoaderError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Zip(#[from] async_zip::error::ZipError),
    #[error(transparent)]
    Image(#[from] image::error::ImageError),
    /// No [`ResourcePackManager`](crate::ResourcePackManager) was provided,
    /// so the loader cannot track assets.
    ///
    /// Use [`AssetServer::load_with_settings`] with a
    /// [`ResourcePackLoaderSettings`] to load resource packs.
    #[error("No ResourcePackManager was provided")]
    NoResourcePackManager,
}

impl AssetLoader for ResourcePackLoader {
    type Asset = ResourcePack;
    type Settings = ResourcePackLoaderSettings;
    type Error = ResourcePackLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        // If no settings were provided, return an error
        if settings.is_none() {
            return Box::pin(async { Err(ResourcePackLoaderError::NoResourcePackManager) });
        }

        // TODO: This can be parallelized, but still done one pack at a time.
        Box::pin(async {
            // Read the entire file into memory
            let mut file = Vec::new();
            reader.read_to_end(&mut file).await?;
            let file = ZipFileReader::new(file).await?;
            let size = file.file().entries().len();

            // Create a new resource pack
            let mut pack = ResourcePack::default();

            // Iterate over each file
            for index in 0..size {
                let mut entry = file.reader_with_entry(index).await?;

                let filename = entry.entry().filename().clone();
                let Ok(mut filename) = filename.as_str() else {
                    warn!(
                        "Unable to read file `{filename:?}` from `{}`",
                        load_context.path().display()
                    );
                    continue;
                };

                // Skip directories
                if filename.ends_with('/') {
                    continue;
                }

                match filename {
                    // Parse the `pack.mcmeta` file
                    "pack.mcmeta" => {
                        pack.meta = PackMcMeta::parse(&mut entry).await.unwrap_or_else(|err| {
                            warn!(
                                "Unable to parse `pack.mcmeta` for `{:?}`: `{err}`",
                                load_context.path().display()
                            );
                            PackMcMeta::default()
                        });
                        continue;
                    }
                    // Load the `pack.png` file
                    "pack.png" => {
                        // Read the image into memory
                        let mut bytes = Vec::new();
                        entry.read_to_end(&mut bytes).await?;

                        // Decode the image
                        let dyn_img = ImageReader::new(std::io::Cursor::new(bytes))
                            .with_guessed_format()?
                            .decode()?;
                        let image = Image::from_dynamic(dyn_img, false);

                        // Set the pack icon
                        pack.icon = Some(
                            load_context.labeled_asset_scope(String::from("meta_icon"), |_| image),
                        );
                        continue;
                    }
                    _ => {}
                }

                // Remove the 'assets' directory from the filename
                if filename.starts_with("assets/") {
                    filename = &filename[7..];

                    // Remove the namespace from the filename
                    let Some((namespace, filename)) = filename.split_once('/') else {
                        continue;
                    };

                    // Parse the `sounds.json` file
                    if filename == "sounds.json" {
                        ResourcePackAudioSettings::parse(&mut entry, settings, load_context)
                            .await?;

                        continue;
                    }

                    // Create a resource key
                    let Ok(resource_key) = ResourceKey::try_from(format!("{namespace}:{filename}"))
                    else {
                        warn!(
                            "Unable to create resource key for `{filename:?}` from `{}`",
                            load_context.path().display()
                        );
                        continue;
                    };

                    // Load the file
                    load_file(
                        &mut pack,
                        filename,
                        resource_key,
                        &mut entry,
                        load_context,
                        settings,
                    )
                    .await?;
                }
            }

            // Return the resource pack.
            Ok(pack)
        })
    }

    fn extensions(&self) -> &[&str] { &["zip", "jar"] }
}

/// Load a file from the resource pack.
async fn load_file(
    pack: &mut ResourcePack,
    filename: &str,
    resource_key: ResourceKey,
    entry: &mut ZipEntryReader<'_, futures_lite::io::Cursor<&[u8]>, WithEntry<'_>>,
    load_context: &mut LoadContext<'_>,
    settings: &ResourcePackLoaderSettings,
) -> Result<(), ResourcePackLoaderError> {
    #[allow(clippy::match_same_arms)]
    match filename.split('/').next().unwrap() {
        "blockstates" => {
            // TODO: Parse and load blockstates
        }
        "font" => {
            // TODO: Create and load fonts
        }
        "lang" => {
            // TODO: Load language files
        }
        "models" => {
            // TODO: Parse and load models
        }
        "particles" => {
            // TODO: Parse and load particles
        }
        "sounds" => {
            trace!("Loading file `{resource_key}` from `{}`", load_context.path().display());

            if filename.ends_with(".mcmeta") {
                // TODO: Handle mcmeta files
            } else {
                // Load the audio
                if let Some(handle) =
                    audio::load_audio(&resource_key, entry, load_context, settings).await?
                {
                    // Insert the audio into the resource pack
                    pack.audio.insert(resource_key, handle);
                }
            }
        }
        "texts" => {
            // TODO: Load text files
        }
        "textures" => {
            trace!("Loading file `{resource_key}` from `{}`", load_context.path().display());

            if filename.ends_with(".mcmeta") {
                // TODO: Handle mcmeta files
            } else {
                // Load the texture
                if let Some(handle) =
                    textures::load_texture(&resource_key, entry, load_context, settings).await?
                {
                    // Insert the texture into the resource pack
                    pack.textures.insert(resource_key, handle);
                }
            }
        }
        _ => {}
    }

    Ok(())
}
