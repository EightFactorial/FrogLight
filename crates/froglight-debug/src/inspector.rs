use bevy::prelude::*;

use crate::DebugUpdateSet;

/// A [`Resource`] that enables/disables the Bevy World Inspector.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Reflect, Resource)]
#[reflect(Resource)]
pub struct InspectorEnable(pub bool);

impl InspectorEnable {
    /// Adds the [`InspectorEnable`] resource and systems to the given [`App`].
    pub(super) fn build(app: &mut App) {
        // Add and register the InspectorEnable resource
        app.register_type::<InspectorEnable>().init_resource::<crate::InspectorEnable>();

        app.add_systems(Update, Self::toggle_inspector.in_set(DebugUpdateSet));
    }

    /// Toggle the Bevy World Inspector using the `F3` + `I` key combination.
    fn toggle_inspector(input: Res<Input<KeyCode>>, mut res: ResMut<Self>) {
        if (input.just_pressed(KeyCode::F3) && input.pressed(KeyCode::I))
            || (input.pressed(KeyCode::F3) && input.just_pressed(KeyCode::I))
        {
            res.toggle();
        }
    }

    /// Returns `true` if the Bevy World Inspector is enabled.
    pub(crate) fn is_enabled(res: Res<Self>) -> bool { **res }

    /// Enables the Bevy World Inspector.
    pub fn enable(&mut self) { *self = Self(true); }

    /// Disables the Bevy World Inspector.
    pub fn disable(&mut self) { *self = Self(false); }

    /// Toggles the state of the Bevy World Inspector.
    pub fn toggle(&mut self) { *self = Self(!**self); }
}
