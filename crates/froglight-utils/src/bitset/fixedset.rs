use core::{convert::Infallible, fmt::Debug, marker::PhantomData};

#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use bitvec::prelude::*;
use derive_more::{Deref, DerefMut, From, Into};
#[cfg(feature = "io")]
use froglight_io::prelude::*;

/// A fixed-size a array of bytes, represented as bits.
///
/// Allows for efficient storage and manipulation of bits.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(opaque, Debug, Default, Clone))]
pub struct FixedBitSet<const N: usize>(BitArray<[u8; N.div_ceil(8)], LocalBits>)
where [(); N.div_ceil(8)]: Sized;

impl<const N: usize> FixedBitSet<N>
where [(); N.div_ceil(8)]: Sized
{
    /// Create a new [`FixedBitSet`] with all bits initialized to `false`.
    #[inline]
    #[must_use]
    pub const fn new() -> Self { Self(BitArray::ZERO) }

    /// Create a new [`FixedBitSet`] from the given array.
    #[inline]
    #[must_use]
    pub const fn from_data(data: [u8; N.div_ceil(8)]) -> Self {
        Self(BitArray { _ord: PhantomData, data })
    }

    /// Create a new [`FixedBitSet`] from the given array.
    #[must_use]
    pub fn from_bools(data: [bool; N]) -> Self { Self::from_fn(|i| data[i]) }

    /// Create a new `[bool; N]` from the bits in this [`FixedBitSet`].
    #[must_use]
    pub fn into_bools(self) -> [bool; N] { core::array::from_fn(|i| self[i]) }

    /// Create a new [`FixedBitSet`] from the given closure.
    ///
    /// See [`core::array::from_fn`] for more details.
    #[must_use]
    pub fn from_fn<F>(mut f: F) -> Self
    where F: FnMut(usize) -> bool {
        Self::try_from_fn::<_, Infallible>(|i| Ok(f(i))).unwrap_or_else(|_| unreachable!())
    }

    /// Attempt to create a new [`FixedBitSet`] from the given closure.
    ///
    /// See [`core::array::try_from_fn`] for more details.
    #[must_use]
    pub fn try_from_fn<F, Err>(mut f: F) -> Result<Self, Err>
    where F: FnMut(usize) -> Result<bool, Err> {
        let mut array = Self::new();
        for i in 0..N {
            BitSlice::set(&mut array, i, f(i)?);
        }
        Ok(array)
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "io")]
impl<const N: usize> FrogRead for FixedBitSet<N>
where [(); N.div_ceil(8)]: Sized
{
    #[inline]
    fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, ReadError> {
        <[u8; N.div_ceil(8)]>::frog_read(buffer).map(Self::from_data)
    }
}

#[cfg(feature = "io")]
impl<const N: usize> FrogWrite for FixedBitSet<N>
where [(); N.div_ceil(8)]: Sized
{
    #[inline]
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        <[u8; N.div_ceil(8)]>::frog_write(&self.0.data, buffer)
    }

    #[inline]
    fn frog_len(&self) -> usize { N.div_ceil(8) }
}
