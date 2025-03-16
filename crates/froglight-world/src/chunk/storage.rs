use super::Section;

/// A vertical slice of the world.
pub enum ChunkStorage {
    /// A large chunk.
    ///
    /// Typically used for overworld chunks.
    Large(ArrayChunkStorage<24, -64>),
    /// A normal chunk.
    ///
    /// Typically used for nether and end chunks.
    Normal(ArrayChunkStorage<16, 0>),
    /// A chunk of some other size.
    ///
    /// May be used for custom worlds or other special cases.
    Other(VecChunkStorage),
}

impl ChunkStorage {
    /// The maximum height of this chunk in blocks.
    ///
    /// This is the build limit for this chunk.
    #[must_use]
    pub fn height_max(&self) -> usize {
        match self {
            ChunkStorage::Large(..) => 384 - 64,
            ChunkStorage::Normal(..) => 256,
            ChunkStorage::Other(storage) => {
                storage.0.len().saturating_mul(Section::HEIGHT).saturating_add_signed(storage.1)
            }
        }
    }

    /// The minimum height of this chunk in blocks.
    ///
    /// This is the bedrock floor for this chunk.
    #[must_use]
    pub fn height_min(&self) -> isize {
        match self {
            ChunkStorage::Large(..) => -64,
            ChunkStorage::Normal(..) => 0,
            ChunkStorage::Other(storage) => storage.1,
        }
    }

    /// The total height of this chunk in blocks.
    ///
    /// This is the total height of the chunk
    /// from the bedrock floor to the build limit.
    #[must_use]
    pub fn total_height(&self) -> usize {
        match self {
            ChunkStorage::Large(..) => 384,
            ChunkStorage::Normal(..) => 256,
            ChunkStorage::Other(storage) => storage.0.len().saturating_mul(Section::HEIGHT),
        }
    }

    /// The total volume of this chunk in blocks.
    #[must_use]
    pub fn total_volume(&self) -> usize { self.total_height() * Section::WIDTH * Section::DEPTH }

    /// The the number of [`Section`]s in this chunk.
    #[must_use]
    pub fn sections(&self) -> usize {
        match self {
            ChunkStorage::Large(..) => 24,
            ChunkStorage::Normal(..) => 16,
            ChunkStorage::Other(storage) => storage.0.len(),
        }
    }

    /// Get a reference to the [`Section`]s in this chunk.
    #[must_use]
    pub fn sections_ref(&self) -> &[Section] {
        match self {
            ChunkStorage::Large(storage) => storage.0.as_ref(),
            ChunkStorage::Normal(storage) => storage.0.as_ref(),
            ChunkStorage::Other(storage) => storage.0.as_ref(),
        }
    }

    /// Get a mutable reference to the [`Section`]s in this chunk.
    #[must_use]
    pub fn sections_mut(&mut self) -> &mut [Section] {
        match self {
            ChunkStorage::Large(storage) => storage.0.as_mut(),
            ChunkStorage::Normal(storage) => storage.0.as_mut(),
            ChunkStorage::Other(storage) => storage.0.as_mut(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A vertical slice of the world.
///
/// Has a constant, known number of sections and a known offset.
pub struct ArrayChunkStorage<const SECTIONS: usize, const OFFSET: isize>(Box<[Section; SECTIONS]>);

// -------------------------------------------------------------------------------------------------

/// A vertical slice of the world.
///
/// Has a variable number of sections and a known offset.
pub struct VecChunkStorage(Vec<Section>, isize);
