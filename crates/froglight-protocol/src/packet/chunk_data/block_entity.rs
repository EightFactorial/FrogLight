use simdnbt::owned::Nbt;

use crate::{
    common::ChunkBlockPosition,
    protocol::{FrogRead, FrogVarRead, FrogVarWrite, FrogWrite, ReadError, WriteError},
};

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
impl std::fmt::Debug for BlockEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockEntity")
            .field("position", &self.position)
            .field("kind", &self.kind)
            .field("data", &self.data.len())
            .finish()
    }
}

impl FrogRead for BlockEntity {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        let byte = u8::fg_read(buf)?;
        let y = u32::from(u16::fg_read(buf)?);

        Ok(Self {
            position: ChunkBlockPosition { x: (byte >> 4) & 0x0F, y, z: byte & 0x0F },
            kind: u32::fg_var_read(buf)?,
            data: Nbt::fg_read(buf)?,
        })
    }
}

impl FrogWrite for BlockEntity {
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        let mut byte = 0;
        byte |= (self.position.x & 0x0F) << 4;
        byte |= self.position.z & 0x0F;
        byte.fg_write(buf)?;

        u16::try_from(self.position.y)?.fg_write(buf)?;
        self.kind.fg_var_write(buf)?;
        self.data.fg_write(buf)?;
        Ok(())
    }
}
