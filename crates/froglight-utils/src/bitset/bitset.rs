#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use bitvec::prelude::*;
use derive_more::{Deref, DerefMut, From, Into};
#[cfg(feature = "io")]
use froglight_io::prelude::*;

/// A [`Vec`] of [`u64`] integers, represented as bits.
///
/// Allows for efficient storage and manipulation of bits.
#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(opaque, Debug, Default, Clone))]
pub struct BitSet(BitVec<u64, LocalBits>);

impl BitSet {
    /// Create a new empty [`BitSet`].
    #[inline]
    #[must_use]
    pub const fn new() -> Self { Self(BitVec::EMPTY) }

    /// Create a new [`BitSet`] from the given [`Vec`].
    #[must_use]
    pub fn from_data(data: Vec<u64>) -> Self { Self(BitVec::from_vec(data)) }

    /// Create a new [`BitSet`] from the given slice.
    #[must_use]
    pub fn from_slice(slice: &[u64]) -> Self { Self(BitVec::from_slice(slice)) }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "io")]
impl FrogRead for BitSet {
    #[inline]
    fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, ReadError> {
        <Vec<u64>>::frog_read(buffer).map(Self::from_data)
    }
}

#[cfg(feature = "io")]
impl FrogWrite for BitSet {
    #[inline]
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        self.as_raw_slice().frog_write(buffer)
    }

    #[inline]
    fn frog_len(&self) -> usize { self.as_raw_slice().frog_len() }
}
