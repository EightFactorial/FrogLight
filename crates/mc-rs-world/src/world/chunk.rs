use std::io::Cursor;

use bevy::{prelude::*, utils::HashMap};
use derive_more::{From, Into};
use mc_rs_protocol::types::{packets::chunk_data::ChunkDataPacket, position::ChunkPos};

use super::{heightmap::HeightMapType, section::Section, tasks::DecodeResult, HeightMap};

#[derive(Debug, Default, Clone, PartialEq, Eq, From, Into, Deref, DerefMut, Component)]
pub struct ChunkPosition(ChunkPos);

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub struct Chunk {
    pub sections: Vec<Section>,
    pub heightmaps: HashMap<HeightMapType, HeightMap>,
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            sections: vec![Section::default(); Self::SECTION_COUNT],
            heightmaps: HashMap::new(),
        }
    }
}

impl Chunk {
    pub const WORLD_HEIGHT: usize = 384;
    pub const SECTION_COUNT: usize = Self::WORLD_HEIGHT / Section::SECTION_HEIGHT;

    pub(super) async fn decode_chunk(data: ChunkDataPacket) -> DecodeResult {
        let mut sections = Vec::with_capacity(Self::SECTION_COUNT);

        let mut section_data = Cursor::new(data.data.as_slice());
        for _ in 0..Self::SECTION_COUNT {
            sections.push(Section::decode_section(&mut section_data).await?);
        }

        // TODO: Decode heightmap NBT data
        let heightmaps = HashMap::new();

        Ok(Self {
            sections,
            heightmaps,
        })
    }
}
