use async_zip::base::read::{WithEntry, ZipEntryReader};
use bevy_asset::AsyncReadExt;
use bevy_log::error;
use futures_lite::io::Cursor;

use super::ResourcePackLoader;
use crate::asset_manager::soundmap::SoundMap;

/// Reads the sounds.json file
pub(super) async fn read_sounds(
    loader: &ResourcePackLoader,
    mut reader: ZipEntryReader<'_, Cursor<&[u8]>, WithEntry<'_>>,
) {
    let mut buffer = Vec::new();
    if let Err(err) = reader.read_to_end(&mut buffer).await {
        error!("Failed to read \"sounds.json\": \"{err:?}\"");
        return;
    }

    // Deserialize the sounds.json file
    match serde_json::from_slice::<SoundMap>(&buffer) {
        Ok(deserialized) => {
            // Insert any new sounds into the asset manager
            let mut sounds = loader.sounds.write();
            for (key, item) in deserialized.0 {
                if !sounds.contains_key(&key) {
                    sounds.insert(key, item);
                }
            }
        }
        Err(err) => {
            error!("Failed to parse \"sounds.json\": \"{err:?}\"");
        }
    }
}
