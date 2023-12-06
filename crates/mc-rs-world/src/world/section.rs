use std::io::Cursor;

use mc_rs_protocol::buffer::Decode;

use super::{
    container::{BiomeContainer, BlockContainer, Container},
    tasks::ChunkDecodeError,
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Section {
    pub blocks: u16,
    pub states: Container<BlockContainer>,
    pub biomes: Container<BiomeContainer>,
}

impl Section {
    pub const SECTION_WIDTH: usize = 16;
    pub const SECTION_HEIGHT: usize = 16;
    pub const SECTION_DEPTH: usize = 16;
}

impl Section {
    pub(super) async fn decode_section(
        cursor: &mut Cursor<&[u8]>,
    ) -> Result<Self, ChunkDecodeError> {
        let blocks = u16::decode(cursor).map_err(|_| ChunkDecodeError::InvalidSection)?;
        let states = Container::decode_container(cursor).await?;
        let biomes = Container::decode_container(cursor).await?;

        Ok(Self {
            blocks,
            states,
            biomes,
        })
    }
}
