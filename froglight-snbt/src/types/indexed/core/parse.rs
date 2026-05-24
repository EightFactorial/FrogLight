use alloc::vec::Vec;
use core::range::Range;

use memchr::{Memchr, Memchr2};
use smallvec::SmallVec;
use uuid::Uuid;

use crate::types::indexed::{
    IndexedSnbt,
    core::StrCore,
    index::{EntryIndex, Index, ValueIndex},
};

pub(crate) fn parse_snbt(root: &str) -> Result<IndexedSnbt<'_, StrCore<'_>>, ()> {
    let compounds = get_item_bounds(root.as_bytes(), b'{', b'}')?;
    let lists = get_item_bounds(root.as_bytes(), b'[', b']')?;

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

// -------------------------------------------------------------------------------------------------

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

    entries: &mut Vec<EntryIndex>,
    item_ranges: &mut Vec<Range<usize>>,
) -> Result<(), ()> {
    // SAFETY: The caller ensures this is safe.
    let (range, mut slice) = if NAMED {
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
    loop {
        if slice.is_empty() {
            break;
        }

        // Read the entry's name.
        slice = slice.trim_start();
        let name = read_string_key(slice)?;
        let name_start = range.start + name.len();
        unsafe {
            slice = slice.get_unchecked(name.len()..);
        }

        // Skip the `:`, which is required.
        slice = slice.trim_start();
        if let Some(stripped) = slice.strip_prefix(':') {
            slice = stripped;
        } else {
            return Err(());
        }

        // Read the entry's value.
        slice = slice.trim_start();
        let entry = read_unknown_value(slice, range.end - slice.len(), compounds, lists)?;
        let entry_start = range.start + entry.len();
        unsafe {
            slice = slice.get_unchecked(entry.len()..);
        }

        // Skip the ',' if there is one.
        slice = slice.trim_start();
        if let Some(stripped) = slice.strip_prefix(',') {
            slice = stripped;
        }

        entries.push(parse_entry(name, name_start, entry, entry_start, compounds, lists)?);
    }

    // Add the item's `entries` range
    item_ranges.push(Range { start: range_start, end: entries.len() });

    Ok(())
}

// -------------------------------------------------------------------------------------------------

fn parse_entry(
    name: &str,
    name_start: usize,
    entry: &str,
    entry_start: usize,

    compounds: &[Range<usize>],
    lists: &[Range<usize>],
) -> Result<EntryIndex, ()> {
    let (name, _name_settings) = parse_string(name, name_start)?;

    // Support `bool(...)` and `uuid(...)` operations.
    if let Some(bool) = entry.strip_prefix("bool(")
        && let Some(bool) = bool.strip_suffix(')')
    {
        let (value, _entry_settings) = parse_bool(bool, entry_start)?;
        return Ok(EntryIndex::new(name, value));
    } else if let Some(uuid) = entry.strip_prefix("uuid(")
        && let Some(uuid) = uuid.strip_suffix(')')
    {
        let (value, _entry_settings) = parse_uuid(uuid, entry_start)?;
        return Ok(EntryIndex::new(name, value));
    }

    match entry.chars().next().ok_or(())? {
        // A number, as un-quoted strings cannot start with these.
        '0'..='9' | '+' | '-' | '.' => {
            let (value, _entry_settings) = parse_number(entry, entry_start)?;
            Ok(EntryIndex::new(name, value))
        }
        // A string, either quoted or un-quoted.
        'a'..='z' | 'A'..='Z' | '_' | '\"' | '\'' => {
            let (value, _entry_settings) = parse_string(entry, entry_start)?;
            Ok(EntryIndex::new(name, ValueIndex::String(value)))
        }

        // A compound.
        '{' => {
            let compound =
                compounds.iter().position(|range| range.start == entry_start).ok_or(())?;
            Ok(EntryIndex::new(name, ValueIndex::Compound(Index::new(compound))))
        }
        // A list.
        '[' => {
            let list = lists.iter().position(|range| range.start == entry_start).ok_or(())?;
            Ok(EntryIndex::new(name, ValueIndex::List(Index::new(list + compounds.len()))))
        }

        _ => Err(()),
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "std")]
fn parse_string(content: &str, start: usize) -> Result<(Index<str>, ()), ()> {
    /// [`Regex`] for un-quoted strings: 0-9, A-Z, a-z, _, -, ., and +
    static REGEX: std::sync::LazyLock<regex_lite::Regex> =
        std::sync::LazyLock::new(|| regex_lite::Regex::new(r"^[0-9A-Za-z_.+-]+$").unwrap());

    if content.starts_with(['\"', '\'']) || REGEX.is_match(content) {
        Ok((Index::new(start), ()))
    } else {
        Err(())
    }
}

#[cfg(not(feature = "std"))]
fn parse_string(content: &str, start: usize) -> Result<(Index<str>, ()), ()> {
    if content.starts_with(['\"', '\''])
        || content
            .chars()
            .all(|c| matches!(c, '0'..='9' | 'A'..='Z' | 'a'..='z' | '_' | '-' | '.' | '+'))
    {
        Ok((Index::new(start), ()))
    } else {
        Err(())
    }
}

fn parse_number(content: &str, start: usize) -> Result<(ValueIndex, ()), ()> {
    // Check for a hex or binary prefix
    if let Some(src) = content.strip_prefix("0x") {
        return if u32::from_str_radix(src, 16).is_ok() {
            Ok((ValueIndex::Int(Index::new(start)), ()))
        } else {
            Err(())
        };
    } else if let Some(src) = content.strip_prefix("0b") {
        return if u32::from_str_radix(src, 2).is_ok() {
            Ok((ValueIndex::Int(Index::new(start)), ()))
        } else {
            Err(())
        };
    }

    todo!()
}

/// Returns a `ValueIndex::Byte` if the content is a valid boolean.
///
/// Expects content to be in one of the following formats:
///     - `true` (true)
///     - `false` (false)
///     - `0` (false)
///     - `5` (true)
///     - `10` (true)
fn parse_bool(content: &str, start: usize) -> Result<(ValueIndex, ()), ()> {
    if matches!(content, "true" | "false") || content.chars().all(|c| c.is_ascii_digit()) {
        Ok((ValueIndex::Int(Index::new(start)), ()))
    } else {
        Err(())
    }
}

/// Returns a `ValueIndex::IntArray` if the content is a valid UUID.
///
/// Expects content to be in the format:
/// `"f81d4fae-7dec-11d0-a765-00a0c91e6bf6"`
fn parse_uuid(content: &str, start: usize) -> Result<(ValueIndex, ()), ()> {
    let Some(trimmed) = content.strip_prefix("\"") else { return Err(()) };
    let Some(trimmed) = trimmed.strip_suffix("\"") else { return Err(()) };

    match Uuid::try_parse(trimmed) {
        Ok(..) => Ok((ValueIndex::IntArray(Index::new(start)), ())),
        Err(_) => Err(()),
    }
}

// -------------------------------------------------------------------------------------------------

fn read_string_key(content: &str) -> Result<&str, ()> {
    // Get the first char, or return an empty string.
    let Some(first) = content.chars().next() else { return Ok(content) };

    match first {
        // An un-quoted string, which ends at the next `:`.
        'a'..='z' | 'A'..='Z' | '_' => {
            let end = content.as_bytes().iter().position(|&c| c == b':').ok_or(())?;
            Ok(unsafe { content.get_unchecked(..end) })
        }
        // A quoted string, which ends at the next un-escaped `"` or `'`.
        c @ ('\"' | '\'') => {
            let mut quote_end = Option::<usize>::None;
            for index in Memchr::new(c as u8, content.as_bytes()) {
                if let Some(previous) = index.checked_sub(1).and_then(|i| content.as_bytes().get(i))
                    && *previous == b'\\'
                {
                } else {
                    quote_end = Some(index);
                    break;
                }
            }

            let end = quote_end.ok_or(())?;
            Ok(unsafe { content.get_unchecked(..end) })
        }
        _ => Err(()),
    }
}

fn read_unknown_value<'a>(
    content: &'a str,
    position: usize,
    compounds: &[Range<usize>],
    lists: &[Range<usize>],
) -> Result<&'a str, ()> {
    // Support `bool(...)` and `uuid(...)` operations.
    if content.starts_with("bool(")
        && let Some((bool, _)) = content.split_once(')')
    {
        return Ok(bool);
    } else if content.starts_with("uuid(")
        && let Some((uuid, _)) = content.split_once(')')
    {
        return Ok(uuid);
    }

    match content.chars().next().ok_or(())? {
        // An un-quoted number or string, which ends at the next `,` or the end of the item.
        'a'..='z' | 'A'..='Z' | '0'..='9' | '+' | '-' | '.' | '_' => {
            if let Some(end) = Memchr::new(b',', content.as_bytes()).next() {
                Ok(unsafe { content.get_unchecked(..end) })
            } else {
                Ok(content)
            }
        }

        // An quoted string, which ends at the next un-escaped `"` or `'`.
        c @ ('\"' | '\'') => {
            let mut quote_end = Option::<usize>::None;
            for index in Memchr::new(c as u8, content.as_bytes()) {
                if let Some(previous) = index.checked_sub(1).and_then(|i| content.as_bytes().get(i))
                    && *previous == b'\\'
                {
                } else {
                    quote_end = Some(index);
                    break;
                }
            }

            let end = quote_end.ok_or(())?;
            Ok(unsafe { content.get_unchecked(..end) })
        }

        // A compound.
        '{' => {
            let compound = compounds.iter().find(|range| range.start == position).ok_or(())?;
            Ok(unsafe { content.get_unchecked(..compound.end - position) })
        }
        // A list.
        '[' => {
            let list = lists.iter().find(|range| range.start == position).ok_or(())?;
            Ok(unsafe { content.get_unchecked(..list.end - position) })
        }

        _ => Err(()),
    }
}
