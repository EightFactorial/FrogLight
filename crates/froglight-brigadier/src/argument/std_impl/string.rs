use std::borrow::Cow;

use bevy_ecs::world::World;
use bevy_reflect::func::ArgValue;
use smol_str::SmolStr;

use crate::argument::{ArgumentError, ArgumentParser};

/// A macro for implementing the [`ArgumentParser`] trait for strings.
macro_rules! impl_string {
    ($($ty:ty),*) => {
        $(
            impl ArgumentParser for $ty {
                type Arg = Self;
                fn parse_input<'a>(
                    arguments: &'a str,
                    _: &World,
                ) -> Result<(ArgValue<'a>, &'a str), ArgumentError> {
                    if arguments.trim().is_empty() {
                        Err(ArgumentError::DoesNotMatch)
                    } else {
                        let (start, end) = arguments.trim().split_once(' ').unwrap_or((arguments, ""));
                        Ok((ArgValue::Owned(Box::new(<$ty>::from(start))), end))
                   }
                }
            }
        )*
    };
}

impl_string!(String, SmolStr);

impl ArgumentParser for Cow<'static, str> {
    type Arg = Self;
    fn parse_input<'a>(
        arguments: &'a str,
        _: &World,
    ) -> Result<(ArgValue<'a>, &'a str), ArgumentError> {
        if arguments.trim().is_empty() {
            Err(ArgumentError::DoesNotMatch)
        } else {
            let (start, end) = arguments.trim().split_once(' ').unwrap_or((arguments, ""));
            Ok((ArgValue::Owned(Box::new(Cow::<'static, str>::from(start.to_string()))), end))
        }
    }
}
