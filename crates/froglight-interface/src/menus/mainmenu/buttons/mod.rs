use bevy::prelude::*;

mod multiplayer;
pub use multiplayer::MainMenuMultiplayerButton;

mod quit;
pub use quit::MainMenuQuitButton;

mod settings;
pub use settings::MainMenuSettingsButton;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    multiplayer::build(app);
    quit::build(app);
    settings::build(app);
}

/// A marker [`Component`] for the button container of the main menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct MainMenuButtonNode;

impl MainMenuButtonNode {
    pub(crate) fn build(world: &mut World, parent: Entity) {
        // Create the button container
        let bundle = NodeBundle::default();

        // Spawn the button container
        let node =
            world.spawn((Self, Name::new("MainMenuButtonNode"), bundle)).set_parent(parent).id();

        // Build buttons
        MainMenuMultiplayerButton::build(world, node);
        MainMenuSettingsButton::build(world, node);
        MainMenuQuitButton::build(world, node);
    }
}
