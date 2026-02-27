//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:store_cookie"

use alloc::vec::Vec;

use froglight_common::prelude::Identifier;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct StoreCookieS2CPacket {
    pub cookie: Identifier<'static>,
    pub payload: Vec<u8>,
}
