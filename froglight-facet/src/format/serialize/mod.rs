//! TODO

use facet::Facet;

mod error;
// pub use error::SerializeError;

mod iterator;
// pub use iterator::SerializeIterator;

/// A trait for types that can be deserialized.
pub trait Serialize<'de> {}

impl<'facet, T: Facet<'facet>> Serialize<'facet> for T {}
