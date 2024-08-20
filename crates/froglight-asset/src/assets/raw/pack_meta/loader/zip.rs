use async_zip::base::read::seek::ZipFileReader;
use bevy_asset::{io::Reader, AssetLoader, LoadContext, LoadedAsset};
use bevy_log::error;
use futures_lite::io::BufReader;

use crate::assets::{
    raw::pack::loader::{ResourcePackLoaderError, ResourcePackZipLoader},
    ResourcePackMeta,
};

/// An [`AssetLoader`] for loading [`ResourcePackMeta`]s.
///
/// This is useful when you only need the metadata of a
/// [`ResourcePack`](crate::assets::ResourcePack).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourcePackMetaZipLoader;

impl AssetLoader for ResourcePackMetaZipLoader {
    type Asset = ResourcePackMeta;
    type Settings = ();
    type Error = ResourcePackLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        (): &'a (),
        context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        Self::load_metadata_zipfile(reader, context).await
    }

    fn extensions(&self) -> &[&str] { &["zip"] }
}

impl ResourcePackMetaZipLoader {
    async fn load_metadata_zipfile(
        reader: &mut Reader<'_>,
        context: &mut LoadContext<'_>,
    ) -> Result<ResourcePackMeta, ResourcePackLoaderError> {
        let mut zip = ZipFileReader::new(BufReader::new(reader)).await?;

        // Find the index of the `pack.mcmeta` and `pack.png` files.
        let mut icon_index: Option<usize> = None;
        let mut mcmeta_index: Option<usize> = None;

        for (index, entry) in zip.file().entries().iter().enumerate() {
            match entry.filename().as_str() {
                Ok("pack.mcmeta") => {
                    mcmeta_index = Some(index);
                    if icon_index.is_some() {
                        break;
                    }
                }
                Ok("pack.png") => {
                    icon_index = Some(index);
                    if mcmeta_index.is_some() {
                        break;
                    }
                }
                _ => {}
            }
        }

        // Load the `pack.mcmeta` metadata file.
        let mcmeta_index = mcmeta_index.ok_or(ResourcePackLoaderError::NoMetadata)?;
        let mut mcmeta_reader = zip.reader_with_entry(mcmeta_index).await?;

        let mut mcmeta: ResourcePackMeta =
            ResourcePackZipLoader::load_zipped_asset_no_lock(&mut mcmeta_reader, context)
                .await
                .map(LoadedAsset::take)?;

        // Load the `pack.png` icon file, if it exists.
        if let Some(icon_index) = icon_index {
            if let Ok(mut icon_reader) = zip.reader_with_entry(icon_index).await {
                match ResourcePackZipLoader::load_zipped_asset_no_lock(&mut icon_reader, context)
                    .await
                {
                    Ok(loaded_icon) => {
                        mcmeta.icon = Some(
                            context.add_loaded_labeled_asset(String::from("pack.png"), loaded_icon),
                        );
                    }
                    Err(err) => error!("ResourcePackMeta: Failed to load icon, {err}"),
                }
            }
        }

        Ok(mcmeta)
    }
}
