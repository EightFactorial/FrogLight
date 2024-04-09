use froglight_macros::FrogReadWrite;

/// Flags for relative position.
///
/// Used to determine which position fields are relative and which are absolute.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(bitset, tests = ["read_verify", "write_verify"], bytes = [0])]
pub struct RelativePositionFlags {
    /// The X-coordinate
    pub x: bool,
    /// The Y-coordinate
    pub y: bool,
    /// The Z-coordinate
    pub z: bool,
    /// The yaw
    pub yaw: bool,
    /// The pitch
    pub pitch: bool,
}
