//! Protocol `767`
//!
//! Used by Minecraft `1.21.0` - `1.21.0`
//!
//! @generated by `froglight-generator` #248246d
#![allow(clippy::module_inception)]

pub mod configuration;
pub mod handshake;
pub mod login;
pub mod play;
pub mod status;

/// Protocol `767`
///
/// Used by Minecraft `1.21.0` - `1.21.0`
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct V1_21_0;
impl crate::traits::Version for V1_21_0 {
    const ID: i32 = 767;
}
