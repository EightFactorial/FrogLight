use mc_rs_macros::Transcode;
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, Transcode)]
#[mctest(tests = ["decode", "transcode"], bytes = [0])]
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

impl From<ConnectionIntent> for i32 {
    fn from(value: ConnectionIntent) -> Self {
        match value {
            ConnectionIntent::Handshake => -1,
            ConnectionIntent::Game => 0,
            ConnectionIntent::Status => 1,
            ConnectionIntent::Login => 2,
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

impl From<ConnectionIntent> for i16 {
    fn from(value: ConnectionIntent) -> Self {
        match value {
            ConnectionIntent::Handshake => -1,
            ConnectionIntent::Game => 0,
            ConnectionIntent::Status => 1,
            ConnectionIntent::Login => 2,
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

impl From<ConnectionIntent> for i8 {
    fn from(value: ConnectionIntent) -> Self {
        match value {
            ConnectionIntent::Handshake => -1,
            ConnectionIntent::Game => 0,
            ConnectionIntent::Status => 1,
            ConnectionIntent::Login => 2,
        }
    }
}
