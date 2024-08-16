use async_zip::base::read::{seek::ZipFileReader, WithEntry, ZipEntryReader};
use bevy_asset::{
    io::Reader, Asset, AssetLoader, AssetPath, AsyncReadExt, LoadContext, LoadedAsset,
};
use bevy_render::texture::Image;
use futures_lite::io::{BufReader, Cursor};

use crate::assets::{raw::pack::loader::ResourcePackLoaderError, ResourcePackMeta};

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
                Self::load_zipped_asset(String::from("pack.png").into(), &mut icon_reader, context)
                    .await?;
            icon_handle = Some(
                context.add_loaded_labeled_asset::<Image>(String::from("pack.png"), loaded_icon),
            );
        }

        // Get the index of the `pack.mcmeta` file.
        let Some(mcmeta_index) = zip.file().entries().iter().position(|entry| {
            entry.filename().as_str().map_or(false, |filename| filename == "pack.mcmeta")
        }) else {
            return Err(ResourcePackLoaderError::NoMetadata);
        };

        // Load the `pack.mcmeta` file.
        let mut mcmeta_reader = zip.reader_with_entry(mcmeta_index).await?;
        Self::load_zipped_asset::<ResourcePackMeta>(
            String::from("pack.mcmeta").into(),
            &mut mcmeta_reader,
            context,
        )
        .await
        .map(|loaded| {
            let mut mcmeta = loaded.take();

            // If the icon was not already set, set it.
            if mcmeta.icon.is_none() {
                #[cfg(debug_assertions)]
                if icon_handle.is_some() {
                    bevy_log::debug!(
                        "ResourcePackMeta: Using ZIP icon for \"{}\"",
                        context.asset_path()
                    );
                }

                mcmeta.icon = icon_handle;
            }

            mcmeta
        })
    }

    #[allow(clippy::cast_possible_truncation)]
    async fn load_zipped_asset<A: Asset>(
        asset_path: AssetPath<'static>,
        entry_reader: &mut ZipEntryReader<'_, BufReader<&mut Reader<'_>>, WithEntry<'_>>,
        context: &mut LoadContext<'_>,
    ) -> Result<LoadedAsset<A>, ResourcePackLoaderError> {
        // Decompress the file into a buffer.
        let uncompressed_size = entry_reader.entry().uncompressed_size();
        let mut uncompressed_buffer = Vec::with_capacity(uncompressed_size as usize);
        entry_reader.read_to_end(&mut uncompressed_buffer).await?;
        let mut uncompressed_cursor = Cursor::new(uncompressed_buffer);

        // Use the asset's `AssetLoader` to load the asset.
        let nested_loader = context.loader().with_asset_type::<A>();
        let direct_loader = nested_loader.direct().with_reader(&mut uncompressed_cursor);
        direct_loader.load(asset_path).await.map_err(|_| ResourcePackLoaderError::NoMetadata)
    }
}
