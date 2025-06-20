//! Packets for [`V1_21_4`]
//!
//! This file is auto-generated. Disable this by adding an `@manual` tag.
//!
//! @generated by {COMMIT_HASH}

#[cfg(feature = "v1_21_4")]
use froglight_common::version::V1_21_4;

#[cfg(feature = "v1_21_4")]
use crate::state::{Config, Handshake, Login, Play, Status, ValidState};

pub mod config;
pub mod handshake;
pub mod login;
pub mod play;
pub mod status;

#[cfg(feature = "v1_21_4")]
impl ValidState<Config> for V1_21_4 {
    type Clientbound = config::ClientboundConfigPackets;
    type Serverbound = config::ServerboundConfigPackets;
}
#[cfg(feature = "v1_21_4")]
impl ValidState<Handshake> for V1_21_4 {
    type Clientbound = handshake::ClientboundHandshakePackets;
    type Serverbound = handshake::ServerboundHandshakePackets;
}
#[cfg(feature = "v1_21_4")]
impl ValidState<Login> for V1_21_4 {
    type Clientbound = login::ClientboundLoginPackets;
    type Serverbound = login::ServerboundLoginPackets;
}
#[cfg(feature = "v1_21_4")]
impl ValidState<Play> for V1_21_4 {
    type Clientbound = play::ClientboundPlayPackets;
    type Serverbound = play::ServerboundPlayPackets;
}
#[cfg(feature = "v1_21_4")]
impl ValidState<Status> for V1_21_4 {
    type Clientbound = status::ClientboundStatusPackets;
    type Serverbound = status::ServerboundStatusPackets;
}

#[cfg(feature = "v1_21_4")]
pub mod prelude {
    //! Re-exports of all packet types.

    pub use froglight_common::version::V1_21_4;

    pub use super::{config::*, handshake::*, login::*, play::*, status::*};
}
