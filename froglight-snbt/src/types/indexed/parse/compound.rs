use alloc::{collections::VecDeque, vec::Vec};
use core::range::Range;

use crate::types::indexed::{
    entry::{EntryIndex, ValueIndex},
    index::Index,
    parse::{Cursor, value},
};

pub(super) fn parse<'data>(
    mut cursor: Cursor<'data>,
    compounds: &[Range<usize>],
    lists: &[Range<usize>],
    strings: &[Range<usize>],
    entries: &mut Vec<EntryIndex>,
    queue: &mut VecDeque<(Cursor<'data>, usize, bool)>,
) -> Result<ValueIndex, ()> {
    let start = entries.len();
    cursor.next_expect('{')?;

    // Empty compound
    if cursor.trim_start().peek() == Some('}') {
        // SAFETY: We just built the compound
        let range = Range { start, end: start };
        return Ok(unsafe { ValueIndex::Compound(Index::new(range, ())) });
    }

    loop {
        // Name (String)
        let name = value::parse_string::<false>(cursor.trim_start(), strings)?;
        let ValueIndex::String(name) = name else { unsafe { core::hint::unreachable_unchecked() } };

        // ':'
        cursor.trim_start().next_expect(':')?;
        // Value
        let value =
            value::parse_value(cursor.trim_start(), compounds, lists, strings, entries, queue)?;

        entries.push(EntryIndex::new(name, value));

        // ',' or '}'
        match cursor.trim_start().peek() {
            Some(',') => cursor.next(),
            Some('}') | None => break,
            _ => return Err(()),
        };
    }

    // SAFETY: We just built the compound
    let range = Range { start, end: entries.len() };
    Ok(unsafe { ValueIndex::Compound(Index::new(range, ())) })
}
