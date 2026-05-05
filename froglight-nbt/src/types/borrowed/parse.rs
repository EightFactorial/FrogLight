#![allow(clippy::result_unit_err, reason = "WIP")]
#![expect(dead_code, reason = "WIP")]

use alloc::{collections::VecDeque, vec::Vec};
use core::range::Range;

use froglight_mutf8::prelude::MStr;

use crate::{
    prelude::*,
    types::borrowed::{reference::BorrowedIndex, value::BorrowedValueIndex},
};

#[allow(clippy::unnecessary_wraps, reason = "WIP")]
pub(super) fn parse_nbt_ref(root: &[u8], named: bool) -> Result<IndexedNbtRef<'_>, ()> {
    let (name, entries, indexes) = parse_nbt(root, named)?;

    // SAFETY: `entries` and `indexes` were created for `root`.
    Ok(unsafe { IndexedNbtRef::new_unchecked(root, name, entries, indexes) })
}

#[allow(clippy::unnecessary_wraps, reason = "WIP")]
pub(super) fn parse_nbt_mut(root: &mut [u8], named: bool) -> Result<IndexedNbtMut<'_>, ()> {
    let (name, entries, indexes) = parse_nbt(root, named)?;

    // SAFETY: `entries` and `indexes` were created for `root`.
    Ok(unsafe { IndexedNbtMut::new_unchecked(root, name, entries, indexes) })
}

#[expect(clippy::type_complexity, reason = "Returns multiple parsed components")]
fn parse_nbt(
    root: &[u8],
    named: bool,
) -> Result<(Option<BorrowedIndex<MStr>>, Vec<IndexedEntry>, Vec<Range<usize>>), ()> {
    let mut cursor = SliceCursor::new(root);
    let mut entries = Vec::new();
    let mut indexes = Vec::new();

    // All NBT starts with a compound tag
    if !matches!(cursor.next()?, NbtTagType::COMPOUND) {
        return Err(());
    }

    // Read a name if this is a named structure
    let name = named.then(|| read_string(&mut cursor)).transpose()?;

    // Prepare a queue of items to read
    let mut items = VecDeque::with_capacity(1);
    // Add the root compound to the queue
    items.push_back((cursor, true));

    // Read all items until the queue is empty
    while let Some((mut cursor, named)) = items.pop_front() {
        if named {
            read_compound(&mut cursor, &mut entries, &mut indexes, &mut items)?;
        } else {
            read_list(&mut cursor, &mut entries, &mut indexes, &mut items)?;
        }
    }

    Ok((name, entries, indexes))
}

