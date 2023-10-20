use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ClientboundHealthUpdatePacket {
    pub health: f32,
    #[var]
    pub hunger: u32,
    pub saturation: f32,
}
