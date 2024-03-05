use froglight_macros::FrogReadWrite;

/// The intent of a connection.
///
/// # Note
/// Versions before [`1.20.2`](crate::versions::v1_20_2::V1_20_2) did not
/// have the [`ConnectionIntent::Configuration`] intent.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
// TODO: #[frog(tests = ["read_example", "write_default"], bytes = [254])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum ConnectionIntent {
    /// The [`Handshaking`](crate::states::Handshaking) state
    #[default]
    Handshaking = -1,
    /// The [`Play`](crate::states::Play) state
    Play = 0,
    /// The [`Status`](crate::states::Status) state
    Status = 1,
    /// The [`Login`](crate::states::Login) state
    Login = 2,
    /// The [`Configuration`](crate::states::Configuration) state
    Configuration = 3,
}

macro_rules! impl_try_from {
    (i $($t:ty),*) => {
        $(
            impl TryFrom<$t> for ConnectionIntent {
                type Error = ();
                fn try_from(intent: $t) -> Result<Self, Self::Error> {
                    match intent {
                        -1 => Ok(ConnectionIntent::Handshaking),
                        0 => Ok(ConnectionIntent::Play),
                        1 => Ok(ConnectionIntent::Status),
                        2 => Ok(ConnectionIntent::Login),
                        3 => Ok(ConnectionIntent::Configuration),
                        _ => Err(()),
                    }
                }
            }
            impl From<ConnectionIntent> for $t {
                fn from(intent: ConnectionIntent) -> Self {
                    match intent {
                        ConnectionIntent::Handshaking => -1,
                        ConnectionIntent::Play => 0,
                        ConnectionIntent::Status => 1,
                        ConnectionIntent::Login => 2,
                        ConnectionIntent::Configuration => 3,
                    }
                }
            }
        )*
    };
    (u $($t:ty),*) => {
        $(
            impl TryFrom<$t> for ConnectionIntent {
                type Error = ();
                fn try_from(intent: $t) -> Result<Self, Self::Error> {
                    match intent {
                        0 => Ok(ConnectionIntent::Play),
                        1 => Ok(ConnectionIntent::Status),
                        2 => Ok(ConnectionIntent::Login),
                        3 => Ok(ConnectionIntent::Configuration),
                        _ => Err(()),
                    }
                }
            }
            impl TryFrom<ConnectionIntent> for $t {
                type Error = ();
                fn try_from(intent: ConnectionIntent) -> Result<Self, Self::Error> {
                    match intent {
                        ConnectionIntent::Play => Ok(0),
                        ConnectionIntent::Status => Ok(1),
                        ConnectionIntent::Login => Ok(2),
                        ConnectionIntent::Configuration => Ok(3),
                        _ => Err(()),
                    }
                }
            }
        )*
    };
}
impl_try_from!(i i8, i16, i32, i64, i128, isize);
impl_try_from!(u u8, u16, u32, u64, u128, usize);
