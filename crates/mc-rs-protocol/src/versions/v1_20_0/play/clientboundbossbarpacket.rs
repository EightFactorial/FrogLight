use mc_rs_macros::Transcode;
use uuid::Uuid;

use crate::types::packets::boss_bar::BossBarAction;

#[derive(Debug, Clone, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])]
pub struct ClientboundBossBarPacket {
    pub bar_uuid: Uuid,
    pub bar_action: BossBarAction,
}
