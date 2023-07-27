use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundWorldBorderInitializePacket {
    pub center_x: f64,
    pub center_z: f64,
    pub old_size: f64,
    pub new_size: f64,
    #[var]
    pub speed: u64,
    #[var]
    pub portal_blocks: u32,
    #[var]
    pub warning_blocks: u32,
    #[var]
    pub warning_time: u32,
}
