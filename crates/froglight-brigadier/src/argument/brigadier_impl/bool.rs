use bevy_ecs::world::World;
use bevy_reflect::{func::ArgValue, prelude::*};

use crate::prelude::{ArgumentError, ArgumentParser, ReflectArgumentParser};

/// A brigadier-compatible [`bool`] argument parser.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Debug, Default, PartialEq, ArgumentParser)]
pub struct BrigadierBool;

impl ArgumentParser for BrigadierBool {
    type Arg = bool;

    fn parse_input<'a>(
        &self,
        arguments: &'a str,
        _: &World,
    ) -> Result<(ArgValue<'a>, &'a str), ArgumentError> {
        let (start, end) = arguments.trim_start().split_once(' ').unwrap_or((arguments, ""));

        if start.eq_ignore_ascii_case("true") {
            Ok((ArgValue::Owned(Box::new(true)), end))
        } else if start.eq_ignore_ascii_case("false") {
            Ok((ArgValue::Owned(Box::new(false)), end))
        } else {
            Err(ArgumentError::DoesNotMatch)
        }
    }
}
