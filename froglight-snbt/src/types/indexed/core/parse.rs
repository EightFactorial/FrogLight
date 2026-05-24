use alloc::vec::Vec;
use core::range::Range;

use memchr::{Memchr, Memchr2, Memchr3};
use smallvec::SmallVec;

use crate::types::indexed::{
    IndexedSnbt,
    core::StrCore,
    index::{EntryIndex, Index, ValueIndex},
};

pub(crate) fn parse_snbt(root: &str) -> Result<IndexedSnbt<'_, StrCore<'_>>, ()> {
    let compounds = item_bounds(root.as_bytes(), b'{', b'}')?;
    let lists = item_bounds(root.as_bytes(), b'[', b']')?;

    let mut entries = Vec::with_capacity(compounds.len() + lists.len());
    let mut ranges = Vec::with_capacity(compounds.len() + lists.len());

    unsafe {
        for index in 0..compounds.len() {
            parse_item::<true>(root, index, &compounds, &lists, &mut entries, &mut ranges)?;
        }
        for index in 0..lists.len() {
            parse_item::<false>(root, index, &compounds, &lists, &mut entries, &mut ranges)?;
        }
    }

    // SAFETY: `entries` and `ranges` were created from `root`.
    Ok(IndexedSnbt::new(unsafe {
        StrCore::new(root, entries.into_boxed_slice(), ranges.into_boxed_slice())
    }))
}

/// Calculate the bounds of items based on the provided `start` and `end` chars.
fn item_bounds(root: &[u8], start: u8, end: u8) -> Result<Vec<Range<usize>>, ()> {
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
    if history.is_empty() {
        bounds.sort_unstable_by_key(|r| r.start);
        Ok(bounds)
    } else {
        Err(())
    }
}

// -------------------------------------------------------------------------------------------------

/// # SAFETY
///
/// The caller must ensure:
///   - If `NAMED` is `true`, `index` must be a valid index into `compounds`.
///   - If `NAMED` is `false`, `index` must be a valid index into `lists`.
///   - All pairs of `{` and `}` must have indices in `compounds`.
///   - All pairs of `[` and `]` must have indices in `lists`.
#[inline]
unsafe fn parse_item<const NAMED: bool>(
    root: &str,
    index: usize,

    compounds: &[Range<usize>],
    lists: &[Range<usize>],

    entries: &mut Vec<EntryIndex>,
    item_ranges: &mut Vec<Range<usize>>,
) -> Result<(), ()> {
    // SAFETY: The caller ensures this is safe.
    let (range, slice) = if NAMED {
        let range = unsafe { *compounds.get_unchecked(index) };
        let slice = unsafe { root.get_unchecked(range) };
        (range, slice)
    } else {
        let range = unsafe { *lists.get_unchecked(index) };
        let slice = unsafe { root.get_unchecked(range) };
        (range, slice)
    };

    // Track the starting `entries` index
    let range_start = entries.len();

    // Parse each entry in the item.
    let mut cursor = Cursor::new(slice, range.start);
    cursor.trim_start();

    while !cursor.remaining().is_empty() {
        entries.push(parse_entry::<NAMED>(&mut cursor, compounds, lists)?);
        cursor.trim_start();

        // If there are more entries, they must be separated by a comma.
        if NAMED && !cursor.remaining().is_empty() {
            if cursor.peek_char() == Some(',') {
                cursor.position += 1;
                debug_assert!(cursor.position <= cursor.slice.len());
                cursor.trim_start();
            } else {
                return Err(());
            }
        }
    }

    // Add the item's `entries` range
    item_ranges.push(Range { start: range_start, end: entries.len() });

    Ok(())
}

#[inline]
fn parse_entry<const NAMED: bool>(
    cursor: &mut Cursor<'_>,
    compounds: &[Range<usize>],
    lists: &[Range<usize>],
) -> Result<EntryIndex, ()> {
    // Read the name if `NAMED`
    let name = if NAMED {
        let position = cursor.root_position();

        if cursor.peek_char() == Some('\"') {
            // Read past the name, until the next unescaped `"` character.
            cursor.until_char(b'\"', true, true).ok_or(())?;
        } else if cursor.peek_char() == Some('\'') {
            // Read past the name, until the next unescaped `'` character.
            cursor.until_char(b'\'', true, true).ok_or(())?;
        }

        // Read until the next `:` character.
        cursor.until_char(b':', false, true).ok_or(())?;

        Index::<str>::new(position)
    } else {
        Index::<str>::new(0)
    };

    // Read the value
    let value = parse_value(cursor, compounds, lists)?;

    Ok(EntryIndex::new(name, value))
}

