use std::{
    fmt::Debug,
    ops::{Bound, RangeBounds},
    str::FromStr,
};

use bevy_ecs::world::World;
use bevy_reflect::{func::ArgValue, prelude::*};

use crate::prelude::{ArgumentError, ArgumentParser, ReflectArgumentParser};

/// A brigadier parser for [`i32`]s with optional bounds.
pub type BrigadierInt = BrigadierRange<i32>;
/// A brigadier parser for [`i64`]s with optional bounds.
pub type BrigadierLong = BrigadierRange<i64>;
/// A brigadier parser for [`f32`]s with optional bounds.
pub type BrigadierFloat = BrigadierRange<f32>;
/// A brigadier parser for [`f64`]s with optional bounds.
pub type BrigadierDouble = BrigadierRange<f64>;

// -------------------------------------------------------------------------------------------------

/// A brigadier parser for values with optional bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Debug, Default, PartialEq, ArgumentParser)]
pub struct BrigadierRange<T: Debug + Clone + PartialOrd + FromStr + Reflect + TypePath> {
    /// The minimum value that the value can be.
    pub min: Bound<T>,
    /// The maximum value that the value can be.
    pub max: Bound<T>,
}

impl<T: Debug + Clone + PartialOrd + FromStr + Reflect + TypePath> Default for BrigadierRange<T> {
    fn default() -> Self { Self::const_new(Bound::Unbounded, Bound::Unbounded) }
}

impl<T: Debug + Clone + PartialOrd + FromStr + Reflect + TypePath> BrigadierRange<T> {
    /// Creates a new [`BrigadierRange`] with the given bounds.
    #[must_use]
    pub const fn const_new(min: Bound<T>, max: Bound<T>) -> Self { Self { min, max } }

    /// Creates a new [`BrigadierRange`] with the given bounds.
    #[must_use]
    pub fn new(range: impl RangeBounds<T>) -> Self {
        Self::const_new(range.start_bound().cloned(), range.end_bound().cloned())
    }
}

// -------------------------------------------------------------------------------------------------

impl<T: Debug + Clone + PartialOrd + FromStr + Reflect + TypePath> ArgumentParser
    for BrigadierRange<T>
{
    type Arg = T;

    fn parse_input<'a>(
        &self,
        arguments: &'a str,
        _: &World,
    ) -> Result<(ArgValue<'a>, &'a str), ArgumentError> {
        let (start, end) = arguments.trim_start().split_once(' ').unwrap_or((arguments, ""));
        let value: T = start.parse::<T>().map_err(|_| ArgumentError::DoesNotMatch)?;

        if (self.min.clone(), self.max.clone()).contains(&value) {
            Ok((ArgValue::Owned(Box::new(value)), end))
        } else {
            Err(ArgumentError::DoesNotMatch)
        }
    }
}
