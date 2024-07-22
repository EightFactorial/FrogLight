use bevy_asset::{io::Reader, AssetLoader, LoadContext};

/// An [`AssetLoader`] that loads a [`ResourcePack`] from a zip file.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourcePackZipLoader;

impl AssetLoader for ResourcePackZipLoader {
    type Asset = ();
    type Settings = ();
    type Error = std::io::Error;

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
    #[allow(clippy::unused_async)]
    async fn load(
        _reader: &mut Reader<'_>,
        _context: &mut LoadContext<'_>,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }
}
