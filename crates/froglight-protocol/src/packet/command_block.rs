use froglight_macros::FrogReadWrite;

/// The mode of a command block
///
/// TODO: Document this enum
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0])]
pub enum CommandBlockMode {
    /// Sequence mode
    #[default]
    Sequence,
    /// Auto mode
    Auto,
    /// Redstone mode
    Redstone,
}
