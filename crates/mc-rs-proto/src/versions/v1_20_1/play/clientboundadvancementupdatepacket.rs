use mc_rs_macros::Transcode;
use hashbrown::HashMap;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundAdvancementUpdatePacket {
    pub a: bool,
    pub b: HashMap,
    pub c: Vec,
    pub d: HashMap,
}
