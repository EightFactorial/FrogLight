#![allow(clippy::result_unit_err, dead_code, reason = "WIP")]

use alloc::{collections::VecDeque, vec::Vec};
use core::range::Range;

use froglight_mutf8::prelude::MStr;

use crate::types::indexed::{
    IndexedNbt,
    alloc::SliceCore,
    core::{Mut, Ref},
    index::{EntryIndex, Index},
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
#[expect(clippy::ptr_arg, unused, reason = "WIP")]
fn parse_item<const NAMED: bool>(
    cursor: Cursor<'_>,
    entries: &mut Vec<EntryIndex>,
    ranges: &mut Vec<Range<usize>>,

    counter: &mut usize,
    queue: &mut VecDeque<(Cursor<'_>, bool)>,
) -> Result<(), ()> {
    todo!()
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

// -------------------------------------------------------------------------------------------------

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
