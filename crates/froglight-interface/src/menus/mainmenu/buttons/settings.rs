use bevy::prelude::*;

use super::{create_button, create_text, MainMenuMultiplayerButton};
use crate::menus::{
    mainmenu::{systemset::MainMenuUpdateSet, MainMenuSettingsButtonEvent},
    InterfaceMenuState,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<MainMenuSettingsButton>();

    app.add_systems(
        Update,
        MainMenuSettingsButton::on_settings_button
            .ambiguous_with(MainMenuMultiplayerButton::on_multiplayer_button)
            .run_if(any_with_component::<MainMenuSettingsButton>)
            .in_set(MainMenuUpdateSet),
    );
}

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

impl MainMenuSettingsButton {
    /// Sets the [`InterfaceMenuState`] to
    /// [`InterfaceMenuState::SettingsMenu`] when the settings button is
    /// clicked.
    ///
    /// Also sends a [`MainMenuSettingsButtonEvent`] event.
    pub(super) fn on_settings_button(
        query: Query<&Interaction, (With<Self>, Changed<Interaction>)>,

        mut events: EventWriter<MainMenuSettingsButtonEvent>,
        mut state: ResMut<NextState<InterfaceMenuState>>,
    ) {
        if query.iter().any(|i| matches!(i, Interaction::Pressed)) {
            debug!("Clicked MainMenuSettingsButton");
            state.set(InterfaceMenuState::SettingsMenu);
            events.send(MainMenuSettingsButtonEvent);
        }
    }
}
