use std::{
    ffi::OsStr,
    io::{Cursor, Read, Seek},
    path::Path,
};

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    render::texture::{CompressedImageFormats, ImageSampler, ImageType},
    utils::HashMap,
};
use compact_str::CompactString;
use mc_rs_core::ResourceLocation;
use thiserror::Error;
use zip::ZipArchive;

use super::{ResourcePackAsset, ResourcePackMetaContainer};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourcePackLoader;

#[derive(Debug, Error)]
pub enum ResourcePackLoaderError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Texture error: {0}")]
    Texture(#[from] bevy::render::texture::TextureError),
    #[error("Invalid path")]
    InvalidPath,
}

impl AssetLoader for ResourcePackLoader {
    type Error = ResourcePackLoaderError;
    type Asset = ResourcePackAsset;
    type Settings = ();

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            match load_context.path().extension().map(OsStr::to_str) {
                Some(Some("zip")) => {
                    let mut bytes = Vec::new();
                    reader.read_to_end(&mut bytes).await?;

                    let zip = ZipArchive::new(Cursor::new(bytes))?;
                    ResourcePackLoader::load_zip(zip, load_context)
                }
                _ => Err(ResourcePackLoaderError::InvalidPath),
            }
        })
    }

    fn extensions(&self) -> &[&str] { &["zip"] }
}

impl ResourcePackLoader {
    /// Loads a resourcepack from a zip archive.
    fn load_zip(
        mut zip: ZipArchive<impl Read + Seek>,
        load_context: &mut LoadContext,
    ) -> Result<ResourcePackAsset, ResourcePackLoaderError> {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Loading resourcepack: {}", load_context.asset_path());

        Ok(ResourcePackAsset {
            icon: Self::read_icon(&mut zip, load_context)?,
            mcmeta: Self::read_mcmeta(&mut zip, load_context)?,
            textures: Self::read_textures(&mut zip, load_context)?,
        })
    }

    /// Reads the pack.png file from the zip archive.
    fn read_icon(
        zip: &mut ZipArchive<impl Read + Seek>,
        load_context: &mut LoadContext,
    ) -> Result<Option<Handle<Image>>, ResourcePackLoaderError> {
        let mut image_context = load_context.begin_labeled_asset();

        // Get the pack.png file from the zip archive.
        let Ok(file) = zip.by_name("pack.png") else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            info!("No pack.png found in {}", load_context.asset_path());
            return Ok(None);
        };

        // Load the image from the file.
        let image = Image::from_buffer(
            &file.bytes().collect::<Result<Vec<_>, _>>()?,
            ImageType::Extension("png"),
            CompressedImageFormats::all(),
            false,
            ImageSampler::Default,
        )?;

        // Add the image to the asset server.
        let icon = image_context.add_labeled_asset(String::from("pack_icon"), image);

        Ok(Some(icon))
    }

    /// Reads the pack.mcmeta file from the zip archive.
    fn read_mcmeta(
        zip: &mut ZipArchive<impl Read + Seek>,
        load_context: &mut LoadContext,
    ) -> Result<ResourcePackMetaContainer, ResourcePackLoaderError> {
        // Get the pack.mcmeta file from the zip archive.
        let Ok(file) = zip.by_name("pack.mcmeta") else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            warn!("No pack.mcmeta found in {}", load_context.asset_path());
            return Ok(None.into());
        };

        // Deserialize the mcmeta file.
        match serde_json::from_reader(file) {
            Ok(mcmeta) => Ok(mcmeta),
            Err(err) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                error!(
                    "Failed to read {}/pack.mcmeta, {err}",
                    load_context.asset_path()
                );
                Err(err.into())
            }
        }
    }

    /// Reads all textures from the zip archive.
    fn read_textures(
        zip: &mut ZipArchive<impl Read + Seek>,
        load_context: &mut LoadContext,
    ) -> Result<HashMap<ResourceLocation, Handle<Image>>, ResourcePackLoaderError> {
        let mut textures = HashMap::new();

        // Iterate over all files in the zip archive.
        for file_index in 0..zip.len() {
            let Ok(file) = zip.by_index(file_index) else {
                #[cfg(any(debug_assertions, feature = "debug"))]
                warn!(
                    "Failed to read file {} in {}",
                    file_index,
                    load_context.asset_path()
                );
                continue;
            };

            // Skip all directories.
            if file.is_dir() {
                continue;
            }

            // Try to get the resource location from the file path.
            let Some(file_path) = file.enclosed_name() else {
                continue;
            };
            let Some(key) = Self::path_to_resource_location(file_path) else {
                continue;
            };

            // Create a new image context for each texture.
            let mut image_context = load_context.begin_labeled_asset();

            // Get the file extension.
            let Some(ext) = file_path
                .extension()
                .and_then(OsStr::to_str)
                .map(String::from)
            else {
                continue;
            };

            // Load the image from the file.
            let image = Image::from_buffer(
                &file.bytes().collect::<Result<Vec<_>, _>>()?,
                ImageType::Extension(&ext),
                CompressedImageFormats::all(),
                false,
                ImageSampler::default(),
            )?;

            // Add the image to the asset server.
            let handle = image_context.add_labeled_asset(key.to_string(), image);

            // Add the texture to the hashmap.
            textures.insert(key, handle);
        }

        #[cfg(any(debug_assertions, feature = "debug"))]
        {
            trace!("{}:\n{textures:#?}", load_context.asset_path());

            debug!(
                "Loaded {} textures from {}",
                textures.len(),
                load_context.asset_path()
            );
        }

        Ok(textures)
    }

    fn path_to_resource_location(file_path: &Path) -> Option<ResourceLocation> {
        let ext = file_path.extension()?.to_str()?;

        // Only open texture files.
        if !matches!(ext, "png" | "jpg" | "jpeg") {
            return None;
        }

        // Skip all files that are not in the assets directory.
        let mut path_iter = file_path.iter();
        if path_iter.next() != Some(OsStr::new("assets")) {
            return None;
        }

        // Use the next path component as the namespace.
        let Some(namespace) = path_iter.next().and_then(OsStr::to_str) else {
            return None;
        };

        // Skip all files that are not in the textures directory.
        if path_iter.next() != Some(OsStr::new("textures")) {
            return None;
        }

        // Remove the file extension from the path.
        let path = path_iter.as_path().to_str()?;
        let path = path.trim_end_matches(ext).trim_end_matches('.');

        // Create the resource location.
        let mut key = CompactString::from(namespace);
        key.push(':');
        key.push_str(path);

        match ResourceLocation::try_from(key) {
            Some(key) => Some(key),
            None => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                error!(
                    "Failed to parse resource location from {}",
                    file_path.display()
                );
                None
            }
        }
    }
}
