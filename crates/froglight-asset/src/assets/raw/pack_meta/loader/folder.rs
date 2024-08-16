use bevy_asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext};

use crate::assets::{
    raw::{pack::loader::ResourcePackLoaderError, pack_meta::meta::PackMcMeta},
    ResourcePackMeta,
};

/// An [`AssetLoader`] for loading [`ResourcePackMeta`]s.
///
/// This is useful when you only need the metadata of a
/// [`ResourcePack`](crate::assets::ResourcePack).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourcePackMetaLoader;

impl AssetLoader for ResourcePackMetaLoader {
    type Asset = ResourcePackMeta;
    type Settings = ();
    type Error = ResourcePackLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        (): &'a (),
        context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        Self::read_pack_metadata(reader, context).await
    }

    fn extensions(&self) -> &[&str] { &["mcmeta"] }
}

impl ResourcePackMetaLoader {
    /// Load a [`ResourcePackMeta`] from a reader.
    ///
    /// # Note
    /// This requires the reader to be reading a `pack.mcmeta` file.
    pub(crate) async fn read_pack_metadata(
        reader: &mut Reader<'_>,
        _: &mut LoadContext<'_>,
    ) -> Result<ResourcePackMeta, ResourcePackLoaderError> {
        Ok(ResourcePackMeta::from({
            let mut buffer = String::new();
            reader.read_to_string(&mut buffer).await?;
            serde_json::from_str::<PackMcMeta>(&buffer)?
        }))
    }
}
