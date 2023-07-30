use mc_rs_macros::Transcode;

use crate::types::packets::resource_pack::ResourcePackAction;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundResourcePackStatusPacket {
    pub action: ResourcePackAction,
}