#[inline]
fn parse_value(
    cursor: &mut Cursor<'_>,
    compounds: &[Range<usize>],
    lists: &[Range<usize>],
) -> Result<ValueIndex, ()> {
    cursor.trim_start();

    match cursor.peek_char().ok_or(())? {
        // Numeric values start with a digit, a sign, or a decimal point.
        '0'..='9' | '-' | '+' | '.' => todo!(),

        // Un-quoted strings can start with a letter or an underscore.
        'a'..='z' | 'A'..='Z' | '_' => {
            let position = cursor.root_position();
            if cursor
                .until(
                    false,
                    |c| !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '+' | '.'| '_'),
                )
                .is_none()
            {
                // If we reached the end of the slice, move the cursor to the end.
                cursor.position = cursor.slice.len();
                debug_assert!(cursor.position <= cursor.slice.len());
            }

            Ok(ValueIndex::String(Index::new(position)))
        }
        // Quoted strings start with either a single or double quote.
        c @ ('\"' | '\'') => {
            let position = cursor.root_position();
            cursor.until_char(c as u8, true, true).ok_or(())?;

            Ok(ValueIndex::String(Index::new(position)))
        }

        // Compound objects start with `{`.
        '{' => {
            // Find the compound object that starts at our position.
            let position = cursor.root_position() + 1;
            let (index, range) =
                compounds.iter().enumerate().find(|(_, range)| range.start == position).unwrap();

            // Advance the cursor past the compound object.
            cursor.position += range.end + 2 - range.start;
            debug_assert!(cursor.position <= cursor.slice.len());

            Ok(ValueIndex::Compound(Index::new(index)))
        }
        // List objects start with `[`.
        '[' => {
            // Find the list object that starts at our position.
            let position = cursor.root_position() + 1;
            let (index, range) =
                lists.iter().enumerate().find(|(_, range)| range.start == position).unwrap();

            // Advance the cursor past the list object.
            cursor.position += range.end + 2 - range.start;
            debug_assert!(cursor.position <= cursor.slice.len());

            Ok(ValueIndex::List(Index::new(index + compounds.len())))
        }

        _ => Err(()),
    }
}

// -------------------------------------------------------------------------------------------------

// -------------------------------------------------------------------------------------------------

struct Cursor<'a> {
    slice: &'a str,
    position: usize,
    start: usize,
}

impl<'a> Cursor<'a> {
    /// Create a new [`Cursor`], where the slice starts at index `start`.
    const fn new(slice: &'a str, start: usize) -> Self { Self { slice, position: 0, start } }

    /// Get the local position of the cursor within the slice.
    const fn position(&self) -> usize { self.position }

    /// Get the global position of the cursor within the slice.
    const fn root_position(&self) -> usize { self.start + self.position }

    /// Get the remaining slice from the current position.
    fn remaining(&self) -> &'a str {
        // SAFETY: `position` is always within bounds.
        unsafe { self.slice.get_unchecked(self.position..) }
    }

    /// Advance the cursor past any leading whitespace.
    #[inline]
    fn trim_start(&mut self) {
        let slice = self.remaining();
        let trimmed = slice.trim_start();

        self.position += slice.len() - trimmed.len();
        debug_assert!(self.position <= self.slice.len());
    }

    /// Peek the next character without advancing the cursor.
    #[inline]
    fn peek_char(&self) -> Option<char> { self.remaining().chars().next() }

    /// Advance the cursor until some condition is met.
    ///
    /// - `including` determines whether the character should be included in the
    ///   returned slice.
    fn until(&mut self, including: bool, mut f: impl FnMut(char) -> bool) -> Option<&'a str> {
        let mut slice = self.remaining();

        let mut found = false;
        for (index, char) in slice.char_indices() {
            if f(char) {
                found = true;
                slice = unsafe { slice.get_unchecked(..index) };

                self.position += index;
                if including {
                    self.position += char.len_utf8();
                }
                debug_assert!(self.position <= self.slice.len());

                break;
            }
        }

        found.then_some(slice)
    }

    /// Advance the cursor until the target character is found.
    ///
    /// - `escaped` determines whether the target characters can be escaped with
    ///   a backslash.
    ///
    /// - `including` determines whether the target character should be included
    ///   in the returned slice.
    fn until_char(&mut self, target: u8, escaped: bool, including: bool) -> Option<&'a str> {
        let mut slice = self.remaining();

        let mut found = false;
        for index in Memchr::new(target, slice.as_bytes()) {
            if escaped && index.checked_sub(1).and_then(|i| slice.as_bytes().get(i)) == Some(&b'\\')
            {
                continue;
            }

            found = true;
            slice = unsafe { slice.get_unchecked(..index) };

            self.position += index;
            if including {
                self.position += 1;
            }
            debug_assert!(self.position <= self.slice.len());

            break;
        }

        found.then_some(slice)
    }

    /// Advance the cursor until any of the target characters is found.
    ///
    /// - `escaped` determines whether the target characters can be escaped with
    ///   a backslash.
    ///
    /// - `including` determines whether the target character should be included
    ///   in the returned slice.
    fn until_char2(
        &mut self,
        target_a: u8,
        target_b: u8,
        escaped: bool,
        including: bool,
    ) -> Option<&'a str> {
        let mut slice = self.remaining();

        let mut found = false;
        for index in Memchr2::new(target_a, target_b, slice.as_bytes()) {
            if escaped && index.checked_sub(1).and_then(|i| slice.as_bytes().get(i)) == Some(&b'\\')
            {
                continue;
            }

            found = true;
            slice = unsafe { slice.get_unchecked(..index) };

            self.position += index;
            if including {
                self.position += 1;
            }
            debug_assert!(self.position <= self.slice.len());

            break;
        }

        found.then_some(slice)
    }

    /// Advance the cursor until any of the target characters is found.
    ///
    /// - `escaped` determines whether the target characters can be escaped with
    ///   a backslash.
    ///
    /// - `including` determines whether the target character should be included
    ///   in the returned slice.
    fn until_char3(
        &mut self,
        target_a: u8,
        target_b: u8,
        target_c: u8,
        escaped: bool,
        including: bool,
    ) -> Option<&'a str> {
        let mut slice = self.remaining();

        let mut found = false;
        for index in Memchr3::new(target_a, target_b, target_c, slice.as_bytes()) {
            if escaped && index.checked_sub(1).and_then(|i| slice.as_bytes().get(i)) == Some(&b'\\')
            {
                continue;
            }

            found = true;
            slice = unsafe { slice.get_unchecked(..index) };

            self.position += index;
            if including {
                self.position += 1;
            }
            debug_assert!(self.position <= self.slice.len());

            break;
        }

        found.then_some(slice)
    }
}
