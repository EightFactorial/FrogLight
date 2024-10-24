use froglight_macros::FrogReadWrite;

use crate::{
    common::PlayerHand,
    packet::{ChatVisibility, ParticleMode, PlayerModelFlags},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ClientOptionsPacket {
    pub language: String,
    pub view_distance: u8,
    pub chat_visibility: ChatVisibility,
    pub chat_colors: bool,
    pub model_customization: PlayerModelFlags,
    pub main_hand: PlayerHand,
    pub text_filtering_enabled: bool,
    pub allows_listing: bool,
    pub particle_stats: ParticleMode,
}
