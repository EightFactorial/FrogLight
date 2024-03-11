use bevy::prelude::*;

use super::{create_button, create_text};

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.register_type::<MainMenuSettingsButton>(); }

/// A marker [`Component`] for the settings button of the main menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct MainMenuSettingsButton;

impl MainMenuSettingsButton {
    pub(crate) fn build(world: &mut World, parent: Entity) {
        // Create the button
        let button = create_button();
        let text = create_text("Options", world);

        // Spawn the button
        world
            .spawn((
                Self,
                button,
                Name::new("MainMenuSettingsButton"),
                Outline::new(Val::Px(1.0), Val::Auto, Color::GRAY),
            ))
            .with_children(|button| {
                button.spawn(text);
            })
            .set_parent(parent);
    }
}
