#![allow(clippy::result_unit_err, dead_code, reason = "WIP")]

use alloc::{collections::VecDeque, vec::Vec};
use core::range::Range;

use froglight_mutf8::prelude::MStr;

use crate::types::indexed::{
    IndexedNbt,
    alloc::SliceCore,
    core::{Mut, Ref},
    index::{EntryIndex, Index, ValueIndex},
    types::{IndexedListType, IndexedMapType},
};

pub(crate) fn parse_nbt_ref(
    root: &[u8],
    named: bool,
) -> Result<IndexedNbt<'_, Ref, SliceCore<'_, Ref>>, ()> {
    let (length, name, entries, ranges) = parse_nbt(root, named)?;

    // SAFETY: `slice` is a subslice of `root`.
    // SAFETY: `entries` and `ranges` were created using `root`.
    unsafe {
        let slice = root.get_unchecked(..length);
        let core = SliceCore::new(slice, entries, ranges);
        Ok(IndexedNbt::<Ref, SliceCore<'_, Ref>>::new_core(core, name))
    }
}

pub(crate) fn parse_nbt_mut(
    root: &mut [u8],
    named: bool,
) -> Result<IndexedNbt<'_, Mut, SliceCore<'_, Mut>>, ()> {
    let (length, name, entries, ranges) = parse_nbt(root, named)?;

    // SAFETY: `slice` is a subslice of `root`.
    // SAFETY: `entries` and `ranges` were created using `root`.
    unsafe {
        let slice = root.get_unchecked_mut(..length);
        let core = SliceCore::new(slice, entries, ranges);
        Ok(IndexedNbt::<Mut, SliceCore<'_, Mut>>::new_core(core, name))
    }
}

#[expect(clippy::type_complexity, reason = "Returns multiple parsed components")]
fn parse_nbt(
    root: &[u8],
    named: bool,
) -> Result<(usize, Option<Index<MStr>>, Vec<EntryIndex>, Vec<Range<usize>>), ()> {
    let mut cursor = Cursor::new(root);
    let mut entries = Vec::new();
    let mut indexes = Vec::new();

    // All NBT starts with a compound tag
    if !matches!(cursor.next()?, COMPOUND) {
        return Err(());
    }

    // Read a name if this is a named structure
    let name = named.then(|| read_string(&mut cursor)).transpose()?;

    // Prepare a queue of items to read
    let mut counter = 1;
    let mut queue = VecDeque::with_capacity(1);

    // Add the root compound to the queue
    queue.push_back((cursor, true));

    // Process the queue until it's empty
    while let Some((cursor, named)) = queue.pop_front() {
        if named {
            parse_item::<true>(cursor, &mut entries, &mut indexes, &mut counter, &mut queue)?;
        } else {
            parse_item::<false>(cursor, &mut entries, &mut indexes, &mut counter, &mut queue)?;
        }
    }

    // TODO: Shrink to exact size.
    let length = root.len();
    Ok((length, name, entries, indexes))
}

#[inline]
#[allow(clippy::too_many_lines, reason = "Handles both compounds and lists")]
fn parse_item<'data, const NAMED: bool>(
    mut cursor: Cursor<'data>,
    entries: &mut Vec<EntryIndex>,
    ranges: &mut Vec<Range<usize>>,

    counter: &mut usize,
    queue: &mut VecDeque<(Cursor<'data>, bool)>,
) -> Result<(), ()> {
    let start = entries.len();

    if NAMED {
        loop {
            let tag = cursor.next()?;
            if tag == END {
                break;
            }

            let name = read_string(&mut cursor)?;

            match tag {
                BYTE => {
                    let value = read_byte(&mut cursor)?;
                    entries.push(EntryIndex::new(name, ValueIndex::Byte(value)));
                }
                SHORT => {
                    let value = read_short(&mut cursor)?;
                    entries.push(EntryIndex::new(name, ValueIndex::Short(value)));
                }
                INT => {
                    let value = read_int(&mut cursor)?;
                    entries.push(EntryIndex::new(name, ValueIndex::Int(value)));
                }
                LONG => {
                    let value = read_long(&mut cursor)?;
                    entries.push(EntryIndex::new(name, ValueIndex::Long(value)));
                }
                FLOAT => {
                    let value = read_float(&mut cursor)?;
                    entries.push(EntryIndex::new(name, ValueIndex::Float(value)));
                }
                DOUBLE => {
                    let value = read_double(&mut cursor)?;
                    entries.push(EntryIndex::new(name, ValueIndex::Double(value)));
                }
                BYTE_ARRAY => {
                    let value = read_byte_array(&mut cursor)?;
                    entries.push(EntryIndex::new(name, ValueIndex::ByteArray(value)));
                }
                STRING => {
                    let value = read_string(&mut cursor)?;
                    entries.push(EntryIndex::new(name, ValueIndex::String(value)));
                }
                LIST => {
                    let value = read_list(&mut cursor, counter, queue)?;
                    entries.push(EntryIndex::new(name, ValueIndex::List(value)));
                }
                COMPOUND => {
                    let value = read_compound(&mut cursor, counter, queue)?;
                    entries.push(EntryIndex::new(name, ValueIndex::Compound(value)));
                }
                INT_ARRAY => {
                    let value = read_int_array(&mut cursor)?;
                    entries.push(EntryIndex::new(name, ValueIndex::IntArray(value)));
                }
                LONG_ARRAY => {
                    let value = read_long_array(&mut cursor)?;
                    entries.push(EntryIndex::new(name, ValueIndex::LongArray(value)));
                }
                _ => return Err(()),
            }
        }
    } else {
        /// An empty, placeholder name for unnamed entries.
        const NAME: Index<MStr> = Index::new(0);

        let tag = cursor.next()?;
        let length = u32::from_be_bytes(cursor.next_arr::<4>()?);

        for _ in 0..length {
            match tag {
                // Primitive types are calculated from the list's index, so skip them.
                BYTE => {
                    read_byte(&mut cursor)?;
                }
                SHORT => {
                    read_short(&mut cursor)?;
                }
                INT => {
                    read_int(&mut cursor)?;
                }
                LONG => {
                    read_long(&mut cursor)?;
                }
                FLOAT => {
                    read_float(&mut cursor)?;
                }
                DOUBLE => {
                    read_double(&mut cursor)?;
                }

                BYTE_ARRAY => {
                    let value = read_byte_array(&mut cursor)?;
                    entries.push(EntryIndex::new(NAME, ValueIndex::ByteArray(value)));
                }
                STRING => {
                    let value = read_string(&mut cursor)?;
                    entries.push(EntryIndex::new(NAME, ValueIndex::String(value)));
                }
                COMPOUND => {
                    let value = read_compound(&mut cursor, counter, queue)?;
                    entries.push(EntryIndex::new(NAME, ValueIndex::Compound(value)));
                }
                LIST => {
                    let value = read_list(&mut cursor, counter, queue)?;
                    entries.push(EntryIndex::new(NAME, ValueIndex::List(value)));
                }
                INT_ARRAY => {
                    let value = read_int_array(&mut cursor)?;
                    entries.push(EntryIndex::new(NAME, ValueIndex::IntArray(value)));
                }
                LONG_ARRAY => {
                    let value = read_long_array(&mut cursor)?;
                    entries.push(EntryIndex::new(NAME, ValueIndex::LongArray(value)));
                }
                _ => return Err(()),
            }
        }
    }

    ranges.push(Range { start, end: entries.len() });

    Ok(())
}

// -------------------------------------------------------------------------------------------------

#[inline]
fn read_byte(cursor: &mut Cursor<'_>) -> Result<Index<u8>, ()> {
    let position = cursor.pos();
    let _ = cursor.next_arr::<1>()?;
    Ok(Index::new(position))
}

#[inline]
fn read_short(cursor: &mut Cursor<'_>) -> Result<Index<u16>, ()> {
    let position = cursor.pos();
    let _ = cursor.next_arr::<2>()?;
    Ok(Index::new(position))
}

#[inline]
fn read_int(cursor: &mut Cursor<'_>) -> Result<Index<u32>, ()> {
    let position = cursor.pos();
    let _ = cursor.next_arr::<4>()?;
    Ok(Index::new(position))
}

#[inline]
fn read_long(cursor: &mut Cursor<'_>) -> Result<Index<u64>, ()> {
    let position = cursor.pos();
    let _ = cursor.next_arr::<8>()?;
    Ok(Index::new(position))
}

#[inline]
fn read_float(cursor: &mut Cursor<'_>) -> Result<Index<f32>, ()> {
    let position = cursor.pos();
    let _ = cursor.next_arr::<4>()?;
    Ok(Index::new(position))
}

#[inline]
fn read_double(cursor: &mut Cursor<'_>) -> Result<Index<f64>, ()> {
    let position = cursor.pos();
    let _ = cursor.next_arr::<8>()?;
    Ok(Index::new(position))
}

#[inline]
fn read_byte_array(cursor: &mut Cursor<'_>) -> Result<Index<[u8]>, ()> {
    let position = cursor.pos();

    let length_bytes = cursor.next_arr::<4>()?;
    let length = usize::try_from(u32::from_be_bytes(length_bytes)).map_err(|_| ())?;
    let _ = cursor.next_slice(length)?;

    Ok(Index::new(position))
}

#[inline]
fn read_int_array(cursor: &mut Cursor<'_>) -> Result<Index<[u32]>, ()> {
    let position = cursor.pos();

    let length_bytes = cursor.next_arr::<4>()?;
    let length = usize::try_from(u32::from_be_bytes(length_bytes)).map_err(|_| ())?;
    let _ = cursor.next_slice(length * 4)?;

    Ok(Index::new(position))
}

#[inline]
fn read_long_array(cursor: &mut Cursor<'_>) -> Result<Index<[u64]>, ()> {
    let position = cursor.pos();

    let length_bytes = cursor.next_arr::<4>()?;
    let length = usize::try_from(u32::from_be_bytes(length_bytes)).map_err(|_| ())?;
    let _ = cursor.next_slice(length * 8)?;

    Ok(Index::new(position))
}

#[inline]
fn read_string(cursor: &mut Cursor<'_>) -> Result<Index<MStr>, ()> {
    let position = cursor.pos();

    let length_bytes = cursor.next_arr::<2>()?;
    let length = usize::from(u16::from_be_bytes(length_bytes));

    let content = cursor.next_slice(length)?;
    MStr::from_mutf8(content)?;

    Ok(Index::new(position))
}

#[inline]
fn read_compound<'data>(
    cursor: &mut Cursor<'data>,
    counter: &mut usize,
    queue: &mut VecDeque<(Cursor<'data>, bool)>,
) -> Result<Index<IndexedMapType>, ()> {
    let index = Index::new(*counter);
    *counter += 1;

    queue.push_back((cursor.clone(), true));
    skip_item::<true>(cursor, 0)?;

    Ok(index)
}

#[inline]
fn read_list<'data>(
    cursor: &mut Cursor<'data>,
    counter: &mut usize,
    queue: &mut VecDeque<(Cursor<'data>, bool)>,
) -> Result<Index<IndexedListType>, ()> {
    // Reserve the highest bit for list types
    const UNRESERVED_BITS: usize = usize::BITS as usize - 1;
    const RESERVED_BITS: usize = 1 << UNRESERVED_BITS;
    const HIGHEST_INDEX: usize = RESERVED_BITS - 1;

    let tag = cursor.peek()?;
    let index = match tag {
        // Use the cursor's position as the index for primitive types
        BYTE | SHORT | INT | LONG | FLOAT | DOUBLE => {
            // Fail to parse if we've exhausted all data indexes somehow
            if cursor.pos() >= HIGHEST_INDEX {
                return Err(());
            }

            Index::new(cursor.pos())
        }
        // Otherwise, assign a range index and parse it later
        _ => {
            // Fail to parse if we've exhausted all range indexes somehow
            if *counter >= HIGHEST_INDEX {
                return Err(());
            }

            // Index + set the highest bit
            let index = Index::<IndexedListType>::new(*counter | RESERVED_BITS);
            *counter += 1;

            queue.push_back((cursor.clone(), false));
            index
        }
    };

    skip_item::<false>(cursor, 0)?;

    // Store the tag in the highest bits of the index for later retrieval
    Ok(index)
}

#[inline]
fn skip_item<const NAMED: bool>(cursor: &mut Cursor<'_>, depth: usize) -> Result<(), ()> {
    if depth > 512 {
        return Err(());
    }

    if NAMED {
        loop {
            let tag = cursor.next()?;
            if tag == END {
                break;
            }

            read_string(cursor)?;

            match tag {
                BYTE => {
                    read_byte(cursor)?;
                }
                SHORT => {
                    read_short(cursor)?;
                }
                INT => {
                    read_int(cursor)?;
                }
                LONG => {
                    read_long(cursor)?;
                }
                FLOAT => {
                    read_float(cursor)?;
                }
                DOUBLE => {
                    read_double(cursor)?;
                }
                BYTE_ARRAY => {
                    read_byte_array(cursor)?;
                }
                STRING => {
                    read_string(cursor)?;
                }
                COMPOUND => {
                    skip_item::<true>(cursor, depth + 1)?;
                }
                LIST => {
                    skip_item::<false>(cursor, depth + 1)?;
                }
                INT_ARRAY => {
                    read_int_array(cursor)?;
                }
                LONG_ARRAY => {
                    read_long_array(cursor)?;
                }
                _ => return Err(()),
            }
        }
    } else {
        let tag = cursor.next()?;
        let length = u32::from_be_bytes(cursor.next_arr::<4>()?);

        for _ in 0..length {
            match tag {
                BYTE => {
                    read_byte(cursor)?;
                }
                SHORT => {
                    read_short(cursor)?;
                }
                INT => {
                    read_int(cursor)?;
                }
                LONG => {
                    read_long(cursor)?;
                }
                FLOAT => {
                    read_float(cursor)?;
                }
                DOUBLE => {
                    read_double(cursor)?;
                }
                BYTE_ARRAY => {
                    read_byte_array(cursor)?;
                }
                STRING => {
                    read_string(cursor)?;
                }
                COMPOUND => {
                    skip_item::<true>(cursor, depth + 1)?;
                }
                LIST => {
                    skip_item::<false>(cursor, depth + 1)?;
                }
                INT_ARRAY => {
                    read_int_array(cursor)?;
                }
                LONG_ARRAY => {
                    read_long_array(cursor)?;
                }
                _ => return Err(()),
            }
        }
    }

    Ok(())
}

// -------------------------------------------------------------------------------------------------

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

// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
struct Cursor<'a> {
    data: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    /// Create a new [`Cursor`] over the given byte slice.
    #[inline]
    #[must_use]
    const fn new(data: &'a [u8]) -> Self { Self { data, position: 0 } }

    /// Get the current position of the cursor.
    #[inline]
    #[must_use]
    const fn pos(&self) -> usize { self.position }

    /// Read the next byte without advancing the cursor.
    #[inline]
    fn peek(&self) -> Result<u8, ()> { self.data.get(self.position).copied().ok_or(()) }

    /// Read the next byte from the cursor.
    #[inline]
    fn next(&mut self) -> Result<u8, ()> {
        let byte = self.data.get(self.position).ok_or(())?;
        self.position += 1;
        Ok(*byte)
    }

    /// Read the next N bytes from the cursor.
    #[inline]
    fn next_arr<const N: usize>(&mut self) -> Result<[u8; N], ()> {
        // SAFETY: `next_slice` always returns the correct length slice.
        self.next_slice(N).map(|s| unsafe { s.try_into().unwrap_unchecked() })
    }

    /// Read the next N bytes from the cursor.
    #[inline]
    fn next_slice(&mut self, n: usize) -> Result<&'a [u8], ()> {
        let slice = self.data.get(self.position..self.position + n).ok_or(())?;
        self.position += n;
        Ok(slice)
    }
}
