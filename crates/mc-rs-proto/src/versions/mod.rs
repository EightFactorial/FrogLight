use strum::{EnumIter, IntoEnumIterator};

use crate::Version;

pub mod state;

pub mod v1_20_1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Versions {
    V1_20_1(v1_20_1::V1_20_1),
}

impl Versions {
    /// Returns the protocol version id.
    pub fn id(&self) -> i32 {
        match self {
            Versions::V1_20_1(_) => <v1_20_1::V1_20_1 as Version>::ID,
        }
    }
}

impl TryFrom<i32> for Versions {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Versions::iter().find(|v| v.id() == value).ok_or(())
    }
}
