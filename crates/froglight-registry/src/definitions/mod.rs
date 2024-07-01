//! Data structures and traits for defining registries.

// Re-exports components for convenience.
#[cfg(feature = "hashbrown")]
pub use hashbrown::hash_map;
pub use serde_json::value;

mod block_storage;
pub use block_storage::*;

pub mod errors;

mod traits;
pub use traits::*;
