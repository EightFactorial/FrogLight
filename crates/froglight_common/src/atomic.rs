//! Atomic floating-point types.

use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// A wrapper around [`AtomicU32`] for [`f32`] values.
#[repr(transparent)]
#[derive(Debug, Default)]
pub struct AtomicF32(AtomicU32);

impl AtomicF32 {
    /// Create a new [`AtomicF32`] from the given [`f32`].
    #[must_use]
    pub const fn new(value: f32) -> Self { Self(AtomicU32::new(value.to_bits())) }

    /// Store a new [`f32`] value into the atomic variable.
    pub fn store(&self, value: f32, ordering: Ordering) { self.0.store(value.to_bits(), ordering) }

    /// Load the current [`f32`] value from the atomic variable.
    #[must_use]
    pub fn load(&self, ordering: Ordering) -> f32 { f32::from_bits(self.0.load(ordering)) }
}

// -------------------------------------------------------------------------------------------------

/// A wrapper around [`AtomicU64`] for [`f64`] values.
#[repr(transparent)]
#[derive(Debug, Default)]
pub struct AtomicF64(AtomicU64);

impl AtomicF64 {
    /// Create a new [`AtomicF64`] from the given [`f64`].
    #[must_use]
    pub const fn new(value: f64) -> Self { Self(AtomicU64::new(value.to_bits())) }

    /// Store a new [`f64`] value into the atomic variable.
    pub fn store(&self, value: f64, ordering: Ordering) { self.0.store(value.to_bits(), ordering) }

    /// Load the current [`f64`] value from the atomic variable.
    #[must_use]
    pub fn load(&self, ordering: Ordering) -> f64 { f64::from_bits(self.0.load(ordering)) }
}
