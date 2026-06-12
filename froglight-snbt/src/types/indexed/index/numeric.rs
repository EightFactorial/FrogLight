//! TODO

use core::{fmt, num::NonZeroU8};

use bitflags::bitflags;
use lexical::{
    FromLexicalWithOptions, NumberFormatBuilder, ParseFloatOptions, ParseIntegerOptions,
};

use crate::types::indexed::index::{
    Index, Indexable, IndexableValue,
    bool::{BoolDescription, BooleanOperation},
};

/// An integer value.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Integer;

impl Indexable for Integer {
    type Description = IntegerDescription;
}

/// The radix of an integer value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntegerRadix {
    /// A value in base 2.
    Binary,
    /// A value in base 10.
    Decimal,
    /// A value in base 16.
    Hexadecimal,
}

/// The type of an integer value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntegerType {
    /// An untyped value.
    ///
    /// Equivalent to [`IntegerType::Int`].
    None,
    /// A [`bool`] value.
    Bool,
    /// A [`u8`] value.
    Byte,
    /// A [`u16`] value.
    Short,
    /// A [`u32`] value.
    Int,
    /// A [`u64`] value.
    Long,
}

/// The signness of an integer value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntegerSignness {
    /// An untyped value.
    ///
    /// Equivalent to [`IntegerSignness::Unsigned`].
    None,
    /// A signed value.
    Signed,
    /// An unsigned value.
    Unsigned,
}

/// A description of an integer value.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntegerDescription(IntegerFlags);

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    struct IntegerFlags: u8 {
        const RADIX_MASK = 0b0000_0011;
        const TYPE_MASK = 0b0001_1100;
        const SIGNNESS_MASK = 0b0110_0000;
        const BOOLEAN_MASK = 0b1000_0000;
    }
}

impl IntegerDescription {
    /// Create a new [`IntegerDescription`].
    #[must_use]
    pub const fn new(
        radix: IntegerRadix,
        ty: IntegerType,
        sign: IntegerSignness,
        operation: BooleanOperation,
    ) -> Self {
        let mut flags = IntegerFlags::empty();

        flags = match radix {
            IntegerRadix::Binary => flags.union(IntegerFlags::from_bits_retain(0b0000_0000)),
            IntegerRadix::Decimal => flags.union(IntegerFlags::from_bits_retain(0b0000_0001)),
            IntegerRadix::Hexadecimal => flags.union(IntegerFlags::from_bits_retain(0b0000_0010)),
        };

        flags = match ty {
            IntegerType::None => flags.union(IntegerFlags::from_bits_retain(0b0000_0000)),
            IntegerType::Bool => flags.union(IntegerFlags::from_bits_retain(0b0000_0100)),
            IntegerType::Byte => flags.union(IntegerFlags::from_bits_retain(0b0000_1000)),
            IntegerType::Short => flags.union(IntegerFlags::from_bits_retain(0b0000_1100)),
            IntegerType::Int => flags.union(IntegerFlags::from_bits_retain(0b0001_0000)),
            IntegerType::Long => flags.union(IntegerFlags::from_bits_retain(0b0001_0100)),
        };

        flags = match sign {
            IntegerSignness::None => flags.union(IntegerFlags::from_bits_retain(0b0000_0000)),
            IntegerSignness::Signed => flags.union(IntegerFlags::from_bits_retain(0b0010_0000)),
            IntegerSignness::Unsigned => flags.union(IntegerFlags::from_bits_retain(0b0100_0000)),
        };

        flags = match operation {
            BooleanOperation::False => flags.union(IntegerFlags::from_bits_retain(0b0000_0000)),
            BooleanOperation::True => flags.union(IntegerFlags::from_bits_retain(0b1000_0000)),
        };

        Self(flags)
    }

