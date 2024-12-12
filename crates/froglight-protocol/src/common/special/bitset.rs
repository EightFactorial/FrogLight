use std::{
    io::{Cursor, Write},
    ops::Not,
};

use bitvec::{array::BitArray, order::Msb0};

use crate::protocol::{FrogRead, FrogWrite, ReadError, WriteError};

/// A fixed-size bitset.
///
/// `N` is the number of bits in the bitset,
/// and must be greater than `0`.
///
/// # Example
/// ```rust
/// use froglight_protocol::common::BitSet;
///
/// let mut bitset = BitSet::<3>::new();
/// assert_eq!(bitset.get_bit(0), Some(false));
/// assert_eq!(bitset.get_bit(1), Some(false));
/// assert_eq!(bitset.get_bit(2), Some(false));
///
/// bitset.set_bit(0, false);
/// bitset.set_bit(1, true);
/// bitset.set_bit(2, false);
///
/// assert_eq!(bitset.get_bit(0), Some(false));
/// assert_eq!(bitset.get_bit(1), Some(true));
/// assert_eq!(bitset.get_bit(2), Some(false));
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitSet<const N: usize>(BitArray<[u8; N.div_ceil(8)], Msb0>)
where
    [(); N.div_ceil(8)]:,
    Assert<{ N > 0 }>: IsTrue;

