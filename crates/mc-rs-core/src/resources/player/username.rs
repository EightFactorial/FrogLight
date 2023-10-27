use bevy::prelude::*;
use compact_str::CompactString;

pub(super) fn setup(app: &mut App) {
    // TODO: Add a way to change the username
    app.insert_resource(Username(CompactString::from("MC-RS")));
}

/// The username of the player.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Resource)]
pub struct Username(CompactString);

impl Username {
    /// Create a new username
    pub fn new(username: impl Into<CompactString>) -> Self { Self(username.into()) }

    /// Create a new username from an inline string
    ///
    /// Note: Trying to create a long string that can't be inlined, will fail to build.
    pub const fn new_inline(username: &str) -> Self { Self(CompactString::new_inline(username)) }
}

impl From<Username> for CompactString {
    fn from(value: Username) -> Self { value.0 }
}

impl From<Username> for String {
    fn from(value: Username) -> Self { value.0.into() }
}

impl From<CompactString> for Username {
    fn from(username: CompactString) -> Self { Self(username) }
}

impl From<String> for Username {
    fn from(value: String) -> Self { Self(CompactString::from(value)) }
}

impl From<&str> for Username {
    fn from(value: &str) -> Self { Self(CompactString::from(value)) }
}