fn read_compound<'data>(
    cursor: &mut SliceCursor<'data>,
    entries: &mut Vec<IndexedEntry>,
    indexes: &mut Vec<Range<usize>>,
    items: &mut VecDeque<(SliceCursor<'data>, bool)>,
) -> Result<(), ()> {
    let start = entries.len();

    while let Ok(tag) = cursor.next() {
        if tag == NbtTagType::END {
            break;
        }

        let name = read_string(cursor)?;

        match tag {
            NbtTagType::BYTE => {
                let value = read_byte(cursor)?;
                entries.push(IndexedEntry::new(name, BorrowedValueIndex::Byte(value)));
            }
            NbtTagType::SHORT => {
                let value = read_short(cursor)?;
                entries.push(IndexedEntry::new(name, BorrowedValueIndex::Short(value)));
            }
            NbtTagType::INT => {
                let value = read_int(cursor)?;
                entries.push(IndexedEntry::new(name, BorrowedValueIndex::Int(value)));
            }
            NbtTagType::LONG => {
                let value = read_long(cursor)?;
                entries.push(IndexedEntry::new(name, BorrowedValueIndex::Long(value)));
            }
            NbtTagType::FLOAT => {
                let value = read_float(cursor)?;
                entries.push(IndexedEntry::new(name, BorrowedValueIndex::Float(value)));
            }
            NbtTagType::DOUBLE => {
                let value = read_double(cursor)?;
                entries.push(IndexedEntry::new(name, BorrowedValueIndex::Double(value)));
            }
            NbtTagType::STRING => {
                let value = read_string(cursor)?;
                entries.push(IndexedEntry::new(name, BorrowedValueIndex::String(value)));
            }
            NbtTagType::BYTE_ARRAY => {
                let value = read_byte_array(cursor)?;
                entries.push(IndexedEntry::new(name, BorrowedValueIndex::ByteArray(value)));
            }
            NbtTagType::INT_ARRAY => {
                let value = read_int_array(cursor)?;
                entries.push(IndexedEntry::new(name, BorrowedValueIndex::IntArray(value)));
            }
            NbtTagType::LONG_ARRAY => {
                let value = read_long_array(cursor)?;
                entries.push(IndexedEntry::new(name, BorrowedValueIndex::LongArray(value)));
            }

            NbtTagType::LIST => {
                let index = unsafe { BorrowedIndex::new(cursor.position()) };

                items.push_back((cursor.clone(), false));
                read_until_list_end(cursor, 0)?;

                entries.push(IndexedEntry::new(name, BorrowedValueIndex::List(index)));
            }
            NbtTagType::COMPOUND => {
                let index = unsafe { BorrowedIndex::new(cursor.position()) };

                items.push_back((cursor.clone(), true));
                read_until_compound_end(cursor, 0)?;

                entries.push(IndexedEntry::new(name, BorrowedValueIndex::Compound(index)));
            }

            _ => return Err(()),
        }
    }

    indexes.push(Range { start, end: entries.len() });

    Ok(())
}

fn read_list<'data>(
    cursor: &mut SliceCursor<'data>,
    entries: &mut Vec<IndexedEntry>,
    indexes: &mut Vec<Range<usize>>,
    items: &mut VecDeque<(SliceCursor<'data>, bool)>,
) -> Result<(), ()> {
    let start = entries.len();

    let Ok(tag) = cursor.next() else { todo!() };
    let length = u32::from_be_bytes(cursor.take::<4>()?);

    let name = unsafe { BorrowedIndex::new(0) };

    for _ in 0..length {
        match tag {
            NbtTagType::BYTE => {
                let value = read_byte(cursor)?;
                entries.push(IndexedEntry::new(name, BorrowedValueIndex::Byte(value)));
            }
            NbtTagType::SHORT => {
                let value = read_short(cursor)?;
                entries.push(IndexedEntry::new(name, BorrowedValueIndex::Short(value)));
            }
            NbtTagType::INT => {
                let value = read_int(cursor)?;
                entries.push(IndexedEntry::new(name, BorrowedValueIndex::Int(value)));
            }
            NbtTagType::LONG => {
                let value = read_long(cursor)?;
                entries.push(IndexedEntry::new(name, BorrowedValueIndex::Long(value)));
            }
            NbtTagType::FLOAT => {
                let value = read_float(cursor)?;
                entries.push(IndexedEntry::new(name, BorrowedValueIndex::Float(value)));
            }
            NbtTagType::DOUBLE => {
                let value = read_double(cursor)?;
                entries.push(IndexedEntry::new(name, BorrowedValueIndex::Double(value)));
            }
            NbtTagType::STRING => {
                let value = read_string(cursor)?;
                entries.push(IndexedEntry::new(name, BorrowedValueIndex::String(value)));
            }
            NbtTagType::BYTE_ARRAY => {
                let value = read_byte_array(cursor)?;
                entries.push(IndexedEntry::new(name, BorrowedValueIndex::ByteArray(value)));
            }
            NbtTagType::INT_ARRAY => {
                let value = read_int_array(cursor)?;
                entries.push(IndexedEntry::new(name, BorrowedValueIndex::IntArray(value)));
            }
            NbtTagType::LONG_ARRAY => {
                let value = read_long_array(cursor)?;
                entries.push(IndexedEntry::new(name, BorrowedValueIndex::LongArray(value)));
            }

            NbtTagType::LIST => {
                let index = unsafe { BorrowedIndex::new(cursor.position()) };

                items.push_back((cursor.clone(), false));
                read_until_list_end(cursor, 0)?;

                entries.push(IndexedEntry::new(name, BorrowedValueIndex::List(index)));
            }
            NbtTagType::COMPOUND => {
                let index = unsafe { BorrowedIndex::new(cursor.position()) };

                items.push_back((cursor.clone(), true));
                read_until_compound_end(cursor, 0)?;

                entries.push(IndexedEntry::new(name, BorrowedValueIndex::Compound(index)));
            }

            _ => return Err(()),
        }
    }

    indexes.push(Range { start, end: entries.len() });

    Ok(())
}

