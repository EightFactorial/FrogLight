use froglight_macros::FrogReadWrite;

/// The action to perform on the advancement tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [1])]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub enum AdvancementTabAction {
    /// Open the advancement tab.
    Open,
    /// Close the advancement tab.
    Close,
}