    /// Get the [`IntegerRadix`] of this value.
    #[must_use]
    pub const fn radix(&self) -> IntegerRadix {
        match self.0.intersection(IntegerFlags::RADIX_MASK).bits() {
            0b0000_0000 => IntegerRadix::Binary,
            0b0000_0001 => IntegerRadix::Decimal,
            0b0000_0010 => IntegerRadix::Hexadecimal,
            #[cfg(debug_assertions)]
            _ => unreachable!(),
            #[cfg(not(debug_assertions))]
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    /// Set the [`IntegerRadix`] of this value.
    #[must_use]
    pub const fn with_radix(self, radix: IntegerRadix) -> Self {
        Self::new(radix, self.ty(), self.signness(), self.boolean_operation())
    }

    /// Get the [`IntegerType`] of this value.
    #[must_use]
    pub const fn ty(&self) -> IntegerType {
        match self.0.intersection(IntegerFlags::TYPE_MASK).bits() {
            0b0000_0000 => IntegerType::None,
            0b0000_0100 => IntegerType::Bool,
            0b0000_1000 => IntegerType::Byte,
            0b0000_1100 => IntegerType::Short,
            0b0001_0000 => IntegerType::Int,
            0b0001_0100 => IntegerType::Long,
            #[cfg(debug_assertions)]
            _ => unreachable!(),
            #[cfg(not(debug_assertions))]
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    /// Set the [`IntegerType`] of this value.
    #[must_use]
    pub const fn with_ty(self, ty: IntegerType) -> Self {
        Self::new(self.radix(), ty, self.signness(), self.boolean_operation())
    }

    /// Get the [`IntegerSignness`] of this value.
    #[must_use]
    pub const fn signness(&self) -> IntegerSignness {
        match self.0.intersection(IntegerFlags::SIGNNESS_MASK).bits() {
            0b0000_0000 => IntegerSignness::None,
            0b0010_0000 => IntegerSignness::Signed,
            0b0100_0000 => IntegerSignness::Unsigned,
            #[cfg(debug_assertions)]
            _ => unreachable!(),
            #[cfg(not(debug_assertions))]
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    /// Set the [`IntegerSignness`] of this value.
    #[must_use]
    pub const fn with_signness(self, signness: IntegerSignness) -> Self {
        Self::new(self.radix(), self.ty(), signness, self.boolean_operation())
    }

    /// Get the [`BooleanOperation`] of this value.
    #[must_use]
    pub const fn boolean_operation(&self) -> BooleanOperation {
        match self.0.intersection(IntegerFlags::BOOLEAN_MASK).bits() {
            0b0000_0000 => BooleanOperation::False,
            0b1000_0000 => BooleanOperation::True,
            #[cfg(debug_assertions)]
            _ => unreachable!(),
            #[cfg(not(debug_assertions))]
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    /// Set the [`BooleanOperation`] of this value.
    #[must_use]
    pub const fn with_boolean_operation(self, operation: BooleanOperation) -> Self {
        Self::new(self.radix(), self.ty(), self.signness(), operation)
    }
}

impl fmt::Debug for IntegerDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IntegerDescription")
            .field("radix", &self.radix())
            .field("ty", &self.ty())
            .field("signness", &self.signness())
            .field("boolean_operation", &self.boolean_operation())
            .finish()
    }
}

// -------------------------------------------------------------------------------------------------

/// An integer value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntegerValue {
    /// A [`u8`] value.
    Byte(u8),
    /// A [`u16`] value.
    Short(u16),
    /// A [`u32`] value.
    Int(u32),
    /// A [`u64`] value.
    Long(u64),
}

#[expect(clippy::cast_possible_truncation, reason = "Expected Behavior")]
impl IntegerValue {
    /// Get this [`IntegerValue`] as a [`bool`].
    #[must_use]
    pub const fn as_bool(self) -> bool {
        match self {
            Self::Byte(v) => v != 0,
            Self::Short(v) => v != 0,
            Self::Int(v) => v != 0,
            Self::Long(v) => v != 0,
        }
    }

    /// Get this [`IntegerValue`] as a [`u8`].
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::Byte(v) => v,
            Self::Short(v) => v as u8,
            Self::Int(v) => v as u8,
            Self::Long(v) => v as u8,
        }
    }

    /// Get this [`IntegerValue`] as a [`u16`].
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        match self {
            Self::Byte(v) => v as u16,
            Self::Short(v) => v,
            Self::Int(v) => v as u16,
            Self::Long(v) => v as u16,
        }
    }

    /// Get this [`IntegerValue`] as a [`u32`].
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        match self {
            Self::Byte(v) => v as u32,
            Self::Short(v) => v as u32,
            Self::Int(v) => v,
            Self::Long(v) => v as u32,
        }
    }

    /// Get this [`IntegerValue`] as a [`u64`].
    #[must_use]
    pub const fn as_u64(self) -> u64 {
        match self {
            Self::Byte(v) => v as u64,
            Self::Short(v) => v as u64,
            Self::Int(v) => v as u64,
            Self::Long(v) => v,
        }
    }
}

pub(crate) static INTEGER_DECIMAL_FORMAT: u128 = NumberFormatBuilder::new()
    .no_exponent_notation(true)
    .digit_separator(NonZeroU8::new(b'_'))
    .integer_digit_separator_flags(true)
    .leading_digit_separator(false)
    .trailing_digit_separator(false)
    .internal_digit_separator(true)
    .consecutive_digit_separator(true)
    .build_strict();

pub(crate) static INTEGER_BINARY_FORMAT: u128 = NumberFormatBuilder::new()
    .radix(2)
    .exponent_base(NonZeroU8::new(2))
    .exponent_radix(NonZeroU8::new(2))
    .base_prefix(NonZeroU8::new(b'b'))
    .case_sensitive_base_prefix(true)
    .no_exponent_notation(true)
    .digit_separator(NonZeroU8::new(b'_'))
    .integer_digit_separator_flags(true)
    .leading_digit_separator(false)
    .trailing_digit_separator(false)
    .internal_digit_separator(true)
    .consecutive_digit_separator(true)
    .build_strict();

pub(crate) static INTEGER_HEXADECIMAL_FORMAT: u128 = NumberFormatBuilder::new()
    .radix(16)
    .exponent_base(NonZeroU8::new(16))
    .exponent_radix(NonZeroU8::new(16))
    .base_prefix(NonZeroU8::new(b'x'))
    .case_sensitive_base_prefix(true)
    .no_exponent_notation(true)
    .digit_separator(NonZeroU8::new(b'_'))
    .integer_digit_separator_flags(true)
    .leading_digit_separator(false)
    .trailing_digit_separator(false)
    .internal_digit_separator(true)
    .consecutive_digit_separator(true)
    .build_strict();

pub(crate) static INTEGER_OPTIONS: ParseIntegerOptions =
    ParseIntegerOptions::builder().no_multi_digit(true).build_strict();

pub(crate) static INTEGER_MULTIDIGIT_OPTIONS: ParseIntegerOptions =
    ParseIntegerOptions::builder().no_multi_digit(false).build_strict();

impl IndexableValue for Integer {
    type Value<'a> = IntegerValue;

