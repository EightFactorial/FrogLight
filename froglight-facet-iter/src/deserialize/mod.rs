//! TODO

mod error;
pub use error::DeserializeError;

mod future;
pub use future::DeserializerFuture;

pub(crate) mod iterator;
pub use iterator::{DeserializeItem, IteratorStack};

pub(crate) mod logic;
pub use logic::{Deserializer, Item};
