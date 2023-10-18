use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundBookUpdatePacket {
    #[var]
    pub slot: u32,
    pub pages: Vec<String>,
    pub title: Option<String>,
}