    unsafe fn read_value(index: Index<Self>, root: &str) -> Self::Value<'_> {
        // SAFETY: The caller ensures that this is safe.
        let mut slice = unsafe { root.get_unchecked(index.range) };

        let desc = index.description();

        // Handle `IntegerType::Bool` seperately.
        if matches!(desc.ty(), IntegerType::Bool) {
            let desc = BoolDescription::Boolean;
            return unsafe {
                let index = Index::<bool>::new(index.range, desc);
                IntegerValue::Byte(u8::from(index.read_value(root).as_bool()))
            };
        }

        if !matches!(desc.ty(), IntegerType::None)
            && slice
                .chars()
                .last()
                .is_some_and(|c| matches!(c, 'b' | 'B' | 's' | 'S' | 'i' | 'I' | 'l' | 'L'))
        {
            slice = &slice[..slice.len() - 1];

            if !matches!(desc.signness(), IntegerSignness::None)
                && slice.chars().last().is_some_and(|c| matches!(c, 'u' | 's'))
            {
                slice = &slice[..slice.len() - 1];
            }
        }

        // If the string is long, enable multi-digit optimizations.
        let options =
            if slice.len() >= 12 { &INTEGER_MULTIDIGIT_OPTIONS } else { &INTEGER_OPTIONS };

        // SAFETY: `Index` guarantees that this is valid.
        unsafe {
            match desc.ty() {
                // `IntegerType::Bool` is handled above.
                IntegerType::Bool => core::hint::unreachable_unchecked(),
                IntegerType::Byte => match desc.radix() {
                    IntegerRadix::Binary => read_binary::<u8>(slice, options),
                    IntegerRadix::Decimal => read_decimal::<u8>(slice, options),
                    IntegerRadix::Hexadecimal => read_hexadecimal::<u8>(slice, options),
                },
                IntegerType::Short => match desc.radix() {
                    IntegerRadix::Binary => read_binary::<u16>(slice, options),
                    IntegerRadix::Decimal => read_decimal::<u16>(slice, options),
                    IntegerRadix::Hexadecimal => read_hexadecimal::<u16>(slice, options),
                },
                IntegerType::None | IntegerType::Int => match desc.radix() {
                    IntegerRadix::Binary => read_binary::<u32>(slice, options),
                    IntegerRadix::Decimal => read_decimal::<u32>(slice, options),
                    IntegerRadix::Hexadecimal => read_hexadecimal::<u32>(slice, options),
                },
                IntegerType::Long => match desc.radix() {
                    IntegerRadix::Binary => read_binary::<u64>(slice, options),
                    IntegerRadix::Decimal => read_decimal::<u64>(slice, options),
                    IntegerRadix::Hexadecimal => read_hexadecimal::<u64>(slice, options),
                },
            }
        }
    }
}

