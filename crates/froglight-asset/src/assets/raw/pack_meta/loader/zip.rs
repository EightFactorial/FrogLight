use async_zip::{
    base::read::{
        mem::ZipFileReader as MemZipFileReader, seek::ZipFileReader as SeekZipFileReader,
    },
    StoredZipEntry,
};
use bevy_asset::{io::Reader, Asset, AssetLoader, LoadContext, LoadedAsset};
use futures_lite::{
    io::{BufReader, Cursor},
    AsyncBufRead, AsyncSeek,
};

use crate::assets::{
    raw::pack::loader::{ResourcePackLoaderError, ResourcePackZipLoader},
    ResourcePackMeta,
};

/// An [`AssetLoader`] for loading [`ResourcePackMeta`]s from ZIP files.
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
        Self::load_zipfile_metadata(
            &mut SeekZipFileReader::new(BufReader::new(reader)).await?,
            context,
        )
        .await
    }

    fn extensions(&self) -> &[&str] { &["zip"] }
}

impl ResourcePackMetaZipLoader {
    /// Read a [`ResourcePackMeta`] from a ZIP file.
    ///
    /// Requires a reference to a [`SeekZipFileReader`] or [`MemZipFileReader`].
    #[allow(private_bounds)]
    pub(crate) async fn load_zipfile_metadata<'a, R: 'a + AsyncBufRead + AsyncSeek + Unpin>(
        zip_reader: impl Into<ZipEntryReader<'a, R>>,
        context: &mut LoadContext<'_>,
    ) -> Result<ResourcePackMeta, ResourcePackLoaderError> {
        let mut zip_reader = zip_reader.into();
        let (mcmeta_index, icon_index) = zip_reader.metadata_indexes();
        let mcmeta_index = mcmeta_index.ok_or(ResourcePackLoaderError::NoMetadata)?;

        // Load the `pack.mcmeta` metadata file.
        let mcmeta = zip_reader.load_asset(mcmeta_index, context).await?;
        let mut mcmeta: ResourcePackMeta = mcmeta.take();

        // Load the `pack.png` icon file, if it exists.
        if let Some(icon_index) = icon_index {
            if let Ok(loaded_icon) = zip_reader.load_asset(icon_index, context).await {
                mcmeta.icon =
                    Some(context.add_loaded_labeled_asset(String::from("pack.png"), loaded_icon));
            }
        }

        Ok(mcmeta)
    }
}

enum ZipEntryReader<'a, R: AsyncBufRead + AsyncSeek + Unpin> {
    Seek(&'a mut SeekZipFileReader<R>),
    Mem(&'a MemZipFileReader),
}

impl<R: AsyncBufRead + AsyncSeek + Unpin> ZipEntryReader<'_, R> {
    fn entries(&self) -> &[StoredZipEntry] {
        match self {
            Self::Seek(reader) => reader.file().entries(),
            Self::Mem(reader) => reader.file().entries(),
        }
    }

    const PACK_PNG: &'static str = "pack.png";
    const PACK_MCMETA: &'static str = "pack.mcmeta";

    fn metadata_indexes(&self) -> (Option<usize>, Option<usize>) {
        let mut icon_index: Option<usize> = None;
        let mut mcmeta_index: Option<usize> = None;
        for (index, entry) in self.entries().iter().enumerate() {
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

    async fn load_asset<A: Asset>(
        &mut self,
        index: usize,
        context: &mut LoadContext<'_>,
    ) -> Result<LoadedAsset<A>, ResourcePackLoaderError> {
        match self {
            Self::Seek(reader) => {
                let mut entry_reader = reader.reader_with_entry(index).await?;
                ResourcePackZipLoader::async_load_zipped_asset(&mut entry_reader, context).await
            }
            Self::Mem(reader) => {
                let mut entry_reader = reader.reader_with_entry(index).await?;
                ResourcePackZipLoader::async_load_zipped_asset(&mut entry_reader, context).await
            }
        }
    }
}

impl<'a, R: AsyncBufRead + AsyncSeek + Unpin> From<&'a mut SeekZipFileReader<R>>
    for ZipEntryReader<'a, R>
{
    fn from(reader: &'a mut SeekZipFileReader<R>) -> Self { Self::Seek(reader) }
}

impl<'a> From<&'a MemZipFileReader> for ZipEntryReader<'a, Cursor<&[u8]>> {
    fn from(reader: &'a MemZipFileReader) -> Self { Self::Mem(reader) }
}
impl<'a> From<&'a mut MemZipFileReader> for ZipEntryReader<'a, Cursor<&[u8]>> {
    fn from(reader: &'a mut MemZipFileReader) -> Self { Self::Mem(reader) }
}
