//! TODO

/// A marker type for a list of values.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct IndexedListType;

/// A marker type for a map of named values.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct IndexedMapType;
