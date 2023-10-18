use strum::{EnumIter, IntoEnumIterator};

use crate::Version;

pub mod state;

pub mod v1_20_0;

pub type DefaultVersion = v1_20_0::V1_20_0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Versions {
    S23W32A,
    S23W31A,
    V1_20_0(v1_20_0::V1_20_0),
    S23W18A,
    S23W17A,
    S23W16A,
    S23W14A,
    S23W13A,
    S23W12A,
    V1_19_4,
    V1_19_3,
    V1_19_1,
    V1_19_0,
    V1_18_2,
    V1_18_0,
    V1_17_1,
    V1_17_0,
    V1_16_4,
    V1_16_3,
    V1_16_2,
    V1_16_1,
    V1_16_0,
}

impl Versions {
    /// Returns if the version is supported.
    pub fn is_supported(&self) -> bool { matches!(self, Self::V1_20_0(_)) }

    /// Returns the protocol version id.
    pub fn id(&self) -> i32 {
        match self {
            Self::S23W32A => 0x40000091,
            Self::S23W31A => 0x40000090,
            Self::V1_20_0(_) => <v1_20_0::V1_20_0 as Version>::ID,
            Self::S23W18A => 0x40000085,
            Self::S23W17A => 0x40000084,
            Self::S23W16A => 0x40000083,
            Self::S23W14A => 0x40000082,
            Self::S23W13A => 0x40000080,
            Self::S23W12A => 0x4000007F,
            Self::V1_19_4 => 762,
            Self::V1_19_3 => 761,
            Self::V1_19_1 => 760,
            Self::V1_19_0 => 759,
            Self::V1_18_2 => 758,
            Self::V1_18_0 => 757,
            Self::V1_17_1 => 756,
            Self::V1_17_0 => 755,
            Self::V1_16_4 => 754,
            Self::V1_16_3 => 753,
            Self::V1_16_2 => 751,
            Self::V1_16_1 => 736,
            Self::V1_16_0 => 735,
        }
    }
}

impl TryFrom<i32> for Versions {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Versions::iter().find(|v| v.id() == value).ok_or(())
    }
}
