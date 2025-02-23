use super::{NbtStreamError, PrefixedArray};
use crate::{mutf8::Mutf8Str, nbt::NbtTag};

/// A reference to an NBT compound.
///
/// The raw form of [`NbtCompound`](crate::nbt::NbtCompound).
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NbtCompoundRef<'a>(&'a [u8]);

impl<'a> NbtCompoundRef<'a> {
    /// Get the next tag in the [`NbtCompoundRef`].
    ///
    /// Returns `None` if there are no more tags.
    #[must_use]
    #[expect(clippy::missing_panics_doc)]
    pub const fn next_tag(&mut self) -> Option<(&'a Mutf8Str, NbtTagRef<'a>)> {
        let Some((&tag, data)) = self.0.split_first() else { return None };

        if tag == NbtTag::END {
            None
        } else {
            // Get the name
            let (&length, data) = data.split_first_chunk::<2>().unwrap();
            let (name, data) = data.split_at(u16::from_be_bytes(length) as usize);
            // Get the tag
            let Ok(length) = NbtTagRef::size_of_tag(tag, data) else { return None };
            let (tag_data, remaining) = data.split_at(length);

            // Update the remaining data
            self.0 = remaining;

            // SAFETY: The data is valid NBT.
            Some((Mutf8Str::from_bytes(name), unsafe { NbtTagRef::from_bytes(tag, tag_data) }))
        }
    }

    /// Get the internal data of the [`NbtCompoundRef`].
    #[inline]
    #[must_use]
    pub const fn as_bytes(&self) -> &'a [u8] { self.0 }

