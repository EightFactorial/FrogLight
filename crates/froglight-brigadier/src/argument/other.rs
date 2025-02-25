use bevy_ecs::world::World;
use bevy_reflect::func::ArgValue;
use froglight_common::Identifier;
use uuid::Uuid;

use super::{ArgumentError, ArgumentParser};

impl ArgumentParser for Identifier {
    type Arg = Identifier;
    fn parse_input<'a>(
        arguments: &'a str,
        _: &World,
    ) -> Result<(ArgValue<'a>, &'a str), ArgumentError> {
        let (start, end) = arguments.trim_start().split_once(' ').unwrap_or((arguments, ""));
        match Identifier::try_new(start) {
            Some(value) => Ok((ArgValue::Owned(Box::new(value)), end)),
            None => Err(ArgumentError::DoesNotMatch),
        }
    }
}

impl ArgumentParser for Uuid {
    type Arg = Uuid;
    fn parse_input<'a>(
        arguments: &'a str,
        _: &World,
    ) -> Result<(ArgValue<'a>, &'a str), ArgumentError> {
        let (start, end) = arguments.trim_start().split_once(' ').unwrap_or((arguments, ""));
        let value = start.parse::<Uuid>().map_err(|_| ArgumentError::DoesNotMatch)?;
        Ok((ArgValue::Owned(Box::new(value)), end))
    }
}
