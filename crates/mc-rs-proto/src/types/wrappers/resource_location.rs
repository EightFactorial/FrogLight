use std::{convert::Infallible, str::FromStr};

use compact_str::CompactString;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

use crate::buffer::{Decode, DecodeError, Encode, EncodeError};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ResourceLocation(CompactString);

impl ResourceLocation {
    pub const DEFAULT_NAMESPACE: CompactString = CompactString::new_inline("minecraft:");

    pub fn new(s: impl Into<CompactString>) -> Self {
        let s = s.into();

        if s.contains(':') {
            Self(s)
        } else {
            let mut string = Self::DEFAULT_NAMESPACE.clone();
            string.push_str(&s);

            Self(string)
        }
    }

    pub fn split(&self) -> (&str, &str) {
        self.split_once(':')
            .expect("ResourceLocation must contain a ':'")
    }
}

impl From<ResourceLocation> for CompactString {
    fn from(value: ResourceLocation) -> Self { value.0 }
}

impl From<CompactString> for ResourceLocation {
    fn from(value: CompactString) -> Self { Self::new(value) }
}

impl From<ResourceLocation> for String {
    fn from(value: ResourceLocation) -> Self { value.to_string() }
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
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        self.0.encode(buf)
    }
}

impl Decode for ResourceLocation {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        Ok(Self(CompactString::decode(buf)?))
    }
}
