use alloc::{collections::VecDeque, string::String, vec::Vec};
use core::range::Range;

use memchr::{Memchr, Memchr2};
use smallvec::SmallVec;

mod cursor;
use cursor::Cursor;

mod compound;
mod list;
mod value;

use crate::types::indexed::{
    IndexedSnbt,
    core::SliceCore,
    entry::{EntryIndex, ValueIndex},
    index::{
        Index,
        string::{StringDescription, StringQuotes},
    },
};

pub(super) fn parse_snbt(mut root: &str) -> Result<IndexedSnbt<SliceCore<'_>>, ()> {
    let mut strings = string_ranges(root)?;
    let mut compounds = create_ranges::<true>(b'{', b'}', root, &strings)?;
    let mut lists = create_ranges::<false>(b'[', b']', root, &strings)?;
    retain_ranges(&mut compounds, &mut lists, &mut strings);

    // Resize `root` to just the outer-most compound.
    // SAFETY: `compounds` is guaranteed to have at least one element, the root.
    root = unsafe { root.get_unchecked(*compounds.first().unwrap_unchecked()) };

    // Create a list of entries and a queue of cursors to process.
    let mut entries = Vec::with_capacity(4);
    let mut queue = VecDeque::<(Cursor<'_>, usize, bool)>::with_capacity(4);

    // Push the root compound into the queue.
    entries.push(EntryIndex::new(NULL_STRING, PLACEHOLDER_COMPOUND));
    queue.push_front((Cursor::new(root), 0, true));

    while let Some((cursor, callback, is_compound)) = queue.pop_back() {
        let index = if is_compound {
            compound::parse(cursor, &compounds, &lists, &strings, &mut entries, &mut queue)?
        } else {
            list::parse(cursor, &compounds, &lists, &strings, &mut entries, &mut queue)?
        };

        // SAFETY: `callback` is guaranteed to be a valid index into `entries`.
        let callback = unsafe { entries.get_unchecked_mut(callback) };

        // Overwrite the placeholder value with the correct index/range.
        if is_compound {
            debug_assert!(matches!(callback.value(), ValueIndex::Compound(_)));
            *callback = EntryIndex::new(callback.name(), index);
        } else {
            debug_assert!(matches!(callback.value(), ValueIndex::List(_)));
            *callback = EntryIndex::new(callback.name(), index);
        }
    }

    // Check that no placeholder values remain.
    #[cfg(debug_assertions)]
    for entry in &entries {
        debug_assert_ne!(entry.value().range(), Range { start: 0, end: 0 });
    }

    Ok(IndexedSnbt::new(unsafe { SliceCore::new(root, entries.into_boxed_slice()) }))
}

/// A placeholder [`Index<IndexedMapType>`], must be replaced later with the
/// correct range.
const PLACEHOLDER_COMPOUND: ValueIndex =
    ValueIndex::Compound(unsafe { Index::new(Range { start: 0, end: 0 }, ()) });

/// A placeholder [`Index<IndexedListType>`], must be replaced later with the
/// correct range.
const PLACEHOLDER_LIST: ValueIndex =
    ValueIndex::List(unsafe { Index::new(Range { start: 0, end: 0 }, ()) });

/// A null [`Index<String>`], must only be used for entries without a name.
const NULL_STRING: Index<String> =
    unsafe { Index::new(Range { start: 0, end: 0 }, StringDescription::new(StringQuotes::None)) };

// -------------------------------------------------------------------------------------------------

#[inline(always)]
#[expect(clippy::inline_always, reason = "Performance")]
fn retain_ranges(
    compounds: &mut Vec<Range<usize>>,
    lists: &mut Vec<Range<usize>>,
    strings: &mut Vec<Range<usize>>,
) {
    // Remove any ranges not contained within the first compound.
    let first = unsafe { *compounds.first().unwrap_unchecked() };

    compounds.retain(|r| first.start <= r.start && r.end <= first.end);
    lists.retain(|r| first.start <= r.start && r.end <= first.end);
    strings.retain(|r| first.start <= r.start && r.end <= first.end);
}

#[inline(always)]
#[expect(clippy::inline_always, reason = "Performance")]
fn string_ranges(root: &str) -> Result<Vec<Range<usize>>, ()> {
    let mut ranges = Vec::with_capacity(8);
    let mut last = Option::<usize>::None;

    for index in Memchr::new(b'\"', root.as_bytes()) {
        // Check if the quote is escaped
        if let Some(char) = index.checked_sub(1).and_then(|i| root.as_bytes().get(i))
            && *char == b'\\'
        {
            continue;
        }

        if let Some(last) = last.take() {
            // If we have a last index, this is the end of a string range
            ranges.push(Range { start: last, end: index + 1 });
        } else {
            // Otherwise start a new string range
            last = Some(index);
        }
    }

    // TODO: Error type
    if last.is_some() {
        return Err(());
    }

    for index in Memchr::new(b'\'', root.as_bytes()) {
        // Check if the quote is escaped
        if let Some(char) = index.checked_sub(1).and_then(|i| root.as_bytes().get(i))
            && *char == b'\\'
        {
            continue;
        }

        if let Some(last) = last.take() {
            // If we have a last index, this is the end of a string range
            ranges.push(Range { start: last, end: index + 1 });
        } else {
            // Otherwise start a new string range
            last = Some(index);
        }
    }

    // TODO: Error type
    if last.is_some() {
        return Err(());
    }

    // Sort the ranges by their start index.
    ranges.sort_unstable_by_key(|r| r.start);

    Ok(ranges)
}

#[inline(always)]
#[expect(clippy::inline_always, reason = "Performance")]
fn create_ranges<const REQUIRED: bool>(
    char_start: u8,
    char_end: u8,
    root: &str,
    strings: &[Range<usize>],
) -> Result<Vec<Range<usize>>, ()> {
    let mut ranges = Vec::with_capacity(8);
    let mut queue = SmallVec::<[usize; 8]>::new_const();

    let mut string_skip = 0usize;
    for index in Memchr2::new(char_start, char_end, root.as_bytes()) {
        // Skip if the index is within a string range.
        // This can be done since we're reading left-to-right.
        if let Some(string) =
            strings.iter().skip(string_skip).position(|range| range.contains(&index))
        {
            string_skip = string;
            continue;
        }

        let char = unsafe { *root.as_bytes().get_unchecked(index) };
        if char == char_start {
            // Start a new compound range
            queue.push(index);
        } else if char == char_end
            && let Some(start) = queue.pop()
        {
            // End the current compound range
            ranges.push(Range { start, end: index });
        } else {
            // TODO: Error type
            return Err(());
        }
    }

    // TODO: Error type
    if !queue.is_empty() {
        return Err(());
    }

    // TODO: Error type
    if REQUIRED && ranges.is_empty() {
        return Err(());
    }

    // Sort the ranges by their start index.
    ranges.sort_unstable_by_key(|r| r.start);

    Ok(ranges)
}
