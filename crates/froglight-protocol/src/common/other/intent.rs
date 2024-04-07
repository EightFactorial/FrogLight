use froglight_macros::FrogReadWrite;

/// The intent of a connection.
///
/// # Note
/// Versions before [`1.20.2`](crate::versions::v1_20_2::V1_20_2) did not
/// have the [`ConnectionIntent::Configuration`] intent.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [2])]
pub enum ConnectionIntent {
    /// The [`Handshaking`](crate::states::Handshaking) state
    Handshaking = -1,
    /// The [`Play`](crate::states::Play) state
    Play = 0,
    /// The [`Status`](crate::states::Status) state
    Status = 1,
    /// The [`Login`](crate::states::Login) state
    #[default]
    Login = 2,
    /// The [`Configuration`](crate::states::Configuration) state
    Configuration = 3,
}

impl ConnectionIntent {
    /// Converts a value to a connection intent.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::ConnectionIntent;
    ///
    /// assert_eq!(ConnectionIntent::from_u32(0), Some(ConnectionIntent::Play));
    /// assert_eq!(ConnectionIntent::from_u32(1), Some(ConnectionIntent::Status));
    /// assert_eq!(ConnectionIntent::from_u32(2), Some(ConnectionIntent::Login));
    /// assert_eq!(ConnectionIntent::from_u32(3), Some(ConnectionIntent::Configuration));
    /// assert_eq!(ConnectionIntent::from_u32(4), None);
    /// assert_eq!(ConnectionIntent::from_u32(5), None);
    /// // etc...
    /// ```
    #[inline]
    #[must_use]
    pub fn from_u32(value: u32) -> Option<Self> { Self::try_from(value).ok() }

    /// Converts a value to a connection intent.
    ///
    /// # Example
    /// ```rust
    /// use froglight_protocol::common::ConnectionIntent;
    ///
    /// assert_eq!(ConnectionIntent::from_i32(-3), None);
    /// assert_eq!(ConnectionIntent::from_i32(-2), None);
    /// assert_eq!(ConnectionIntent::from_i32(-1), Some(ConnectionIntent::Handshaking));
    /// assert_eq!(ConnectionIntent::from_i32(0), Some(ConnectionIntent::Play));
    /// assert_eq!(ConnectionIntent::from_i32(1), Some(ConnectionIntent::Status));
    /// assert_eq!(ConnectionIntent::from_i32(2), Some(ConnectionIntent::Login));
    /// assert_eq!(ConnectionIntent::from_i32(3), Some(ConnectionIntent::Configuration));
    /// assert_eq!(ConnectionIntent::from_i32(4), None);
    /// assert_eq!(ConnectionIntent::from_i32(5), None);
    /// // etc...
    /// ```
    #[inline]
    #[must_use]
    pub fn from_i32(value: i32) -> Option<Self> { Self::try_from(value).ok() }
}

impl From<ConnectionIntent> for u8 {
    fn from(intent: ConnectionIntent) -> u8 {
        match intent {
            ConnectionIntent::Handshaking => 0,
            ConnectionIntent::Play => 1,
            ConnectionIntent::Status => 2,
            ConnectionIntent::Login => 3,
            ConnectionIntent::Configuration => 4,
        }
    }
}

impl From<ConnectionIntent> for u32 {
    fn from(intent: ConnectionIntent) -> u32 { u32::from(u8::from(intent)) }
}

impl From<ConnectionIntent> for i8 {
    fn from(intent: ConnectionIntent) -> i8 {
        match intent {
            ConnectionIntent::Handshaking => 0,
            ConnectionIntent::Play => 1,
            ConnectionIntent::Status => 2,
            ConnectionIntent::Login => 3,
            ConnectionIntent::Configuration => 4,
        }
    }
}

impl From<ConnectionIntent> for i32 {
    fn from(intent: ConnectionIntent) -> i32 { i32::from(i8::from(intent)) }
}

impl TryFrom<u8> for ConnectionIntent {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> { Self::try_from(u32::from(value)) }
}

impl TryFrom<u32> for ConnectionIntent {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ConnectionIntent::Play),
            1 => Ok(ConnectionIntent::Status),
            2 => Ok(ConnectionIntent::Login),
            3 => Ok(ConnectionIntent::Configuration),
            _ => Err(()),
        }
    }
}

impl TryFrom<i8> for ConnectionIntent {
    type Error = ();
    fn try_from(value: i8) -> Result<Self, Self::Error> { Self::try_from(i32::from(value)) }
}

impl TryFrom<i32> for ConnectionIntent {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(ConnectionIntent::Handshaking),
            0 => Ok(ConnectionIntent::Play),
            1 => Ok(ConnectionIntent::Status),
            2 => Ok(ConnectionIntent::Login),
            3 => Ok(ConnectionIntent::Configuration),
            _ => Err(()),
        }
    }
}
