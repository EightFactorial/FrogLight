use async_zip::base::read::seek::ZipFileReader;
use bevy_asset::{io::Reader, AssetLoader, LoadContext};
use futures_lite::io::BufReader;

use super::ResourcePackLoaderError;
use crate::assets::{raw::pack_meta::ResourcePackMetaZipLoader, ResourcePack};

/// An [`AssetLoader`] for loading [`ResourcePack`]s from ZIP files.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourcePackZipLoader;

impl AssetLoader for ResourcePackZipLoader {
    type Asset = ResourcePack;
    type Settings = ();
    type Error = ResourcePackLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        (): &'a (),
        context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        Self::load_from_zip(reader, context).await
    }

    fn extensions(&self) -> &[&str] { &["zip"] }
}

#[allow(clippy::unused_async)]
impl ResourcePackZipLoader {
    async fn load_from_zip(
        reader: &mut Reader<'_>,
        context: &mut LoadContext<'_>,
    ) -> Result<ResourcePack, ResourcePackLoaderError> {
        let mut zip = ZipFileReader::new(BufReader::new(reader)).await?;

        // Load the `pack.mcmeta` metadata file.
        let meta = ResourcePackMetaZipLoader::read_pack_metadata_zip(&mut zip, context).await?;
        let meta = context.add_labeled_asset(String::from("pack.mcmeta"), meta);

        let _resourcepack = ResourcePack::new(meta);

        todo!()
    }
}