impl<const N: usize> BitSet<N>
where
    [(); N.div_ceil(8)]:,
    Assert<{ N > 0 }>: IsTrue,
{
    /// Create a new empty bitset.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::BitSet;
    ///
    /// let bitset = BitSet::<3>::new();
    /// assert_eq!(bitset.get_bit(0), Some(false));
    /// assert_eq!(bitset.get_bit(1), Some(false));
    /// assert_eq!(bitset.get_bit(2), Some(false));
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self { Self(BitArray::<[u8; N.div_ceil(8)], Msb0>::ZERO) }

    /// Create a bitset from an array of bytes.
    ///
    /// # Note
    /// Any bits over the length of the bitset will be set to 0.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::BitSet;
    ///
    /// let bitset = BitSet::<4>::from_array([0b10101010]);
    /// assert_eq!(bitset.get_byte(0), Some(0b10100000));
    /// ```
    #[inline]
    #[must_use]
    pub fn from_array(array: [u8; N.div_ceil(8)]) -> Self {
        let mut array = BitArray::new(array);

        // Set the bits after the last bit to 0.
        for i in N..array.len() {
            array.set(i, false);
        }

        Self(array)
    }

    /// Get the value of the bit at the given index.
    ///
    /// Returns `None` if the index is out of bounds.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::BitSet;
    ///
    /// let mut bitset = BitSet::<8>::new();
    /// assert_eq!(bitset.set_byte(0, 0b10101010), Some(0));
    ///
    /// assert_eq!(bitset.get_byte(0), Some(0b10101010));
    ///
    /// assert_eq!(bitset.get_bit(0), Some(true));
    /// assert_eq!(bitset.get_bit(1), Some(false));
    /// assert_eq!(bitset.get_bit(2), Some(true));
    /// // ...
    /// ```
    #[must_use]
    pub fn get_bit(&self, index: usize) -> Option<bool> {
        if index < N {
            self.0.get(index).as_deref().copied()
        } else {
            None
        }
    }

    /// Set the value of the bit at the given index.
    ///
    /// Returns the previous value,
    /// or `None` if the index is out of bounds.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::BitSet;
    ///
    /// let mut bitset = BitSet::<4>::new();
    /// assert_eq!(bitset.get_bit(0), Some(false));
    /// assert_eq!(bitset.get_bit(1), Some(false));
    /// assert_eq!(bitset.get_bit(2), Some(false));
    /// assert_eq!(bitset.get_bit(3), Some(false));
    ///
    /// assert_eq!(bitset.set_bit(1, true), Some(false));
    /// assert_eq!(bitset.set_bit(2, true), Some(false));
    ///
    /// assert_eq!(bitset.get_bit(0), Some(false));
    /// assert_eq!(bitset.get_bit(1), Some(true));
    /// assert_eq!(bitset.get_bit(2), Some(true));
    /// assert_eq!(bitset.get_bit(3), Some(false));
    /// ```
    pub fn set_bit(&mut self, index: usize, value: bool) -> Option<bool> {
        self.0.get_mut(index).map(|mut bit| bit.replace(value))
    }

    /// Flip the value of the bit at the given index.
    ///
    /// Returns the previous value,
    /// or `None` if the index is out of bounds.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::BitSet;
    ///
    /// let mut bitset = BitSet::<4>::new();
    /// assert_eq!(bitset.all(), false);
    ///
    /// assert_eq!(bitset.flip_bit(0), Some(false));
    ///
    /// assert_eq!(bitset.get_bit(0), Some(true));
    /// assert_eq!(bitset.get_bit(1), Some(false));
    /// assert_eq!(bitset.get_bit(2), Some(false));
    /// assert_eq!(bitset.get_bit(3), Some(false));
    /// ```
    pub fn flip_bit(&mut self, index: usize) -> Option<bool> {
        if let Some(mut bit) = self.0.get_mut(index) {
            let flip = bit.eq(&false);
            Some(bit.replace(flip))
        } else {
            None
        }
    }

    /// Get the value of the byte at the given index.
    ///
    /// Returns `None` if the index is out of bounds.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::BitSet;
    ///
    /// let mut bitset = BitSet::<8>::new();
    /// assert_eq!(bitset.set_byte(0, 0b10101010), Some(0));
    ///
    /// assert_eq!(bitset.get_byte(0), Some(0b10101010));
    ///
    /// assert_eq!(bitset.get_bit(0), Some(true));
    /// assert_eq!(bitset.get_bit(1), Some(false));
    /// assert_eq!(bitset.get_bit(2), Some(true));
    /// // ...
    /// ```
    #[must_use]
    pub fn get_byte(&self, index: usize) -> Option<u8> { self.0.data.get(index).copied() }

    /// Set the value of the byte at the given index.
    ///
    /// Returns the previous value,
    /// or `None` if the index is out of bounds.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::BitSet;
    ///
    /// let mut bitset = BitSet::<8>::new();
    /// assert_eq!(bitset.set_byte(0, 0b10101010), Some(0));
    ///
    /// assert_eq!(bitset.get_byte(0), Some(0b10101010));
    ///
    /// assert_eq!(bitset.get_bit(0), Some(true));
    /// assert_eq!(bitset.get_bit(1), Some(false));
    /// assert_eq!(bitset.get_bit(2), Some(true));
    /// // ...
    /// ```
    pub fn set_byte(&mut self, index: usize, value: u8) -> Option<u8> {
        self.0.data.get_mut(index).map(|byte| std::mem::replace(byte, value))
    }

    /// Flip the value of the byte at the given index.
    ///
    /// Returns the previous value,
    /// or `None` if the index is out of bounds.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::BitSet;
    ///
    /// let mut bitset = BitSet::<8>::new();
    /// assert_eq!(bitset.set_byte(0, 0b10101010), Some(0));
    ///
    /// assert_eq!(bitset.flip_byte(0), Some(0b10101010));
    ///
    /// assert_eq!(bitset.get_byte(0), Some(0b01010101));
    /// ```
    pub fn flip_byte(&mut self, index: usize) -> Option<u8> {
        if let Some(byte) = self.0.data.get_mut(index) {
            let flip = byte.not();
            Some(std::mem::replace(byte, flip))
        } else {
            None
        }
    }

    /// Check if all bits are set.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::BitSet;
    ///
    /// let mut bitset = BitSet::<8>::new();
    /// assert!(!bitset.all());
    ///
    /// assert_eq!(bitset.set_byte(0, 0b11111111), Some(0));
    /// assert_eq!(bitset.get_byte(0), Some(0b11111111));
    ///
    /// assert!(bitset.all());
    /// ```
    #[inline]
    #[must_use]
    pub fn all(&self) -> bool { self.0.all() }

    /// Get the number of bits in the bitset.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::BitSet;
    ///
    /// let bitset = BitSet::<8>::new();
    /// assert_eq!(bitset.bit_len(), 8);
    /// ```
    #[inline]
    #[must_use]
    pub const fn bit_len(&self) -> usize { N }

    /// Get the number of bytes in the bitset.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::BitSet;
    ///
    /// assert_eq!(BitSet::<1>::new().byte_len(), 1);
    /// assert_eq!(BitSet::<8>::new().byte_len(), 1);
    ///
    /// assert_eq!(BitSet::<9>::new().byte_len(), 2);
    /// assert_eq!(BitSet::<16>::new().byte_len(), 2);
    ///
    /// assert_eq!(BitSet::<17>::new().byte_len(), 3);
    /// assert_eq!(BitSet::<24>::new().byte_len(), 3);
    /// ```
    #[inline]
    #[must_use]
    pub const fn byte_len(&self) -> usize { N.div_ceil(8) }
}

