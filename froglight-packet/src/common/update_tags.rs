//! TODO

use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};

#[cfg(feature = "facet")]
use facet_minecraft as mc;
use foldhash::fast::RandomState;
use froglight_common::prelude::Identifier;
use indexmap::IndexMap;

/// A map of registry identifiers to their tags and values.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct TagMap(pub IndexMap<Identifier<'static>, Vec<TagValue>, RandomState>);

/// A registry tag and it's values.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct TagValue {
    /// The identifier of the tag.
    pub identifier: Identifier<'static>,
    /// The values of the tag.
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub values: Vec<i32>,
}

// -------------------------------------------------------------------------------------------------

impl AsRef<IndexMap<Identifier<'static>, Vec<TagValue>, RandomState>> for TagMap {
    fn as_ref(&self) -> &IndexMap<Identifier<'static>, Vec<TagValue>, RandomState> { &self.0 }
}
impl AsMut<IndexMap<Identifier<'static>, Vec<TagValue>, RandomState>> for TagMap {
    fn as_mut(&mut self) -> &mut IndexMap<Identifier<'static>, Vec<TagValue>, RandomState> {
        &mut self.0
    }
}

impl Deref for TagMap {
    type Target = IndexMap<Identifier<'static>, Vec<TagValue>, RandomState>;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for TagMap {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
