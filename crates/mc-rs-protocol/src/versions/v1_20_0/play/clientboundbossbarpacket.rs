use mc_rs_macros::Transcode;

use crate::types::{packets::boss_bar::BossBarAction, EntityUuid};

#[derive(Debug, Clone, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])]
pub struct ClientboundBossBarPacket {
    pub bar_uuid: EntityUuid,
    pub bar_action: BossBarAction,
}
