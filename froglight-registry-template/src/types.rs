//! Re-exports of types based on enabled features.

cfg_select! {
    feature = "std" => {
        pub use std::sync::{LazyLock, OnceLock};
    }
    _ => {
        pub use ::once_cell::sync::{Lazy as LazyLock, OnceCell as OnceLock};
    }
}

pub use parking_lot::{Mutex, RwLock};
