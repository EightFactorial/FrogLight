//! TODO

// TODO: Write the `AppStorage` macro
// pub use froglight_macros::AppStorage;

pub use crate::global_storage;

mod global;
pub use global::IndexedGlobalStorage;

mod local;
pub use local::IndexedLocalStorage;

#[cfg(feature = "reflect")]
mod reflect;
#[cfg(feature = "reflect")]
pub use reflect::AppStorageReflect;

mod wrapper;
pub use wrapper::StorageWrapper;
