use froglight_macros::FrogReadWrite;

/// The type of debug sample
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0])]
pub enum DebugSampleType {
    /// Information about the tick speed
    TickTime,
}
