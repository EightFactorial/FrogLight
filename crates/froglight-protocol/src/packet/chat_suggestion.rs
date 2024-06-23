use froglight_macros::FrogReadWrite;

/// A chat suggestion action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0])]
pub enum ChatSuggestionAction {
    /// Add a chat suggestion.
    Add,
    /// Remove a chat suggestion.
    Remove,
    /// Set the chat suggestions.
    Set,
}
