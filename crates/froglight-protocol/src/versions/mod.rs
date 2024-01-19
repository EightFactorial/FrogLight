//! Versions and version-dependent structs and enums
//!
//! TODO: Better documentation

/// A Protocol version
///
/// Different versions of the protocol have different states and packets.
pub trait Version: 'static + Copy + Eq {
    /// The protocol version number
    const PROTOCOL_VERSION: i32;
}
