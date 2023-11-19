use bevy::{app::AppExit, prelude::*, ui::FocusPolicy};

use crate::{
    assets::textureatlases::{atlases::WidgetAtlas, AtlasFromWorld},
    menus::{
        main_menu::MainMenuState,
        traits::{AddMenuResource, MenuComponent},
    },
    resources::{font::DefaultTextStyle, scale::GuiScaleComponent},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct MainMenuButtons;

impl MenuComponent for MainMenuButtons {
    fn setup(app: &mut App) {
        // Add systems to handle button presses.
        app.add_systems(
            PreUpdate,
            (
                MultiplayerButton::pressed,
                OptionsButton::pressed,
                QuitButton::pressed,
            )
                .run_if(in_state(MainMenuState::Main).and_then(MainMenuButton::any_interactions)),
        );

        // Add a system to change the button's texture.
        app.add_systems(
            Update,
            MainMenuButtons::button_hover
                .run_if(in_state(MainMenuState::Main).and_then(MainMenuButton::any_interactions)),
        );
        // Update the button's texture immediately after the state is changed.
        app.add_systems(OnEnter(MainMenuState::Main), MainMenuButtons::button_hover);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building MainMenuButtons");

        // Spawn MainMenuButtons node
        let entity = world
            .spawn((
                MainMenuButtons,
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        position_type: PositionType::Absolute,

                        width: Val::Percent(50.0),
                        height: Val::Percent(50.0),

                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    background_color: Color::GRAY.into(),
                    ..Default::default()
                },
                #[cfg(any(debug_assertions, feature = "debug"))]
                Outline {
                    width: Val::Px(1.0),
                    color: Color::BLUE,
                    ..Default::default()
                },
            ))
            .set_parent(parent)
            .id();

        // Build buttons
        Self::build_button::<MultiplayerButton>("Multiplayer", entity, world);
        Self::build_button::<OptionsButton>("Options", entity, world);
        Self::build_button::<QuitButton>("Quit", entity, world);
    }
}

impl MainMenuButtons {
    /// Create a button with the given text.
    fn build_button<T: Default + Component>(text: &str, parent: Entity, world: &mut World) {
        let (handle, index) = world
            .get_atlas_and_index(WidgetAtlas, WidgetAtlas::BUTTON_MENU)
            .unwrap();
        let handle = handle.clone();

        // Add the texture atlas to the menu resources.
        world.add_menu_resource(handle.clone().untyped());

        // Create button
        let button = world
            .spawn((
                T::default(),
                MainMenuButton,
                ButtonBundle {
                    style: Style {
                        margin: UiRect::vertical(Val::Px(5.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ))
            .set_parent(parent)
            .id();

        // Create button background
        let button_background = world
            .spawn((
                MainMenuButton,
                AtlasImageBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    texture_atlas: handle,
                    texture_atlas_image: index,
                    focus_policy: FocusPolicy::Pass,
                    ..Default::default()
                },
                GuiScaleComponent::new(200, 20),
            ))
            .set_parent(button)
            .id();

        let font_style = world.resource::<DefaultTextStyle>().clone();

        // Create button text
        world
            .spawn(TextBundle {
                text: Text::from_section(text, font_style.into()),
                focus_policy: FocusPolicy::Pass,
                ..Default::default()
            })
            .set_parent(button_background);
    }

    #[allow(clippy::type_complexity)]
    /// Change the button's texture based on the [`Interaction`] state.
    fn button_hover(
        query: Query<(&Children, &Interaction), With<MainMenuButton>>,
        mut image: Query<&mut UiTextureAtlasImage, With<MainMenuButton>>,
    ) {
        for (children, interaction) in query.iter() {
            children.iter().for_each(|child| {
                if let Ok(mut image) = image.get_mut(*child) {
                    image.index = match interaction {
                        Interaction::Pressed => WidgetAtlas::BUTTON_MENU_SELECTED,
                        Interaction::Hovered => WidgetAtlas::BUTTON_MENU_HIGHLIGHTED,
                        Interaction::None => WidgetAtlas::BUTTON_MENU,
                    };
                }
            });
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct MainMenuButton;

impl MainMenuButton {
    /// Return true if any buttons have been interacted with.
    fn any_interactions(query: Query<(), (Changed<Interaction>, With<MainMenuButton>)>) -> bool {
        !query.is_empty()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct MultiplayerButton;

impl MultiplayerButton {
    /// Set the [`MainMenuState`] to [`MainMenuState::Multiplayer`] if the button is pressed.
    fn pressed(
        query: Query<&Interaction, With<MultiplayerButton>>,
        mut state: ResMut<NextState<MainMenuState>>,
    ) {
        if query.iter().any(|int| matches!(int, Interaction::Pressed)) {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("MultiplayerButton pressed");

            state.set(MainMenuState::Multiplayer);
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct OptionsButton;

impl OptionsButton {
    /// Set the [`MainMenuState`] to [`MainMenuState::Settings`] if the button is pressed.
    fn pressed(
        query: Query<&Interaction, With<OptionsButton>>,
        mut state: ResMut<NextState<MainMenuState>>,
    ) {
        if query.iter().any(|int| matches!(int, Interaction::Pressed)) {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("OptionsButton pressed");

            state.set(MainMenuState::Settings);
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct QuitButton;

impl QuitButton {
    /// Send an [`AppExit`] event if the button is pressed.
    fn pressed(query: Query<&Interaction, With<QuitButton>>, mut exit: EventWriter<AppExit>) {
        if query.iter().any(|int| matches!(int, Interaction::Pressed)) {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("QuitButton pressed");

            exit.send(AppExit);
        }
    }
}
