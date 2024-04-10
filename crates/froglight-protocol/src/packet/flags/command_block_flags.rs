use froglight_macros::FrogReadWrite;

/// The flags of a command block
///
/// TODO: Document this struct
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(bitset, tests = ["read_verify", "write_verify"], bytes = [0])]
pub struct CommandBlockFlags {
    /// Track output
    pub track_output: bool,
    /// Conditional
    pub conditional: bool,
    /// Automatic
    pub automatic: bool,
}
