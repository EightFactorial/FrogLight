//! TODO

#[cfg(feature = "v1_21_5")]
use froglight_common::version::V1_21_5;

#[cfg(feature = "v1_21_5")]
use crate::state::{Handshake, Status, ValidState};

// pub mod config;
pub mod handshake;
// pub mod login;
pub mod play;
pub mod status;

#[cfg(feature = "v1_21_5")]
impl ValidState<Handshake> for V1_21_5 {
    type Clientbound = handshake::ClientboundHandshakePackets;
    type Serverbound = handshake::ServerboundHandshakePackets;
}
#[cfg(feature = "v1_21_5")]
impl ValidState<Status> for V1_21_5 {
    type Clientbound = status::ClientboundStatusPackets;
    type Serverbound = status::ServerboundStatusPackets;
}

#[cfg(feature = "v1_21_5")]
#[expect(unreachable_pub, unused_imports)]
pub mod prelude {
    //! Re-exports of all packet types.

    pub use froglight_common::version::V1_21_5;

    pub use super::{handshake::*, play::*, status::*};
}
