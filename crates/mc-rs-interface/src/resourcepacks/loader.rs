use std::{
    ffi::OsStr,
    io::{Cursor, Read, Seek},
};

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    render::texture::{CompressedImageFormats, ImageSampler, ImageType},
    utils::HashMap,
};
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
            // TODO: Load textures
            textures: HashMap::new(),
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
}
