use compact_str::CompactString;
use serde::{Deserialize, Serialize};

/// The type of world.
///
/// By default, the world type is [`WorldType::Overworld`].
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorldType {
    /// The Overworld
    #[default]
    #[serde(rename = "minecraft:overworld", alias = "overworld", alias = "Overworld")]
    Overworld,
    /// The Nether
    #[serde(rename = "minecraft:the_nether", alias = "nether", alias = "Nether")]
    Nether,
    /// The End
    #[serde(rename = "minecraft:the_end", alias = "end", alias = "End")]
    End,
    /// A custom world type
    Other(CompactString),
}

impl WorldType {
    /// Key for the Overworld
    pub const OVERWORLD_KEY: &'static CompactString =
        &CompactString::new_inline("minecraft:overworld");
    /// Display name for the Overworld
    pub const OVERWORLD_DISPLAY: &'static CompactString = &CompactString::new_inline("Overworld");

    /// Key for the Nether
    pub const NETHER_KEY: &'static CompactString =
        &CompactString::new_inline("minecraft:the_nether");
    /// Display name for the Nether
    pub const NETHER_DISPLAY: &'static CompactString = &CompactString::new_inline("Nether");

    /// Key for the End
    pub const END_KEY: &'static CompactString = &CompactString::new_inline("minecraft:the_end");
    /// Display name for the End
    pub const END_DISPLAY: &'static CompactString = &CompactString::new_inline("End");

    /// Get the [`CompactString`] key of a [`WorldType`].
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::WorldType;
    ///
    /// assert_eq!(WorldType::Overworld.as_key(), WorldType::OVERWORLD_KEY);
    /// assert_eq!(WorldType::Nether.as_key(), WorldType::NETHER_KEY);
    /// assert_eq!(WorldType::End.as_key(), WorldType::END_KEY);
    /// ```
    #[must_use]
    pub fn as_key(&self) -> &CompactString {
        match self {
            WorldType::Overworld => Self::OVERWORLD_KEY,
            WorldType::Nether => Self::NETHER_KEY,
            WorldType::End => Self::END_KEY,
            WorldType::Other(s) => s,
        }
    }

    /// Convert a [`WorldType`] into a [`CompactString`].
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::WorldType;
    ///
    /// assert_eq!(WorldType::Overworld.into_key(), WorldType::OVERWORLD_KEY);
    /// assert_eq!(WorldType::Nether.into_key(), WorldType::NETHER_KEY);
    /// assert_eq!(WorldType::End.into_key(), WorldType::END_KEY);
    /// ```
    #[must_use]
    pub fn into_key(self) -> CompactString {
        match self {
            WorldType::Overworld => Self::OVERWORLD_KEY.clone(),
            WorldType::Nether => Self::NETHER_KEY.clone(),
            WorldType::End => Self::END_KEY.clone(),
            WorldType::Other(s) => s,
        }
    }

    /// Get the display name of a [`WorldType`].
    ///
    /// # Example
    /// ```rust
    /// use froglight_world::WorldType;
    ///
    /// assert_eq!(WorldType::Overworld.display_name(), WorldType::OVERWORLD_DISPLAY);
    /// assert_eq!(WorldType::Nether.display_name(), WorldType::NETHER_DISPLAY);
    /// assert_eq!(WorldType::End.display_name(), WorldType::END_DISPLAY);
    /// ```
    #[must_use]
    pub fn display_name(&self) -> &CompactString {
        match self {
            WorldType::Overworld => Self::OVERWORLD_DISPLAY,
            WorldType::Nether => Self::NETHER_DISPLAY,
            WorldType::End => Self::END_DISPLAY,
            WorldType::Other(s) => s,
        }
    }
}

impl AsRef<CompactString> for WorldType {
    fn as_ref(&self) -> &CompactString { self.as_key() }
}

impl AsRef<str> for WorldType {
    fn as_ref(&self) -> &str { self.as_key().as_str() }
}
