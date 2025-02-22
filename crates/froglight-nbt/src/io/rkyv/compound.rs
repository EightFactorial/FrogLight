use crate::{mutf8::Mutf8Str, nbt::NbtTag};

/// An unsized representation of an [`NbtCompound`].
#[repr(transparent)]
pub struct ArchivedNbtCompound<'a>(Vec<(&'a Mutf8Str, ArchivedNbtTag<'a>)>);

impl<'a> ArchivedNbtCompound<'a> {
    /// Try to create a new [`UnsizedNbtCompound`] from a byte slice,
    /// returning the compound and remaining bytes.
    ///
    /// Returns `None` if the bytes are invalid.
    pub fn try_from_bytes(bytes: &'a [u8]) -> Option<(Self, &'a [u8])> {
        let (&tag, mut bytes) = bytes.split_first()?;
        let mut tag = tag;

        let mut compound = Vec::new();
        while tag != NbtTag::END {
            // Get the name and check if it's valid
            let name = Mutf8Str::from_bytes(bytes);
            if !name.is_valid() {
                return None;
            };

            // Get the tag data and remaining bytes
            let (data, rem) = ArchivedNbtTag::try_from_tag_bytes(tag, bytes)?;
            bytes = rem;

            compound.push((name, data));

            // Get the next tag
            let (&next, rem) = bytes.split_first()?;
            tag = next;
            bytes = rem;
        }

        Some((Self(compound), bytes))
    }

    fn try_size_of(bytes: &'a [u8], validate: bool) -> Option<usize> { todo!() }
}

// -------------------------------------------------------------------------------------------------

pub struct ArchivedNbtTag<'a>(u8, &'a [u8]);

impl<'a> ArchivedNbtTag<'a> {
    /// Try to create a new [`ArchivedNbtTag`] from a byte slice,
    /// returning the tag and the remaining bytes.
    pub fn try_from_bytes(bytes: &'a [u8]) -> Option<(Self, &'a [u8])> {
        let (&tag, data) = bytes.split_first()?;
        Self::try_from_tag_bytes(tag, data)
    }

    #[inline]
    fn try_from_tag_bytes(tag: u8, bytes: &'a [u8]) -> Option<(Self, &'a [u8])> {
        let size = Self::try_size_of(tag, bytes, true)?;
        bytes.split_at_checked(size).map(|(data, rem)| (Self(tag, data), rem))
    }

    fn try_size_of(tag: u8, bytes: &'a [u8], validate: bool) -> Option<usize> {
        match tag {
            NbtTag::BYTE => Some(1),
            NbtTag::SHORT => Some(2),
            NbtTag::INT | NbtTag::FLOAT => Some(4),
            NbtTag::LONG | NbtTag::DOUBLE => Some(8),
            NbtTag::STRING => {
                let result: usize = 2usize
                    + usize::try_from(u16::from_be_bytes(*bytes.first_chunk::<2>()?)).ok()?;

                // Check the string if validation is enabled and it's not empty
                if (validate && result > 2)
                    && Mutf8Str::from_bytes(bytes.get(2..result)?).is_valid()
                {
                    None
                } else {
                    Some(result)
                }
            }
            NbtTag::LIST => {
                let (&tag, remaining) = bytes.split_first()?;
                ArchivedNbtListTag::try_size_of(tag, remaining, validate)
            }
            NbtTag::COMPOUND => ArchivedNbtCompound::try_size_of(bytes, validate),
            NbtTag::BYTE_ARRAY => todo!(),
            NbtTag::INT_ARRAY => todo!(),
            NbtTag::LONG_ARRAY => todo!(),
            _ => None,
        }
    }
}

// -------------------------------------------------------------------------------------------------

pub struct ArchivedNbtListTag<'a>(u8, &'a [u8]);

impl<'a> ArchivedNbtListTag<'a> {
    fn try_size_of(tag: u8, bytes: &'a [u8], validate: bool) -> Option<usize> {
        let length: usize = u32::from_be_bytes(*bytes.first_chunk::<4>()?).try_into().ok()?;
        match tag {
            NbtTag::END => Some(4),
            NbtTag::BYTE => Some(4 + (1 * length)),
            NbtTag::SHORT => Some(4 + (2 * length)),
            NbtTag::INT | NbtTag::FLOAT => Some(4 + (4 * length)),
            NbtTag::LONG | NbtTag::DOUBLE => Some(4 + (8 * length)),
            NbtTag::LIST => todo!(),
            NbtTag::COMPOUND => todo!(),
            NbtTag::BYTE_ARRAY => todo!(),
            NbtTag::INT_ARRAY => todo!(),
            NbtTag::LONG_ARRAY => todo!(),
            _ => None,
        }
    }
}
