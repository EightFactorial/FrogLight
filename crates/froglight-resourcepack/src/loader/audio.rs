use std::sync::Arc;

use async_zip::base::read::{WithEntry, ZipEntryReader};
use bevy::{asset::LoadContext, prelude::*};
use froglight_core::data::ResourceKey;
use futures_lite::AsyncReadExt;

use crate::{loader::ResourcePackLoaderError, settings::ResourcePackLoaderSettings};

pub(super) async fn load_audio(
    resource_key: &ResourceKey,
    entry: &mut ZipEntryReader<'_, futures_lite::io::Cursor<&[u8]>, WithEntry<'_>>,
    load_context: &mut LoadContext<'_>,
    settings: &ResourcePackLoaderSettings,
) -> Result<Option<Handle<AudioSource>>, ResourcePackLoaderError> {
    let manager = settings.0.as_ref().unwrap();

    {
        let audio = manager.audio_assets.read();
        if audio.contains_key(resource_key) {
            #[cfg(feature = "logging")]
            trace!(
                "Skipping `{resource_key}` from `{}` as it already exists",
                load_context.path().display()
            );
            return Ok(None);
        }
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
    let mut audio_assets = manager.audio_assets.write();
    audio_assets.insert(resource_key.clone(), handle);

    Ok(Some(weak))
}