cfg_select! {
    debug_assertions => {
        #[inline]
        unsafe fn read_binary<T: Into<IntegerValue> + FromLexicalWithOptions>(
            slice: &str,
            options: &T::Options,
        ) -> IntegerValue {
            lexical::parse_with_options::<T, &str, INTEGER_BINARY_FORMAT>(slice, options).unwrap().into()
        }

        #[inline]
        unsafe fn read_decimal<T: Into<IntegerValue> + FromLexicalWithOptions>(
            slice: &str,
            options: &T::Options,
        ) -> IntegerValue {
            lexical::parse_with_options::<T, &str, INTEGER_DECIMAL_FORMAT>(slice, options).unwrap().into()
        }

        #[inline]
        unsafe fn read_hexadecimal<T: Into<IntegerValue> + FromLexicalWithOptions>(
            slice: &str,
            options: &T::Options,
        ) -> IntegerValue {
            lexical::parse_with_options::<T, &str, INTEGER_HEXADECIMAL_FORMAT>(slice, options).unwrap().into()
        }
    }
    _ => {
        #[inline]
        unsafe fn read_binary<T: Into<IntegerValue> + FromLexicalWithOptions>(
            slice: &str,
            options: &T::Options,
        ) -> IntegerValue {
            // SAFETY: `Index` guarantees that this is valid.
            unsafe { lexical::parse_with_options::<T, &str, INTEGER_BINARY_FORMAT>(slice, options).unwrap_unchecked().into() }
        }

        #[inline]
        unsafe fn read_decimal<T: Into<IntegerValue> + FromLexicalWithOptions>(
            slice: &str,
            options: &T::Options,
        ) -> IntegerValue {
            // SAFETY: `Index` guarantees that this is valid.
            unsafe { lexical::parse_with_options::<T, &str, INTEGER_DECIMAL_FORMAT>(slice, options).unwrap_unchecked().into() }
        }

        #[inline]
        unsafe fn read_hexadecimal<T: Into<IntegerValue> + FromLexicalWithOptions>(
            slice: &str,
            options: &T::Options,
        ) -> IntegerValue {
            // SAFETY: `Index` guarantees that this is valid.
            unsafe { lexical::parse_with_options::<T, &str, INTEGER_HEXADECIMAL_FORMAT>(slice, options).unwrap_unchecked().into() }
        }
    }
}

impl From<u8> for IntegerValue {
    #[inline]
    fn from(value: u8) -> Self { Self::Byte(value) }
}
impl From<u16> for IntegerValue {
    #[inline]
    fn from(value: u16) -> Self { Self::Short(value) }
}
impl From<u32> for IntegerValue {
    #[inline]
    fn from(value: u32) -> Self { Self::Int(value) }
}
impl From<u64> for IntegerValue {
    #[inline]
    fn from(value: u64) -> Self { Self::Long(value) }
}

impl From<IntegerValue> for u8 {
    #[inline]
    fn from(value: IntegerValue) -> Self { value.as_u8() }
}
impl From<IntegerValue> for u16 {
    #[inline]
    fn from(value: IntegerValue) -> Self { value.as_u16() }
}
impl From<IntegerValue> for u32 {
    #[inline]
    fn from(value: IntegerValue) -> Self { value.as_u32() }
}
impl From<IntegerValue> for u64 {
    #[inline]
    fn from(value: IntegerValue) -> Self { value.as_u64() }
}

// -------------------------------------------------------------------------------------------------

/// A floating-point value.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Float;

/// The representation of a floating-point value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatRepresentation {
    /// A value in decimal notation.
    Decimal,
    /// A value in exponential notation.
    Exponential,
}

/// The type of a floating-point value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatType {
    /// A [`f32`] value.
    Float,
    /// A [`f64`] value.
    Double,
}

impl Indexable for Float {
    type Description = FloatDescription;
}

/// A description of a floating-point value.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FloatDescription(FloatFlags);

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    struct FloatFlags: u8 {
        const REPR_MASK = 0b0000_0001;
        const TYPE_MASK = 0b0000_0010;
    }
}

impl FloatDescription {
    /// Create a new [`FloatDescription`].
    #[must_use]
    pub const fn new(repr: FloatRepresentation, ty: FloatType) -> Self {
        let mut flags = FloatFlags::empty();

        flags = match repr {
            FloatRepresentation::Decimal => flags.union(FloatFlags::from_bits_retain(0b0000_0000)),
            FloatRepresentation::Exponential => {
                flags.union(FloatFlags::from_bits_retain(0b0000_0001))
            }
        };

        flags = match ty {
            FloatType::Float => flags.union(FloatFlags::from_bits_retain(0b0000_0000)),
            FloatType::Double => flags.union(FloatFlags::from_bits_retain(0b0000_0010)),
        };

        Self(flags)
    }

