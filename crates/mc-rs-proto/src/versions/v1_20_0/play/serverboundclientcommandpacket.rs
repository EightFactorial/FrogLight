use mc_rs_macros::Transcode;

use crate::types::packets::command_action::CommandAction;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundClientCommandPacket {
    #[var]
    pub id: u32,
    pub action: CommandAction,
    #[var]
    pub data: u32,
}
