//! This file is auto-generated. Disable this by adding a `manual` tag.
//!
//! @manual packet for "minecraft:client_information"

use crate::common::client_information::ClientInformation;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct ClientInformationC2SPacket {
    pub information: ClientInformation,
}
