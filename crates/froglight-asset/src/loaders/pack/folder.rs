use bevy_asset::{io::Reader, AssetLoader, LoadContext};

use crate::assets::unprocessed::ResourcePack;

/// An [`AssetLoader`] that loads a [`ResourcePack`] from a folder.
///
/// Must be used to load the `pack.mcmeta` file.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourcePackFolderLoader;

/// An error that can occur when loading a [`ResourcePack`] from a folder.
#[derive(Debug, thiserror::Error)]
pub enum ResourcePackFolderError {
    /// An IO error that occurred while reading the folder.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// An error that occurred while deserializing the folder.
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

impl AssetLoader for ResourcePackFolderLoader {
    type Asset = ResourcePack;
    type Settings = ();
    type Error = std::io::Error;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        (): &'a Self::Settings,
        context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        ResourcePackFolderLoader::load(reader, context).await
    }

    fn extensions(&self) -> &[&str] { &["mcmeta"] }
}

impl ResourcePackFolderLoader {
    #[allow(clippy::unused_async)]
    async fn load(
        _reader: &mut Reader<'_>,
        _context: &mut LoadContext<'_>,
    ) -> Result<ResourcePack, std::io::Error> {
        todo!()
    }
}
