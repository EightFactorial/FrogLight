#![allow(clippy::wildcard_imports, reason = "Cleanliness")]
#![expect(unused, reason = "WIP")]

use alloc::{collections::VecDeque, string::String};
use core::range::Range;

use crate::types::indexed::{
    entry::{EntryIndex, ValueIndex},
    index::{Index, bool::*, numeric::*, string::*},
    parse::{Cursor, PLACEHOLDER_COMPOUND, PLACEHOLDER_LIST},
};

pub(super) fn parse_value<'root>(
    cursor: &mut Cursor<'root>,
    compounds: &[Range<usize>],
    lists: &[Range<usize>],
    strings: &[Range<usize>],
    entries: &[EntryIndex],
    queue: &mut VecDeque<(Cursor<'root>, usize, bool)>,
) -> Result<ValueIndex, ()> {
    // Handle operations
    let remaining = cursor.remaining();
    if remaining.starts_with("bool(") {
        return parse_bool(cursor).map(ValueIndex::Bool);
    } else if remaining.starts_with("uuid(") {
        todo!();
    }

    match cursor.peek().ok_or(())? {
        // String
        'A'..='Z' | 'a'..='z' | '_' | '\"' | '\'' => parse_string::<true>(cursor, strings),
        // Numeric
        '0'..='9' | '-' | '+' | '.' => parse_numeric(cursor),

        // List
        '[' => {
            let start = cursor.position();
            let Some(range) = lists.iter().find(|r| r.start == start) else {
                #[cfg(debug_assertions)]
                unreachable!("List range not found when it should be known!?");
                #[cfg(not(debug_assertions))]
                unsafe {
                    core::hint::unreachable_unchecked()
                }
            };

            // Skip over the list and queue it for later
            let list_cursor = cursor.split_range(*range);
            queue.push_back((list_cursor, entries.len(), false));
            cursor.next_expect(']')?;

            Ok(PLACEHOLDER_LIST)
        }
        // Compound
        '{' => {
            let start = cursor.position();
            let Some(range) = compounds.iter().find(|r| r.start == start) else {
                #[cfg(debug_assertions)]
                unreachable!("Compound range not found when it should be known!?");
                #[cfg(not(debug_assertions))]
                unsafe {
                    core::hint::unreachable_unchecked()
                }
            };

            // Skip over the compound and queue it for later
            let compound_cursor = cursor.split_range(*range);
            queue.push_back((compound_cursor, entries.len(), true));
            cursor.next_expect('}')?;

            Ok(PLACEHOLDER_COMPOUND)
        }

        _ => Err(()),
    }
}

// -------------------------------------------------------------------------------------------------

pub(super) fn parse_bool(cursor: &mut Cursor<'_>) -> Result<Index<bool>, ()> {
    let start = cursor.position();
    let remaining = cursor.remaining();

    if remaining.starts_with("true") {
        cursor.take_slice(4);

        let range = Range { start, end: cursor.position() };
        let desc = BoolDescription::bool();
        Ok(unsafe { Index::new(range, desc) })
    } else if remaining.starts_with("false") {
        cursor.take_slice(5);

        let range = Range { start, end: cursor.position() };
        let desc = BoolDescription::bool();
        Ok(unsafe { Index::new(range, desc) })
    } else if remaining.starts_with("bool(") {
        cursor.take_slice(5);

        let value = parse_numeric(cursor)?;
        cursor.next_expect(')')?;

        match value {
            ValueIndex::Byte(value)
            | ValueIndex::Short(value)
            | ValueIndex::Int(value)
            | ValueIndex::Long(value) => {
                let range = Range { start, end: cursor.position() };
                let desc = BoolDescription::Integer(value.description());
                Ok(unsafe { Index::new(range, desc) })
            }
            _ => Err(()),
        }
    } else {
        Err(())
    }
}

// -------------------------------------------------------------------------------------------------

