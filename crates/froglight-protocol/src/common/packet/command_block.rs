use froglight_macros::FrogReadWrite;

/// The mode of a command block
///
/// TODO: Document this enum
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum CommandBlockMode {
    /// Sequence mode
    #[default]
    Sequence,
    /// Auto mode
    Auto,
    /// Redstone mode
    Redstone,
}

/// The flags of a command block
///
/// TODO: Document this struct
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(bitset = 3, tests = ["read_verify", "write_verify"], bytes = [0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct CommandBlockFlags {
    /// Track output
    pub track_output: bool,
    /// Conditional
    pub conditional: bool,
    /// Automatic
    pub automatic: bool,
}
