//! Re-exports of types based on enabled features.

cfg_select! {
    feature = "std" => {
        pub use std::sync::{OnceLock, LazyLock};
    },
    _ => {
        pub use ::once_cell::sync::{OnceCell as OnceLock, Lazy as LazyLock};
    }
}

pub use parking_lot::{Mutex, RwLock};