pub(super) fn parse_numeric(cursor: &mut Cursor<'_>) -> Result<ValueIndex, ()> {
    // Check for a boolean first.
    let remaining = cursor.remaining();

    if remaining.starts_with("true") {
        let start = cursor.position();
        cursor.take_slice(4);

        let range = Range { start, end: cursor.position() };
        let desc = BoolDescription::bool();
        return Ok(ValueIndex::Bool(unsafe { Index::new(range, desc) }));
    } else if remaining.starts_with("false") {
        let start = cursor.position();
        cursor.take_slice(5);

        let range = Range { start, end: cursor.position() };
        let desc = BoolDescription::bool();
        return Ok(ValueIndex::Bool(unsafe { Index::new(range, desc) }));
    } else if remaining.starts_with("bool(") {
        let start = cursor.position();
        cursor.take_slice(5);

        let value = parse_numeric(cursor)?;
        cursor.next_expect(')')?;

        match value {
            ValueIndex::Byte(value)
            | ValueIndex::Short(value)
            | ValueIndex::Int(value)
            | ValueIndex::Long(value) => {
                let range = Range { start, end: cursor.position() };
                let desc = BoolDescription::Integer(value.description());
                return Ok(ValueIndex::Bool(unsafe { Index::new(range, desc) }));
            }
            _ => return Err(()),
        }
    }

    // Advance until the end of the value.
    let start = cursor.position();
    let slice = cursor.until_char::<false, false, _>(|c| {
        !matches!(c, '0'..='9' | '-' | '+' | '.' | '_' | 'b' | 'e' | 'x' | 'E')
    });

    // Check for a signedness suffix if there is a type suffix after.
    let signedness =
        if matches!(cursor.peek2(), Some('b' | 'B' | 's' | 'S' | 'i' | 'I' | 'l' | 'L')) {
            match cursor.peek() {
                Some('u') => {
                    cursor.next();
                    Ok(IntegerSignness::Unsigned)
                }
                Some('s') => {
                    cursor.next();
                    Ok(IntegerSignness::Signed)
                }

                Some(',') => Ok(IntegerSignness::None),
                _ => Err(()),
            }
        } else {
            Ok(IntegerSignness::None)
        }?;

    // Check for a type suffix.
    match cursor.peek() {
        Some('b' | 'B') => {
            cursor.next();
            parse_integer(slice, start, IntegerType::Byte, signedness)
        }
        Some('s' | 'S') => {
            cursor.next();
            parse_integer(slice, start, IntegerType::Short, signedness)
        }
        Some('i' | 'I') => {
            cursor.next();
            parse_integer(slice, start, IntegerType::Int, signedness)
        }
        Some('l' | 'L') => {
            cursor.next();
            parse_integer(slice, start, IntegerType::Long, signedness)
        }

        Some('f' | 'F') => {
            cursor.next();
            parse_float(slice, start, FloatType::Float)
        }
        Some('d' | 'D') => {
            cursor.next();
            parse_float(slice, start, FloatType::Double)
        }

        _ => parse_integer(slice, start, IntegerType::None, signedness),
    }
}

