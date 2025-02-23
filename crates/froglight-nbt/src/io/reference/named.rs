use super::{NbtCompoundRef, NbtStreamError};
use crate::nbt::NbtTag;

/// A reference to named NBT data.
///
/// The raw form of [`NamedNbt`](crate::nbt::NamedNbt).
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NamedNbtRef<'a>(&'a [u8]);

impl<'a> NamedNbtRef<'a> {
    /// Create a new [`NamedNbtRef`] from the given data.
    ///
    /// This is useful for compile-time checks.
    ///
    /// # Panics
    /// Panics if the data is invalid NBT.
    #[must_use]
    pub const fn new(data: &'a [u8]) -> Self {
        match Self::try_new(data) {
            Ok((named, _)) => named,
            Err(_) => panic!("Attempted to create `NamedNbtRef` over invalid data"),
        }
    }

    /// Create a new [`NamedNbtRef`] from the given data.
    ///
    /// # Warning
    /// This will not check ahead of time if the data is valid.
    #[inline]
    #[must_use]
    pub const fn new_unchecked(data: &'a [u8]) -> Self { Self(data) }

    /// Try to create a new [`NamedNbtRef`] from the given data,
    /// returning the remaining data if successful.
    ///
    /// # Errors
    /// Returns an error if the data is invalid NBT.
    pub const fn try_new(data: &'a [u8]) -> Result<(Self, &'a [u8]), NbtStreamError> {
        match Self::size_of(data) {
            Ok(size) => {
                let (data, remaining) = data.split_at(size);
                Ok((Self::new_unchecked(data), remaining))
            }
            Err(err) => Err(err),
        }
    }

    /// Get the size of the [`NamedNbtRef`] from the given data,
    /// or an error if the data is invalid.
    ///
    /// # Errors
    /// Returns an error if the data is invalid NBT.
    const fn size_of(data: &'a [u8]) -> Result<usize, NbtStreamError> {
        // Take the tag from the data.
        if let Some((&tag, data)) = data.split_first() {
            // If the tag is `END`, then the stream is empty.
            if tag == NbtTag::END {
                return Ok(1);
            }

            // Take the name from the data.
            if let Some((&length, data)) = data.split_first_chunk::<2>() {
                let length = u16::from_be_bytes(length) as usize;
                #[allow(clippy::used_underscore_binding, unreachable_code, unused_variables)]
                if let Some((_str, data)) = data.split_at_checked(length) {
                    //

                    // Add the tag and name to the result
                    return match UnnamedNbtRef::size_of_tag(tag, data) {
                        Ok(size) => Ok(1 + (2 + length) + size),
                        Err(err) => Err(err),
                    };
                }
            }
        }

        Err(NbtStreamError::EndOfStream)
    }

    /// Returns the number of bytes that this [`NamedNbtRef`] represents.
    #[inline]
    #[must_use]
    #[expect(clippy::len_without_is_empty)]
    pub const fn len(&self) -> usize { self.0.len() }
}

// -------------------------------------------------------------------------------------------------

/// A reference to unnamed NBT data.
///
/// The raw form of [`UnnamedNbt`](crate::nbt::UnnamedNbt).
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UnnamedNbtRef<'a>(&'a [u8]);

impl<'a> UnnamedNbtRef<'a> {
    /// Create a new [`UnnamedNbtRef`] from the given data.
    ///
    /// This is useful for compile-time checks.
    ///
    /// # Panics
    /// Panics if the data is invalid NBT.
    #[must_use]
    pub const fn new(data: &'a [u8]) -> Self {
        match Self::try_new(data) {
            Ok((named, _)) => named,
            Err(_) => panic!("Attempted to create `NamedNbtRef` over invalid data"),
        }
    }

    /// Create a new [`UnnamedNbtRef`] from the given data.
    ///
    /// # Warning
    /// This will not check ahead of time if the data is valid.
    #[inline]
    #[must_use]
    pub const fn new_unchecked(data: &'a [u8]) -> Self { Self(data) }

    /// Try to create a new [`UnnamedNbtRef`] from the given data,
    /// returning the remaining data if successful.
    ///
    /// # Errors
    /// Returns an error if the data is invalid NBT.
    pub const fn try_new(data: &'a [u8]) -> Result<(Self, &'a [u8]), NbtStreamError> {
        match Self::size_of(data) {
            Ok(size) => {
                let (data, remaining) = data.split_at(size);
                Ok((Self::new_unchecked(data), remaining))
            }
            Err(err) => Err(err),
        }
    }

    /// Get the size of the [`UnnamedNbtRef`] from the given data,
    /// or an error if the data is invalid.
    ///
    /// # Errors
    /// Returns an error if the data is invalid NBT.
    const fn size_of(data: &'a [u8]) -> Result<usize, NbtStreamError> {
        // Take the tag from the data.
        if let Some((&tag, data)) = data.split_first() {
            UnnamedNbtRef::size_of_tag(tag, data)
        } else {
            Err(NbtStreamError::EndOfStream)
        }
    }

    const fn size_of_tag(tag: u8, data: &'a [u8]) -> Result<usize, NbtStreamError> {
        match tag {
            NbtTag::END => Ok(1),
            NbtTag::COMPOUND => NbtCompoundRef::size_of(data),
            other => Err(NbtStreamError::InvalidTag(other)),
        }
    }

    /// Returns the number of bytes that this [`UnnamedNbtRef`] represents.
    #[inline]
    #[must_use]
    #[expect(clippy::len_without_is_empty)]
    pub const fn len(&self) -> usize { self.0.len() }
}
