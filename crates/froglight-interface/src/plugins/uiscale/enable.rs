use bevy::prelude::*;

/// A [`Resource`] that enables and disables automatic UI scaling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Resource, Reflect)]
#[reflect(Resource)]
pub struct UiScaleEnable(bool);

impl Default for UiScaleEnable {
    fn default() -> Self { UiScaleEnable(true) }
}

impl UiScaleEnable {
    /// Create a new [`UiScaleEnable`] with the given state
    #[must_use]
    pub fn new(state: bool) -> Self { UiScaleEnable(state) }

    /// Set the state of automatic UI scaling
    pub fn set(&mut self, state: bool) { *self = UiScaleEnable(state); }

    /// Enable automatic UI scaling
    pub fn enable(&mut self) { *self = UiScaleEnable(true); }

    /// Disable automatic UI scaling
    pub fn disable(&mut self) { *self = UiScaleEnable(false); }

    /// Toggle automatic UI scaling
    pub fn toggle(&mut self) { *self = UiScaleEnable(!self.0); }

    /// Check if automatic UI scaling is enabled
    #[must_use]
    pub(crate) fn is_enabled(state: Res<Self>) -> bool { state.state() }

    /// Get the current state of automatic UI scaling
    #[must_use]
    pub fn state(&self) -> bool { self.0 }
}