    /// Create a new [`NbtCompoundRef`] from the given data.
    ///
    /// # Safety
    /// The caller must ensure that the data is valid NBT.
    #[inline]
    #[must_use]
    pub const unsafe fn from_bytes(data: &'a [u8]) -> Self { Self(data) }

    /// Get the size of the [`NbtCompoundRef`] from the given data,
    /// or an error if the data is invalid.
    pub(super) const fn size_of(data: &[u8]) -> Result<usize, NbtStreamError> {
        // Take the tag from the data.
        if let Some((&tag, mut loop_data)) = data.split_first() {
            // Keep track of current tag and total size.
            let mut size = 1;
            let mut tag = tag;

            // While the tag is not `END`, keep reading tags.
            while tag != NbtTag::END {
                //

                // Take the name from the data.
                if let Some((&length, data)) = loop_data.split_first_chunk::<2>() {
                    let name_length = u16::from_be_bytes(length) as usize;
                    #[allow(clippy::used_underscore_binding, unreachable_code, unused_variables)]
                    if let Some((_str, data)) = data.split_at_checked(name_length) {
                        //

                        // Take the tag data from the data.
                        match NbtTagRef::size_of_tag(tag, data) {
                            Err(err) => return Err(err),
                            Ok(tag_length) => {
                                // Get the next tag and data.
                                if let Some((_, data)) = data.split_at_checked(tag_length) {
                                    if let Some((&next, data)) = data.split_first() {
                                        // Add the tag, name, and tag data to the result
                                        size += 1 + (2 + name_length) + tag_length;

                                        tag = next;
                                        loop_data = data;

                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }

                // Data ended unexpectedly.
                return Err(NbtStreamError::EndOfStream);
            }

            // Read full `NbtCompoundRef`, return size.
            return Ok(size);
        }

        Err(NbtStreamError::EndOfStream)
    }
}

// -------------------------------------------------------------------------------------------------

/// A reference to an NBT tag.
///
/// The raw form of [`NbtTag`](crate::nbt::NbtTag).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NbtTagRef<'a>(u8, &'a [u8]);

/// The data of an [`NbtTagRef`].
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum NbtTagRefData<'a> {
    /// A signed 8-bit integer.
    Byte(i8) = NbtTag::BYTE,
    /// A signed 16-bit integer.
    Short(i16) = NbtTag::SHORT,
    /// A signed 32-bit integer.
    Int(i32) = NbtTag::INT,
    /// A signed 64-bit integer.
    Long(i64) = NbtTag::LONG,
    /// A 32-bit floating point number.
    Float(f32) = NbtTag::FLOAT,
    /// A 64-bit floating point number.
    Double(f64) = NbtTag::DOUBLE,
    /// An array of signed 8-bit integers.
    ByteArray(PrefixedArray<'a, i8>) = NbtTag::BYTE_ARRAY,
    /// A MUTF-8 string.
    String(&'a Mutf8Str) = NbtTag::STRING,
    /// A [`NbtListTag`].
    List(NbtListTagRef<'a>) = NbtTag::LIST,
    /// An [`NbtCompound`].
    Compound(NbtCompoundRef<'a>) = NbtTag::COMPOUND,
    /// An array of signed 32-bit integers.
    IntArray(PrefixedArray<'a, i32>) = NbtTag::INT_ARRAY,
    /// An array of signed 64-bit integers.
    LongArray(PrefixedArray<'a, i64>) = NbtTag::LONG_ARRAY,
}

impl<'a> NbtTagRef<'a> {
    /// Get the tag of the [`NbtTagRef`].
    #[inline]
    #[must_use]
    pub const fn tag(&self) -> u8 { self.0 }

    /// Get the internal data of the [`NbtTagRef`].
    ///
    /// # Note
    /// This does not include the tag byte.
    ///
    /// See [`NbtTagRef::tag`] for the tag.
    #[inline]
    #[must_use]
    pub const fn tag_bytes(&self) -> &'a [u8] { self.1 }

    /// Get the data of the [`NbtTagRef`] as a [`NbtTagRefData`].
    #[must_use]
    #[expect(clippy::missing_panics_doc)]
    pub const fn tag_data(&self) -> NbtTagRefData<'a> {
        match self.0 {
            #[allow(clippy::cast_possible_wrap)]
            NbtTag::BYTE => {
                let (&data, _) =
                    self.1.split_first().expect("Invalid data length for `NbtTag::BYTE`");
                NbtTagRefData::Byte(data as i8)
            }
            NbtTag::SHORT => {
                let (&data, _) =
                    self.1.split_first_chunk().expect("Invalid data length for `NbtTag::SHORT`");
                NbtTagRefData::Short(i16::from_be_bytes(data))
            }
            NbtTag::INT => {
                let (&data, _) =
                    self.1.split_first_chunk().expect("Invalid data length for `NbtTag::INT`");
                NbtTagRefData::Int(i32::from_be_bytes(data))
            }
            NbtTag::LONG => {
                let (&data, _) =
                    self.1.split_first_chunk().expect("Invalid data length for `NbtTag::LONG`");
                NbtTagRefData::Long(i64::from_be_bytes(data))
            }
            NbtTag::FLOAT => {
                let (&data, _) =
                    self.1.split_first_chunk().expect("Invalid data length for `NbtTag::FLOAT`");
                NbtTagRefData::Float(f32::from_be_bytes(data))
            }
            NbtTag::DOUBLE => {
                let (&data, _) =
                    self.1.split_first_chunk().expect("Invalid data length for `NbtTag::DOUBLE`");
                NbtTagRefData::Double(f64::from_be_bytes(data))
            }
            NbtTag::STRING => {
                let (&length, data) =
                    self.1.split_first_chunk().expect("Invalid data length for `NbtTag::STRING`");
                let (data, _) = data.split_at(u16::from_be_bytes(length) as usize);
                NbtTagRefData::String(Mutf8Str::from_bytes(data))
            }
            NbtTag::LIST => {
                // SAFETY: The data is valid NBT.
                NbtTagRefData::List(unsafe { NbtListTagRef::from_bytes(self.1) })
            }
            NbtTag::COMPOUND => {
                // SAFETY: The data is valid NBT.
                NbtTagRefData::Compound(unsafe { NbtCompoundRef::from_bytes(self.1) })
            }
            NbtTag::BYTE_ARRAY => {
                // SAFETY: The tag guarantees the data type is `i8`.
                NbtTagRefData::ByteArray(unsafe { PrefixedArray::from_bytes(self.1) })
            }
            NbtTag::INT_ARRAY => {
                // SAFETY: The tag guarantees the data type is `i32`.
                NbtTagRefData::IntArray(unsafe { PrefixedArray::from_bytes(self.1) })
            }
            NbtTag::LONG_ARRAY => {
                // SAFETY: The tag guarantees the data type is `i64`.
                NbtTagRefData::LongArray(unsafe { PrefixedArray::from_bytes(self.1) })
            }
            _ => panic!("Found invalid tag when parsing `NbtTagRefData`!"),
        }
    }

    /// Create a new [`NbtTagRef`] from the given data.
    ///
    /// # Safety
    /// The caller must ensure that the data is valid NBT.
    #[inline]
    #[must_use]
    pub const unsafe fn from_bytes(tag: u8, data: &'a [u8]) -> Self { Self(tag, data) }

    /// Get the size of the [`NbtTagRef`] from the given data,
    /// or an error if the data is invalid.
    const fn size_of_tag(tag: u8, data: &[u8]) -> Result<usize, NbtStreamError> {
        match tag {
            NbtTag::BYTE => Ok(std::mem::size_of::<i8>()),
            NbtTag::SHORT => Ok(std::mem::size_of::<i16>()),
            NbtTag::INT => Ok(std::mem::size_of::<i32>()),
            NbtTag::LONG => Ok(std::mem::size_of::<i64>()),
            NbtTag::FLOAT => Ok(std::mem::size_of::<f32>()),
            NbtTag::DOUBLE => Ok(std::mem::size_of::<f64>()),
            NbtTag::STRING => {
                if let Some((&length, _)) = data.split_first_chunk::<2>() {
                    Ok(2 + u16::from_be_bytes(length) as usize)
                } else {
                    Err(NbtStreamError::EndOfStream)
                }
            }
            NbtTag::LIST => NbtListTagRef::size_of(data),
            NbtTag::COMPOUND => NbtCompoundRef::size_of(data),
            NbtTag::BYTE_ARRAY => NbtListTagRef::size_of_array(std::mem::size_of::<i8>(), data),
            NbtTag::INT_ARRAY => NbtListTagRef::size_of_array(std::mem::size_of::<i32>(), data),
            NbtTag::LONG_ARRAY => NbtListTagRef::size_of_array(std::mem::size_of::<i64>(), data),
            other => Err(NbtStreamError::InvalidTag(other)),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A reference to an NBT list tag.
///
/// The raw form of [`NbtListTag`](crate::nbt::NbtListTag).
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NbtListTagRef<'a>(&'a [u8]);

impl<'a> NbtListTagRef<'a> {
    /// Create a new [`NbtListTagRef`] from the given data.
    ///
    /// # Safety
    /// The caller must ensure that the data is valid NBT.
    #[inline]
    #[must_use]
    pub const unsafe fn from_bytes(data: &'a [u8]) -> Self { Self(data) }

    /// Get the size of the [`NbtListTagRef`] from the given data,
    /// or an error if the data is invalid.
    pub(super) const fn size_of(data: &[u8]) -> Result<usize, NbtStreamError> {
        if let Some((&tag, data)) = data.split_first() {
            let result = match tag {
                NbtTag::END => Ok(4),
                NbtTag::BYTE => Self::size_of_array(std::mem::size_of::<i8>(), data),
                NbtTag::SHORT => Self::size_of_array(std::mem::size_of::<i16>(), data),
                NbtTag::INT => Self::size_of_array(std::mem::size_of::<i32>(), data),
                NbtTag::LONG => Self::size_of_array(std::mem::size_of::<i64>(), data),
                NbtTag::FLOAT => Self::size_of_array(std::mem::size_of::<f32>(), data),
                NbtTag::DOUBLE => Self::size_of_array(std::mem::size_of::<f64>(), data),
                NbtTag::STRING => Self::size_of_string_array(data),
                NbtTag::LIST => Self::size_of_list_array(data),
                NbtTag::COMPOUND => Self::size_of_component_array(data),
                NbtTag::BYTE_ARRAY => Self::size_of_array_array(std::mem::size_of::<i8>(), data),
                NbtTag::INT_ARRAY => Self::size_of_array_array(std::mem::size_of::<i32>(), data),
                NbtTag::LONG_ARRAY => Self::size_of_array_array(std::mem::size_of::<i64>(), data),
                other => Err(NbtStreamError::InvalidTag(other)),
            };

            // Add the tag size to the result.
            match result {
                Ok(size) => Ok(1 + size),
                Err(err) => Err(err),
            }
        } else {
            Err(NbtStreamError::EndOfStream)
        }
    }

    const fn size_of_array(item: usize, data: &[u8]) -> Result<usize, NbtStreamError> {
        if let Some((&length, _data)) = data.split_first_chunk::<4>() {
            Ok(4 + (item * u32::from_be_bytes(length) as usize))
        } else {
            Err(NbtStreamError::EndOfStream)
        }
    }

    const fn size_of_array_array(item: usize, data: &[u8]) -> Result<usize, NbtStreamError> {
        if let Some((&main_len, mut data)) = data.split_first_chunk::<4>() {
            let main_len = u32::from_be_bytes(main_len) as usize;

            // Track the total size and current index.
            let mut total = 4;
            let mut index = 0;

            // Read each item in the array.
            while index < main_len {
                match Self::size_of_array(item, data) {
                    Err(err) => return Err(err),
                    Ok(size) => {
                        if let Some((_, rem)) = data.split_at_checked(size) {
                            // Increase the total size and set the new data.
                            total += size;
                            data = rem;

                            index += 1;
                            continue;
                        }
                    }
                }

                return Err(NbtStreamError::EndOfStream);
            }

            Ok(total)
        } else {
            Err(NbtStreamError::EndOfStream)
        }
    }

    const fn size_of_string_array(data: &[u8]) -> Result<usize, NbtStreamError> {
        if let Some((&main_len, mut data)) = data.split_first_chunk::<4>() {
            let main_len = u32::from_be_bytes(main_len) as usize;

            // Track the total size and current index.
            let mut total = 4;
            let mut index = 0;

            // Read each string in the array.
            while index < main_len {
                if let Some((&length, string_data)) = data.split_first_chunk::<2>() {
                    let length = u16::from_be_bytes(length) as usize;
                    if let Some((_, rem)) = string_data.split_at_checked(length) {
                        // Increase the total size and set the new data.
                        total += 2 + length;
                        data = rem;

                        index += 1;
                        continue;
                    }
                }

                return Err(NbtStreamError::EndOfStream);
            }

            Ok(total)
        } else {
            Err(NbtStreamError::EndOfStream)
        }
    }

    const fn size_of_list_array(data: &[u8]) -> Result<usize, NbtStreamError> {
        if let Some((&main_len, mut data)) = data.split_first_chunk::<4>() {
            let main_len = u32::from_be_bytes(main_len) as usize;

            // Track the total size and current index.
            let mut total = 4;
            let mut index = 0;

            // Read each list in the array.
            while index < main_len {
                match Self::size_of(data) {
                    Err(err) => return Err(err),
                    Ok(size) => {
                        if let Some((_, rem)) = data.split_at_checked(size) {
                            // Increase the total size and set the new data.
                            total += size;
                            data = rem;

                            index += 1;
                            continue;
                        }
                    }
                }

                return Err(NbtStreamError::EndOfStream);
            }

            Ok(total)
        } else {
            Err(NbtStreamError::EndOfStream)
        }
    }

    const fn size_of_component_array(data: &[u8]) -> Result<usize, NbtStreamError> {
        if let Some((&main_len, mut data)) = data.split_first_chunk::<4>() {
            let main_len = u32::from_be_bytes(main_len) as usize;

            // Track the total size and current index.
            let mut total = 4;
            let mut index = 0;

            // Read each list in the array.
            while index < main_len {
                match NbtCompoundRef::size_of(data) {
                    Err(err) => return Err(err),
                    Ok(size) => {
                        if let Some((_, rem)) = data.split_at_checked(size) {
                            // Increase the total size and set the new data.
                            total += size;
                            data = rem;

                            index += 1;
                            continue;
                        }
                    }
                }
                return Err(NbtStreamError::EndOfStream);
            }

            Ok(total)
        } else {
            Err(NbtStreamError::EndOfStream)
        }
    }
}
