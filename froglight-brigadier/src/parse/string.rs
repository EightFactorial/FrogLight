//! [`String`] [`CommandArgument`] parsers

use alloc::{borrow::Cow, string::String};

use bevy_reflect::{Reflect, std_traits::ReflectDefault};

use crate::parse::{ArgumentParseError, CommandArgument};

impl CommandArgument for String {
    type Output = String;

    fn parse_argument<'a>(
        &self,
        input: &'a str,
    ) -> Result<(Self::Output, &'a str), ArgumentParseError> {
        if input.starts_with('"') {
            todo!("Parse quoted strings")
        } else {
            StringWord.parse_argument(input)
        }
    }
}

impl CommandArgument for Cow<'static, str> {
    type Output = Cow<'static, str>;

    fn parse_argument<'a>(
        &self,
        input: &'a str,
    ) -> Result<(Self::Output, &'a str), ArgumentParseError> {
        String::new().parse_argument(input).map(|(s, rest)| (Cow::Owned(s), rest))
    }
}

// -------------------------------------------------------------------------------------------------

/// A wrapper around [`String`] that parses a single word without spaces.
#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect)]
#[reflect(Debug, Default, Clone, PartialEq, Hash)]
pub struct StringWord;

impl CommandArgument for StringWord {
    type Output = String;

    fn parse_argument<'a>(
        &self,
        input: &'a str,
    ) -> Result<(Self::Output, &'a str), ArgumentParseError> {
        let mut output = String::new();
        let mut chars = input.chars();
        for c in chars.by_ref() {
            if c.is_whitespace() {
                break;
            }
            output.push(c);
        }
        Ok((output, chars.as_str()))
    }
}

// -------------------------------------------------------------------------------------------------

/// A wrapper around [`String`] that consumes the rest of the input.
#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect)]
#[reflect(Debug, Default, Clone, PartialEq, Hash)]
pub struct StringGreedy;

impl CommandArgument for StringGreedy {
    type Output = String;

    fn parse_argument<'a>(
        &self,
        input: &'a str,
    ) -> Result<(Self::Output, &'a str), ArgumentParseError> {
        Ok((input.into(), ""))
    }
}
