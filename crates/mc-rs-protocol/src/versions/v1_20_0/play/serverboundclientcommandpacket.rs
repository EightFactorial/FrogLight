use mc_rs_macros::Transcode;

use crate::types::packets::command_action::CommandAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0])]
pub struct ServerboundClientCommandPacket {
    #[var]
    pub id: u32,
    pub action: CommandAction,
    #[var]
    pub data: u32,
}
