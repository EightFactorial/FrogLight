use alloc::string::String;
use core::ops::{Deref, DerefMut};

use bevy_reflect::Reflect;

use super::{ArgumentParseError, ArgumentParser};

impl ArgumentParser for String {
    fn parse(input: &str) -> Result<(Self, &str), ArgumentParseError> {
        if input.starts_with('"') {
            todo!("Parse quoted strings")
        } else {
            StringWord::parse(input).map(|(word, rest)| (word.0, rest))
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// An [`ArgumentParser`] that parses a [`String`] from a single world.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect)]
#[reflect(Debug, Clone, PartialEq, Hash)]
pub struct StringWord(pub String);

impl ArgumentParser for StringWord {
    fn parse(input: &str) -> Result<(Self, &str), ArgumentParseError> {
        let mut output = String::new();
        let mut chars = input.chars();
        for c in chars.by_ref() {
            if c.is_whitespace() {
                break;
            }
            output.push(c);
        }
        Ok((Self(output), chars.as_str()))
    }
}

impl From<StringWord> for String {
    #[inline]
    fn from(value: StringWord) -> Self { value.0 }
}
impl From<String> for StringWord {
    #[inline]
    fn from(value: String) -> Self { Self(value) }
}

impl Deref for StringWord {
    type Target = String;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for StringWord {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

// -------------------------------------------------------------------------------------------------

/// An [`ArgumentParser`] that parses a [`String`] from the rest of the command.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect)]
#[reflect(Debug, Clone, PartialEq, Hash)]
pub struct StringGreedy(pub String);

impl ArgumentParser for StringGreedy {
    fn parse(input: &str) -> Result<(Self, &str), ArgumentParseError> {
        Ok((Self(input.into()), ""))
    }
}

impl From<StringGreedy> for String {
    #[inline]
    fn from(value: StringGreedy) -> Self { value.0 }
}
impl From<String> for StringGreedy {
    #[inline]
    fn from(value: String) -> Self { Self(value) }
}
