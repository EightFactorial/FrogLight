//! TODO
#![allow(missing_docs, reason = "WIP")]

use froglight_common::prelude::Identifier;
#[cfg(feature = "facet")]
use froglight_facet::facet::prelude::*;
use froglight_nbt::prelude::IndexedNbtCow;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(opaque, Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct RegistryDataEntry {
    pub identifier: Identifier<'static>,
    #[cfg_attr(feature = "facet", facet(mc::with = IndexedNbtCow::WITH_OPT_UNNAMED))]
    pub nbt: Option<IndexedNbtCow<'static>>,
}
