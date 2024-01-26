//! Versions and version-dependent structs and enums

/// A Protocol version
pub trait Version: 'static + Default + Copy + Eq {
    /// The protocol version number
    const PROTOCOL_VERSION: i32;
}
