//! TODO

use facet::Facet;

mod error;
// pub use error::DeserializeError;

mod iterator;
pub use iterator::{DeserIter, DeserializeIterator, IteratorStack};

/// A trait for types that can be deserialized.
pub trait Deserialize<'de> {}

impl<'facet, T: Facet<'facet>> Deserialize<'facet> for T {}
