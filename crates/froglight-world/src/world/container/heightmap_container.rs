use bitvec::prelude::{BitVec, Msb0};

use crate::world::chunk::ChunkDecodeError;

/// Storage for a [`Chunk`](crate::world::Chunk)'s heightmap data
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct HeightMapContainer {
    /// Bits per entry
    ///
    /// ceil(log2(world height + 1))
    bits: usize,
    data: BitVec<u64, Msb0>,
}

impl HeightMapContainer {
    /// Create a new [`HeightMapContainer`] from a [`Vec<i64>`](Vec)
    ///
    /// This is especially useful if you have a
    /// [`NbtTag::LongArray`](simdnbt::owned::NbtTag::LongArray)
    ///
    /// # Errors
    /// If the data cannot be converted to a [`BitVec`], an error will be
    /// returned.
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn try_from_vec(height: usize, heightmap_data: Vec<i64>) -> Result<Self, ChunkDecodeError> {
        // Calculate the number of bits required to store the data
        let bits = (height + 1) as f32;
        let bits = bits.log2().ceil() as usize;

        // Convert Vec<i64> to Vec<u64>
        // Do not change bits, as the data is already in the correct format
        let data: Vec<u64> = bytemuck::cast_vec(heightmap_data);
        let data = BitVec::try_from_vec(data).map_err(|_| ChunkDecodeError::BitVec)?;

        Ok(Self { bits, data })
    }
}
