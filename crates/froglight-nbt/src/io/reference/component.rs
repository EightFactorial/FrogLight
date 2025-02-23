use super::NbtStreamError;
use crate::nbt::NbtTag;

/// A reference to an NBT compound.
///
/// The raw form of [`NbtCompound`](crate::nbt::NbtCompound).
#[repr(transparent)]
pub struct NbtCompoundRef<'a>(&'a [u8]);

impl NbtCompoundRef<'_> {
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
#[repr(transparent)]
pub struct NbtTagRef<'a>(&'a [u8]);

impl NbtTagRef<'_> {
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
pub struct NbtListTagRef<'a>(&'a [u8]);

impl NbtListTagRef<'_> {
    /// Get the size of the [`NbtListTagRef`] from the given data,
    /// or an error if the data is invalid.
    const fn size_of(data: &[u8]) -> Result<usize, NbtStreamError> {
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
