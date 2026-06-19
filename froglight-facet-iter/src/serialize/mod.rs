//! TODO

mod error;
pub use error::SerializeError;

mod future;
pub use future::SerializerFuture;

pub(crate) mod iterator;
pub use iterator::{IteratorStack, SerializeItem};

pub(crate) mod logic;
pub use logic::{Item, Serializer};
