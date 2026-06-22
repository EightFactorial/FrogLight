//! TODO

mod error;
pub use error::SerializeError;

mod future;
pub use future::SerializerFuture;

pub(crate) mod item;
pub use item::{Item, SerializeItem};

pub(crate) mod logic;
pub use logic::Serializer;
