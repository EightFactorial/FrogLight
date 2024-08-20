use async_lock::Mutex;
use async_zip::base::read::{mem::ZipFileReader, WithEntry, ZipEntryReader};
use bevy_asset::{io::Reader, Asset, AssetLoader, AsyncReadExt, Handle, LoadContext, LoadedAsset};
use bevy_log::error;
use bevy_tasks::AsyncComputeTaskPool;
use froglight_common::ResourceKey;
use futures_lite::{io::Cursor, AsyncBufRead};

use super::{EntryType, ResourcePackLoaderError};
use crate::assets::{ResourcePack, ResourcePackMeta};

/// An [`AssetLoader`] for loading [`ResourcePack`]s from ZIP files.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourcePackZipLoader;

impl AssetLoader for ResourcePackZipLoader {
    type Asset = ResourcePack;
    type Settings = ();
    type Error = ResourcePackLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        (): &'a (),
        context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        Self::load_from_zip(reader, context).await
    }

    fn extensions(&self) -> &[&str] { &["zip"] }
}

#[allow(clippy::unused_async)]
impl ResourcePackZipLoader {
    async fn load_from_zip(
        reader: &mut Reader<'_>,
        context: &mut LoadContext<'_>,
    ) -> Result<ResourcePack, ResourcePackLoaderError> {
        let mut zip_buffer = Vec::new();
        reader.read_to_end(&mut zip_buffer).await?;
        let mut zip = ZipFileReader::new(zip_buffer).await?;

        // Load the `pack.mcmeta` metadata file.
        let meta = Self::load_mcmeta_from_zip(&mut zip, context).await?;
        let meta = context.add_labeled_asset(String::from("pack.mcmeta"), meta);

        // Create a new `ResourcePack` with the metadata.
        let resourcepack = Mutex::new(ResourcePack::new(meta));
        let context = Mutex::new(context);

        // Normally this would be done in the `IoTaskPool`,
        // but we're decompressing thousands of assets from large zip files.
        AsyncComputeTaskPool::get().scope(|pool| {
            // Iterate over all entries in the ZIP file.
            for (index, entry) in zip.file().entries().iter().enumerate() {
                let Some((entry_type, asset_key)) = EntryType::from_entry(entry) else {
                    continue;
                };

                let resourcepack = &resourcepack;
                let context = &context;
                let zip = &zip;

                // Process every entry in a separate task.
                pool.spawn(async move {
                    if let Ok(entry_reader) = zip.reader_with_entry(index).await {
                        if let Err(err) = Self::process_zip_entry(
                            entry_type,
                            asset_key,
                            resourcepack,
                            entry_reader,
                            context,
                        )
                        .await
                        {
                            error!("ResourcePack: Failed to load asset, {err}");
                        }
                    }
                });
            }
        });

        Ok(resourcepack.into_inner())
    }

    async fn process_zip_entry<R: AsyncBufRead + Unpin>(
        entry_type: EntryType,
        asset_key: ResourceKey,
        resourcepack: &Mutex<ResourcePack>,
        mut entry_reader: ZipEntryReader<'_, R, WithEntry<'_>>,
        context: &Mutex<&mut LoadContext<'_>>,
    ) -> Result<(), ResourcePackLoaderError> {
        match entry_type {
            // EntryType::BlockModel => {
            //     let asset_handle =
            //         Self::get_zipped_asset(&asset_key, &mut entry_reader,
            // context).await?;     resourcepack.block_models.
            // insert(asset_key, asset_handle); }
            EntryType::Language => {
                let asset_handle =
                    Self::add_zipped_asset(&asset_key, &mut entry_reader, context).await?;
                resourcepack.lock().await.languages.insert(asset_key, asset_handle);
            }
            EntryType::ResourcePack => {
                let asset_handle =
                    Self::add_zipped_asset(&asset_key, &mut entry_reader, context).await?;
                resourcepack.lock().await.children.insert(asset_key, asset_handle);
            }
            EntryType::Sound => {
                let asset_handle =
                    Self::add_zipped_asset(&asset_key, &mut entry_reader, context).await?;
                resourcepack.lock().await.sounds.insert(asset_key, asset_handle);
            }
            EntryType::SoundMap => {
                let asset_handle =
                    Self::add_zipped_asset(&asset_key, &mut entry_reader, context).await?;
                resourcepack.lock().await.sound_maps.insert(asset_key, asset_handle);
            }
            EntryType::Texture => {
                let asset_handle =
                    Self::add_zipped_asset(&asset_key, &mut entry_reader, context).await?;
                resourcepack.lock().await.textures.insert(asset_key, asset_handle);
            }
            // EntryType::TextureAtlas => {
            //     let asset_handle =
            //         Self::get_zipped_asset(&asset_key, &mut entry_reader,
            // context).await?;     resourcepack.texture_atlases.
            // insert(asset_key, asset_handle); },
            _ => {}
        }
        Ok(())
    }

