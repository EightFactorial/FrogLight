use std::io::Cursor;

use super::{palette::Palette, tasks::ChunkDecodeError};

pub mod traits;
use traits::ContainerType;

mod biomes;
pub use biomes::BiomeContainer;

mod blocks;
pub use blocks::BlockContainer;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Container<T: ContainerType> {
    pub bits: u8,
    pub palette: Palette,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: ContainerType> Container<T> {
    pub(super) async fn decode_container(
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<Self, ChunkDecodeError> {
        T::decode_container(cursor).await
    }
}
