//! TODO

use core::fmt;

use bitflags::bitflags;

use crate::types::indexed::index::Indexable;

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
    }
}

impl IntegerDescription {
    /// Create a new [`IntegerDescription`].
    #[must_use]
    pub const fn new(radix: IntegerRadix, ty: IntegerType, sign: IntegerSignness) -> Self {
        let mut flags = IntegerFlags::empty();

        flags = match radix {
            IntegerRadix::Binary => flags.union(IntegerFlags::from_bits_retain(0b0000_0000)),
            IntegerRadix::Decimal => flags.union(IntegerFlags::from_bits_retain(0b0000_0001)),
            IntegerRadix::Hexadecimal => flags.union(IntegerFlags::from_bits_retain(0b0000_0010)),
        };

        flags = match ty {
            IntegerType::None => flags.union(IntegerFlags::from_bits_retain(0b0000_0000)),
            IntegerType::Byte => flags.union(IntegerFlags::from_bits_retain(0b0000_0100)),
            IntegerType::Short => flags.union(IntegerFlags::from_bits_retain(0b0000_1000)),
            IntegerType::Int => flags.union(IntegerFlags::from_bits_retain(0b0000_1100)),
            IntegerType::Long => flags.union(IntegerFlags::from_bits_retain(0b0001_0000)),
        };

        flags = match sign {
            IntegerSignness::None => flags.union(IntegerFlags::from_bits_retain(0b0000_0000)),
            IntegerSignness::Signed => flags.union(IntegerFlags::from_bits_retain(0b0010_0000)),
            IntegerSignness::Unsigned => flags.union(IntegerFlags::from_bits_retain(0b0100_0000)),
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

    /// Get the [`IntegerType`] of this value.
    #[must_use]
    pub const fn ty(&self) -> IntegerType {
        match self.0.intersection(IntegerFlags::TYPE_MASK).bits() {
            0b0000_0000 => IntegerType::None,
            0b0000_0100 => IntegerType::Byte,
            0b0000_1000 => IntegerType::Short,
            0b0000_1100 => IntegerType::Int,
            0b0001_0000 => IntegerType::Long,
            #[cfg(debug_assertions)]
            _ => unreachable!(),
            #[cfg(not(debug_assertions))]
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
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
}

impl fmt::Debug for IntegerDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IntegerDescription")
            .field("radix", &self.radix())
            .field("ty", &self.ty())
            .field("signness", &self.signness())
            .finish()
    }
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
}

impl fmt::Debug for FloatDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FloatDescription")
            .field("repr", &self.repr())
            .field("ty", &self.ty())
            .finish()
    }
}
