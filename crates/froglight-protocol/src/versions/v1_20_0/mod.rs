//! Protocol 763
//!
//! Used by Minecraft 1.20.0 - 1.20.1
//!
//! @generated by `froglight-generator #00f1b4b`

use crate::traits::Version;

pub mod handshaking;
pub mod login;
pub mod play;
pub mod status;
/// Protocol 763
///
/// Used by Minecraft 1.20.0 - 1.20.1
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct V1_20_0;
impl Version for V1_20_0 {
    const PROTOCOL_VERSION: i32 = 763i32;
}
