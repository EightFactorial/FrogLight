use bevy::{prelude::*, ui::FocusPolicy};

mod multiplayer;
pub use multiplayer::MainMenuMultiplayerButton;

mod quit;
pub use quit::MainMenuQuitButton;

mod settings;
pub use settings::MainMenuSettingsButton;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.add_event::<MainMenuMultiplayerButtonEvent>()
        .add_event::<MainMenuSettingsButtonEvent>()
        .add_event::<MainMenuQuitButtonEvent>();

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
        let bundle = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,

                top: Val::Percent(33.5),
                width: Val::Px(184.0),
                height: Val::Px(112.0),

                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        };

        // Spawn the button container
        let node = world
            .spawn((
                Self,
                bundle,
                Name::new("MainMenuButtonNode"),
                Outline::new(Val::Px(1.0), Val::Auto, Color::WHITE),
            ))
            .set_parent(parent)
            .id();

        // Build buttons
        MainMenuMultiplayerButton::build(world, node);
        MainMenuSettingsButton::build(world, node);
        MainMenuQuitButton::build(world, node);
    }
}

impl MainMenuButtonNode {
    /// Highlights the buttons of the main menu when the mouse hovers over them.
    #[allow(clippy::type_complexity)]
    fn _highlight_buttons(
        query_container_buttons: Query<&Children, With<Self>>,
        query_button_children: Query<
            (&Children, &Interaction),
            (With<Button>, Changed<Interaction>),
        >,
        mut query_atlas: Query<&mut TextureAtlas>,
    ) {
        let Ok(node_children) = query_container_buttons.get_single() else {
            warn!("MainMenuButtonNode either not found or has multiple entities");
            return;
        };

        // For each child of the button container
        for node_child in node_children {
            // If the child is a button and it's interaction has changed
            if let Ok((button_children, button_interaction)) =
                query_button_children.get(*node_child)
            {
                // TODO: Use the correct atlas indexes
                let _atlas_index = match button_interaction {
                    Interaction::Pressed => 0,
                    Interaction::Hovered => 1,
                    Interaction::None => 2,
                };

                // For each child of the button
                for button_child in button_children {
                    // If the child has a texture atlas
                    if let Ok(mut _atlas) = query_atlas.get_mut(*button_child) {
                        // TODO: Change the texture atlas index
                    }
                }
            }
        }
    }
}

/// An [`Event`] sent when the multiplayer button of the main menu is clicked.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct MainMenuMultiplayerButtonEvent;

/// An [`Event`] sent when the settings button of the main menu is clicked.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct MainMenuSettingsButtonEvent;

/// An [`Event`] sent when the quit button of the main menu is clicked.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct MainMenuQuitButtonEvent;

// TODO: Find the correct button size
fn create_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(100.0),
            height: Val::Px(20.0),

            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::all(Val::Px(4.0)),
            ..Default::default()
        },
        background_color: BackgroundColor(Color::NONE),
        focus_policy: FocusPolicy::Block,
        ..Default::default()
    }
}

// TODO: Use the actual font
fn create_text(text: &str, _world: &mut World) -> TextBundle {
    text_bundle(text, Handle::<Font>::default())
}

// TODO: Use the correct font size
fn text_bundle(text: &str, font: Handle<Font>) -> TextBundle {
    TextBundle {
        style: Style::default(),
        text: Text::from_section(text, TextStyle { font, font_size: 18.0, color: Color::WHITE }),
        background_color: BackgroundColor(Color::NONE),
        focus_policy: FocusPolicy::Pass,
        ..Default::default()
    }
}
