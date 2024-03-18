use std::fmt::Debug;

use froglight_macros::FrogReadWrite;
use simdnbt::owned::Nbt;

use crate::{
    common::ChunkBlockPosition,
    io::{FrogRead, FrogVarRead, FrogVarWrite, FrogWrite},
};

/// A chunk of data representing a section of the world.
#[derive(Clone, PartialEq, FrogReadWrite)]
pub struct ChunkDataPacket {
    /// The heightmap data for the chunk.
    pub heightmaps: Nbt,
    /// The chunk blocks and biomes.
    pub data: Vec<u8>,
    /// The block entities in the chunk.
    pub entities: Vec<BlockEntity>,
}

#[allow(clippy::missing_fields_in_debug)]
impl Debug for ChunkDataPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChunkDataPacket").field("entities", &self.entities).finish()
    }
}

/// A block entity in a chunk.
#[derive(Clone, PartialEq)]
pub struct BlockEntity {
    /// The position of the block entity.
    pub position: ChunkBlockPosition,
    /// The kind of block entity.
    pub kind: u32,
    /// The data of the block entity.
    pub data: Nbt,
}

#[allow(clippy::missing_fields_in_debug)]
impl Debug for BlockEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockEntity")
            .field("position", &self.position)
            .field("kind", &self.kind)
            .finish()
    }
}

impl FrogRead for BlockEntity {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
    where
        Self: Sized,
    {
        let byte = u8::fg_read(buf)?;
        let y = usize::try_from(i16::fg_read(buf)?).expect("Invalid block entity position");

        Ok(Self {
            position: ChunkBlockPosition { x: (byte >> 4) & 0x0F, y, z: byte & 0x0F },
            kind: u32::fg_var_read(buf)?,
            data: Nbt::fg_read(buf)?,
        })
    }
}

impl FrogWrite for BlockEntity {
    fn fg_write(
        &self,
        buf: &mut (impl std::io::Write + ?Sized),
    ) -> Result<(), crate::io::WriteError> {
        let mut byte = 0;
        byte |= (self.position.x & 0x0F) << 4;
        byte |= self.position.z & 0x0F;
        byte.fg_write(buf)?;

        i16::try_from(self.position.y)?.fg_write(buf)?;
        self.kind.fg_var_write(buf)?;
        self.data.fg_write(buf)?;
        Ok(())
    }
}
