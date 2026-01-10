//! Wrappers around atomics that can be enabled/disabled via feature flags.
#![allow(dead_code, reason = "Functions may not be used depending on feature flags")]

#[cfg(feature = "atomic")]
use core::sync::atomic::{AtomicBool, AtomicU16, AtomicU32, Ordering};

macro_rules! maybe {
    ($ident:ident, $inner:ty, $atomic:ty) => {
        /// A wrapper around a possibly atomic value.
        #[repr(transparent)]
        #[derive(Debug, Default)]
        pub(crate) struct $ident {
            #[cfg(feature = "atomic")]
            inner: $atomic,
            #[cfg(not(feature = "atomic"))]
            inner: $inner,
        }

        impl $ident {
            /// Create a new possibly atomic value.
            #[must_use]
            pub(crate) const fn new(value: $inner) -> Self {
                Self {
                    #[cfg(feature = "atomic")]
                    inner: <$atomic>::new(value),
                    #[cfg(not(feature = "atomic"))]
                    inner: value,
                }
            }

            /// Get the inner value.
            #[must_use]
            pub(crate) fn get(&self) -> $inner {
                #[cfg(feature = "atomic")]
                {
                    self.inner.load(Ordering::Relaxed)
                }
                #[cfg(not(feature = "atomic"))]
                {
                    self.inner
                }
            }

            /// Get the inner non-atomic value.
            #[inline]
            #[must_use]
            #[cfg(not(feature = "atomic"))]
            pub(crate) const fn get_const(&self) -> $inner { self.inner }

            /// Set the inner value.
            pub(crate) fn set(&mut self, value: $inner) {
                #[cfg(feature = "atomic")]
                {
                    self.inner.store(value, Ordering::Relaxed);
                }
                #[cfg(not(feature = "atomic"))]
                {
                    self.inner = value;
                }
            }

            /// Set the inner value using atomics.
            #[cfg(feature = "atomic")]
            pub(crate) fn set_atomic(&self, value: $inner) {
                self.inner.store(value, Ordering::Relaxed);
            }
        }
    };
}

maybe!(MaybeAtomicBool, bool, AtomicBool);
maybe!(MaybeAtomicU16, u16, AtomicU16);
maybe!(MaybeAtomicU32, u32, AtomicU32);
maybe!(MaybeAtomicF32, f32, AtomicF32);

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "atomic")]
#[derive(Debug, Default)]
pub(crate) struct AtomicF32(AtomicU32);

#[cfg(feature = "atomic")]
impl AtomicF32 {
    pub(crate) const fn new(value: f32) -> Self { Self(AtomicU32::new(value.to_bits())) }

    pub(crate) fn load(&self, order: Ordering) -> f32 { f32::from_bits(self.0.load(order)) }

    pub(crate) fn store(&self, value: f32, order: Ordering) { self.0.store(value.to_bits(), order) }
}