// -------------------------------------------------------------------------------------------------

fn read_until_compound_end(cursor: &mut SliceCursor<'_>, depth: usize) -> Result<(), ()> {
    if depth >= 512 {
        return Err(());
    }

    loop {
        let next = cursor.next()?;

        if next == NbtTagType::END {
            return Ok(());
        }

        read_string(cursor)?;

        match next {
            NbtTagType::BYTE => {
                read_byte(cursor)?;
            }
            NbtTagType::SHORT => {
                read_short(cursor)?;
            }
            NbtTagType::INT => {
                read_int(cursor)?;
            }
            NbtTagType::LONG => {
                read_long(cursor)?;
            }
            NbtTagType::FLOAT => {
                read_float(cursor)?;
            }
            NbtTagType::DOUBLE => {
                read_double(cursor)?;
            }
            NbtTagType::BYTE_ARRAY => {
                read_byte_array(cursor)?;
            }
            NbtTagType::STRING => {
                read_string(cursor)?;
            }
            NbtTagType::INT_ARRAY => {
                read_int_array(cursor)?;
            }
            NbtTagType::LONG_ARRAY => {
                read_long_array(cursor)?;
            }

            NbtTagType::LIST => read_until_list_end(cursor, depth + 1)?,
            NbtTagType::COMPOUND => read_until_compound_end(cursor, depth + 1)?,

            _ => return Err(()),
        }
    }
}

fn read_until_list_end(cursor: &mut SliceCursor<'_>, depth: usize) -> Result<(), ()> {
    if depth >= 512 {
        return Err(());
    }

    let tag = cursor.next()?;
    let length = u32::from_be_bytes(cursor.take::<4>()?);

    for _ in 0..length {
        match tag {
            NbtTagType::BYTE => {
                read_byte(cursor)?;
            }
            NbtTagType::SHORT => {
                read_short(cursor)?;
            }
            NbtTagType::INT => {
                read_int(cursor)?;
            }
            NbtTagType::LONG => {
                read_long(cursor)?;
            }
            NbtTagType::FLOAT => {
                read_float(cursor)?;
            }
            NbtTagType::DOUBLE => {
                read_double(cursor)?;
            }
            NbtTagType::BYTE_ARRAY => {
                read_byte_array(cursor)?;
            }
            NbtTagType::STRING => {
                read_string(cursor)?;
            }
            NbtTagType::INT_ARRAY => {
                read_int_array(cursor)?;
            }
            NbtTagType::LONG_ARRAY => {
                read_long_array(cursor)?;
            }

            NbtTagType::LIST => read_until_list_end(cursor, depth + 1)?,
            NbtTagType::COMPOUND => read_until_compound_end(cursor, depth + 1)?,

            _ => return Err(()),
        }
    }

    Ok(())
}

// -------------------------------------------------------------------------------------------------

/// Parse a [`u8`] from the cursor.
///
/// # Errors
///
/// Returns an error if the cursor does not hold a byte.
#[inline]
fn read_byte(cursor: &mut SliceCursor<'_>) -> Result<BorrowedIndex<u8>, ()> {
    let position = cursor.position();
    let _ = cursor.next()?;

    // SAFETY: We just validated the cursor held a byte
    Ok(unsafe { BorrowedIndex::new(position) })
}

