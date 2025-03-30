use std::ops::{Bound, RangeBounds};

use bevy_ecs::world::World;
use bevy_reflect::{func::ArgValue, prelude::*};

use crate::prelude::{ArgumentError, ArgumentParser, ReflectArgumentParser};

/// A brigadier-compatible [`f64`] argument parser.
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Debug, Default, PartialEq, ArgumentParser)]
pub struct BrigadierDouble {
    /// The minimum value that the argument can be.
    pub min: Bound<f64>,
    /// The maximum value that the argument can be.
    pub max: Bound<f64>,
}

impl BrigadierDouble {
    /// Creates a new [`BrigadierDouble`] with the given inclusive bounds.
    #[must_use]
    pub const fn const_new(min: Option<f64>, max: Option<f64>) -> Self {
        Self {
            min: match min {
                Some(min) => Bound::Included(min),
                None => Bound::Unbounded,
            },
            max: match max {
                Some(max) => Bound::Included(max),
                None => Bound::Unbounded,
            },
        }
    }

    /// Creates a new [`BrigadierDouble`] with the given bounds.
    #[must_use]
    pub fn new(range: impl RangeBounds<f64>) -> Self {
        Self { min: range.start_bound().cloned(), max: range.end_bound().cloned() }
    }
}

impl Default for BrigadierDouble {
    fn default() -> Self { Self { min: Bound::Unbounded, max: Bound::Unbounded } }
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

        if (match self.min {
            Bound::Included(start) => start <= double,
            Bound::Excluded(start) => start < double,
            Bound::Unbounded => true,
        }) && (match self.max {
            Bound::Included(end) => double <= end,
            Bound::Excluded(end) => double < end,
            Bound::Unbounded => true,
        }) {
            Ok((ArgValue::Owned(Box::new(double)), end))
        } else {
            Err(ArgumentError::DoesNotMatch)
        }
    }
}
