use froglight_macros::FrogReadWrite;

/// A chat suggestion action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum ChatSuggestionAction {
    /// Add a chat suggestion.
    Add,
    /// Remove a chat suggestion.
    Remove,
    /// Set the chat suggestions.
    Set,
}
