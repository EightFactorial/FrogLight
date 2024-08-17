use async_zip::base::read::seek::ZipFileReader;
use bevy_asset::{io::Reader, AssetLoader, LoadContext};
use bevy_log::error;
use bevy_render::texture::Image;
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
        Self::read_pack_metadata_zip(&mut zip, context).await
    }

    pub(crate) async fn read_pack_metadata_zip(
        zip: &mut ZipFileReader<BufReader<&mut Reader<'_>>>,
        context: &mut LoadContext<'_>,
    ) -> Result<ResourcePackMeta, ResourcePackLoaderError> {
        // Load the `pack.png` icon if it exists.
        let mut icon_handle = None;
        if let Some(icon_index) = zip.file().entries().iter().position(|entry| {
            entry.filename().as_str().map_or(false, |filename| filename == "pack.png")
        }) {
            let mut icon_reader = zip.reader_with_entry(icon_index).await?;
            let loaded_icon =
                ResourcePackZipLoader::load_zipped_asset(&mut icon_reader, context).await?;
            icon_handle = Some(
                context.add_loaded_labeled_asset::<Image>(String::from("pack.png"), loaded_icon),
            );
        }

        #[cfg(debug_assertions)]
        bevy_log::debug!(
            "ResourcePackMeta: \"{}\" Icon -> \"{icon_handle:?}\"",
            context.asset_path()
        );

        // Get the index of the `pack.mcmeta` file.
        let Some(mcmeta_index) = zip.file().entries().iter().position(|entry| {
            entry.filename().as_str().map_or(false, |filename| filename == "pack.mcmeta")
        }) else {
            error!("ResourcePackMeta: No metadata found for \"{}\"", context.asset_path());
            return Err(ResourcePackLoaderError::NoMetadata);
        };

        // Load the `pack.mcmeta` file.
        let mut mcmeta_reader = zip.reader_with_entry(mcmeta_index).await?;
        ResourcePackZipLoader::load_zipped_asset::<ResourcePackMeta>(&mut mcmeta_reader, context)
            .await
            .map(|loaded| {
                let mut mcmeta = loaded.take();

                // If the icon was not already set, set it.
                if matches!((&mcmeta.icon, &icon_handle), (None, Some(_))) {
                    #[cfg(debug_assertions)]
                    bevy_log::debug!(
                        "ResourcePackMeta: \"{}\" Zip Icon -> \"{icon_handle:?}\"",
                        context.asset_path()
                    );

                    mcmeta.icon = icon_handle;
                }

                mcmeta
            })
    }
}
