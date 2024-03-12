use bevy::prelude::*;

use super::{create_button, create_text, MainMenuSettingsButton};
use crate::menus::{
    mainmenu::{systemset::MainMenuUpdateSet, MainMenuMultiplayerButtonEvent},
    InterfaceMenuState,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<MainMenuMultiplayerButton>();

    app.add_systems(
        Update,
        MainMenuMultiplayerButton::on_multiplayer_button
            .ambiguous_with(MainMenuSettingsButton::on_settings_button)
            .run_if(any_with_component::<MainMenuMultiplayerButton>)
            .in_set(MainMenuUpdateSet),
    );
}

/// A marker [`Component`] for the multiplayer button of the main menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct MainMenuMultiplayerButton;

impl MainMenuMultiplayerButton {
    pub(crate) fn build(world: &mut World, parent: Entity) {
        // Create the button
        let button = create_button();
        let text = create_text("Multiplayer", world);

        // Spawn the button
        world
            .spawn((
                Self,
                button,
                Name::new("MainMenuMultiplayerButton"),
                Outline::new(Val::Px(1.0), Val::Auto, Color::GRAY),
            ))
            .with_children(|button| {
                button.spawn(text);
            })
            .set_parent(parent);
    }
}

impl MainMenuMultiplayerButton {
    /// Sets the [`InterfaceMenuState`] to
    /// [`InterfaceMenuState::MultiplayerMenu`] when the multiplayer button is
    /// clicked.
    ///
    /// Also sends a [`MainMenuMultiplayerButtonEvent`] event.
    pub(super) fn on_multiplayer_button(
        query: Query<&Interaction, (With<Self>, Changed<Interaction>)>,

        mut events: EventWriter<MainMenuMultiplayerButtonEvent>,
        mut state: ResMut<NextState<InterfaceMenuState>>,
    ) {
        if query.iter().any(|i| matches!(i, Interaction::Pressed)) {
            debug!("Clicked MainMenuMultiplayerButton");
            state.set(InterfaceMenuState::MultiplayerMenu);
            events.send(MainMenuMultiplayerButtonEvent);
        }
    }
}
