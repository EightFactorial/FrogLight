//! TODO

#[cfg(feature = "v1_21_4")]
use froglight_common::version::V1_21_4;

#[cfg(feature = "v1_21_4")]
use crate::state::{Handshake, Status, ValidState};

// pub mod config;
pub mod handshake;
// pub mod login;
pub mod play;
pub mod status;

#[cfg(feature = "v1_21_4")]
impl ValidState<Handshake> for V1_21_4 {
    type Clientbound = handshake::ClientboundHandshakePackets;
    type Serverbound = handshake::ServerboundHandshakePackets;
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

    pub use super::{handshake::*, play::*, status::*};
}
