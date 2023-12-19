use mc_rs_macros::Transcode;

use super::{BlockPos, ChunkPos};

/// A block's position in a chunk.
///
/// This is a relative position, not an absolute position.
/// To get the absolute position, use [`ChunkBlockPos::to_block_pos`](ChunkBlockPos).
///
/// # Examples
/// ```rust
/// use mc_rs_protocol::types::position::{BlockPos, ChunkPos, ChunkBlockPos};
///
/// let default = ChunkBlockPos::default();
/// assert_eq!(default, ChunkBlockPos::new(0, 0, 0));
///
/// let chunk_pos = ChunkPos::new(3, -4);
/// let chunk_block_pos = ChunkBlockPos::new(0, 1, 0);
/// assert_eq!(BlockPos::new(48, 1, -64), chunk_block_pos.to_block_pos(chunk_pos));
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0, 0, 0, 0, 0, 0])]
pub struct ChunkBlockPos {
    pub x: u8,
    pub y: i32,
    pub z: u8,
}

impl ChunkBlockPos {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };

    /// Creates a new [`ChunkBlockPos`] from the given coordinates.
    ///
    /// # Warning
    /// This does not check if the coordinates are valid.
    ///
    /// # Examples
    /// ```rust
    /// use mc_rs_protocol::types::position::ChunkBlockPos;
    ///
    /// let default = ChunkBlockPos::default();
    /// assert_eq!(default, ChunkBlockPos::new(0, 0, 0));
    /// ```
    pub const fn new(x: u8, y: i32, z: u8) -> Self { Self { x, y, z } }

    /// Creates a new [`ChunkBlockPos`] from the given coordinates.
    ///
    /// Returns `None` if the coordinates are invalid.
    ///
    /// NOTE: This assumes that chunks are `16x384x16 (X,Y,Z)`,
    /// and that the `Y` coordinate is shifted by `-64`.
    ///
    /// # Examples
    /// ```rust
    /// use mc_rs_protocol::types::position::ChunkBlockPos;
    ///
    /// let default = ChunkBlockPos::default();
    /// assert_eq!(default, ChunkBlockPos::try_new(0, 0, 0).unwrap());
    ///
    /// assert!(ChunkBlockPos::try_new(16, 0, 0).is_none());
    /// assert!(ChunkBlockPos::try_new(0, 0, 16).is_none());
    ///
    /// assert!(ChunkBlockPos::try_new(0, 321, 0).is_none());
    /// assert!(ChunkBlockPos::try_new(0, -65, 0).is_none());
    /// ```
    pub const fn try_new(x: u8, y: i32, z: u8) -> Option<Self> {
        if x < 16 && -64 <= y && y <= 320 && z < 16 {
            Some(Self { x, y, z })
        } else {
            None
        }
    }

    /// Creates a new [`ChunkBlockPos`] from the given block index.
    ///
    /// This is zero-indexed, so the first block in the chunk is 0, the second is 1, etc.
    ///
    /// NOTE: This assumes that chunks are `16xYx16 (X,Y,Z)`.
    ///
    /// # Examples
    /// ```rust
    /// use mc_rs_protocol::types::position::ChunkBlockPos;
    ///
    /// let default = ChunkBlockPos::default();
    /// assert_eq!(default, ChunkBlockPos::from_index(0));
    ///
    /// let _1_0_0 = ChunkBlockPos::new(1, 0, 0);
    /// assert_eq!(_1_0_0, ChunkBlockPos::from_index(1));
    ///
    /// let _15_0_0 = ChunkBlockPos::new(15, 0, 0);
    /// assert_eq!(_15_0_0, ChunkBlockPos::from_index(15));
    ///
    /// let _0_0_1 = ChunkBlockPos::new(0, 0, 1);
    /// assert_eq!(_0_0_1, ChunkBlockPos::from_index(16));
    /// ```
    pub const fn from_index(index: usize) -> Self {
        Self {
            x: (index % 16) as u8,
            z: ((index / 16) % 16) as u8,
            y: ((index / 16) / 16) as i32,
        }
    }

    /// Converts this [`ChunkBlockPos`] to a block index.
    ///
    /// This is zero-indexed, so the first block in the chunk is 0, the second is 1, etc.
    ///
    /// # Examples
    /// ```rust
    /// use mc_rs_protocol::types::position::ChunkBlockPos;
    ///
    /// let default = ChunkBlockPos::default();
    /// assert_eq!(0, default.as_index());
    ///
    /// let _1_0_0 = ChunkBlockPos::new(1, 0, 0);
    /// assert_eq!(1, _1_0_0.as_index());
    /// ```
    pub const fn as_index(&self) -> usize {
        let x = (self.x % 16) as usize;
        let z = (self.z % 16) as usize;

        let mut y = self.y % 16;
        if y < 0 {
            y += 16;
        }

        x + (z * 16) + (y as usize * 16 * 16)
    }

    /// Converts this [`ChunkBlockPos`] to a [`BlockPos`] in the given [`ChunkPos`].
    ///
    /// This needs to know the [`ChunkPos`] because [`ChunkBlockPos`]es are relative to the chunk,
    /// not the world.
    ///
    /// # Examples
    /// ```rust
    /// use mc_rs_protocol::types::position::{BlockPos, ChunkBlockPos, ChunkPos};
    ///
    /// let chunk_block_pos = ChunkBlockPos::new(0, 0, 0);
    ///
    /// let chunk_pos = ChunkPos::new(0, 0);
    /// assert_eq!(BlockPos::new(0, 0, 0), chunk_block_pos.to_block_pos(chunk_pos));
    ///
    /// let chunk_pos = ChunkPos::new(1, 0);
    /// assert_eq!(BlockPos::new(16, 0, 0), chunk_block_pos.to_block_pos(chunk_pos));
    ///
    /// let chunk_pos = ChunkPos::new(0, -1);
    /// assert_eq!(BlockPos::new(0, 0, -16), chunk_block_pos.to_block_pos(chunk_pos));
    /// ```
    pub fn to_block_pos(self, chunk_pos: ChunkPos) -> BlockPos {
        BlockPos::new(
            chunk_pos.x * 16 + self.x as i32,
            self.y,
            chunk_pos.y * 16 + self.z as i32,
        )
    }
}

