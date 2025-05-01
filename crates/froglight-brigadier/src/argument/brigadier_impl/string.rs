#[cfg(not(feature = "std"))]
use alloc::{
    boxed::Box,
    string::{String, ToString},
};

use bevy_ecs::world::World;
use bevy_reflect::{func::ArgValue, prelude::*};

use crate::prelude::{ArgumentError, ArgumentParser, ReflectArgumentParser};

/// A brigadier parser that consumes a single word.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Debug, Default, PartialEq, ArgumentParser)]
pub struct BrigadierWord;

impl ArgumentParser for BrigadierWord {
    type Arg = String;

    fn parse_input<'a>(
        &self,
        mut arguments: &'a str,
        _: &World,
    ) -> Result<(ArgValue<'a>, &'a str), ArgumentError> {
        arguments = arguments.trim_start();
        if arguments.is_empty() || arguments.starts_with(['"', '\'']) {
            Err(ArgumentError::DoesNotMatch)
        } else {
            let (start, end) = arguments.split_once(' ').unwrap_or((arguments, ""));
            Ok((ArgValue::Owned(Box::new(start.to_string())), end))
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A brigadier parser that consumes a word or quoted phrase.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Debug, Default, PartialEq, ArgumentParser)]
pub struct BrigadierPhrase;

impl ArgumentParser for BrigadierPhrase {
    type Arg = String;

    fn parse_input<'a>(
        &self,
        mut arguments: &'a str,
        _: &World,
    ) -> Result<(ArgValue<'a>, &'a str), ArgumentError> {
        arguments = arguments.trim_start();
        if arguments.is_empty() {
            Err(ArgumentError::DoesNotMatch)
        } else if arguments.starts_with(['"', '\'']) {
            todo!()
        } else {
            let (start, end) = arguments.split_once(' ').unwrap_or((arguments, ""));
            Ok((ArgValue::Owned(Box::new(start.to_string())), end))
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A brigadier parser that consumes the rest of the input.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Debug, Default, PartialEq, ArgumentParser)]
pub struct BrigadierTail;

impl ArgumentParser for BrigadierTail {
    type Arg = String;

    fn parse_input<'a>(
        &self,
        mut arguments: &'a str,
        _: &World,
    ) -> Result<(ArgValue<'a>, &'a str), ArgumentError> {
        arguments = arguments.trim_start();
        if arguments.is_empty() {
            Err(ArgumentError::DoesNotMatch)
        } else {
            Ok((ArgValue::Owned(Box::new(arguments.to_string())), ""))
        }
    }
}
