//! Wrappers around atomics that can be enabled/disabled via feature flags.

#[cfg(feature = "atomic")]
use core::sync::atomic::{AtomicU32, Ordering};

/// A wrapper around an [`AtomicU32`].
#[derive(Debug, Default)]
pub struct MaybeAtomicU32 {
    #[cfg(feature = "atomic")]
    inner: AtomicU32,
    #[cfg(not(feature = "atomic"))]
    inner: u32,
}

impl MaybeAtomicU32 {
    /// Create a new [`MaybeAtomicU32`].
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self {
            #[cfg(feature = "atomic")]
            inner: AtomicU32::new(value),
            #[cfg(not(feature = "atomic"))]
            inner: value,
        }
    }

    /// Get the inner [`u32`] value.
    #[must_use]
    pub fn get(&self) -> u32 {
        #[cfg(feature = "atomic")]
        {
            self.inner.load(Ordering::Relaxed)
        }
        #[cfg(not(feature = "atomic"))]
        {
            self.inner
        }
    }

    /// Set the inner [`u32`] value.
    pub fn set(&mut self, value: u32) {
        #[cfg(feature = "atomic")]
        {
            self.inner.store(value, Ordering::Relaxed);
        }
        #[cfg(not(feature = "atomic"))]
        {
            self.inner = value;
        }
    }
}