impl From<BlockPos> for ChunkBlockPos {
    fn from(value: BlockPos) -> Self {
        let mut x = value.x % 16;
        let mut z = value.z % 16;

        if x < 0 {
            x += 16;
        }
        if z < 0 {
            z += 16;
        }

        Self {
            x: x as u8,
            y: value.y,
            z: z as u8,
        }
    }
}

#[test]
fn from_index() {
    assert_eq!(ChunkBlockPos::new(0, 0, 0), ChunkBlockPos::from_index(0));
    assert_eq!(ChunkBlockPos::new(1, 0, 0), ChunkBlockPos::from_index(1));
    assert_eq!(ChunkBlockPos::new(15, 0, 0), ChunkBlockPos::from_index(15));
    assert_eq!(ChunkBlockPos::new(0, 0, 1), ChunkBlockPos::from_index(16));
    assert_eq!(ChunkBlockPos::new(1, 0, 1), ChunkBlockPos::from_index(17));
    assert_eq!(ChunkBlockPos::new(15, 0, 1), ChunkBlockPos::from_index(31));
    assert_eq!(ChunkBlockPos::new(0, 0, 15), ChunkBlockPos::from_index(240));
    assert_eq!(ChunkBlockPos::new(1, 0, 15), ChunkBlockPos::from_index(241));
    assert_eq!(
        ChunkBlockPos::new(15, 0, 15),
        ChunkBlockPos::from_index(255)
    );
    assert_eq!(ChunkBlockPos::new(0, 1, 0), ChunkBlockPos::from_index(256));
    assert_eq!(ChunkBlockPos::new(1, 1, 0), ChunkBlockPos::from_index(257));
    assert_eq!(ChunkBlockPos::new(15, 1, 0), ChunkBlockPos::from_index(271));
    assert_eq!(ChunkBlockPos::new(0, 1, 1), ChunkBlockPos::from_index(272));
    assert_eq!(ChunkBlockPos::new(1, 1, 1), ChunkBlockPos::from_index(273));
    assert_eq!(ChunkBlockPos::new(15, 1, 1), ChunkBlockPos::from_index(287));
    assert_eq!(ChunkBlockPos::new(0, 1, 15), ChunkBlockPos::from_index(496));
    assert_eq!(ChunkBlockPos::new(1, 1, 15), ChunkBlockPos::from_index(497));
    assert_eq!(
        ChunkBlockPos::new(15, 1, 15),
        ChunkBlockPos::from_index(511)
    );
    assert_eq!(
        ChunkBlockPos::new(0, 15, 0),
        ChunkBlockPos::from_index(3840)
    );
    assert_eq!(
        ChunkBlockPos::new(1, 15, 0),
        ChunkBlockPos::from_index(3841)
    );
    assert_eq!(
        ChunkBlockPos::new(15, 15, 0),
        ChunkBlockPos::from_index(3855)
    );
    assert_eq!(
        ChunkBlockPos::new(0, 15, 1),
        ChunkBlockPos::from_index(3856)
    );
    assert_eq!(
        ChunkBlockPos::new(1, 15, 1),
        ChunkBlockPos::from_index(3857)
    );
    assert_eq!(
        ChunkBlockPos::new(15, 15, 1),
        ChunkBlockPos::from_index(3871)
    );
    assert_eq!(
        ChunkBlockPos::new(0, 15, 15),
        ChunkBlockPos::from_index(4080)
    );
    assert_eq!(
        ChunkBlockPos::new(1, 15, 15),
        ChunkBlockPos::from_index(4081)
    );
    assert_eq!(
        ChunkBlockPos::new(15, 15, 15),
        ChunkBlockPos::from_index(4095)
    );
}
