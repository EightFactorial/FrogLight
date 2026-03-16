use bevy_reflect::{Reflect, std_traits::ReflectDefault};
use uuid::Uuid;

use super::{ArgumentParseError, ArgumentParser};

impl ArgumentParser for Uuid {
    type Data = UuidType;

    #[inline]
    fn parse<'a>(input: &'a str, data: &UuidType) -> Result<(Self, &'a str), ArgumentParseError> {
        match data {
            UuidType::Hyphenated => todo!("TODO: Hyphenated UUIDs"),
            UuidType::Simple => todo!("TODO: Simple UUIDs"),
            UuidType::Braced => todo!("TODO: Braced UUIDs"),
            UuidType::Integer => {
                u128::parse(input, &()).map(|(val, rem)| (Uuid::from_u128(val), rem))
            }
        }
    }
}

/// The type of [`Uuid`] to parse.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Debug, Default, Clone, PartialEq, Hash)]
pub enum UuidType {
    /// The default, hyphenated UUID format.
    ///
    /// See [`Hyphenated`](uuid::fmt::Hyphenated) for more details.
    #[default]
    Hyphenated,
    /// A simple, non-hyphenated UUID format.
    ///
    /// See [`Simple`](uuid::fmt::Simple) for more details.
    Simple,
    /// A braced UUID format, enclosed in curly braces.
    ///
    /// See [`Braced`](uuid::fmt::Braced) for more details.
    Braced,
    /// An integer representation of a UUID.
    ///
    /// See [`Uuid::from_u128`] for more details.
    Integer,
}
