//! Registry values and version-specific implementations.

mod complex;

mod simple;
pub use simple::SimpleRegistry;

mod traits;
pub(crate) use traits::sealed::RegistryType;
pub use traits::{
    ConvertKey, DefaultRegistry, InitializeRegistry, RuntimeRegistry, UnknownKeyError,
};
