use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct LoginKeyC2SPacket {
    pub encrypted_secret_key: Vec<u8>,
    pub nonce: Vec<u8>,
}
