use async_zip::base::read::{seek::ZipFileReader, WithEntry, ZipEntryReader};
use bevy_asset::{io::Reader, Asset, AssetLoader, AsyncReadExt, Handle, LoadContext, LoadedAsset};
use froglight_common::ResourceKey;
use futures_lite::io::{BufReader, Cursor};

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
        let mut zip = ZipFileReader::new(BufReader::new(reader)).await?;

        // Load the `pack.mcmeta` metadata file.
        let meta = ResourcePackMetaZipLoader::read_pack_metadata_zip(&mut zip, context).await?;
        let meta = context.add_labeled_asset(String::from("pack.mcmeta"), meta);

        // Create a new `ResourcePack` with the metadata.
        let mut resourcepack = ResourcePack::new(meta);

        // Iterate over all entries in the ZIP file.
        #[allow(clippy::unnecessary_to_owned)]
        for (index, entry) in zip.file().entries().to_vec().into_iter().enumerate() {
            let Some((entry_type, asset_key)) = EntryType::from_entry(&entry) else {
                continue;
            };
            let Ok(mut entry_reader) = zip.reader_with_entry(index).await else { continue };

            match entry_type {
                // EntryType::BlockModel => {
                //     let asset_handle =
                //         Self::get_zipped_asset(&asset_key, &mut entry_reader, context).await?;
                //     resourcepack.block_models.insert(asset_key, asset_handle);
                // }
                EntryType::Language => {
                    let asset_handle =
                        Self::add_zipped_asset(&asset_key, &mut entry_reader, context).await?;
                    resourcepack.languages.insert(asset_key, asset_handle);
                }
                EntryType::ResourcePack => {
                    let asset_handle =
                        Self::add_zipped_asset(&asset_key, &mut entry_reader, context).await?;
                    resourcepack.children.insert(asset_key, asset_handle);
                }
                EntryType::Sound => {
                    let asset_handle =
                        Self::add_zipped_asset(&asset_key, &mut entry_reader, context).await?;
                    resourcepack.sounds.insert(asset_key, asset_handle);
                }
                EntryType::SoundMap => {
                    let asset_handle =
                        Self::add_zipped_asset(&asset_key, &mut entry_reader, context).await?;
                    resourcepack.sound_maps.insert(asset_key, asset_handle);
                }
                EntryType::Texture => {
                    let asset_handle =
                        Self::add_zipped_asset(&asset_key, &mut entry_reader, context).await?;
                    resourcepack.textures.insert(asset_key, asset_handle);
                }
                // EntryType::TextureAtlas => {
                //     let asset_handle =
                //         Self::get_zipped_asset(&asset_key, &mut entry_reader, context).await?;
                //     resourcepack.texture_atlases.insert(asset_key, asset_handle);
                // },
                _ => continue,
            }
        }

        Ok(resourcepack)
    }

    async fn add_zipped_asset<A: Asset>(
        asset_key: &ResourceKey,
        entry_reader: &mut ZipEntryReader<'_, BufReader<&mut Reader<'_>>, WithEntry<'_>>,
        context: &mut LoadContext<'_>,
    ) -> Result<Handle<A>, ResourcePackLoaderError> {
        let loaded_asset = Self::load_zipped_asset(entry_reader, context).await?;
        Ok(context.add_loaded_labeled_asset(asset_key.to_string(), loaded_asset))
    }

    /// Returns a [`LoadedAsset`] from a [`ZipEntryReader`].
    ///
    /// # Errors
    /// Returns an
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) async fn load_zipped_asset<A: Asset>(
        entry_reader: &mut ZipEntryReader<'_, BufReader<&mut Reader<'_>>, WithEntry<'_>>,
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
