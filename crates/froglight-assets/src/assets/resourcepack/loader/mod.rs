use async_std::sync::Mutex;
use async_zip::{base::read::mem::ZipFileReader, StoredZipEntry};
use bevy_asset::{io::Reader, AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy_derive::Deref;
use bevy_ecs::world::{FromWorld, World};
use bevy_log::{error, warn};
use bevy_reflect::TypePath;
use froglight_protocol::common::ResourceKey;
use futures_concurrency::{
    concurrent_stream::ConcurrentStream, future::FutureGroup, stream::StreamExt,
};
use thiserror::Error;

use super::ResourcePack;
use crate::AssetManager;

mod audio;
mod texture;

/// An [`AssetLoader`] for [`ResourcePacks`](ResourcePack).
#[derive(Debug, Clone, Deref, TypePath)]
pub struct ResourcePackLoader(AssetManager);

impl FromWorld for ResourcePackLoader {
    fn from_world(world: &mut World) -> Self {
        let manager = world.get_resource_or_insert_with(|| {
            warn!("ResourcePackLoader requires an AssetManager, creating...");
            AssetManager::default()
        });

        Self(manager.clone())
    }
}

impl AssetLoader for ResourcePackLoader {
    type Asset = ResourcePack;
    type Error = ResourcePackError;
    type Settings = ();

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        (): &(),
        context: &'a mut LoadContext<'_>,
    ) -> BoxedFuture<'a, Result<ResourcePack, ResourcePackError>> {
        Box::pin(Self::load(self, reader, context))
    }
}

impl ResourcePackLoader {
    /// Loads a [`ResourcePack`] from a [`Reader`].
    async fn load(
        &self,
        reader: &mut Reader<'_>,
        context: &mut LoadContext<'_>,
    ) -> Result<ResourcePack, ResourcePackError> {
        let pack = Mutex::new(ResourcePack::default());
        let context = Mutex::new(context);

        // Debug timing.
        let start = std::time::Instant::now();

        // Create a ZIP file reader
        let reader = {
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer).await?;
            ZipFileReader::new(buffer).await?
        };

        // Create a future group to read all entries concurrently.
        let mut group = FutureGroup::new();
        for (index, entry) in reader.file().entries().iter().enumerate() {
            // Only read the entry if it's supported.
            if Self::should_read(entry) {
                group.insert(self.read_entry(index, &reader, &pack, &context));
            }
        }

        bevy_log::debug!("Reading `{}` files...", group.len());

        // Read all entries concurrently.
        group.co().collect::<Vec<()>>().await;

        // Debug timing.
        bevy_log::debug!(
            "Loaded `{}` in {} seconds",
            context.lock().await.asset_path(),
            start.elapsed().as_secs_f64()
        );

        Ok(pack.into_inner())
    }

    /// Determines whether an entry should be read.
    fn should_read(entry: &StoredZipEntry) -> bool {
        if let Ok(filename) = entry.filename().as_str() {
            // Check for files in the "assets" directory
            if filename.starts_with("assets/") {
                // Check for supported file extensions
                matches!(filename.split('.').last(), Some("png" | "ogg"))
            } else {
                // Check for "pack.mcmeta" and "pack.png"
                matches!(filename, "pack.mcmeta" | "pack.png")
            }
        } else {
            #[cfg(debug_assertions)]
            warn!("Error reading `ZipEntry` filename");
            false
        }
    }

    /// Reads an entry from a ZIP file.
    async fn read_entry(
        &self,
        index: usize,
        reader: &ZipFileReader,
        pack: &Mutex<ResourcePack>,
        context: &Mutex<&mut LoadContext<'_>>,
    ) {
        // Get the entry from the reader.
        let Ok(file) = reader.reader_with_entry(index).await else {
            error!("Error reading `ZipEntry` #{index}");
            return;
        };

        // Get the filename of the entry.
        //
        // SAFETY: The filename was checked in `should_read`.
        let filename = file.entry().filename().as_str().unwrap();

        // Check for "pack.mcmeta" and "pack.png"
        match filename {
            "pack.mcmeta" | "pack.png" => {
                // TODO: Read the pack.mcmeta and pack.png files
                return;
            }
            _ => {}
        }

        // Convert the filename to a `ResourceKey`.
        let Some(filekey) = Self::filename_to_resourcekey(filename) else {
            warn!("Error loading: \"{filename}\"");
            return;
        };

        // Read the file type based on the extension.
        match filename.split('.').last() {
            Some("png") => {
                // Read a texture file if it hasn't been read yet.
                if !self.textures.read().contains_key(&filekey) {
                    texture::read(self, file, filekey, pack, context).await;
                }
            }
            Some("ogg") => {
                // Read an audio file if it hasn't been read yet.
                if !self.sounds.read().contains_key(&filekey) {
                    audio::read(self, file, filekey, pack, context).await;
                }
            }
            _ => {
                warn!("Unable to load file: \"{filename}\"");
            }
        }
    }

    /// Converts a file path to a [`ResourceKey`].
    fn filename_to_resourcekey(filename: &str) -> Option<ResourceKey> {
        let ext = filename.split('.').last()?;
        let stripped = filename.strip_prefix("assets/").unwrap().strip_suffix(ext).unwrap();
        ResourceKey::try_new(stripped[..stripped.len() - 1].replacen('/', ":", 1)).ok()
    }
}

/// Errors that can occur while loading a [`ResourcePack`].
#[derive(Debug, Error)]
pub enum ResourcePackError {
    /// An I/O error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// A ZIP error occurred.
    #[error(transparent)]
    Zip(#[from] async_zip::error::ZipError),
}
