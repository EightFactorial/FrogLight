#![expect(unused, reason = "WIP")]

use alloc::{collections::VecDeque, vec::Vec};
use core::range::Range;

use crate::types::indexed::{
    entry::{EntryIndex, ValueIndex},
    index::{
        Index,
        bool::{BoolDescription, BooleanOperation},
        numeric::{Integer, IntegerDescription, IntegerRadix, IntegerSignness, IntegerType},
    },
    parse::{Cursor, NULL_STRING, value},
};

pub(super) fn parse<'data>(
    mut cursor: Cursor<'data>,
    compounds: &[Range<usize>],
    lists: &[Range<usize>],
    strings: &[Range<usize>],
    entries: &mut Vec<EntryIndex>,
    queue: &mut VecDeque<(Cursor<'data>, usize, bool)>,
) -> Result<ValueIndex, ()> {
    cursor.next_expect('[')?;

    let remaining = cursor.trim_start().remaining();
    if remaining.starts_with("B;") || remaining.starts_with("I;") || remaining.starts_with("L;") {
        return parse_array(&mut cursor, entries);
    }

    let start = entries.len();

    loop {
        // Value
        let value =
            value::parse_value(cursor.trim_start(), compounds, lists, strings, entries, queue)?;

        entries.push(EntryIndex::new(NULL_STRING, value));

        // ',' or ']'
        match cursor.trim_start().peek() {
            Some(',') => {
                cursor.next_expect(',')?;
            }
            Some(']') => break,
            _ => return Err(()),
        }
    }

    let range = Range { start, end: entries.len() };
    Ok(unsafe { ValueIndex::List(Index::new(range, ())) })
}

fn parse_array(cursor: &mut Cursor<'_>, entries: &mut Vec<EntryIndex>) -> Result<ValueIndex, ()> {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum ListType {
        Byte,
        Int,
        Long,
    }

    let remaining = cursor.remaining();
    let mut list_type = if remaining.starts_with("B;") {
        ListType::Byte
    } else if remaining.starts_with("I;") {
        ListType::Int
    } else if remaining.starts_with("L;") {
        ListType::Long
    } else {
        return Err(());
    };
    cursor.take_slice(2);

    let start = entries.len();

    loop {
        let value = value::parse_numeric(cursor.trim_start())?;

        match list_type {
            ListType::Byte if matches!(value, ValueIndex::Bool(..) | ValueIndex::Byte(..)) => {
                let value = match value {
                    ValueIndex::Bool(bool) => ValueIndex::Bool(bool),
                    ValueIndex::Byte(byte) => ValueIndex::Byte(byte),
                    _ => unsafe { core::hint::unreachable_unchecked() },
                };

                entries.push(EntryIndex::new(NULL_STRING, value));
            }
            ListType::Int => {
                if matches!(
                    value,
                    ValueIndex::Bool(..)
                        | ValueIndex::Byte(..)
                        | ValueIndex::Short(..)
                        | ValueIndex::Int(..)
                ) {
                    let value = match value {
                        ValueIndex::Bool(bool) => ValueIndex::Bool(bool),
                        ValueIndex::Byte(byte) => ValueIndex::Int(byte),
                        ValueIndex::Short(short) => ValueIndex::Int(short),
                        ValueIndex::Int(int) => ValueIndex::Int(int),
                        _ => unsafe { core::hint::unreachable_unchecked() },
                    };

                    entries.push(EntryIndex::new(NULL_STRING, value));
                }
            }
            ListType::Long
                if matches!(
                    value,
                    ValueIndex::Bool(..)
                        | ValueIndex::Byte(..)
                        | ValueIndex::Short(..)
                        | ValueIndex::Int(..)
                        | ValueIndex::Long(..)
                ) =>
            {
                let value = match value {
                    ValueIndex::Bool(bool) => ValueIndex::Bool(bool),
                    ValueIndex::Byte(byte) => ValueIndex::Long(byte),
                    ValueIndex::Short(short) => ValueIndex::Long(short),
                    ValueIndex::Int(int) => ValueIndex::Long(int),
                    ValueIndex::Long(long) => ValueIndex::Long(long),
                    _ => unsafe { core::hint::unreachable_unchecked() },
                };

                // `Long`s can be any integer type.
                entries.push(EntryIndex::new(NULL_STRING, value));
            }
            _ => return Err(()),
        }

        // ',' or ']'
        match cursor.trim_start().peek() {
            Some(',') => {
                cursor.next_expect(',')?;
            }
            Some(']') => break,
            _ => return Err(()),
        }
    }

    let range = Range { start, end: entries.len() };
    match list_type {
        ListType::Byte => Ok(unsafe { ValueIndex::ByteArray(Index::new(range, ())) }),
        ListType::Int => Ok(unsafe { ValueIndex::IntArray(Index::new(range, ())) }),
        ListType::Long => Ok(unsafe { ValueIndex::LongArray(Index::new(range, ())) }),
    }
}
