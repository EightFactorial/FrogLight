use core::num::NonZeroU8;

use lexical::{
    NumberFormatBuilder, ParseFloatOptions, ParseFloatOptionsBuilder, ParseIntegerOptions,
    ParseIntegerOptionsBuilder,
};

use crate::types::indexed::location::{
    Location,
    settings::{
        FloatSettings, IntegerFormat, IntegerSettings, IntegerSignedness, IntegerSuffix,
        StringQuotes, StringSettings,
    },
};

pub trait TypeLocation {
    type Settings: Sized;
    type Value<'a>: Sized;

    /// Read a value from the given location and root string.
    ///
    /// # Safety
    ///
    /// The caller must ensure the location is valid for the given root string.
    #[must_use]
    unsafe fn read_from<'a>(location: &Location<Self>, root: &'a str) -> Self::Value<'a>;
}

macro_rules! impl_location {
    ( $settings:ident: $($ty:ident),+ => $block:tt ) => {
        $( impl_location!( $settings @ $ty : $ty => $block ); )+
    };
    ( $settings:ident @ $value:ty : $ty:ident => { $($tt:tt)* } ) => {
        impl TypeLocation for $ty {
            type Settings = $settings;
            type Value<'a> = $value;

            $($tt)*
        }
    };
}

// -------------------------------------------------------------------------------------------------

#[rustfmt::skip]
pub(crate) const INTEGER_DECIMAL_FORMAT: u128 = NumberFormatBuilder::new()
    .digit_separator(NonZeroU8::new(b'_')) // Set `_` as the digit separator.
    .internal_digit_separator(true)             // Allow digit separators between digits (`1_000`).
    .leading_digit_separator(false)             // Disallow digit separators at the start of the number (`_123`).
    .trailing_digit_separator(false)            // Disallow digit separators at the end of the number (`123_`).
    .consecutive_digit_separator(true)          // Allow consecutive digit separators (`1__000`).
    .build_strict();

#[rustfmt::skip]
pub(crate) const INTEGER_BINARY_FORMAT: u128 = NumberFormatBuilder::new()
    .radix(2)                                         // Set the radix to `2` for binary numbers.
    .exponent_base(NonZeroU8::new(2))
    .exponent_radix(NonZeroU8::new(2))
    .base_prefix(NonZeroU8::new(b'b'))                // Set `b` as the prefix for binary numbers (`0b1010`).
    .case_sensitive_base_prefix(true)           // Make the base prefix case-sensitive (`0b` is valid, but `0B` is not).
    .digit_separator(NonZeroU8::new(b'_')) // Set `_` as the digit separator.
    .internal_digit_separator(true)             // Allow digit separators between digits (`0b1_000`).
    .leading_digit_separator(false)             // Disallow digit separators at the start of the number (`0b_1010`).
    .trailing_digit_separator(false)            // Disallow digit separators at the end of the number (`0b1010_`).
    .consecutive_digit_separator(true)          // Allow consecutive digit separators (`0b1__000`).
    .build_strict();

#[rustfmt::skip]
pub(crate) const INTEGER_HEX_FORMAT: u128 = NumberFormatBuilder::new()
    .radix(16)                                        // Set the radix to `16` for hexadecimal numbers.
    .exponent_base(NonZeroU8::new(16))
    .exponent_radix(NonZeroU8::new(16))
    .base_prefix(NonZeroU8::new(b'x'))                // Set `x` as the prefix for binary numbers (`0x1A3F`).
    .case_sensitive_base_prefix(true)           // Make the base prefix case-sensitive (`0x` is valid, but `0X` is not).
    .digit_separator(NonZeroU8::new(b'_')) // Set `_` as the digit separator.
    .internal_digit_separator(true)             // Allow digit separators between digits (`0x1_A3F`).
    .leading_digit_separator(false)             // Disallow digit separators at the start of the number (`0x_1A3F`).
    .trailing_digit_separator(false)            // Disallow digit separators at the end of the number (`0x1A3F_`).
    .consecutive_digit_separator(true)          // Allow consecutive digit separators (`0x1__A3F`).
    .build_strict();

/// [`ParseIntegerOptions`] optimized for short integers.
pub(crate) static INTEGER_SHORT_OPTIONS: ParseIntegerOptions =
    ParseIntegerOptionsBuilder::new().no_multi_digit(true).build_strict();
/// [`ParseIntegerOptions`] optimized for long integers.
pub(crate) static INTEGER_LONG_OPTIONS: ParseIntegerOptions =
    ParseIntegerOptionsBuilder::new().no_multi_digit(false).build_strict();

