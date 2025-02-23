use super::NbtStreamError;
use crate::nbt::NbtTag;

#[repr(transparent)]
pub struct NbtComponentRef<'a>(&'a [u8]);

impl<'a> NbtComponentRef<'a> {
    /// Get the size of the [`NbtComponentRef`] from the given data,
    /// or an error if the data is invalid.
    pub(super) const fn size_of(data: &'a [u8]) -> Result<usize, NbtStreamError> {
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
                    if let Some((_, data)) = data.split_at_checked(2 + name_length) {
                        //

                        // Take the tag data from the data.
                        match NbtTagRef::size_of(data) {
                            Err(err) => return Err(err),
                            Ok(tag_length) => {
                                //

                                // Set the next tag and data.
                                if let Some((&next, remaining)) = data.split_first() {
                                    // Add the tag, name, and tag data to the result
                                    size += 1 + (2 + name_length) + tag_length;

                                    tag = next;
                                    loop_data = remaining;

                                    continue;
                                }
                            }
                        }
                    }
                }

                // Data ended unexpectedly.
                return Err(NbtStreamError::EndOfStream);
            }

            // Read full `NbtComponentRef`, return size.
            return Ok(size);
        }

        Err(NbtStreamError::EndOfStream)
    }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
pub struct NbtTagRef<'a>(&'a [u8]);

impl<'a> NbtTagRef<'a> {
    /// Get the size of the [`NbtTagRef`] from the given data,
    /// or an error if the data is invalid.
    const fn size_of(_data: &'a [u8]) -> Result<usize, NbtStreamError> { todo!() }
}
