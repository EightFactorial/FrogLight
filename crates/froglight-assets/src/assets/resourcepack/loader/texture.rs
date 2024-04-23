use async_std::sync::Mutex;
use async_zip::base::read::{WithEntry, ZipEntryReader};
use bevy_asset::{AsyncReadExt, LoadContext};
use bevy_log::error;
use bevy_render::{
    render_asset::RenderAssetUsages,
    texture::{Image, ImageSampler},
};
use froglight_protocol::common::ResourceKey;
use futures_lite::io::Cursor;
use image::io::Reader as ImageReader;

use super::ResourcePackLoader;
use crate::assets::resourcepack::ResourcePack;

/// Reads a texture file from a ZIP entry.
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

    // Read the image.
    let reader =
        match ImageReader::new(std::io::Cursor::new(buffer.as_slice())).with_guessed_format() {
            Ok(reader) => reader,
            Err(err) => {
                error!("Failed to read image: {filekey} -> \"{err:?}\"");
                return;
            }
        };

    // Decode the image.
    match reader.decode() {
        Ok(dynamic_image) => {
            // Load the image into a Bevy texture.
            let mut image = Image::from_dynamic(dynamic_image, true, RenderAssetUsages::all());
            image.sampler = ImageSampler::nearest();

            // Create a handle for the image.
            let handle = context.lock().await.labeled_asset_scope(filekey.to_string(), |_| image);

            // Insert the handle into the pack and manager.
            pack.lock().await.textures.insert(filekey.clone(), handle.clone_weak());
            loader.textures.write().insert(filekey, handle);
        }
        Err(err) => {
            error!("Failed to decode image: {filekey} -> \"{err:?}\"");
        }
    }
}
