use bevy::prelude::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.register_type::<MainMenuSettingsButton>(); }

/// A marker [`Component`] for the settings button of the main menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct MainMenuSettingsButton;

impl MainMenuSettingsButton {
    pub(crate) fn build(world: &mut World, parent: Entity) {
        // Create the button
        let bundle = ButtonBundle::default();

        // Spawn the button
        world.spawn((Self, Name::new("MainMenuSettingsButton"), bundle)).set_parent(parent);
    }
}
