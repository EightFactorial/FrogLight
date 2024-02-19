//! Protocol 764
//!
//! Used by Minecraft 1.20.2
//!
//! @generated by `froglight-generator #00f1b4b`

use bevy_reflect::Reflect;

use crate::traits::Version;

pub mod configuration;
pub mod handshaking;
pub mod login;
pub mod play;
pub mod status;
/// Protocol 764
///
/// Used by Minecraft 1.20.2
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct V1_20_2;
impl Version for V1_20_2 {
    const PROTOCOL_VERSION: i32 = 764i32;
}