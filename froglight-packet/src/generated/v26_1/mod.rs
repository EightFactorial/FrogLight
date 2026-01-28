//! @generated packets for v26.1.x

use froglight_common::version::V26_1;

use crate::version::*;

pub mod configuration;
pub mod handshake;
pub mod login;
pub mod play;
pub mod status;

impl PacketVersion for V26_1 {
    type Config = Config;
    type Handshake = Handshake;
    type Login = Login;
    type Play = Play;
    type Status = Status;
}
