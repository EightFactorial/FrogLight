pub mod settings;

mod traits;
pub use traits::TypeLocation;

pub struct Location<T: TypeLocation + ?Sized> {
    index: usize,
    length: usize,
    settings: T::Settings,
}

impl<T: TypeLocation + ?Sized> Location<T> {
    /// Create a new [`Locataion`] with the given index, length, and settings.
    #[inline]
    #[must_use]
    pub const fn new(index: usize, length: usize, settings: T::Settings) -> Self {
        Self { index, length, settings }
    }

    /// Create a new [`Locataion`] using the given slice, start index, and
    /// settings.
    #[inline]
    #[must_use]
    pub const fn new_using(slice: &str, start: usize, settings: T::Settings) -> Self {
        Self::new(start, slice.len(), settings)
    }

    /// Get the settings of this location.
    #[inline]
    #[must_use]
    pub const fn settings(&self) -> &T::Settings { &self.settings }

    /// Reads the value from the given root string.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the location is valid for the given root
    /// string.
    #[inline]
    #[must_use]
    pub unsafe fn read_from<'a>(&self, root: &'a str) -> T::Value<'a> {
        // SAFETY: The caller ensures this is safe.
        unsafe { T::read_from(self, root) }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntegerValue {
    Byte(u8),
    Short(u16),
    Int(u32),
    Long(u64),
}

pub enum IntegerLocation {
    Byte(Location<u8>),
    Short(Location<u16>),
    Int(Location<u32>),
    Long(Location<u64>),
}

impl IntegerLocation {
    /// Reads the integer value from the given root string.
    ///
    ///
    /// # Safety
    ///
    /// The caller must ensure that the location is valid for the given root
    /// string.
    #[must_use]
    unsafe fn read_from(&self, root: &str) -> IntegerValue {
        // SAFETY: The caller ensures this is safe.
        unsafe {
            match self {
                IntegerLocation::Byte(l) => IntegerValue::Byte(l.read_from(root)),
                IntegerLocation::Short(l) => IntegerValue::Short(l.read_from(root)),
                IntegerLocation::Int(l) => IntegerValue::Int(l.read_from(root)),
                IntegerLocation::Long(l) => IntegerValue::Long(l.read_from(root)),
            }
        }
    }

    /// Reads the integer value from the given root string.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the location is valid for the given root
    /// string.
    #[must_use]
    unsafe fn read_from_u64(&self, root: &str) -> u64 {
        // SAFETY: The caller ensures this is safe.
        match unsafe { self.read_from(root) } {
            IntegerValue::Byte(v) => u64::from(v),
            IntegerValue::Short(v) => u64::from(v),
            IntegerValue::Int(v) => u64::from(v),
            IntegerValue::Long(v) => v,
        }
    }

    /// Reads the boolean value from the given root string.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the location is valid for the given root
    /// string.
    #[must_use]
    unsafe fn read_from_bool(&self, root: &str) -> bool {
        // SAFETY: The caller ensures this is safe.
        match unsafe { self.read_from(root) } {
            IntegerValue::Byte(v) => v != 0,
            IntegerValue::Short(v) => v != 0,
            IntegerValue::Int(v) => v != 0,
            IntegerValue::Long(v) => v != 0,
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FloatType {
    Float(f32),
    Double(f64),
}

pub enum FloatLocation {
    Float(Location<f32>),
    Double(Location<f64>),
}

impl FloatLocation {
    /// Reads the float value from the given root string.
    ///
    ///
    /// # Safety
    ///
    /// The caller must ensure that the location is valid for the given root
    /// string.
    #[must_use]
    unsafe fn read_from(&self, root: &str) -> FloatType {
        // SAFETY: The caller ensures this is safe.
        unsafe {
            match self {
                FloatLocation::Float(l) => FloatType::Float(l.read_from(root)),
                FloatLocation::Double(l) => FloatType::Double(l.read_from(root)),
            }
        }
    }

    /// Reads the float value from the given root string.
    ///
    /// # Safety
    ///
    ///
    /// The caller must ensure that the location is valid for the given root
    /// string.
    #[must_use]
    unsafe fn read_from_f64(&self, root: &str) -> f64 {
        // SAFETY: The caller ensures this is safe.
        match unsafe { self.read_from(root) } {
            FloatType::Float(v) => f64::from(v),
            FloatType::Double(v) => v,
        }
    }
}
