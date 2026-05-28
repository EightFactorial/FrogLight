//! TODO

use alloc::string::String;
use core::fmt;

use bitflags::bitflags;

use crate::types::indexed::index::Indexable;

impl Indexable for String {
    type Description = StringDescription;
}

/// The radix of an integer value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StringQuotes {
    /// A string without quotes.
    None,
    /// A string with single quotes (`''`).
    Single,
    /// A string with double quotes (`""`).
    Double,
}

/// A description of an integer value.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringDescription(StringFlags);

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    struct StringFlags: u8 {
        const QUOTE_MASK = 0b0000_0011;
    }
}

impl StringDescription {
    /// Create a new [`StringDescription`].
    #[must_use]
    pub const fn new(quotes: StringQuotes) -> Self {
        let mut flags = StringFlags::empty();

        flags = match quotes {
            StringQuotes::None => flags.union(StringFlags::from_bits_truncate(0b0000_0000)),
            StringQuotes::Single => flags.union(StringFlags::from_bits_truncate(0b0000_0001)),
            StringQuotes::Double => flags.union(StringFlags::from_bits_truncate(0b0000_0010)),
        };

        Self(flags)
    }

    /// Get the [`StringQuotes`] of this value.
    #[must_use]
    pub const fn quotes(&self) -> StringQuotes {
        match self.0.intersection(StringFlags::QUOTE_MASK).bits() {
            0b0000_0000 => StringQuotes::None,
            0b0000_0001 => StringQuotes::Single,
            0b0000_0010 => StringQuotes::Double,
            #[cfg(debug_assertions)]
            _ => unreachable!(),
            #[cfg(not(debug_assertions))]
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}

impl fmt::Debug for StringDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StringDescription").field("quotes", &self.quotes()).finish()
    }
}
