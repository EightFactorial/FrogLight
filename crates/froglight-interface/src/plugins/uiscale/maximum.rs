use std::num::NonZeroU32;

use bevy::prelude::*;

/// The maximum scale for the UI
///
/// [`None`] is equivalent to `Auto`
#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Resource, Reflect)]
#[reflect(Resource)]
pub struct UiScaleMaximum(Option<u32>);

impl UiScaleMaximum {
    /// Create a new [`UiScaleMaximum`] with the given maximum scale
    #[must_use]
    pub fn new(maximum: Option<NonZeroU32>) -> Self { UiScaleMaximum(maximum.map(Into::into)) }

    /// Set the maximum scale
    pub fn set(&mut self, maximum: Option<NonZeroU32>) {
        *self = UiScaleMaximum(maximum.map(Into::into));
    }

    /// Get the current maximum scale
    #[must_use]
    pub fn get(&self) -> Option<NonZeroU32> {
        if let Some(max) = self.0 {
            NonZeroU32::new(max)
        } else {
            None
        }
    }

    /// Check if the maximum scale is `None`
    ///
    /// (i.e. the UI scale is not set to `Auto`)
    #[must_use]
    pub fn is_some(&self) -> bool { self.0.is_some() }

    /// Check if the maximum scale is `None`
    ///
    /// (i.e. the UI scale is set to `Auto`)
    #[must_use]
    pub fn is_none(&self) -> bool { self.0.is_none() }
}