/// Parse a [`u16`] from the cursor.
///
/// # Errors
///
/// Returns an error if the cursor does not hold a short.
#[inline]
fn read_short(cursor: &mut SliceCursor<'_>) -> Result<BorrowedIndex<u16>, ()> {
    let position = cursor.position();
    let _ = cursor.take::<2>()?;

    // SAFETY: We just validated the cursor held a short
    Ok(unsafe { BorrowedIndex::new(position) })
}

/// Parse a [`u32`] from the cursor.
///
/// # Errors
///
/// Returns an error if the cursor does not hold an int.
#[inline]
fn read_int(cursor: &mut SliceCursor<'_>) -> Result<BorrowedIndex<u32>, ()> {
    let position = cursor.position();
    let _ = cursor.take::<4>()?;

    // SAFETY: We just validated the cursor held an int
    Ok(unsafe { BorrowedIndex::new(position) })
}

/// Parse a [`u64`] from the cursor.
///
/// # Errors
///
///
/// Returns an error if the cursor does not hold a long.
#[inline]
fn read_long(cursor: &mut SliceCursor<'_>) -> Result<BorrowedIndex<u64>, ()> {
    let position = cursor.position();
    let _ = cursor.take::<8>()?;

    // SAFETY: We just validated the cursor held a long
    Ok(unsafe { BorrowedIndex::new(position) })
}

/// Parse a [`f32`] from the cursor.
///
/// # Errors
///
/// Returns an error if the cursor does not hold a float.
#[inline]
fn read_float(cursor: &mut SliceCursor<'_>) -> Result<BorrowedIndex<f32>, ()> {
    let position = cursor.position();
    let _ = cursor.take::<4>()?;

    // SAFETY: We just validated the cursor held a float
    Ok(unsafe { BorrowedIndex::new(position) })
}

/// Parse a [`f64`] from the cursor.
///
/// # Errors
///
/// Returns an error if the cursor does not hold a double.
#[inline]
fn read_double(cursor: &mut SliceCursor<'_>) -> Result<BorrowedIndex<f64>, ()> {
    let position = cursor.position();
    let _ = cursor.take::<8>()?;

    // SAFETY: We just validated the cursor held a double
    Ok(unsafe { BorrowedIndex::new(position) })
}

/// Parse a string from the cursor.
///
/// # Errors
///
/// Returns an error if the string is invalid.
#[inline]
fn read_string(cursor: &mut SliceCursor<'_>) -> Result<BorrowedIndex<MStr>, ()> {
    let position = cursor.position();
    let length = u16::from_be_bytes(cursor.take::<2>()?);

    let contents = cursor.take_slice(usize::from(length))?;
    froglight_mutf8::types::str::MStr::from_mutf8(contents)?;

    // SAFETY: We just validated that cursor held a string
    Ok(unsafe { BorrowedIndex::new(position) })
}

/// Parse a byte array from the cursor.
///
/// # Errors
///
/// Returns an error if the byte array is invalid.
#[inline]
fn read_byte_array(cursor: &mut SliceCursor<'_>) -> Result<BorrowedIndex<[u8]>, ()> {
    let position = cursor.position();
    let length = u32::from_be_bytes(cursor.take::<4>()?);
    let length = usize::try_from(length).map_err(|_| ())?;
    let _ = cursor.take_slice(length * core::mem::size_of::<u8>())?;

    // SAFETY: We just validated that cursor held a byte array
    Ok(unsafe { BorrowedIndex::new(position) })
}

/// Parse an int array from the cursor.
///
/// # Errors
///
/// Returns an error if the int array is invalid.
#[inline]
fn read_int_array(cursor: &mut SliceCursor<'_>) -> Result<BorrowedIndex<[u32]>, ()> {
    let position = cursor.position();
    let length = u32::from_be_bytes(cursor.take::<4>()?);
    let length = usize::try_from(length).map_err(|_| ())?;
    let _ = cursor.take_slice(length * core::mem::size_of::<u32>())?;

    // SAFETY: We just validated that cursor held an int array
    Ok(unsafe { BorrowedIndex::new(position) })
}

