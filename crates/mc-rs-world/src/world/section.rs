use std::io::Cursor;

use mc_rs_protocol::buffer::Decode;

use super::{
    container::{BiomeContainer, BlockContainer, Container},
    tasks::ChunkDecodeError,
};

/// A [`Section`] is a 16x16x16 section of a [`Chunk`](super::Chunk).
///
/// A [`Section`] contains a counter for the number of non-air blocks in the section,
/// and [`Containers`](super::container::Container) for blocks and biomes.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Section {
    pub block_count: u16,
    pub blocks: Container<BlockContainer>,
    pub biomes: Container<BiomeContainer>,
}

impl Section {
    pub const SECTION_WIDTH: usize = 16;
    pub const SECTION_HEIGHT: usize = 16;
    pub const SECTION_DEPTH: usize = 16;

    pub const SECTION_VOLUME: usize =
        Self::SECTION_WIDTH * Self::SECTION_HEIGHT * Self::SECTION_DEPTH;
}

impl Section {
    pub(super) async fn decode_section(
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<Self, ChunkDecodeError> {
        let block_count = u16::decode(cursor).map_err(|_| ChunkDecodeError::InvalidSection)?;
        let blocks = Container::decode_container(cursor).await?;
        let biomes = Container::decode_container(cursor).await?;

        Ok(Self {
            block_count,
            blocks,
            biomes,
        })
    }
}
