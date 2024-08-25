use std::marker::PhantomData;

use bevy_asset::{io::Reader, Asset, AssetLoader, AsyncReadExt, LoadContext};
use serde::de::DeserializeOwned;

/// An [`AssetLoader`] that uses [`serde_json`] to load assets.
///
/// This loader is used for loading assets that are stored in JSON format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SerdeJsonLoader<A: Asset + DeserializeOwned> {
    _a: PhantomData<A>,
}

impl<A: Asset + DeserializeOwned> Default for SerdeJsonLoader<A> {
    fn default() -> Self { Self { _a: PhantomData } }
}

/// An error that can occur when loading an asset with [`SerdeJsonLoader`].
#[derive(Debug, thiserror::Error)]
pub enum SerdeJsonLoaderError {
    /// An I/O error occurred while reading the asset.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// A deserialization error occurred while parsing the asset.
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

impl<A: Asset + DeserializeOwned> AssetLoader for SerdeJsonLoader<A> {
    type Asset = A;
    type Settings = ();
    type Error = SerdeJsonLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        (): &'a Self::Settings,
        _: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut reader_content = String::new();
        reader.read_to_string(&mut reader_content).await?;
        serde_json::from_str(&reader_content).map_err(SerdeJsonLoaderError::Serde)
    }

    fn extensions(&self) -> &[&str] { &["json"] }
}