/// Parse a long array from the cursor.
///
/// # Errors
///
/// Returns an error if the long array is invalid.
#[inline]
fn read_long_array(cursor: &mut SliceCursor<'_>) -> Result<BorrowedIndex<[u64]>, ()> {
    let position = cursor.position();
    let length = u32::from_be_bytes(cursor.take::<4>()?);
    let length = usize::try_from(length).map_err(|_| ())?;
    let _ = cursor.take_slice(length * core::mem::size_of::<u64>())?;

    // SAFETY: We just validated that cursor held a long array
    Ok(unsafe { BorrowedIndex::new(position) })
}

// -------------------------------------------------------------------------------------------------

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum NbtTagType {
    End,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    ByteArray,
    String,
    List,
    Compound,
    IntArray,
    LongArray,
}

#[rustfmt::skip]
#[allow(dead_code, reason = "Constants")]
impl NbtTagType {
    const END: u8 = 0;
    const BYTE: u8 = 1;
    const SHORT: u8 = 2;
    const INT: u8 = 3;
    const LONG: u8 = 4;
    const FLOAT: u8 = 5;
    const DOUBLE: u8 = 6;
    const BYTE_ARRAY: u8 = 7;
    const STRING: u8 = 8;
    const LIST: u8 = 9;
    const COMPOUND: u8 = 10;
    const INT_ARRAY: u8 = 11;
    const LONG_ARRAY: u8 = 12;

    /// Convert a byte to an [`NbtTagType`].
    const fn from_byte(byte: u8) -> Result<Self, ()> {
        match byte {
            0 => Ok(Self::End),
            1 => Ok(Self::Byte),
            2 => Ok(Self::Short),
            3 => Ok(Self::Int),
            4 => Ok(Self::Long),
            5 => Ok(Self::Float),
            6 => Ok(Self::Double),
            7 => Ok(Self::ByteArray),
            8 => Ok(Self::String),
            9 => Ok(Self::List),
            10 => Ok(Self::Compound),
            11 => Ok(Self::IntArray),
            12 => Ok(Self::LongArray),
            _ => Err(()),
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
struct SliceCursor<'data> {
    input: &'data [u8],
    position: usize,
}

impl<'data> SliceCursor<'data> {
    /// Create a new [`SliceCursor`].
    #[inline]
    #[must_use]
    const fn new(input: &'data [u8]) -> Self { Self { input, position: 0 } }

    /// Get the next byte from the input, advancing the cursor.
    #[inline]
    fn next(&mut self) -> Result<u8, ()> {
        self.input.get(self.position).copied().map_or_else(
            || Err(()),
            |byte| {
                self.position += 1;
                Ok(byte)
            },
        )
    }

    /// Get the next slice of bytes from the input, advancing the cursor.
    #[inline]
    fn take_slice(&mut self, length: usize) -> Result<&'data [u8], ()> {
        self.input.get(self.position..self.position + length).map_or_else(
            || Err(()),
            |slice| {
                self.position += length;
                Ok(slice)
            },
        )
    }

    /// Get the next N bytes from the input, advancing the cursor.
    #[inline]
    fn take<const N: usize>(&mut self) -> Result<[u8; N], ()> {
        // SAFETY: `slice` is guaranteed to have length `N`.
        self.take_slice(N).map(|slice| unsafe { slice.try_into().unwrap_unchecked() })
    }

    /// Get the next byte from the input without advancing the cursor.
    #[inline]
    #[must_use]
    fn peek(&self) -> Option<u8> { self.input.get(self.position).copied() }

    /// Get the number of bytes consumed by the cursor.
    #[inline]
    #[must_use]
    fn position(&self) -> usize { self.position }

    /// Set the cursor's position.
    ///
    /// # Errors
    ///
    /// Returns an error if the position is out of bounds.
    fn set_position(&mut self, position: usize) -> Result<(), ()> {
        if position <= self.input.len() {
            self.position = position;
            Ok(())
        } else {
            Err(())
        }
    }
}
