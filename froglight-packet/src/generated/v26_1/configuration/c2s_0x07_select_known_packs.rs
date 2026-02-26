//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:select_known_packs"

use alloc::vec::Vec;

use crate::common::known_packs::KnownResourcePack;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct SelectKnownPacksC2SPacket {
    pub selected: Vec<KnownResourcePack>,
}