    /// Get the [`FloatRepresentation`] of this value.
    #[must_use]
    pub const fn repr(&self) -> FloatRepresentation {
        match self.0.intersection(FloatFlags::REPR_MASK).bits() {
            0b0000_0000 => FloatRepresentation::Decimal,
            0b0000_0001 => FloatRepresentation::Exponential,
            #[cfg(debug_assertions)]
            _ => unreachable!(),
            #[cfg(not(debug_assertions))]
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    /// Set the [`FloatRepresentation`] of this value.
    #[must_use]
    pub const fn with_repr(self, repr: FloatRepresentation) -> Self { Self::new(repr, self.ty()) }

    /// Get the [`FloatType`] of this value.
    #[must_use]
    pub const fn ty(&self) -> FloatType {
        match self.0.intersection(FloatFlags::TYPE_MASK).bits() {
            0b0000_0000 => FloatType::Float,
            0b0000_0010 => FloatType::Double,
            #[cfg(debug_assertions)]
            _ => unreachable!(),
            #[cfg(not(debug_assertions))]
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    /// Set the [`FloatType`] of this value.
    #[must_use]
    pub const fn with_ty(self, ty: FloatType) -> Self { Self::new(self.repr(), ty) }
}

impl fmt::Debug for FloatDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FloatDescription")
            .field("repr", &self.repr())
            .field("ty", &self.ty())
            .finish()
    }
}

// -------------------------------------------------------------------------------------------------

/// A floating-point value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FloatValue {
    /// A [`f32`] value.
    Float(f32),
    /// A [`f64`] value.
    Double(f64),
}

#[expect(clippy::cast_possible_truncation, reason = "Expected Behavior")]
impl FloatValue {
    /// Get this [`FloatValue`] as a [`f32`].
    #[must_use]
    pub const fn as_f32(self) -> f32 {
        match self {
            Self::Float(v) => v,
            Self::Double(v) => v as f32,
        }
    }

    /// Get this [`FloatValue`] as a [`f64`].
    #[must_use]
    pub const fn as_f64(self) -> f64 {
        match self {
            Self::Float(v) => v as f64,
            Self::Double(v) => v,
        }
    }
}

pub(crate) static FLOAT_FORMAT: u128 = NumberFormatBuilder::new()
    .no_exponent_notation(false)
    .digit_separator(NonZeroU8::new(b'_'))
    .integer_digit_separator_flags(true)
    .leading_digit_separator(false)
    .trailing_digit_separator(false)
    .internal_digit_separator(true)
    .consecutive_digit_separator(true)
    .build_strict();

pub(crate) static FLOAT_OPTIONS: ParseFloatOptions = ParseFloatOptions::builder().build_strict();

impl IndexableValue for Float {
    type Value<'a> = FloatValue;

    unsafe fn read_value(index: Index<Self>, root: &str) -> Self::Value<'_> {
        // SAFETY: The caller ensures that this is safe.
        let slice = unsafe { root.get_unchecked(index.range) };

        // SAFETY: `Index` guarantees that this is valid.
        unsafe {
            match index.description().ty() {
                FloatType::Float => parse_float::<f32>(slice, &FLOAT_OPTIONS),
                FloatType::Double => parse_float::<f64>(slice, &FLOAT_OPTIONS),
            }
        }
    }
}

cfg_select! {
    debug_assertions => {
        #[inline]
        unsafe fn parse_float<T: Into<FloatValue> + FromLexicalWithOptions>(
            slice: &str,
            options: &T::Options,
        ) -> FloatValue {
            lexical::parse_with_options::<T, &str, FLOAT_FORMAT>(slice, options).unwrap().into()
        }
    }
    _ => {
        #[inline]
        unsafe fn parse_float<T: Into<FloatValue> + FromLexicalWithOptions>(
            slice: &str,
            options: &T::Options,
        ) -> FloatValue {
            // SAFETY: `Index` guarantees that this is valid.
            unsafe { lexical::parse_with_options::<T, &str, FLOAT_FORMAT>(slice, &options).unwrap_unchecked().into() }
        }
    }
}

impl From<f32> for FloatValue {
    #[inline]
    fn from(value: f32) -> Self { Self::Float(value) }
}
impl From<f64> for FloatValue {
    #[inline]
    fn from(value: f64) -> Self { Self::Double(value) }
}

impl From<FloatValue> for f32 {
    #[inline]
    fn from(value: FloatValue) -> Self { value.as_f32() }
}
impl From<FloatValue> for f64 {
    #[inline]
    fn from(value: FloatValue) -> Self { value.as_f64() }
}
