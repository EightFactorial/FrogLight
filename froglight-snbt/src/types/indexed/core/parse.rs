use alloc::vec::Vec;
use core::range::Range;

use memchr::Memchr2;
use smallvec::SmallVec;

use crate::types::indexed::{IndexedSnbt, core::StrCore, index::EntryIndex};

pub(crate) fn parse_snbt(root: &str) -> Result<IndexedSnbt<'_, StrCore<'_>>, ()> {
    let compounds = get_item_bounds(root.as_bytes(), b'{', b'}')?;
    let lists = get_item_bounds(root.as_bytes(), b'[', b']')?;

    let mut entries = Vec::with_capacity(compounds.len() + lists.len());
    let mut ranges = Vec::with_capacity(compounds.len() + lists.len());

    let mut counter = 0;
    for index in 0..compounds.len() {
        unsafe {
            parse_item::<true>(
                root,
                index,
                &compounds,
                &lists,
                &mut entries,
                &mut ranges,
                &mut counter,
            )?;
        }
    }
    for index in 0..lists.len() {
        unsafe {
            parse_item::<false>(
                root,
                index,
                &compounds,
                &lists,
                &mut entries,
                &mut ranges,
                &mut counter,
            )?;
        }
    }

    // SAFETY: `entries` and `ranges` were created from `root`.
    Ok(IndexedSnbt::new(unsafe { StrCore::new(root, entries, ranges) }))
}

/// # SAFETY
///
/// The caller must ensure:
///   - If `NAMED` is `true`, `index` must be a valid index into `compounds`.
///   - If `NAMED` is `false`, `index` must be a valid index into `lists`.
///   - All pairs of `{` and `}` must have indices in `compounds`.
///   - All pairs of `[` and `]` must have indices in `lists`.
#[inline]
#[allow(clippy::unnecessary_wraps, reason = "WIP")]
unsafe fn parse_item<const NAMED: bool>(
    root: &str,
    index: usize,
    compounds: &[Range<usize>],
    lists: &[Range<usize>],

    _entries: &mut Vec<EntryIndex>,
    _ranges: &mut Vec<Range<usize>>,
    _counter: &mut usize,
) -> Result<(), ()> {
    // SAFETY: The caller ensures this is safe.
    let (range, slice) = unsafe {
        let range =
            if NAMED { *compounds.get_unchecked(index) } else { *lists.get_unchecked(index) };
        let slice = root.get_unchecked(range);
        (range, slice)
    };

    // TODO: Custom iterator that returns string slices *and* their byte ranges.
    let entry_iter = slice.split({
        let mut skip = 0;
        let mut index = range.start;

        let mut escaped = false;
        move |char| {
            index += 1;
            if skip >= index {
                return false;
            }

            match char {
                // Return `true` for non-escaped commas
                ',' if !escaped => true,
                // Set `escaped` for non-escaped backslashes
                '\\' if !escaped => {
                    escaped = true;
                    false
                }
                // Skip over nested compounds
                '{' if !escaped => {
                    // SAFETY: There is guaranteed to be a compound starting at `index`.
                    let compound =
                        unsafe { compounds.iter().find(|r| r.start == index).unwrap_unchecked() };
                    skip = compound.end;
                    false
                }
                // Skip over nested lists
                '[' if !escaped => {
                    // SAFETY: There is guaranteed to be a list starting at `index`.
                    let list =
                        unsafe { lists.iter().find(|r| r.start == index).unwrap_unchecked() };
                    skip = list.end;
                    false
                }
                _ => {
                    escaped = false;
                    false
                }
            }
        }
    });

    for _entry in entry_iter {}

    Ok(())
}

// -------------------------------------------------------------------------------------------------

fn get_item_bounds(root: &[u8], start: u8, end: u8) -> Result<Vec<Range<usize>>, ()> {
    let mut bounds = Vec::with_capacity(2);

    // Track the indices of `start` characters
    let mut history = SmallVec::<[usize; 8]>::new();

    for index in Memchr2::new(start, end, root) {
        // Skip if the previous character was a backslash.
        if let Some(previous) = index.checked_sub(1).and_then(|i| root.get(i))
            && *previous == b'\\'
        {
            continue;
        }

        // SAFETY: `index` is guaranteed to be within bounds.
        let item = unsafe { *root.get_unchecked(index) };

        if item == start {
            // Store the `start` index for later.
            history.push(index + 1);
        } else if item == end {
            // Take the last `start` index and pair it with the current index.
            if let Some(start) = history.pop() {
                bounds.push(Range { start, end: index });
            } else {
                return Err(());
            }
        } else {
            #[cfg(debug_assertions)]
            unreachable!("An unexpected character was found during parsing: {item}");

            // SAFETY: `item` is guaranteed to be either `start` or `end`.
            #[cfg(not(debug_assertions))]
            unsafe {
                core::hint::unreachable_unchecked()
            }
        }
    }

    // Ensure all `start` characters have been matched with an `end` character.
    if history.is_empty() { Ok(bounds) } else { Err(()) }
}
