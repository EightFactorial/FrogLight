#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntegerSettings(IntegerFlags);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntegerFormat {
    Decimal,
    Hexadecimal,
    Binary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntegerSignedness {
    None,
    Signed,
    Unsigned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntegerSuffix {
    None,
    Byte,
    Short,
    Int,
    Long,
}

impl IntegerSettings {
    /// Create a new [`IntegerSettings`] with the given format, signedness, and
    /// suffix.
    #[must_use]
    pub const fn new(
        format: IntegerFormat,
        signedness: IntegerSignedness,
        suffix: IntegerSuffix,
    ) -> Self {
        let mut flags = IntegerFlags::empty();

        flags = match format {
            IntegerFormat::Decimal => flags,
            IntegerFormat::Hexadecimal => flags.union(IntegerFlags::from_bits_retain(0b0100_0000)),
            IntegerFormat::Binary => flags.union(IntegerFlags::from_bits_retain(0b1000_0000)),
        };

        flags = match signedness {
            IntegerSignedness::None => flags,
            IntegerSignedness::Signed => flags.union(IntegerFlags::from_bits_retain(0b0000_1000)),
            IntegerSignedness::Unsigned => flags.union(IntegerFlags::from_bits_retain(0b0001_0000)),
        };

        flags = match suffix {
            IntegerSuffix::None => flags,
            IntegerSuffix::Byte => flags.union(IntegerFlags::from_bits_retain(0b0000_0001)),
            IntegerSuffix::Short => flags.union(IntegerFlags::from_bits_retain(0b0000_0010)),
            IntegerSuffix::Int => flags.union(IntegerFlags::from_bits_retain(0b0000_0011)),
            IntegerSuffix::Long => flags.union(IntegerFlags::from_bits_retain(0b0000_0100)),
        };

        Self(flags)
    }

    /// Get the [`IntegerFormat`] of this [`IntegerSettings`].
    #[must_use]
    pub const fn format(&self) -> IntegerFormat {
        match self.0.intersection(IntegerFlags::FORMAT).bits() {
            0b0000_0000 => IntegerFormat::Decimal,
            0b0100_0000 => IntegerFormat::Hexadecimal,
            0b1000_0000 => IntegerFormat::Binary,
            #[cfg(debug_assertions)]
            _ => unreachable!(),
            #[cfg(not(debug_assertions))]
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    /// Get the [`IntegerSignedness`] of this [`IntegerSettings`].
    #[must_use]
    pub const fn signedness(&self) -> IntegerSignedness {
        match self.0.intersection(IntegerFlags::SIGN).bits() {
            0b0000_0000 => IntegerSignedness::None,
            0b0000_1000 => IntegerSignedness::Signed,
            0b0001_0000 => IntegerSignedness::Unsigned,
            #[cfg(debug_assertions)]
            _ => unreachable!(),
            #[cfg(not(debug_assertions))]
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    /// Get the [`IntegerSuffix`] of this [`IntegerSettings`].
    #[must_use]
    pub const fn suffix(&self) -> IntegerSuffix {
        match self.0.intersection(IntegerFlags::SUFFIX).bits() {
            0b0000_0000 => IntegerSuffix::None,
            0b0000_0001 => IntegerSuffix::Byte,
            0b0000_0010 => IntegerSuffix::Short,
            0b0000_0011 => IntegerSuffix::Int,
            0b0000_0100 => IntegerSuffix::Long,
            #[cfg(debug_assertions)]
            _ => unreachable!(),
            #[cfg(not(debug_assertions))]
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FloatSettings(FloatFlags);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatFormat {
    Decimal,
    Exponential,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatSuffix {
    Float,
    Double,
}

impl FloatSettings {
    /// Create a new [`FloatSettings`] with the given format and suffix.
    #[must_use]
    pub const fn new(format: FloatFormat, suffix: FloatSuffix) -> Self {
        let mut flags = FloatFlags::empty();

        flags = match format {
            FloatFormat::Decimal => flags,
            FloatFormat::Exponential => flags.union(FloatFlags::from_bits_retain(0b1000_0000)),
        };

        flags = match suffix {
            FloatSuffix::Float => flags,
            FloatSuffix::Double => flags.union(FloatFlags::from_bits_retain(0b0000_0001)),
        };

        Self(flags)
    }

    /// Get the [`FloatFormat`] of this [`FloatSettings`].
    #[must_use]
    pub const fn format(&self) -> FloatFormat {
        match self.0.intersection(FloatFlags::FORMAT).bits() {
            0b0000_0000 => FloatFormat::Decimal,
            0b1000_0000 => FloatFormat::Exponential,
            #[cfg(debug_assertions)]
            _ => unreachable!(),
            #[cfg(not(debug_assertions))]
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    /// Get the [`FloatSuffix`] of this [`FloatSettings`].
    #[must_use]
    pub const fn suffix(&self) -> FloatSuffix {
        match self.0.intersection(FloatFlags::SUFFIX).bits() {
            0b0000_0000 => FloatSuffix::Float,
            0b0000_0001 => FloatSuffix::Double,
            #[cfg(debug_assertions)]
            _ => unreachable!(),
            #[cfg(not(debug_assertions))]
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringSettings(StringFlags);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StringQuotes {
    None,
    Single,
    Double,
}

impl StringSettings {
    /// Create a new [`StringSettings`] with the given quotes.
    #[must_use]
    pub const fn new(quotes: StringQuotes) -> Self {
        let mut flags = StringFlags::empty();

        flags = match quotes {
            StringQuotes::None => flags,
            StringQuotes::Single => flags.union(StringFlags::from_bits_retain(0b0000_0001)),
            StringQuotes::Double => flags.union(StringFlags::from_bits_retain(0b0000_0010)),
        };

        Self(flags)
    }

    /// Get the [`StringQuotes`] of this [`StringSettings`].
    #[must_use]
    pub const fn quotes(&self) -> StringQuotes {
        match self.0.intersection(StringFlags::QUOTES).bits() {
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

// -------------------------------------------------------------------------------------------------

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct IntegerFlags: u8 {
        const SUFFIX = 0b0000_0111;
        const SIGN = 0b0001_1000;
        const FORMAT = 0b1100_0000;

        const UNUSED = 0b0010_0000;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct FloatFlags: u8 {
        const SUFFIX = 0b0000_0001;
        const FORMAT = 0b1000_0000;

        const UNUSED = 0b0111_1110;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct StringFlags: u8 {
        const QUOTES = 0b0000_0011;

        const UNUSED = 0b1111_1100;
    }
}