#[expect(clippy::too_many_lines, reason = "Large match statements")]
pub(super) fn parse_integer(
    mut slice: &str,
    start: usize,
    mut ty: IntegerType,
    sign: IntegerSignness,
) -> Result<ValueIndex, ()> {
    use lexical::parse_with_options as parse;
    const FALSE: BooleanOperation = BooleanOperation::False;

    let radix = if slice.len() > 2 {
        if slice.starts_with("0b") {
            IntegerRadix::Binary
        } else if slice.starts_with("0x") {
            IntegerRadix::Hexadecimal
        } else {
            IntegerRadix::Decimal
        }
    } else {
        IntegerRadix::Decimal
    };

    if let Some(c) = slice.chars().last() {
        match c {
            'b' | 'B' => {
                ty = IntegerType::Byte;
                slice = &slice[..slice.len() - 1];
            }
            's' | 'S' => {
                ty = IntegerType::Short;
                slice = &slice[..slice.len() - 1];
            }
            'i' | 'I' => {
                ty = IntegerType::Int;
                slice = &slice[..slice.len() - 1];
            }
            'l' | 'L' => {
                ty = IntegerType::Long;
                slice = &slice[..slice.len() - 1];
            }
            _ => {}
        }
    }

    // If the string is long, enable multi-digit optimizations.
    let opt = if slice.len() >= 12 { &INTEGER_MULTIDIGIT_OPTIONS } else { &INTEGER_OPTIONS };

    match (ty, sign) {
        (IntegerType::Byte, IntegerSignness::None | IntegerSignness::Unsigned) => {
            match radix {
                IntegerRadix::Binary => parse::<u8, &str, INTEGER_BINARY_FORMAT>(slice, opt),
                IntegerRadix::Decimal => parse::<u8, &str, INTEGER_DECIMAL_FORMAT>(slice, opt),
                IntegerRadix::Hexadecimal => {
                    parse::<u8, &str, INTEGER_HEXADECIMAL_FORMAT>(slice, opt)
                }
            }
            .map_err(|_| ())?;

            Ok(unsafe {
                ValueIndex::Byte(Index::new_from(
                    slice,
                    start,
                    IntegerDescription::new(radix, ty, sign, FALSE),
                ))
            })
        }
        (IntegerType::Byte, IntegerSignness::Signed) => {
            match radix {
                IntegerRadix::Binary => parse::<i8, &str, INTEGER_BINARY_FORMAT>(slice, opt),
                IntegerRadix::Decimal => parse::<i8, &str, INTEGER_DECIMAL_FORMAT>(slice, opt),
                IntegerRadix::Hexadecimal => {
                    parse::<i8, &str, INTEGER_HEXADECIMAL_FORMAT>(slice, opt)
                }
            }
            .map_err(|_| ())?;

            Ok(unsafe {
                ValueIndex::Byte(Index::new_from(
                    slice,
                    start,
                    IntegerDescription::new(radix, ty, sign, FALSE),
                ))
            })
        }

        (IntegerType::Short, IntegerSignness::None | IntegerSignness::Unsigned) => {
            match radix {
                IntegerRadix::Binary => parse::<u16, &str, INTEGER_BINARY_FORMAT>(slice, opt),
                IntegerRadix::Decimal => parse::<u16, &str, INTEGER_DECIMAL_FORMAT>(slice, opt),
                IntegerRadix::Hexadecimal => {
                    parse::<u16, &str, INTEGER_HEXADECIMAL_FORMAT>(slice, opt)
                }
            }
            .map_err(|_| ())?;

            Ok(unsafe {
                ValueIndex::Short(Index::new_from(
                    slice,
                    start,
                    IntegerDescription::new(radix, ty, sign, FALSE),
                ))
            })
        }
        (IntegerType::Short, IntegerSignness::Signed) => {
            match radix {
                IntegerRadix::Binary => parse::<i16, &str, INTEGER_BINARY_FORMAT>(slice, opt),
                IntegerRadix::Decimal => parse::<i16, &str, INTEGER_DECIMAL_FORMAT>(slice, opt),
                IntegerRadix::Hexadecimal => {
                    parse::<i16, &str, INTEGER_HEXADECIMAL_FORMAT>(slice, opt)
                }
            }
            .map_err(|_| ())?;

            Ok(unsafe {
                ValueIndex::Short(Index::new_from(
                    slice,
                    start,
                    IntegerDescription::new(radix, ty, sign, FALSE),
                ))
            })
        }

        (
            IntegerType::None | IntegerType::Int,
            IntegerSignness::None | IntegerSignness::Unsigned,
        ) => {
            match radix {
                IntegerRadix::Binary => parse::<u32, &str, INTEGER_BINARY_FORMAT>(slice, opt),
                IntegerRadix::Decimal => parse::<u32, &str, INTEGER_DECIMAL_FORMAT>(slice, opt),
                IntegerRadix::Hexadecimal => {
                    parse::<u32, &str, INTEGER_HEXADECIMAL_FORMAT>(slice, opt)
                }
            }
            .map_err(|_| ())?;

            Ok(unsafe {
                ValueIndex::Int(Index::new_from(
                    slice,
                    start,
                    IntegerDescription::new(radix, ty, sign, FALSE),
                ))
            })
        }
        (IntegerType::None | IntegerType::Int, IntegerSignness::Signed) => {
            match radix {
                IntegerRadix::Binary => parse::<i32, &str, INTEGER_BINARY_FORMAT>(slice, opt),
                IntegerRadix::Decimal => parse::<i32, &str, INTEGER_DECIMAL_FORMAT>(slice, opt),
                IntegerRadix::Hexadecimal => {
                    parse::<i32, &str, INTEGER_HEXADECIMAL_FORMAT>(slice, opt)
                }
            }
            .map_err(|_| ())?;

            Ok(unsafe {
                ValueIndex::Int(Index::new_from(
                    slice,
                    start,
                    IntegerDescription::new(radix, ty, sign, FALSE),
                ))
            })
        }

        (IntegerType::Long, IntegerSignness::None | IntegerSignness::Unsigned) => {
            match radix {
                IntegerRadix::Binary => parse::<u64, &str, INTEGER_BINARY_FORMAT>(slice, opt),
                IntegerRadix::Decimal => parse::<u64, &str, INTEGER_DECIMAL_FORMAT>(slice, opt),
                IntegerRadix::Hexadecimal => {
                    parse::<u64, &str, INTEGER_HEXADECIMAL_FORMAT>(slice, opt)
                }
            }
            .map_err(|_| ())?;

            Ok(unsafe {
                ValueIndex::Long(Index::new_from(
                    slice,
                    start,
                    IntegerDescription::new(radix, ty, sign, FALSE),
                ))
            })
        }
        (IntegerType::Long, IntegerSignness::Signed) => {
            match radix {
                IntegerRadix::Binary => parse::<i64, &str, INTEGER_BINARY_FORMAT>(slice, opt),
                IntegerRadix::Decimal => parse::<i64, &str, INTEGER_DECIMAL_FORMAT>(slice, opt),
                IntegerRadix::Hexadecimal => {
                    parse::<i64, &str, INTEGER_HEXADECIMAL_FORMAT>(slice, opt)
                }
            }
            .map_err(|_| ())?;

            Ok(unsafe {
                ValueIndex::Long(Index::new_from(
                    slice,
                    start,
                    IntegerDescription::new(radix, ty, sign, FALSE),
                ))
            })
        }

        _ => Err(()),
    }
}