/// Read a [`BitSet`] from a buffer.
impl<const N: usize> FrogRead for BitSet<N>
where
    [(); N.div_ceil(8)]:,
    Assert<{ N > 0 }>: IsTrue,
{
    fn fg_read(buf: &mut Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        Ok(Self::from_array(core::array::try_from_fn(|_| u8::fg_read(buf))?))
    }
}

/// Write a [`BitSet`] to a buffer.
impl<const N: usize> FrogWrite for BitSet<N>
where
    [(); N.div_ceil(8)]:,
    Assert<{ N > 0 }>: IsTrue,
{
    fn fg_write(&self, buf: &mut (impl Write + ?Sized)) -> Result<(), WriteError> {
        self.0.data.fg_write(buf)
    }
}

/// Convert a byte array to a [`BitSet`].
impl<const N: usize> From<[u8; N.div_ceil(8)]> for BitSet<N>
where
    [(); N.div_ceil(8)]:,
    Assert<{ N > 0 }>: IsTrue,
{
    fn from(value: [u8; N.div_ceil(8)]) -> Self { Self::from_array(value) }
}

/// Convert a [`BitSet`] to a byte array.
impl<const N: usize> From<BitSet<N>> for [u8; N.div_ceil(8)]
where
    [(); N.div_ceil(8)]:,
    Assert<{ N > 0 }>: IsTrue,
{
    fn from(value: BitSet<N>) -> Self { value.0.data }
}

/// Convert a boolean array to a [`BitSet`].
impl<const N: usize> From<[bool; N]> for BitSet<N>
where
    [(); N.div_ceil(8)]:,
    Assert<{ N > 0 }>: IsTrue,
{
    fn from(value: [bool; N]) -> Self {
        let mut bitarray = BitArray::default();
        for (i, bit) in value.into_iter().enumerate() {
            bitarray.set(i, bit);
        }
        Self(bitarray)
    }
}

/// Convert a [`BitSet`] to a boolean array.
impl<const N: usize> From<BitSet<N>> for [bool; N]
where
    [(); N.div_ceil(8)]:,
    Assert<{ N > 0 }>: IsTrue,
{
    fn from(value: BitSet<N>) -> Self { core::array::from_fn(|i| value.0[i]) }
}

use sealed::{Assert, IsTrue};
/// Asserts that the given value is not zero.
mod sealed {
    /// [`BitSet`](super::BitSet) size must be greater than 0.
    pub trait IsTrue {}
    /// A trait for asserting a boolean value.
    pub enum Assert<const CHECK: bool> {}
    /// Only implemented for `Assert<true>`.
    impl IsTrue for Assert<true> {}
}

#[test]
fn bitset_default() {
    let bitset = BitSet::<1>::default();
    assert!(!bitset.all());
    assert_eq!(bitset.bit_len(), 1);
    assert_eq!(bitset.byte_len(), 1);

    let bitset = BitSet::<8>::default();
    assert!(!bitset.all());
    assert_eq!(bitset.bit_len(), 8);
    assert_eq!(bitset.byte_len(), 1);

    let bitset = BitSet::<9>::default();
    assert!(!bitset.all());
    assert_eq!(bitset.bit_len(), 9);
    assert_eq!(bitset.byte_len(), 2);
}

// TODO: Write a few tests using Proptest
#[test]
fn bitset_getset() {
    let mut bitset = BitSet::<24>::new();
    assert!(!bitset.all());

    assert_eq!(bitset.set_bit(0, true), Some(false));
    assert_eq!(bitset.set_bit(1, true), Some(false));

    assert_eq!(bitset.get_bit(0), Some(true));
    assert_eq!(bitset.get_bit(1), Some(true));
    assert_eq!(bitset.get_bit(2), Some(false));
    assert_eq!(bitset.get_bit(3), Some(false));

    assert_eq!(bitset.set_byte(0, 0b1010_1010), Some(0b1100_0000));
    assert_eq!(bitset.set_byte(1, 0b1010_1010), Some(0b0000_0000));

    assert_eq!(bitset.get_byte(0), Some(0b1010_1010));
    assert_eq!(bitset.get_byte(1), Some(0b1010_1010));

    assert_eq!(bitset.bit_len(), 24);
    assert_eq!(bitset.byte_len(), 3);
}
