use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
pub enum ConnectionIntent {
    Handshake = -1,
    Game = 0,
    Status = 1,
    Login = 2,
}

impl TryFrom<i32> for ConnectionIntent {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(Self::Handshake),
            0 => Ok(Self::Game),
            1 => Ok(Self::Status),
            2 => Ok(Self::Login),
            _ => Err(()),
        }
    }
}

impl TryFrom<i16> for ConnectionIntent {
    type Error = ();

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(Self::Handshake),
            0 => Ok(Self::Game),
            1 => Ok(Self::Status),
            2 => Ok(Self::Login),
            _ => Err(()),
        }
    }
}

impl TryFrom<i8> for ConnectionIntent {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(Self::Handshake),
            0 => Ok(Self::Game),
            1 => Ok(Self::Status),
            2 => Ok(Self::Login),
            _ => Err(()),
        }
    }
}
