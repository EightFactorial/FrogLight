use std::{
    ffi::OsStr,
    io::{Cursor, Read, Seek},
    path::Path,
};

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
};
use compact_str::CompactString;
use mc_rs_core::ResourceLocation;
use thiserror::Error;
use zip::ZipArchive;

use crate::resourcepacks::ResourcePackAsset;

mod icon;
mod mcmeta;
mod textures;

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
    #[error("Unsupported file extension")]
    UnsupportedExtension,
    #[error("Invalid path")]
    InvalidPath,
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourcePackLoader;

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
                Some(Some(ext)) => match self.extensions().contains(&ext) {
                    false => Err(ResourcePackLoaderError::UnsupportedExtension),
                    true => {
                        let mut bytes = Vec::new();
                        reader.read_to_end(&mut bytes).await?;

                        let zip = ZipArchive::new(Cursor::new(bytes))?;
                        ResourcePackLoader::load_zip(zip, load_context)
                    }
                },
                _ => Err(ResourcePackLoaderError::InvalidPath),
            }
        })
    }

    fn extensions(&self) -> &[&str] { &["zip", "jar"] }
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
            icon: icon::read_icon(&mut zip, load_context)?,
            mcmeta: mcmeta::read_mcmeta(&mut zip, load_context)?,
            textures: textures::read_textures(&mut zip, load_context)?,
        })
    }

    fn path_to_resource_location(
        file_path: &Path,
        filter_folder: &str,
    ) -> Option<ResourceLocation> {
        let ext = file_path.extension()?.to_str()?;

        // Skip all files that are not in the assets directory.
        let mut path_iter = file_path.iter();
        if path_iter.next() != Some(OsStr::new("assets")) {
            return None;
        }

        // Use the next path component as the namespace.
        let Some(namespace) = path_iter.next().and_then(OsStr::to_str) else {
            return None;
        };

        // Skip all files that are not in the filtered folder.
        if path_iter.next() != Some(filter_folder.as_ref()) {
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
