use alloc::{borrow::Cow, string::String};

use bevy_reflect::{Reflect, std_traits::ReflectDefault};

use super::{ArgumentParseError, ArgumentParser};

impl ArgumentParser for String {
    type Data = StringType;

    fn parse<'a>(input: &'a str, data: &StringType) -> Result<(Self, &'a str), ArgumentParseError> {
        match data {
            StringType::Default if input.starts_with('"') => todo!("Quoted Strings"),
            StringType::Default | StringType::Word => {
                let (input, remainder) = input.split_once(' ').unwrap_or((input, ""));
                Ok((input.into(), remainder))
            }
            StringType::Greedy => Ok((input.into(), "")),
        }
    }
}

impl ArgumentParser for Cow<'static, str> {
    type Data = StringType;

    #[inline]
    fn parse<'a>(input: &'a str, data: &StringType) -> Result<(Self, &'a str), ArgumentParseError> {
        String::parse(input, data).map(|(s, rest)| (Cow::Owned(s), rest))
    }
}

/// The type of [`String`] to parse.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, Default, Clone, PartialEq, Hash)]
pub enum StringType {
    /// The default string type,
    /// which is either a single word or a quoted string.
    #[default]
    Default,
    /// A single word, separated by spaces.
    Word,
    /// The entire remaining input.
    Greedy,
}
