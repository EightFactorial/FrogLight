//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:accept_teleportation"

#[cfg(feature = "facet")]
use facet_minecraft as mc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct AcceptTeleportationC2SPacket {
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub teleport_id: u32,
}
