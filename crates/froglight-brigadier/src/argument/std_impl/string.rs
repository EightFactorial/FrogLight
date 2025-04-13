use std::borrow::Cow;

use bevy_ecs::world::World;
use bevy_reflect::func::ArgValue;
use smol_str::SmolStr;

use crate::argument::{ArgumentError, ArgumentParser, BrigadierPhrase};

/// A macro for implementing the [`ArgumentParser`] trait for strings.
macro_rules! impl_string {
    ($($ty:ty),*) => {
        $(
            impl ArgumentParser for $ty {
                type Arg = Self;
                fn parse_input<'a>(
                    &self,
                    arguments: &'a str,
                    world: &World,
                ) -> Result<(ArgValue<'a>, &'a str), ArgumentError> {
                    if let (ArgValue::Owned(string), remainder) = BrigadierPhrase.parse_input(arguments, world)? {
                        if let Ok(string) = string.try_take::<String>() {
                            return Ok((ArgValue::Owned(Box::<$ty>::new(string.into())), remainder));
                        }
                    }

                    unreachable!("BrigadierPhrase always returns an owned String");
                }
            }
        )*
    };
}

impl_string!(String, SmolStr, Cow<'static, str>);
