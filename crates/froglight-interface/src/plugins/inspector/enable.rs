use bevy::prelude::*;

/// A [`Resource`] that can be used to enable or disable the inspector.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Reflect, Resource)]
#[reflect(Resource)]
pub struct InspectorEnable(pub(crate) bool);

impl InspectorEnable {
    /// Create a new [`InspectorEnable`]
    ///
    /// Defaults to `false`.
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Enables the inspector.
    pub fn enable(&mut self) { self.0 = true; }

    /// Disables the inspector.
    pub fn disable(&mut self) { self.0 = false; }

    /// Toggles the inspector.
    pub fn toggle(&mut self) { self.0 = !self.0; }

    /// Gets the state of the inspector.
    #[must_use]
    pub fn state(&self) -> bool { self.0 }

    /// A condition to check if the inspector should be enabled.
    pub(super) fn is_inspector_enabled(enable: Res<InspectorEnable>) -> bool { enable.0 }

    /// The [`KeyCode`]s for the keybind to toggle the inspector.
    const KEYBIND: [KeyCode; 2] = [KeyCode::F3, KeyCode::KeyI];

    /// The function to check if the keybind was pressed.
    pub(super) fn inspector_keybind(input: Res<ButtonInput<KeyCode>>, mut enable: ResMut<Self>) {
        if input.any_just_pressed(Self::KEYBIND) && input.all_pressed(Self::KEYBIND) {
            enable.toggle();
        }
    }
}
