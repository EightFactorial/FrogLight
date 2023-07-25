use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundCommandExecutionC2SPacket {
    pub a: String,
    pub b: u64,
    pub c: u64,
}
