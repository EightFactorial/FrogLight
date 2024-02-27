use bevy::prelude::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.register_type::<MainMenuSplashText>(); }

/// A marker [`Component`] for the splash text of the main menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct MainMenuSplashText;

impl MainMenuSplashText {
    pub(crate) fn build(world: &mut World, parent: Entity) {
        // Create the splash text
        let text = TextBundle::default();

        // Spawn the splash text
        world.spawn((MainMenuSplashText, Name::new("MainMenuSplashText"), text)).set_parent(parent);
    }
}
