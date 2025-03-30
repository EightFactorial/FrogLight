use std::cmp::Ordering;

use bevy_ecs::world::World;
use bevy_reflect::{func::ArgValue, prelude::*};

use crate::prelude::{ArgumentError, ArgumentParser, ReflectArgumentParser};

/// A brigadier-compatible [`f64`] argument parser.
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
#[reflect(Debug, Default, PartialEq, ArgumentParser)]
pub struct BrigadierDouble {
    /// The minimum value that the argument can be.
    pub min: Option<f64>,
    /// The maximum value that the argument can be.
    pub max: Option<f64>,
}

impl BrigadierDouble {
    /// Creates a new [`BrigadierDouble`] with the given bounds.
    #[must_use]
    pub const fn with_bounds(min: Option<f64>, max: Option<f64>) -> Self { Self { min, max } }
}

impl ArgumentParser for BrigadierDouble {
    type Arg = f64;

    fn parse_input<'a>(
        &self,
        arguments: &'a str,
        _: &World,
    ) -> Result<(ArgValue<'a>, &'a str), ArgumentError> {
        let (start, end) = arguments.trim().split_once(' ').unwrap_or((arguments, ""));
        let double = start.parse::<f64>().map_err(|_| ArgumentError::DoesNotMatch)?;

        // Check if the value is `>= min` and `<= max`.
        let min = self.min.is_none_or(|min| double.total_cmp(&min) != Ordering::Less);
        let max = self.max.is_none_or(|max| double.total_cmp(&max) != Ordering::Greater);

        if min && max {
            Ok((ArgValue::Owned(Box::new(double)), end))
        } else {
            Err(ArgumentError::DoesNotMatch)
        }
    }
}
