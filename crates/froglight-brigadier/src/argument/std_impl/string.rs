use std::borrow::Cow;

use bevy_ecs::world::World;
use bevy_reflect::func::ArgValue;
use smol_str::SmolStr;

use crate::argument::{ArgumentError, ArgumentParser};

impl ArgumentParser for String {
    type Arg = String;
    fn parse_input<'a>(
        arguments: &'a str,
        _: &World,
    ) -> Result<(ArgValue<'a>, &'a str), ArgumentError> {
        let (start, end) = arguments.trim_start().split_once(' ').unwrap_or((arguments, ""));
        Ok((ArgValue::Owned(Box::new(start.to_string())), end))
    }
}

impl ArgumentParser for SmolStr {
    type Arg = SmolStr;
    fn parse_input<'a>(
        arguments: &'a str,
        _: &World,
    ) -> Result<(ArgValue<'a>, &'a str), ArgumentError> {
        let (start, end) = arguments.trim_start().split_once(' ').unwrap_or((arguments, ""));
        Ok((ArgValue::Owned(Box::new(SmolStr::from(start))), end))
    }
}

impl ArgumentParser for Cow<'static, str> {
    type Arg = Cow<'static, str>;
    fn parse_input<'a>(
        arguments: &'a str,
        _: &World,
    ) -> Result<(ArgValue<'a>, &'a str), ArgumentError> {
        let (start, end) = arguments.trim_start().split_once(' ').unwrap_or((arguments, ""));
        Ok((ArgValue::Owned(Box::new(Cow::from(start).into_owned())), end))
    }
}
