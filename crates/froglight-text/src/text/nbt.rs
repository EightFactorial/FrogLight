use core::any::type_name;

use froglight_nbt::prelude::*;

use super::FormattedText;

impl FromTag for FormattedText {
    fn from_tag(tag: &NbtTag) -> Result<Self, NbtError> {
        match tag {
            NbtTag::String(string) => Ok(Self::from_string(string.to_string_lossy())),
            NbtTag::List(NbtListTag::String(strings)) => {
                let mut strings = strings.iter();
                match strings.next() {
                    Some(first) => Ok(Self::from_string(first.to_string_lossy())
                        .with_children(strings.map(|s| Self::from_string(s.to_string_lossy())))),
                    None => Ok(Self::from_string("")),
                }
            }
            NbtTag::Compound(compound) => Self::from_compound(compound),
            _ => Err(NbtError::MismatchedTag(type_name::<Self>(), "String, List, or Compound")),
        }
    }
}

impl FormattedText {
    /// Parse the type from an [`NbtCompound`].
    ///
    /// # Errors
    /// Returns an error if the type fails to parse.
    pub fn from_compound(_nbt: &NbtCompound) -> Result<Self, NbtError> { todo!() }
}

// -------------------------------------------------------------------------------------------------

impl IntoTag for FormattedText {
    fn into_tag(&self) -> Result<NbtTag, NbtError> { todo!() }
}
