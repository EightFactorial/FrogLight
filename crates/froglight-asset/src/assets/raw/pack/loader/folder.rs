use bevy_asset::{io::Reader, AssetLoader, LoadContext};

use super::ResourcePackLoaderError;
use crate::assets::{raw::pack_meta::ResourcePackMetaLoader, ResourcePack};

/// An [`AssetLoader`] for loading [`ResourcePack`]s from folders.
///
/// Must be pointed to a `pack.mcmeta` file.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourcePackLoader;

impl AssetLoader for ResourcePackLoader {
    type Asset = ResourcePack;
    type Settings = ();
    type Error = ResourcePackLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        (): &'a (),
        context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        Self::load_from_folder(reader, context).await
    }

    fn extensions(&self) -> &[&str] { &["mcmeta"] }
}

impl ResourcePackLoader {
    #[inline]
    async fn load_from_folder(
        reader: &mut Reader<'_>,
        context: &mut LoadContext<'_>,
    ) -> Result<ResourcePack, ResourcePackLoaderError> {
        // Load the `pack.mcmeta` metadata file.
        let meta = ResourcePackMetaLoader::read_pack_metadata(reader, context).await?;
        let meta = context.add_labeled_asset(String::from("pack.mcmeta"), meta);

        let _resourcepack = ResourcePack::new(meta);

        todo!()
    }
}

impl ResourcePackLoader {}
