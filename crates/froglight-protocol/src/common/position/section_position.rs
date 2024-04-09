use super::ChunkPosition;
use crate::protocol::{FrogRead, FrogWrite, ReadError, WriteError};

/// The position of a chunk section.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SectionPosition {
    /// The chunk position.
    pub chunk: ChunkPosition,
    /// The section index.
    pub section: i64,
}

impl SectionPosition {
    /// Creates a new section position.
    #[must_use]
    pub const fn new(x: i64, y: i64, z: i64) -> Self {
        Self { chunk: ChunkPosition::new(x, z), section: y }
    }

    /// Creates a new section position.
    #[must_use]
    pub const fn new_section(chunk: ChunkPosition, section: i64) -> Self { Self { chunk, section } }

    /// Returns the x-coordinate in chunks,
    #[must_use]
    #[inline]
    pub const fn x(&self) -> i64 { self.chunk.x() }
    /// Returns the y index of the section.
    #[must_use]
    #[inline]
    pub const fn y(&self) -> i64 { self.section }
    /// Returns the z-coordinate in chunks.
    #[must_use]
    #[inline]
    pub const fn z(&self) -> i64 { self.chunk.z() }
}

impl FrogRead for SectionPosition {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        let long = i64::fg_read(buf)?;
        Ok(Self::new(long >> 42, long << 44 >> 44, long << 22 >> 42))
    }
}

impl FrogWrite for SectionPosition {
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        let long = ((self.chunk.x & 0x003F_FFFF) << 42)
            | (self.section & 0x000F_FFFF)
            | ((self.chunk.y & 0x003F_FFFF) << 20);
        long.fg_write(buf)
    }
}
