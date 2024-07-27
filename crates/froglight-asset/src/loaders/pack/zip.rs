use async_zip::{
    base::read::{mem::ZipFileReader, WithEntry, ZipEntryReader},
    ZipEntry,
};
use bevy_asset::{io::Reader, Asset, AssetLoader, AsyncReadExt, Handle, LoadContext};
use bevy_log::error;
use bevy_tasks::IoTaskPool;
use froglight_common::ResourceKey;
use futures_lite::io::Cursor;
use parking_lot::{Mutex, RwLock};

use super::EntryType;
use crate::{assets::unprocessed::ResourcePack, ResourcePackMeta};

/// An [`AssetLoader`] that loads a [`ResourcePack`] from a zip file.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourcePackZipLoader;

/// An error that can occur when loading a [`ResourcePack`] from a zip file.
#[derive(Debug, thiserror::Error)]
pub enum ResourcePackZipError {
    /// An error that occurred while reading the zip file.
    #[error(transparent)]
    Zip(#[from] async_zip::error::ZipError),
    /// An IO error that occurred while reading the zip file.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// An error that occurred while deserializing the zip file.
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    /// An error that occurred while loading an asset.
    #[error(transparent)]
    Load(#[from] bevy_asset::LoadDirectError),
}

impl AssetLoader for ResourcePackZipLoader {
    type Asset = ResourcePack;
    type Settings = ();
    type Error = ResourcePackZipError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        (): &'a Self::Settings,
        context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        ResourcePackZipLoader::load(reader, context).await
    }

    fn extensions(&self) -> &[&str] { &["zip"] }
}

impl ResourcePackZipLoader {
    async fn load(
        reader: &mut Reader<'_>,
        context: &mut LoadContext<'_>,
    ) -> Result<ResourcePack, ResourcePackZipError> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).await?;
        let zip = ZipFileReader::new(buffer).await?;

        let mut resourcepack = ResourcePack::default();
        let mut meta = ResourcePackMeta::default();

        {
            let context = async_lock::Mutex::new(&mut *context);
            let resourcepack = RwLock::new(&mut resourcepack);
            let meta = Mutex::new(&mut meta);

            let context_ref = &context;
            let resourcepack_ref = &resourcepack;
            let meta_ref = &meta;

            IoTaskPool::get().scope(|pool| {
                pool.spawn(async {
                    // Iterate over each entry in the zip file.
                    for index in 0..zip.file().entries().len() {
                        let Ok(mut reader) = zip.reader_with_entry(index).await else {
                            continue;
                        };

                        if let Some((key, kind)) = Self::should_read(reader.entry()) {
                            pool.spawn(async move {
                                // Print an error if the entry could not be read, but continue.
                                if let Err(err) = Self::add_to_resourcepack(
                                    key,
                                    kind,
                                    &mut reader,
                                    context_ref,
                                    meta_ref,
                                    resourcepack_ref,
                                )
                                .await
                                {
                                    error!(
                                        "Error reading entry \"{}\": {err}",
                                        reader.entry().filename().as_str().unwrap_or_default()
                                    );
                                }
                            });
                        }
                    }
                });
            });
        }

        // Add the `ResourcePackMeta` to the `ResourcePack`.
        let meta = context.add_labeled_asset(String::from("pack.mcmeta"), meta);
        resourcepack.meta = meta;

        Ok(resourcepack)
    }

    fn should_read(entry: &ZipEntry) -> Option<(ResourceKey, EntryType)> {
        if entry.dir().ok()? {
            return None;
        }

        let filename = entry.filename().as_str().ok()?;
        match EntryType::from_path(filename)? {
            pack if matches!(pack, EntryType::PackMeta | EntryType::PackPng) => {
                Some((ResourceKey::const_new("minecraft:pack"), pack))
            }
            EntryType::SoundMap => {
                let trimmed = filename.trim_start_matches("assets/");
                let namespace = trimmed.split_once('/')?.0;
                let key = ResourceKey::new(format!("{namespace}:sounds"));
                Some((key, EntryType::SoundMap))
            }
            other => {
                let mut split = filename.split('/');
                split.next()?;

                let namespace = split.next()?;
                split.next()?;

                let mut path = split.remainder()?;
                path = path.split_once('.')?.0;

                let key = ResourceKey::try_new(format!("{namespace}:{path}")).ok()?;
                Some((key, other))
            }
        }
    }

    #[allow(clippy::unused_async)]
    async fn add_to_resourcepack(
        key: ResourceKey,
        kind: EntryType,
        reader: &mut ZipEntryReader<'_, Cursor<&[u8]>, WithEntry<'_>>,
        context: &async_lock::Mutex<&mut LoadContext<'_>>,
        meta: &Mutex<&mut ResourcePackMeta>,
        resourcepack: &RwLock<&mut ResourcePack>,
    ) -> Result<(), ResourcePackZipError> {
        let filename = reader.entry().filename().clone().into_string().unwrap_or_default();
        match kind {
            EntryType::Texture => {
                let asset = Self::load_asset(&filename, reader, context).await?;
                resourcepack.write().textures.insert(key, asset);
            }
            EntryType::Sound => {
                let asset = Self::load_asset(&filename, reader, context).await?;
                resourcepack.write().sounds.insert(key, asset);
            }
            EntryType::BlockModel => {
                let asset = Self::load_asset(&filename, reader, context).await?;
                resourcepack.write().block_models.insert(key, asset);
            }
            EntryType::Language => {
                let asset = Self::load_asset(&filename, reader, context).await?;
                resourcepack.write().languages.insert(key, asset);
            }
            EntryType::TextureAtlas => {
                let asset = Self::load_asset(&filename, reader, context).await?;
                resourcepack.write().atlas_definitions.insert(key, asset);
            }
            EntryType::ResourcePack => {
                let asset = Self::load_asset(&filename, reader, context).await?;
                resourcepack.write().children.insert(key, asset);
            }
            EntryType::SoundMap => {
                let asset = Self::load_asset(&filename, reader, context).await?;
                resourcepack.write().sound_definitions.insert(key.namespace().to_string(), asset);
            }
            EntryType::PackMeta => {
                let mut meta_content = String::new();
                reader.read_to_string(&mut meta_content).await?;
                meta.lock().mcmeta = serde_json::from_str(&meta_content)?;
            }
            EntryType::PackPng => {
                meta.lock().icon = Some(Self::load_asset(&filename, reader, context).await?);
            }
        }
        Ok(())
    }

    /// Loads an asset from a zip entry.
    // TODO: Fix needing to use a buffer here.
    async fn load_asset<A: Asset>(
        path: &str,
        reader: &mut ZipEntryReader<'_, Cursor<&[u8]>, WithEntry<'_>>,
        context: &async_lock::Mutex<&mut LoadContext<'_>>,
    ) -> Result<Handle<A>, ResourcePackZipError> {
        let mut buffer = Vec::with_capacity(
            usize::try_from(reader.entry().uncompressed_size()).unwrap_or_default(),
        );
        reader.read_to_end(&mut buffer).await?;
        let mut cursor = Cursor::new(buffer);

        let mut context = context.lock().await;
        let nested_loader = context.loader().with_asset_type::<A>();
        let direct_loader = nested_loader.direct().with_reader(&mut cursor);
        let loaded_asset = direct_loader.load(path.to_string()).await?;
        Ok(context.add_loaded_labeled_asset(path.to_string(), loaded_asset))
    }
}
