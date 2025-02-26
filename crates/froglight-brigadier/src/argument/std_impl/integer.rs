use bevy_ecs::world::World;
use bevy_reflect::func::ArgValue;

use crate::argument::{ArgumentError, ArgumentParser};

/// A macro for implementing the [`ArgumentParser`] trait for numbers.
macro_rules! impl_number {
    ($($ty:ty),*) => {
        $(
            impl ArgumentParser for $ty {
                type Arg = Self;
                fn parse_input<'a>(
                    arguments: &'a str,
                    _: &World,
                ) -> Result<(ArgValue<'a>, &'a str), ArgumentError> {
                    let (start, end) = arguments.trim_start().split_once(' ').unwrap_or((arguments, ""));
                    let value = start.parse::<$ty>().map_err(|_| ArgumentError::DoesNotMatch)?;
                    Ok((ArgValue::Owned(Box::new(value)), end))
                }
            }
        )*
    };
}

// Implement the [`ArgumentParser`] trait for the basic number types.
impl_number!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

// And `bool` too ...
impl ArgumentParser for bool {
    type Arg = Self;
    fn parse_input<'a>(
        arguments: &'a str,
        _: &World,
    ) -> Result<(ArgValue<'a>, &'a str), ArgumentError> {
        let (start, end) = arguments.trim_start().split_once(' ').unwrap_or((arguments, ""));
        let value = start.parse::<bool>().map_err(|_| ArgumentError::DoesNotMatch)?;
        Ok((ArgValue::Owned(Box::new(value)), end))
    }
}
