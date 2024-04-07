// use froglight_macros::FrogReadWrite;

/// The game mode of a player.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// #[frog(tests = ["read_verify", "write_verify"], bytes = [0])]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
pub enum GameMode {
    /// Survival mode.
    #[default]
    Survival,
    /// Creative mode.
    Creative,
    /// Adventure mode.
    Adventure,
    /// Spectator mode.
    Spectator,
}

impl GameMode {
    /// Converts an optional value to a game mode.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::GameMode;
    ///
    /// assert_eq!(GameMode::from_optional(-2), None);
    /// assert_eq!(GameMode::from_optional(-1), None);
    /// assert_eq!(GameMode::from_optional(0), Some(GameMode::Survival));
    /// assert_eq!(GameMode::from_optional(1), Some(GameMode::Creative));
    /// assert_eq!(GameMode::from_optional(2), Some(GameMode::Adventure));
    /// assert_eq!(GameMode::from_optional(3), Some(GameMode::Spectator));
    /// assert_eq!(GameMode::from_optional(4), None);
    /// assert_eq!(GameMode::from_optional(5), None);
    /// // etc...
    /// ```
    #[allow(clippy::cast_sign_loss)]
    #[inline]
    #[must_use]
    pub fn from_optional(value: i8) -> Option<Self> {
        match value {
            value if value < 0 => None,
            value => Self::from_value(value as u8),
        }
    }

    /// Converts a value to a game mode.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::GameMode;
    ///
    /// assert_eq!(GameMode::from_value(0), Some(GameMode::Survival));
    /// assert_eq!(GameMode::from_value(1), Some(GameMode::Creative));
    /// assert_eq!(GameMode::from_value(2), Some(GameMode::Adventure));
    /// assert_eq!(GameMode::from_value(3), Some(GameMode::Spectator));
    /// assert_eq!(GameMode::from_value(4), None);
    /// assert_eq!(GameMode::from_value(5), None);
    /// // etc...
    /// ```
    #[inline]
    #[must_use]
    pub fn from_value(value: u8) -> Option<Self> { Self::try_from(value).ok() }
}

impl TryFrom<u8> for GameMode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> { Self::try_from(u32::from(value)) }
}

impl TryFrom<u32> for GameMode {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Survival),
            1 => Ok(Self::Creative),
            2 => Ok(Self::Adventure),
            3 => Ok(Self::Spectator),
            _ => Err(()),
        }
    }
}

impl From<GameMode> for u8 {
    fn from(value: GameMode) -> Self {
        match value {
            GameMode::Survival => 0,
            GameMode::Creative => 1,
            GameMode::Adventure => 2,
            GameMode::Spectator => 3,
        }
    }
}

impl From<GameMode> for u32 {
    fn from(value: GameMode) -> Self { u32::from(u8::from(value)) }
}
