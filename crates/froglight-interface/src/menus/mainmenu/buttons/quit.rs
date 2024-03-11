use bevy::prelude::*;

use super::{create_button, create_text};

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.register_type::<MainMenuQuitButton>(); }

/// A marker [`Component`] for the quit button of the main menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct MainMenuQuitButton;

impl MainMenuQuitButton {
    pub(crate) fn build(world: &mut World, parent: Entity) {
        // Create the button
        let button = create_button();
        let text = create_text("Quit", world);

        // Spawn the button
        world
            .spawn((
                Self,
                button,
                Name::new("MainMenuQuitButton"),
                Outline::new(Val::Px(1.0), Val::Auto, Color::GRAY),
            ))
            .with_children(|button| {
                button.spawn(text);
            })
            .set_parent(parent);
    }
}
