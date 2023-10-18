use mc_rs_macros::Transcode;
use uuid::Uuid;

use crate::types::packets::boss_bar::BossBarAction;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundBossBarPacket {
    pub bar_uuid: Uuid,
    pub bar_action: BossBarAction,
}
