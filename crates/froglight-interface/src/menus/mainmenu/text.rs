use bevy::prelude::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<MainMenuVersionText>().register_type::<MainMenuNoticeText>();
}

/// A marker [`Component`] for the version text displayed on the main menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct MainMenuVersionText;

impl MainMenuVersionText {
    pub(crate) fn build(world: &mut World, parent: Entity) {
        // Create the version text
        let text = TextBundle::default();

        // Spawn the version text
        world.spawn((Self, Name::new("MainMenuVersionText"), text)).set_parent(parent);
    }
}

/// A marker [`Component`] for the notice text displayed on the main menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct MainMenuNoticeText;

impl MainMenuNoticeText {
    pub(crate) fn build(world: &mut World, parent: Entity) {
        // Create the notice text
        let text = TextBundle::default();

        // Spawn the notice text
        world.spawn((Self, Name::new("MainMenuNoticeText"), text)).set_parent(parent);
    }
}
