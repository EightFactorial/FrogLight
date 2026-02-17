#[cfg(feature = "facet")]
use facet_minecraft as mc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct LoginCompressionS2CPacket {
    #[cfg_attr(feature = "facet", facet(mc::variable))]
    pub compression_threshold: i32,
}
