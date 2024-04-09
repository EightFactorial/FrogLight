use crate::{
    common::SectionBlockPosition,
    protocol::{FrogRead, FrogVarRead, FrogVarWrite, FrogWrite, ReadError, WriteError},
};

/// A chunk of data representing a block state update.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SectionDataPacket {
    /// The position of the block.
    pub position: SectionBlockPosition,
    /// The state of the block.
    pub block_state: u32,
}

impl FrogRead for SectionDataPacket {
    #[allow(clippy::cast_possible_truncation)]
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        let long = u64::fg_var_read(buf)?;

        Ok(Self {
            position: SectionBlockPosition::new(
                ((long >> 8) & 0x0F) as u8,
                (long & 0x0F) as u8,
                ((long >> 4) & 0x0F) as u8,
            ),
            block_state: (long >> 12) as u32,
        })
    }
}

impl FrogWrite for SectionDataPacket {
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        let long: u64 = u64::from(self.block_state) << 12
            | (u64::from(self.position.x) << 8)
            | (u64::from(self.position.y) << 4)
            | u64::from(self.position.z);

        long.fg_var_write(buf)
    }
}
