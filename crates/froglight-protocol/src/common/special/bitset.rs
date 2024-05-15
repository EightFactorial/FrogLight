use std::io::{Cursor, Write};

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
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct BitSet<const N: usize>(BitArray<[u8; N.div_ceil(8)], Msb0>)
where
    [u8; N.div_ceil(8)]:,
    Assert<{ N > 0 }>: IsNotZero;

impl<const N: usize> BitSet<N>
where
    [u8; N.div_ceil(8)]:,
    Assert<{ N > 0 }>: IsNotZero,
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
    #[must_use]
    #[inline]
    pub fn new() -> Self { Self(BitArray::default()) }

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
    #[must_use]
    #[inline]
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
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::BitSet;
    ///
    /// let mut bitset = BitSet::<8>::new();
    /// bitset.set_byte(0, 0b10101010).unwrap();
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

    /// Get the value of the byte at the given index.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::BitSet;
    ///
    /// let mut bitset = BitSet::<8>::new();
    /// bitset.set_byte(0, 0b10101010).unwrap();
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

    /// Set the value of the bit at the given index.
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
    /// bitset.set_bit(1, true);
    /// bitset.set_bit(2, true);
    ///
    /// assert_eq!(bitset.get_bit(0), Some(false));
    /// assert_eq!(bitset.get_bit(1), Some(true));
    /// assert_eq!(bitset.get_bit(2), Some(true));
    /// assert_eq!(bitset.get_bit(3), Some(false));
    /// ```
    ///
    /// # Errors
    /// Returns `Err` if the index is out of bounds.
    #[allow(clippy::result_unit_err)]
    pub fn set_bit(&mut self, index: usize, value: bool) -> Result<bool, ()> {
        if let Some(mut bit) = self.0.get_mut(index) {
            Ok(bit.replace(value))
        } else {
            Err(())
        }
    }

    /// Set the value of the byte at the given index.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::BitSet;
    ///
    /// let mut bitset = BitSet::<8>::new();
    /// bitset.set_byte(0, 0b10101010).unwrap();
    ///
    /// assert_eq!(bitset.get_byte(0), Some(0b10101010));
    ///
    /// assert_eq!(bitset.get_bit(0), Some(true));
    /// assert_eq!(bitset.get_bit(1), Some(false));
    /// assert_eq!(bitset.get_bit(2), Some(true));
    /// // ...
    /// ```
    /// # Errors
    /// Returns `Err` if the index is out of bounds.
    #[allow(clippy::result_unit_err)]
    pub fn set_byte(&mut self, index: usize, value: u8) -> Result<u8, ()> {
        if let Some(byte) = self.0.data.get_mut(index) {
            Ok(std::mem::replace(byte, value))
        } else {
            Err(())
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
    /// assert!(bitset.set_byte(0, 0b11111111).is_ok());
    /// assert!(bitset.all());
    /// ```
    #[must_use]
    #[inline]
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
    #[must_use]
    #[inline]
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
    #[must_use]
    #[inline]
    pub const fn byte_len(&self) -> usize { N.div_ceil(8) }
}

/// Read a [`BitSet`] from a buffer.
impl<const N: usize> FrogRead for BitSet<N>
where
    [u8; N.div_ceil(8)]:,
    Assert<{ N > 0 }>: IsNotZero,
{
    fn fg_read(buf: &mut Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        let arr = core::array::try_from_fn(|_| u8::fg_read(buf))?;
        Ok(Self::from_array(arr))
    }
}

/// Write a [`BitSet`] to a buffer.
impl<const N: usize> FrogWrite for BitSet<N>
where
    [u8; N.div_ceil(8)]:,
    Assert<{ N > 0 }>: IsNotZero,
{
    fn fg_write(&self, buf: &mut (impl Write + ?Sized)) -> Result<(), WriteError> {
        self.0.data.fg_write(buf)
    }
}

/// Convert a byte array to a [`BitSet`].
impl<const N: usize> From<[u8; N.div_ceil(8)]> for BitSet<N>
where
    [u8; N.div_ceil(8)]:,
    Assert<{ N > 0 }>: IsNotZero,
{
    fn from(value: [u8; N.div_ceil(8)]) -> Self { Self::from_array(value) }
}

/// Convert a [`BitSet`] to a byte array.
impl<const N: usize> From<BitSet<N>> for [u8; N.div_ceil(8)]
where
    [u8; N.div_ceil(8)]:,
    Assert<{ N > 0 }>: IsNotZero,
{
    fn from(value: BitSet<N>) -> Self { value.0.data }
}

/// Convert a boolean array to a [`BitSet`].
impl<const N: usize> From<[bool; N]> for BitSet<N>
where
    [u8; N.div_ceil(8)]:,
    Assert<{ N > 0 }>: IsNotZero,
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
    [u8; N.div_ceil(8)]:,
    Assert<{ N > 0 }>: IsNotZero,
{
    fn from(value: BitSet<N>) -> Self { core::array::from_fn(|i| value.0[i]) }
}

use sealed::{Assert, IsNotZero};
/// Asserts that the given value is not zero.
mod sealed {
    /// [`BitSet`](super::BitSet) size must be greater than 0.
    pub trait IsNotZero {}
    /// A trait for asserting a boolean value.
    pub enum Assert<const CHECK: bool> {}
    /// Only implemented for `Assert<true>`.
    impl IsNotZero for Assert<true> {}
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

    assert_eq!(bitset.set_bit(0, true), Ok(false));
    assert_eq!(bitset.set_bit(1, true), Ok(false));

    assert_eq!(bitset.get_bit(0), Some(true));
    assert_eq!(bitset.get_bit(1), Some(true));
    assert_eq!(bitset.get_bit(2), Some(false));
    assert_eq!(bitset.get_bit(3), Some(false));

    assert_eq!(bitset.set_byte(0, 0b1010_1010), Ok(0b1100_0000));
    assert_eq!(bitset.set_byte(1, 0b1010_1010), Ok(0b0000_0000));

    assert_eq!(bitset.get_byte(0), Some(0b1010_1010));
    assert_eq!(bitset.get_byte(1), Some(0b1010_1010));

    assert_eq!(bitset.bit_len(), 24);
    assert_eq!(bitset.byte_len(), 3);
}
