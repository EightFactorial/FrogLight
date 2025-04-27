//! TODO

#[cfg(feature = "v1_21_4")]
pub mod prelude;

pub mod config;
pub mod handshake;
pub mod login;
pub mod play;
pub mod status;

#[cfg(feature = "v1_21_4")]
impl super::state::ValidState<super::state::Handshake> for froglight_common::version::V1_21_4 {
    type Clientbound = handshake::ClientboundHandshakePackets;
    type Serverbound = handshake::ServerboundHandshakePackets;
}
#[cfg(feature = "v1_21_4")]
impl super::state::ValidState<super::state::Status> for froglight_common::version::V1_21_4 {
    type Clientbound = status::ClientboundStatusPackets;
    type Serverbound = status::ServerboundStatusPackets;
}
#[cfg(feature = "v1_21_4")]
impl super::state::ValidState<super::state::Login> for froglight_common::version::V1_21_4 {
    type Clientbound = login::ClientboundLoginPackets;
    type Serverbound = login::ServerboundLoginPackets;
}
#[cfg(feature = "v1_21_4")]
impl super::state::ValidState<super::state::Config> for froglight_common::version::V1_21_4 {
    type Clientbound = config::ClientboundConfigPackets;
    type Serverbound = config::ServerboundConfigPackets;
}
#[cfg(feature = "v1_21_4")]
impl super::state::ValidState<super::state::Play> for froglight_common::version::V1_21_4 {
    type Clientbound = play::ClientboundPlayPackets;
    type Serverbound = play::ServerboundPlayPackets;
}
