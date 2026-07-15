//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:light_update"

#[cfg(feature = "facet")]
use froglight_facet as mc;

use crate::common::light_data::RawLightData;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct LightUpdateS2CPacket {
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub chunk_x: i32,
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub chunk_z: i32,
    pub light_data: RawLightData,
}
