use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundHealthUpdatePacket {
    pub health: f32,
    #[var]
    pub hunger: u32,
    pub saturation: f32,
}
