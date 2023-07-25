use mc_rs_macros::Packet;

#[derive(Debug, Clone, Packet)]
pub struct ServerboundUpdateBeaconC2SPacket {
    pub a: Option,
    pub b: Option,
}
