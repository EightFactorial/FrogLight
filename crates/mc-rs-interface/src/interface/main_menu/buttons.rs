use bevy::{app::AppExit, prelude::*};
use mc_rs_core::schedule::set::MenuSet;

use crate::{interface::state::MainMenuState, traits::interface::SubInterface};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct MainMenuButtons;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct ButtonMultiplayer;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct ButtonSettings;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct ButtonQuit;

impl SubInterface for MainMenuButtons {
    fn setup(app: &mut App) {
        // Add systems to interact with the buttons
        //
        // Needs to be run in PreUpdate, before systems check events.
        // Otherwise, config files will not save, black frames between menu states, etc.
        app.add_systems(
            PreUpdate,
            (
                MainMenuButtons::hightlight_button.run_if(MainMenuButtons::button_hovered),
                (
                    MainMenuButtons::pressed_multiplayer,
                    MainMenuButtons::pressed_settings,
                    MainMenuButtons::pressed_quit,
                )
                    .run_if(MainMenuButtons::button_pressed),
            )
                .in_set(MenuSet),
        );
    }

    fn build(main_menu: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building MainMenuButtons");

        let container = NodeBundle {
            style: Style {
                min_width: Val::Px(400.0),
                width: Val::Vw(40.0),

                min_height: Val::Px(20.0),
                height: Val::Vh(40.0),

                padding: UiRect::all(Val::Px(5.0)),

                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_items: JustifyItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..Default::default()
        };

        let button = world
            .spawn((
                MainMenuButtons,
                #[cfg(any(debug_assertions, feature = "debug"))]
                Outline {
                    color: Color::BLUE,
                    width: Val::Px(1.),
                    ..Default::default()
                },
                container,
            ))
            .with_children(|container| {
                MainMenuButtons::create_button::<ButtonMultiplayer>("Multiplayer", container);
                MainMenuButtons::create_button::<ButtonSettings>("Settings", container);
                MainMenuButtons::create_button::<ButtonQuit>("Quit", container);
            })
            .id();

        world.entity_mut(main_menu).add_child(button);
    }
}

impl MainMenuButtons {
    /// Creates a button with the given text
    // TODO: Clean up button creation
    fn create_button<T: Default + Component>(
        text: impl Into<String>,
        parent: &mut WorldChildBuilder<'_>,
    ) {
        parent
            .spawn((
                T::default(),
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),

                        min_height: Val::Px(25.0),
                        height: Val::Vh(20.0),
                        max_height: Val::Px(50.0),

                        margin: UiRect::all(Val::Px(6.0)),

                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::GRAY),
                    ..Default::default()
                },
            ))
            .with_children(|node| {
                node.spawn(
                    TextBundle::from_section(
                        text,
                        TextStyle {
                            font_size: 20.0,
                            ..Default::default()
                        },
                    )
                    .with_text_alignment(TextAlignment::Center),
                );
            });
    }

    /// Returns true if any button is hovered
    fn button_hovered(query: Query<&Interaction, (Changed<Interaction>, With<Button>)>) -> bool {
        query.iter().any(|int| matches!(int, Interaction::Hovered))
    }

    /// Adds and removes highlights from buttons based on their interaction state
    #[allow(clippy::type_complexity)]
    fn hightlight_button(
        mut query: Query<(&mut Style, &Interaction), (Changed<Interaction>, With<Button>)>,
    ) {
        query.for_each_mut(|(mut _style, int)| match int {
            Interaction::Hovered => {
                // TODO: Add highlight
            }
            Interaction::None => {
                // TODO: Remove highlight
            }
            _ => {}
        });
    }

    /// Returns true if any button is pressed
    fn button_pressed(query: Query<&Interaction, (Changed<Interaction>, With<Button>)>) -> bool {
        query.iter().any(|int| matches!(int, Interaction::Pressed))
    }

    /// Enter the multiplayer menu state when the multiplayer button is pressed
    fn pressed_multiplayer(
        query: Query<&Interaction, (Changed<Interaction>, With<ButtonMultiplayer>)>,
        mut state: ResMut<NextState<MainMenuState>>,
    ) {
        if query.iter().any(|int| matches!(int, Interaction::Pressed)) {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Pressed ButtonMultiplayer");

            state.set(MainMenuState::Multiplayer);
        }
    }

    /// Enter the settings menu state when the settings button is pressed
    fn pressed_settings(
        query: Query<&Interaction, (Changed<Interaction>, With<ButtonSettings>)>,
        mut state: ResMut<NextState<MainMenuState>>,
    ) {
        if query.iter().any(|int| matches!(int, Interaction::Pressed)) {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Pressed ButtonSettings");

            state.set(MainMenuState::Settings);
        }
    }

    /// Exit the application when the quit button is pressed
    fn pressed_quit(
        query: Query<&Interaction, (Changed<Interaction>, With<ButtonQuit>)>,
        mut events: EventWriter<AppExit>,
    ) {
        if query.iter().any(|int| matches!(int, Interaction::Pressed)) {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Pressed ButtonQuit");

            events.send(AppExit);
        }
    }
}
