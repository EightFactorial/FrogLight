use bevy_log::warn;
use bevy_reflect::Reflect;
use bitvec::prelude::{BitVec, Msb0};
use simdnbt::owned::{NbtCompound, NbtTag};

use crate::world::chunk::ChunkDecodeError;

/// Storage for a [`Chunk`](crate::world::Chunk)'s heightmap data
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct HeightMapContainer {
    /// Bits per entry
    ///
    /// Calculated as: `ceil(log2(height + 1))`
    pub(crate) bits: usize,
    #[reflect(ignore)]
    pub(crate) data: BitVec<u64, Msb0>,
}

impl HeightMapContainer {
    /// Create a new [`HeightMapContainer`] from a [`Vec<i64>`](Vec)
    ///
    /// This is especially useful if you have a [`NbtTag::LongArray`].
    ///
    /// # Errors
    /// If the data cannot be converted to a [`BitVec`].
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn try_from_vec(height: usize, heightmap_data: Vec<i64>) -> Result<Self, ChunkDecodeError> {
        // Calculate the number of bits required to store the data
        let bits = (height as f32 + 1.0).log2().ceil() as usize;

        // Convert `Vec<i64>` to `Vec<u64>`
        // Do not change bits, as the data is already in the correct format
        let data: Vec<u64> = bytemuck::cast_vec(heightmap_data);
        let data = BitVec::try_from_vec(data).map_err(|_| ChunkDecodeError::BitVec)?;

        Ok(Self { bits, data })
    }

    /// Try to read a [`HeightMapContainer`] from a [`NbtCompound`].
    ///
    /// # Errors
    /// If the key does not point to a [`NbtTag::LongArray`].
    /// If the data cannot be read into a [`BitVec`].
    pub fn try_from_nbt(
        height: usize,
        key: &str,
        nbt: &mut NbtCompound,
    ) -> Result<Option<Self>, ChunkDecodeError> {
        if let Some(NbtTag::LongArray(motion_blocking)) = nbt.take(key) {
            HeightMapContainer::try_from_vec(height, motion_blocking).map(Some)
        } else {
            warn!("NBT does not contain key: `{key}`");
            Ok(None)
        }
    }
}
