use alloc::{string::String, vec::Vec};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct LoginHelloS2CPacket {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub nonce: Vec<u8>,
    pub needs_authentication: bool,
}
