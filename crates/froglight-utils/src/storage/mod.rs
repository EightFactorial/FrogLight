//! TODO

pub use froglight_macros::AppStorage;

pub use crate::global_storage;

mod global;
pub use global::IndexedGlobalStorage;

mod local;
pub use local::IndexedLocalStorage;

#[cfg(feature = "reflect")]
mod reflect;
#[cfg(feature = "reflect")]
pub use reflect::ReflectAppStorage;

mod wrapper;
pub use wrapper::StorageWrapper;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    #[cfg(feature = "reflect")]
    pub use super::ReflectAppStorage;
    pub use super::{AppStorage, IndexedLocalStorage, StorageWrapper};
}
