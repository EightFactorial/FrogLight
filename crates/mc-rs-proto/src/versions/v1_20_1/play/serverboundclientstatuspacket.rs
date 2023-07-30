use mc_rs_macros::Transcode;

use crate::types::packets::update_status::StatusAction;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundClientStatusPacket {
    pub action: StatusAction,
}
