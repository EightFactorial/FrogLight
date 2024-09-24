use async_lock::{Mutex, RwLock};
use async_zip::base::read::{mem::ZipFileReader, WithEntry, ZipEntryReader};
use bevy_asset::{io::Reader, Asset, AssetLoader, AsyncReadExt, Handle, LoadContext, LoadedAsset};
use bevy_log::error;
use bevy_tasks::AsyncComputeTaskPool;
use froglight_common::ResourceKey;
use futures_lite::{io::Cursor, AsyncBufRead};

use super::{EntryType, ResourcePackLoaderError};
use crate::assets::{raw::pack_meta::ResourcePackMetaZipLoader, ResourcePack};

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
        Self::load_resourcepack_from_zip(reader, context).await
    }

    fn extensions(&self) -> &[&str] { &["zip"] }
}

impl ResourcePackZipLoader {
    #[inline]
    async fn load_resourcepack_from_zip(
        reader: &mut Reader<'_>,
        context: &mut LoadContext<'_>,
    ) -> Result<ResourcePack, ResourcePackLoaderError> {
        #[cfg(debug_assertions)]
        bevy_log::info!("ResourcePack: Starting \"{}\"", context.path().display());

        let zip = {
            let mut zip_buffer = Vec::new();
            reader.read_to_end(&mut zip_buffer).await?;
            ZipFileReader::new(zip_buffer).await?
        };

        // Load the `pack.mcmeta` and `pack.png` files.
        let meta = ResourcePackMetaZipLoader::load_zipfile_metadata(&zip, context).await?;
        let meta = context.add_labeled_asset(String::from("pack.mcmeta"), meta);

        // Create a new `ResourcePack` with the metadata.
        let resourcepack = Mutex::new(ResourcePack::new(meta));
        let context = RwLock::new(&mut *context);

        // Normally this would be done in the `IoTaskPool`,
        // but we're decompressing thousands of assets from large zip files.
        AsyncComputeTaskPool::get().scope(|pool| {
            let context = &context;
            let resourcepack = &resourcepack;
            let zip = &zip;

            // Iterate over all entries in the ZIP file.
            for (index, entry) in zip.file().entries().iter().enumerate() {
                let Some((entry_type, asset_key)) = EntryType::from_entry(entry) else {
                    continue;
                };

                // Process every entry in a separate task.
                pool.spawn(async move {
                    if let Ok(entry_reader) = zip.reader_with_entry(index).await {
                        if let Err(err) = Self::process_zip_entry(
                            entry_type,
                            asset_key,
                            entry_reader,
                            context,
                            resourcepack,
                        )
                        .await
                        {
                            error!("ResourcePack: Failed to load asset, {err}");
                        }
                    } else {
                        error!(
                            "ResourcePack: Failed to read entry, {}",
                            entry.filename().as_str().unwrap_or(asset_key.as_str())
                        );
                    }
                });
            }
        });

        let resourcepack = resourcepack.into_inner();
        let context = context.into_inner();

        #[cfg(debug_assertions)]
        bevy_log::info!("ResourcePack: Finished \"{}\"", context.path().display());

        Ok(resourcepack)
    }

    async fn process_zip_entry<R: AsyncBufRead + Unpin>(
        entry_type: EntryType,
        asset_key: ResourceKey,
        entry_reader: ZipEntryReader<'_, R, WithEntry<'_>>,
        context: &RwLock<&mut LoadContext<'_>>,
        resourcepack: &Mutex<ResourcePack>,
    ) -> Result<(), ResourcePackLoaderError> {
        match entry_type {
            EntryType::BlockModel => {
                let asset_handle =
                    Self::async_add_zipped_asset(&asset_key, entry_reader, context).await?;
                resourcepack.lock().await.block_models.insert(asset_key, asset_handle);
            }
            EntryType::BlockState => {
                let asset_handle =
                    Self::async_add_zipped_asset(&asset_key, entry_reader, context).await?;
                resourcepack.lock().await.block_states.insert(asset_key, asset_handle);
            }
            EntryType::Language => {
                let asset_handle =
                    Self::async_add_zipped_asset(&asset_key, entry_reader, context).await?;
                resourcepack.lock().await.languages.insert(asset_key, asset_handle);
            }
            EntryType::ResourcePack => {
                let asset_handle =
                    Self::async_add_zipped_asset(&asset_key, entry_reader, context).await?;
                resourcepack.lock().await.children.insert(asset_key, asset_handle);
            }
            EntryType::Sound => {
                let asset_handle =
                    Self::async_add_zipped_asset(&asset_key, entry_reader, context).await?;
                resourcepack.lock().await.sounds.insert(asset_key, asset_handle);
            }
            EntryType::SoundMap => {
                let asset_handle =
                    Self::async_add_zipped_asset(&asset_key, entry_reader, context).await?;
                resourcepack.lock().await.sound_maps.insert(asset_key, asset_handle);
            }
            EntryType::Texture => {
                let asset_handle =
                    Self::async_add_zipped_asset(&asset_key, entry_reader, context).await?;
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

    /// Returns a [`Handle`] from a [`ResourceKey`] and [`ZipEntryReader`].
    ///
    /// # Errors
    /// Returns an [`ResourcePackLoaderError`] if the asset could not be loaded.
    async fn async_add_zipped_asset<A: Asset, R: AsyncBufRead + Unpin>(
        asset_key: &ResourceKey,
        mut entry_reader: ZipEntryReader<'_, R, WithEntry<'_>>,
        context: &RwLock<&mut LoadContext<'_>>,
    ) -> Result<Handle<A>, ResourcePackLoaderError> {
        // Load the asset from the ZIP file.
        // Note: This uses `context.read()`, so it can be done concurrently.
        let loaded_asset = {
            let context = context.read().await;
            let mut asset_context = context.begin_labeled_asset();
            Self::async_load_zipped_asset(&mut entry_reader, &mut asset_context).await?
        };

        // Add the loaded asset to the context.
        let mut context = context.write().await;
        Ok(context.add_loaded_labeled_asset(asset_key.to_string(), loaded_asset))
    }

    /// Returns a [`LoadedAsset`] from a [`ZipEntryReader`].
    ///
    /// # Errors
    /// Returns an [`ResourcePackLoaderError`] if the asset could not be loaded.
    #[inline]
    pub(crate) async fn async_load_zipped_asset<A: Asset, R: AsyncBufRead + Unpin>(
        entry_reader: &mut ZipEntryReader<'_, R, WithEntry<'_>>,
        context: &mut LoadContext<'_>,
    ) -> Result<LoadedAsset<A>, ResourcePackLoaderError> {
        // Decompress the file into a buffer.
        #[allow(clippy::cast_possible_truncation)]
        let mut cursor = {
            let mut buf = Vec::with_capacity(entry_reader.entry().uncompressed_size() as usize);
            entry_reader.read_to_end(&mut buf).await?;
            Cursor::new(buf)
        };

        // Use the asset's `AssetLoader` to load the asset.
        let nested_loader = context.loader().with_asset_type::<A>();
        let direct_loader = nested_loader.direct().with_reader(&mut cursor);

        let filename = entry_reader.entry().filename().as_str()?.to_string();
        direct_loader.load(filename).await.map_err(ResourcePackLoaderError::from)
    }
}
