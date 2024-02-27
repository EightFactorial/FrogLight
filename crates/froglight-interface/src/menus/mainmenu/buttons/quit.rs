use bevy::prelude::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.register_type::<MainMenuQuitButton>(); }

/// A marker [`Component`] for the quit button of the main menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct MainMenuQuitButton;

impl MainMenuQuitButton {
    pub(crate) fn build(world: &mut World, parent: Entity) {
        // Create the button
        let bundle = ButtonBundle::default();

        // Spawn the button
        world.spawn((Self, Name::new("MainMenuQuitButton"), bundle)).set_parent(parent);
    }
}
