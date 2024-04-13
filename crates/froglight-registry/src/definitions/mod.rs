//! Registry values and version-specific implementations.

pub use froglight_macros::FrogRegistry;

mod complex;

mod simple;
pub use simple::{DefaultRegistry, SimpleRegistry};

mod traits;
pub use traits::{ConvertKey, ConvertKeyError, InitializeRegistry, MissingKeyError};
