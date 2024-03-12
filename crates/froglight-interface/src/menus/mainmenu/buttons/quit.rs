use bevy::{app::AppExit, prelude::*};

use super::{create_button, create_text};
use crate::menus::mainmenu::{systemset::MainMenuUpdateSet, MainMenuQuitButtonEvent};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<MainMenuQuitButton>();

    app.add_systems(
        Update,
        MainMenuQuitButton::on_quit_button
            .run_if(any_with_component::<MainMenuQuitButton>)
            .in_set(MainMenuUpdateSet),
    );
}

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

impl MainMenuQuitButton {
    /// Send an [`AppExit`] event when the quit button is clicked.
    ///
    /// Also sends a [`MainMenuQuitButtonEvent`] event.
    fn on_quit_button(
        query: Query<&Interaction, (With<Self>, Changed<Interaction>)>,

        mut button_events: EventWriter<MainMenuQuitButtonEvent>,
        mut quit_events: EventWriter<AppExit>,
    ) {
        if query.iter().any(|i| matches!(i, Interaction::Pressed)) {
            debug!("Clicked MainMenuQuitButton");
            button_events.send(MainMenuQuitButtonEvent);
            quit_events.send(AppExit);
        }
    }
}