impl_location!(IntegerSettings: u8, u16, u32, u64 => {
    unsafe fn read_from<'a>(location: &Location<Self>, root: &'a str) -> Self::Value<'a> {
        // SAFETY: The caller ensures this is safe.
        let mut slice: &str = unsafe { root.get_unchecked(location.index..location.index+location.length) };

        // Trim the suffix, which is not part of the actual number.
        match (location.settings.signedness(), location.settings.suffix()) {
            // Nothing to trim.
            (IntegerSignedness::None, IntegerSuffix::None) => {},
            // Trim the last character.
            (IntegerSignedness::None, _) | (_, IntegerSuffix::None) => {
                debug_assert!(slice.len() > 2, "Invalid Integer: {slice:?}?");
                slice = unsafe { slice.get_unchecked(..slice.len()-2) };
            },
            // Trim the last two characters.
            _ => {
                debug_assert!(slice.len() > 3, "Invalid Integer: {slice:?}?");
                slice = unsafe { slice.get_unchecked(..slice.len()-3) };
            }
        }

        // Enable/Disable multi-digit optimizations based on integer length.
        let options = if location.length < 12 { &INTEGER_SHORT_OPTIONS } else { &INTEGER_LONG_OPTIONS };
        // Select the appropriate parser based on the integer format.
        match location.settings.format() {
            // Parse decimal numbers using `INTEGER_DECIMAL_FORMAT`.
            #[cfg(debug_assertions)]
            IntegerFormat::Decimal => lexical::parse_with_options::<_, _, INTEGER_DECIMAL_FORMAT>(slice, options).unwrap(),
            #[cfg(not(debug_assertions))]
            IntegerFormat::Decimal => unsafe { lexical::parse_with_options::<_, _, INTEGER_DECIMAL_FORMAT>(slice, options).unwrap_unchecked() },

            // Parse binary numbers using `INTEGER_BINARY_FORMAT`.
            #[cfg(debug_assertions)]
            IntegerFormat::Binary => lexical::parse_with_options::<_, _, INTEGER_BINARY_FORMAT>(slice, options).unwrap(),
            #[cfg(not(debug_assertions))]
            IntegerFormat::Binary => unsafe { lexical::parse_with_options::<_, _, INTEGER_BINARY_FORMAT>(slice, options).unwrap_unchecked() },

            // Parse hexadecimal numbers using `INTEGER_HEX_FORMAT`.
            #[cfg(debug_assertions)]
            IntegerFormat::Hexadecimal => lexical::parse_with_options::<_, _, INTEGER_HEX_FORMAT>(slice, options).unwrap(),
            #[cfg(not(debug_assertions))]
            IntegerFormat::Hexadecimal => unsafe { lexical::parse_with_options::<_, _, INTEGER_HEX_FORMAT>(slice, options).unwrap_unchecked() },
        }
    }
});

// -------------------------------------------------------------------------------------------------

#[rustfmt::skip]
pub(crate) const FLOAT_FORMAT: u128 = NumberFormatBuilder::new()
    .required_integer_digits(false)             // Allow floats without digits before the decimal point (`.5`).
    .required_fraction_digits(false)            // Allow floats without digits after the decimal point (`1.`).
    .no_exponent_notation(false)                // Allow exponent notation for floats (`1e10`).
    .case_sensitive_exponent(false)             // Make the exponent notation case-insensitive (`1e10` and `1E10`).
    .digit_separator(NonZeroU8::new(b'_')) // Set `_` as the digit separator.
    .internal_digit_separator(true)             // Allow digit separators between digits (`1_000.0`).
    .leading_digit_separator(false)             // Disallow digit separators at the start of the number (`_123.0`).
    .trailing_digit_separator(false)            // Disallow digit separators at the end of the number (`123.0_`).
    .consecutive_digit_separator(true)          // Allow consecutive digit separators (`1__000.0`).
    .build_strict();

pub(crate) static FLOAT_OPTIONS: ParseFloatOptions = ParseFloatOptionsBuilder::new().build_strict();

impl_location!(FloatSettings: f32, f64 => {
    unsafe fn read_from<'a>(location: &Location<Self>, root: &'a str) -> Self::Value<'a> {
        // SAFETY: The caller ensures this is safe.
        let slice: &str = unsafe { root.get_unchecked(location.index..location.index+location.length) };

        // Trim the suffix, which is not part of the actual number.
        debug_assert!(slice.len() > 2, "Invalid Float: {slice:?}?");
        let slice = unsafe { slice.get_unchecked(..slice.len()-2) };

        // Parse the float using `FLOAT_FORMAT`.
        #[cfg(debug_assertions)]
        { lexical::parse_with_options::<_, _, FLOAT_FORMAT>(slice, &FLOAT_OPTIONS).unwrap() }
        #[cfg(not(debug_assertions))]
        unsafe { lexical::parse_with_options::<_, _, FLOAT_FORMAT>(slice, &FLOAT_OPTIONS).unwrap_unchecked() }
    }
});

// -------------------------------------------------------------------------------------------------

impl_location!(StringSettings @ &'a str: str => {
    unsafe fn read_from<'a>(location: &Location<Self>, root: &'a str) -> Self::Value<'a> {
        // SAFETY: The caller ensures this is safe.
        let mut slice: &str = unsafe { root.get_unchecked(location.index..location.index+location.length) };

        // Trim the quotes, which are not part of the actual string.
        if location.settings().quotes() != StringQuotes::None {
            debug_assert!(location.length >= 2, "Invalid String: {slice:?}");
            slice = unsafe { slice.get_unchecked(1..slice.len()-1) }
        }

        slice
    }
});
