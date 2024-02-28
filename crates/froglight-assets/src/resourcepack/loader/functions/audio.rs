use std::sync::Arc;

use async_zip::base::read::{WithEntry, ZipEntryReader};
use bevy_asset::{Handle, LoadContext};
use bevy_audio::AudioSource;
use bevy_log::trace;
use froglight_core::data::ResourceKey;
use futures_lite::{io::Take, AsyncRead, AsyncReadExt};

use crate::{AssetManager, ResourcePackLoader, ResourcePackLoaderError};

#[cfg(feature = "asset_manager")]
pub(crate) async fn load_audio(
    loader: &ResourcePackLoader,
    resource_key: &ResourceKey,
    entry: &mut ZipEntryReader<'_, Take<&mut (dyn AsyncRead + Sync + Send + Unpin)>, WithEntry<'_>>,
    load_context: &mut LoadContext<'_>,
) -> Result<Option<Handle<AudioSource>>, ResourcePackLoaderError> {
    // Check if the audio already exists in the asset manager.

    if loader.sounds.read().contains_key(resource_key) {
        trace!(
            "Skipping `{resource_key}` from `{}` as it already exists",
            load_context.path().display()
        );
        return Ok(None);
    }

    // Read the entire file into memory.
    let mut bytes = Vec::new();
    entry.read_to_end(&mut bytes).await?;

    // Create a new audio source.
    let audio = AudioSource { bytes: Arc::<[u8]>::from(bytes) };

    // Load the audio into the asset manager.
    // Store the strong handle in the ResourcePackManager, and return a weak handle.
    let handle = load_context.labeled_asset_scope(resource_key.to_string(), |_| audio);
    let weak = handle.clone_weak();

    // Insert the texture into the texture assets if it doesn't exist.
    let mut audio_assets = loader.sounds.write();
    audio_assets.insert(resource_key.clone(), handle);

    Ok(Some(weak))
}

#[cfg(not(feature = "asset_manager"))]
pub(crate) async fn load_audio(
    loader: &ResourcePackLoader,
    resource_key: &ResourceKey,
    entry: &mut ZipEntryReader<'_, futures_lite::io::Cursor<&[u8]>, WithEntry<'_>>,
    load_context: &mut LoadContext<'_>,
) -> Result<Option<Handle<AudioSource>>, ResourcePackLoaderError> {
    // Read the entire file into memory.
    let mut bytes = Vec::new();
    entry.read_to_end(&mut bytes).await?;

    // Create and load a new audio source.
    let audio = AudioSource { bytes: Arc::<[u8]>::from(bytes) };
    let handle = load_context.labeled_asset_scope(resource_key.to_string(), |_| audio);

    Ok(Some(handle))
}
