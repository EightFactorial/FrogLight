use async_zip::{
    base::read::{
        mem::ZipFileReader as MemZipFileReader, seek::ZipFileReader as SeekZipFileReader,
    },
    StoredZipEntry,
};
use bevy_asset::{io::Reader, AssetLoader, LoadContext};
use bevy_log::error;
use futures_lite::{io::BufReader, AsyncBufRead, AsyncSeek};

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
        let mut zip = SeekZipFileReader::new(BufReader::new(reader)).await?;
        Self::async_seek_zipfile_metadata(&mut zip, context).await
    }

    fn extensions(&self) -> &[&str] { &["zip"] }
}

// TODO: Collapse `async_seek_zipfile_metadata` and `async_mem_zipfile_metadata`
// into one function.
impl ResourcePackMetaZipLoader {
    /// Read a [`ResourcePackMeta`] from a ZIP file using a
    /// [`SeekZipFileReader`].
    pub(crate) async fn async_seek_zipfile_metadata<R: AsyncBufRead + AsyncSeek + Unpin>(
        zip: &mut SeekZipFileReader<R>,
        context: &mut LoadContext<'_>,
    ) -> Result<ResourcePackMeta, ResourcePackLoaderError> {
        let (mcmeta_index, icon_index) = Self::get_metadata_indexes(zip.file().entries());

        // Load the `pack.mcmeta` metadata file.
        let mcmeta_index = mcmeta_index.ok_or(ResourcePackLoaderError::NoMetadata)?;
        let mut mcmeta_reader = zip.reader_with_entry(mcmeta_index).await?;

        let mcmeta =
            ResourcePackZipLoader::async_load_zipped_asset(&mut mcmeta_reader, context).await?;
        let mut mcmeta: ResourcePackMeta = mcmeta.take();

        // Load the `pack.png` icon file, if it exists.
        if let Some(icon_index) = icon_index {
            if let Ok(mut icon_reader) = zip.reader_with_entry(icon_index).await {
                match ResourcePackZipLoader::async_load_zipped_asset(&mut icon_reader, context)
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

    /// Read a [`ResourcePackMeta`] from a ZIP file using a
    /// [`MemZipFileReader`].
    pub(crate) async fn async_mem_zipfile_metadata(
        zip: &mut MemZipFileReader,
        context: &mut LoadContext<'_>,
    ) -> Result<ResourcePackMeta, ResourcePackLoaderError> {
        let (mcmeta_index, icon_index) = Self::get_metadata_indexes(zip.file().entries());

        // Load the `pack.mcmeta` metadata file.
        let mcmeta_index = mcmeta_index.ok_or(ResourcePackLoaderError::NoMetadata)?;
        let mut mcmeta_reader = zip.reader_with_entry(mcmeta_index).await?;

        let mcmeta =
            ResourcePackZipLoader::async_load_zipped_asset(&mut mcmeta_reader, context).await?;
        let mut mcmeta: ResourcePackMeta = mcmeta.take();

        // Load the `pack.png` icon file, if it exists.
        if let Some(icon_index) = icon_index {
            if let Ok(mut icon_reader) = zip.reader_with_entry(icon_index).await {
                match ResourcePackZipLoader::async_load_zipped_asset(&mut icon_reader, context)
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

    const PACK_PNG: &str = "pack.png";
    const PACK_MCMETA: &str = "pack.mcmeta";

    /// Find the index of the `pack.mcmeta` and `pack.png` files.
    fn get_metadata_indexes(entries: &[StoredZipEntry]) -> (Option<usize>, Option<usize>) {
        let mut icon_index: Option<usize> = None;
        let mut mcmeta_index: Option<usize> = None;
        for (index, entry) in entries.iter().enumerate() {
            match entry.filename().as_str() {
                Ok(Self::PACK_MCMETA) => {
                    mcmeta_index = Some(index);
                    if icon_index.is_some() {
                        break;
                    }
                }
                Ok(Self::PACK_PNG) => {
                    icon_index = Some(index);
                    if mcmeta_index.is_some() {
                        break;
                    }
                }
                _ => {}
            }
        }
        (mcmeta_index, icon_index)
    }
}
