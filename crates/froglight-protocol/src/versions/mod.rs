//! Versions and version-dependent structs and enums

use bevy_reflect::Reflect;

/// A Protocol version
pub trait Version: 'static + Default + Copy + Eq + Reflect {
    /// The protocol version number
    const PROTOCOL_VERSION: i32;
}
