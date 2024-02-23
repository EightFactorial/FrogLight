use async_zip::base::read::{WithEntry, ZipEntryReader};
use bevy::{
    asset::LoadContext,
    prelude::*,
    render::{render_asset::RenderAssetUsages, texture::Image},
};
use froglight_core::data::ResourceKey;
use futures_lite::{io::Take, AsyncRead, AsyncReadExt};
use image::io::Reader as ImageReader;

use crate::{AssetManager, ResourcePackLoader, ResourcePackLoaderError};

#[cfg(feature = "asset_manager")]
pub(crate) async fn load_texture(
    loader: &ResourcePackLoader,
    resource_key: &ResourceKey,
    entry: &mut ZipEntryReader<'_, Take<&mut (dyn AsyncRead + Sync + Send + Unpin)>, WithEntry<'_>>,
    load_context: &mut LoadContext<'_>,
) -> Result<Option<Handle<Image>>, ResourcePackLoaderError> {
    // Check if the texture already exists in the asset manager.

    use bevy::render::{render_resource::TextureDescriptor, texture::ImageSampler};
    if loader.texture_assets.read().contains_key(resource_key) {
        trace!(
            "Skipping `{resource_key}` from `{}` as it already exists",
            load_context.path().display()
        );
        return Ok(None);
    }

    // Read the entire file into memory.
    let mut data = Vec::new();
    entry.read_to_end(&mut data).await?;

    // Decode the image.
    if let Ok(dyn_img) =
        ImageReader::new(std::io::Cursor::new(data)).with_guessed_format()?.decode()
    {
        let mut image = Image::from_dynamic(dyn_img, true, RenderAssetUsages::all());
        image.sampler = ImageSampler::nearest();

        // Load the texture into the asset manager.
        // Store the strong handle in the ResourcePackManager, and return a weak handle.
        let handle = load_context.labeled_asset_scope(resource_key.to_string(), |_| image);
        let weak = handle.clone_weak();

        // Insert the texture into the texture assets if it doesn't exist.
        let mut textures = loader.texture_assets.write();
        textures.insert(resource_key.clone(), handle);

        Ok(Some(weak))
    } else {
        warn!("Unable to decode image `{resource_key}` from `{}`", load_context.path().display());

        Ok(None)
    }
}

#[cfg(not(feature = "asset_manager"))]
pub(crate) async fn load_texture(
    loader: &ResourcePackLoader,
    resource_key: &ResourceKey,
    entry: &mut ZipEntryReader<'_, futures_lite::io::Cursor<&[u8]>, WithEntry<'_>>,
    load_context: &mut LoadContext<'_>,
) -> Result<Option<Handle<Image>>, ResourcePackLoaderError> {
    // Read the entire file into memory.
    let mut data = Vec::new();
    entry.read_to_end(&mut data).await?;

    // Decode the image.
    if let Ok(dyn_img) =
        ImageReader::new(std::io::Cursor::new(data)).with_guessed_format()?.decode()
    {
        let image = Image::from_dynamic(dyn_img, false, RenderAssetUsages::all());
        let handle = load_context.labeled_asset_scope(resource_key.to_string(), |_| image);

        Ok(Some(handle))
    } else {
        warn!("Unable to decode image `{resource_key}` from `{}`", load_context.path().display());

        Ok(None)
    }
}
