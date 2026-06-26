//! Re-exports of types based on enabled features.

use alloc::sync::Arc;

pub use hazarc::ArcBorrow;

cfg_select! {
    feature = "std" => {
        pub use std::sync::{OnceLock, LazyLock};
    },
    all(feature = "once_cell", not(feature = "std")) => {
        pub use once_cell::sync::{OnceCell as OnceLock, Lazy as LazyLock};
    }
}

/// The default `AtomicArc` type using the
/// [`DefaultDomain`](crate::domain::DefaultDomain).
///
/// Alias for [`AtomicArcPtr<Arc<T>, DefaultDomain, W>`][ptr].
///
/// [ptr]: hazarc::atomic::AtomicArcPtr
pub type AtomicArc<T, W = hazarc::write_policy::Concurrent> =
    hazarc::atomic::AtomicArcPtr<Arc<T>, crate::domain::DefaultDomain, W>;

/// The default `AtomicOption` type using the
/// [`DefaultDomain`](crate::domain::DefaultDomain).
///
/// Alias for [`AtomicOptionArcPtr<Arc<T>, DefaultDomain, W>`][ptr].
///
/// [ptr]: hazarc::atomic::AtomicOptionArcPtr
pub type AtomicOption<T, W = hazarc::write_policy::Concurrent> =
    hazarc::atomic::AtomicOptionArcPtr<Arc<T>, crate::domain::DefaultDomain, W>;
