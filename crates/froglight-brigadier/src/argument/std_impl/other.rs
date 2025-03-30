use bevy_ecs::world::World;
use bevy_reflect::func::ArgValue;
use froglight_common::identifier::Identifier;

use crate::argument::{ArgumentError, ArgumentParser};

impl ArgumentParser for Identifier {
    type Arg = Identifier;

    fn parse_input<'a>(
        &self,
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

#[cfg(feature = "uuid")]
impl ArgumentParser for uuid::Uuid {
    type Arg = uuid::Uuid;

    fn parse_input<'a>(
        &self,
        arguments: &'a str,
        _: &World,
    ) -> Result<(ArgValue<'a>, &'a str), ArgumentError> {
        let (start, end) = arguments.trim_start().split_once(' ').unwrap_or((arguments, ""));
        let value = start.parse::<uuid::Uuid>().map_err(|_| ArgumentError::DoesNotMatch)?;
        Ok((ArgValue::Owned(Box::new(value)), end))
    }
}