    async fn add_zipped_asset<A: Asset, R: AsyncBufRead + Unpin>(
        asset_key: &ResourceKey,
        entry_reader: &mut ZipEntryReader<'_, R, WithEntry<'_>>,
        context: &Mutex<&mut LoadContext<'_>>,
    ) -> Result<Handle<A>, ResourcePackLoaderError> {
        let loaded_asset = Self::load_zipped_asset_lock(entry_reader, context).await?;
        Ok(context.lock().await.add_loaded_labeled_asset(asset_key.to_string(), loaded_asset))
    }

    /// Returns a [`LoadedAsset`] from a [`ZipEntryReader`].
    ///
    /// # Errors
    /// Returns an
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) async fn load_zipped_asset_lock<A: Asset, R: AsyncBufRead + Unpin>(
        entry_reader: &mut ZipEntryReader<'_, R, WithEntry<'_>>,
        context: &Mutex<&mut LoadContext<'_>>,
    ) -> Result<LoadedAsset<A>, ResourcePackLoaderError> {
        // Decompress the file into a buffer.
        let uncompressed_size = entry_reader.entry().uncompressed_size();
        let mut uncompressed_buffer = Vec::with_capacity(uncompressed_size as usize);
        entry_reader.read_to_end(&mut uncompressed_buffer).await?;
        let mut cursor = Cursor::new(uncompressed_buffer);

        // Use the asset's `AssetLoader` to load the asset.
        let mut context = context.lock().await;
        let nested_loader = context.loader().with_asset_type::<A>();
        let direct_loader = nested_loader.direct().with_reader(&mut cursor);

        let filename = entry_reader.entry().filename().as_str()?.to_string();
        direct_loader.load(filename).await.map_err(ResourcePackLoaderError::from)
    }
}

impl ResourcePackZipLoader {
    async fn load_mcmeta_from_zip(
        zip: &mut ZipFileReader,
        context: &mut LoadContext<'_>,
    ) -> Result<ResourcePackMeta, ResourcePackLoaderError> {
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
                    Err(err) => error!("ResourcePack: Failed to load icon, {err}"),
                }
            }
        }

        Ok(mcmeta)
    }

    #[allow(clippy::cast_possible_truncation)]
    pub(crate) async fn load_zipped_asset_no_lock<A: Asset, R: AsyncBufRead + Unpin>(
        entry_reader: &mut ZipEntryReader<'_, R, WithEntry<'_>>,
        context: &mut LoadContext<'_>,
    ) -> Result<LoadedAsset<A>, ResourcePackLoaderError> {
        // Decompress the file into a buffer.
        let uncompressed_size = entry_reader.entry().uncompressed_size();
        let mut uncompressed_buffer = Vec::with_capacity(uncompressed_size as usize);
        entry_reader.read_to_end(&mut uncompressed_buffer).await?;
        let mut cursor = Cursor::new(uncompressed_buffer);

        // Use the asset's `AssetLoader` to load the asset.
        let nested_loader = context.loader().with_asset_type::<A>();
        let direct_loader = nested_loader.direct().with_reader(&mut cursor);

        let filename = entry_reader.entry().filename().as_str()?.to_string();
        direct_loader.load(filename).await.map_err(ResourcePackLoaderError::from)
    }
}
