use std::{convert::Infallible, str::FromStr};

use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

use crate::buffer::{Decode, Encode};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ResourceLocation(String);

impl ResourceLocation {
    pub fn new(s: impl Into<String>) -> Self {
        let s: String = s.into();

        if s.contains(':') {
            Self(s)
        } else {
            Self(format!("minecraft:{s}"))
        }
    }
}

impl From<ResourceLocation> for String {
    fn from(value: ResourceLocation) -> Self { value.0 }
}

impl From<&ResourceLocation> for String {
    fn from(value: &ResourceLocation) -> Self { value.0.clone() }
}

impl From<String> for ResourceLocation {
    fn from(value: String) -> Self { Self::new(value) }
}

impl From<&str> for ResourceLocation {
    fn from(value: &str) -> Self { Self::new(value) }
}

impl FromStr for ResourceLocation {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(Self::new(s)) }
}

impl Encode for ResourceLocation {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        self.0.encode(buf)
    }
}

impl Decode for ResourceLocation {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        Ok(Self(String::decode(buf)?))
    }
}
