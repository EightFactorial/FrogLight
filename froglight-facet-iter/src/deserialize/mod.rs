//! TODO

mod error;
pub use error::DeserializeError;

mod future;
pub use future::DeserializerFuture;

pub(crate) mod item;
pub use item::{DeserializeDesc, DeserializeItem, Item};

pub(crate) mod logic;
pub use logic::Deserializer;
