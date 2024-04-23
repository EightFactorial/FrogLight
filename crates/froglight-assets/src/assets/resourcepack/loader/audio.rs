use std::sync::Arc;

use async_std::sync::Mutex;
use async_zip::base::read::{WithEntry, ZipEntryReader};
use bevy_asset::{AsyncReadExt, LoadContext};
use bevy_audio::AudioSource;
use bevy_log::error;
use froglight_protocol::common::ResourceKey;
use futures_lite::io::Cursor;

use super::ResourcePackLoader;
use crate::assets::resourcepack::ResourcePack;

/// Reads an audio file from a ZIP entry.
pub(super) async fn read(
    loader: &ResourcePackLoader,
    mut reader: ZipEntryReader<'_, Cursor<&[u8]>, WithEntry<'_>>,
    filekey: ResourceKey,
    pack: &Mutex<ResourcePack>,
    context: &Mutex<&mut LoadContext<'_>>,
) {
    // Read the file into a non-async buffer.
    let mut buffer = Vec::new();
    if let Err(err) = reader.read_to_end(&mut buffer).await {
        error!("Failed to read: {filekey} -> \"{err:?}\"");
        return;
    }

    // Create an AudioSource from the buffer.
    let audio = AudioSource { bytes: Arc::<[u8]>::from(buffer) };

    // Create a handle for the audio.
    let handle = context.lock().await.labeled_asset_scope(filekey.to_string(), |_| audio);

    // Insert the handle into the pack and manager.
    pack.lock().await.sounds.insert(filekey.clone(), handle.clone_weak());
    loader.audio.write().insert(filekey, handle);
}
