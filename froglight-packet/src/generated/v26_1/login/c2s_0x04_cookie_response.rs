//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:cookie_response"

use alloc::vec::Vec;

use froglight_common::prelude::Identifier;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct CookieResponseC2SPacket {
    pub cookie: Identifier<'static>,
    pub payload: Option<Vec<u8>>,
}
