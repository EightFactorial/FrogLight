use bevy_asset::{io::Reader, AssetLoader, LoadContext};

/// An [`AssetLoader`] that loads a [`ResourcePack`] from a folder.
///
/// Must be used to load the `pack.mcmeta` file.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourcePackFolderLoader;

impl AssetLoader for ResourcePackFolderLoader {
    type Asset = ();
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
    ) -> Result<(), std::io::Error> {
        Ok(())
    }
}