pub(super) fn parse_float(slice: &str, start: usize, ty: FloatType) -> Result<ValueIndex, ()> {
    use lexical::parse_with_options as parse;

    let repr = if slice.contains(['e', 'E']) {
        Ok(FloatRepresentation::Exponential)
    } else if slice.contains('.') {
        Ok(FloatRepresentation::Decimal)
    } else {
        Err(())
    }?;

    match ty {
        FloatType::Float => {
            parse::<f32, &str, FLOAT_FORMAT>(slice, &FLOAT_OPTIONS).map_err(|_| ())?;

            let desc = FloatDescription::new(repr, ty);
            Ok(unsafe { ValueIndex::Float(Index::new_from(slice, start, desc)) })
        }
        FloatType::Double => {
            parse::<f64, &str, FLOAT_FORMAT>(slice, &FLOAT_OPTIONS).map_err(|_| ())?;

            let desc = FloatDescription::new(repr, ty);
            Ok(unsafe { ValueIndex::Double(Index::new_from(slice, start, desc)) })
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// # Generics
///
/// `BOOLEANS` controls whether the strings "true" and "false" are parsed as
/// booleans instead of strings.
///
///  - If `false`, the result is always a `ValueIndex::String`.
pub(super) fn parse_string<const BOOLEANS: bool>(
    cursor: &mut Cursor<'_>,
    strings: &[Range<usize>],
) -> Result<ValueIndex, ()> {
    let start = cursor.position();
    match cursor.peek().ok_or(())? {
        // Unquoted string
        'A'..='Z' | '_' | 'a'..='z' => {
            let slice = cursor.until_char::<false, false, _>(
                |c| !matches!(c, '0'..='9' |  'A'..='Z' |  'a'..='z' |  '_' | '-' |  '.' | '+'),
            );

            if BOOLEANS && matches!(slice, "true" | "false") {
                let desc = BoolDescription::bool();
                return Ok(ValueIndex::Bool(unsafe { Index::new_from(slice, start, desc) }));
            }

            let desc = StringDescription::new(StringQuotes::None);
            Ok(unsafe { ValueIndex::String(Index::new_from(slice, start, desc)) })
        }
        // Quoted string
        c @ ('\"' | '\'') => {
            let Some(&range) = strings.iter().find(|r| r.start == start) else {
                #[cfg(debug_assertions)]
                unreachable!("String range not found when it should be known!?");
                #[cfg(not(debug_assertions))]
                unsafe {
                    core::hint::unreachable_unchecked()
                }
            };

            // Skip over the string
            let _ = cursor.split_range(range);

            let quotes = if c == '\"' { StringQuotes::Double } else { StringQuotes::Single };
            Ok(unsafe { ValueIndex::String(Index::new(range, StringDescription::new(quotes))) })
        }
        _ => Err(()),
    }
}
