use std::{
    ffi::OsStr,
    io::{Cursor, Read, Seek},
    path::Path,
};

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    utils::BoxedFuture,
};
use compact_str::CompactString;
use mc_rs_core::ResourceLocation;
use zip::ZipArchive;

#[cfg(any(debug_assertions, feature = "debug"))]
use bevy::log::{debug, error};

mod components;

mod error;
pub use error::ResourcePackLoaderError;

use super::ResourcePackAsset;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourcePackLoader;

impl AssetLoader for ResourcePackLoader {
    type Error = ResourcePackLoaderError;
    type Asset = ResourcePackAsset;
    type Settings = ();

    fn extensions(&self) -> &[&str] { &["zip", "jar"] }

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            if let Some(Some(ext)) = load_context.path().extension().map(OsStr::to_str) {
                if self.extensions().contains(&ext) {
                    let mut bytes = Vec::new();
                    reader.read_to_end(&mut bytes).await?;

                    let zip = ZipArchive::new(Cursor::new(bytes))?;
                    ResourcePackLoader::load_zip(zip, load_context)
                } else {
                    Err(ResourcePackLoaderError::UnsupportedExtension)
                }
            } else {
                Err(ResourcePackLoaderError::InvalidPath)
            }
        })
    }
}

impl ResourcePackLoader {
    /// Loads a resourcepack from a zip archive.
    fn load_zip(
        mut zip: ZipArchive<impl Read + Seek>,
        load_context: &mut LoadContext,
    ) -> Result<ResourcePackAsset, ResourcePackLoaderError> {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Loading resourcepack: {}", load_context.asset_path());

        // TODO: Read all files in one pass.
        Ok(ResourcePackAsset {
            icon: components::icon::read_icon(&mut zip, load_context)?,
            mcmeta: components::mcmeta::read_mcmeta(&mut zip, load_context)?,
            textures: components::textures::read_textures(&mut zip, load_context)?,
            models: components::models::read_models(&mut zip, load_context)?,
            sounds: components::sounds::read_sounds(&mut zip, load_context)?,
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

        if let Some(key) = ResourceLocation::try_from(key) {
            Some(key)
        } else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!(
                "Failed to parse resource location from {}",
                file_path.display()
            );
            None
        }
    }
}
