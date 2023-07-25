use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundVehicleMovePacket {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f32,
    pub e: f32,
}
