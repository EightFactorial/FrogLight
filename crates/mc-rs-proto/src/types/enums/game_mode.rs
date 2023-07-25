use strum::{Display, EnumString};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Display, EnumString)]
pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator,
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

impl TryFrom<u16> for GameMode {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Survival),
            1 => Ok(Self::Creative),
            2 => Ok(Self::Adventure),
            3 => Ok(Self::Spectator),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for GameMode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Survival),
            1 => Ok(Self::Creative),
            2 => Ok(Self::Adventure),
            3 => Ok(Self::Spectator),
            _ => Err(()),
        }
    }
}
