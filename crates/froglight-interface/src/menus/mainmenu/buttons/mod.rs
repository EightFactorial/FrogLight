use bevy::{prelude::*, ui::FocusPolicy};

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

// TODO: Get the correct button size
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

// TODO: Get the actual font
fn create_text(text: &str, _world: &mut World) -> TextBundle {
    text_bundle(text, Handle::<Font>::default())
}

// TODO: Get the correct font size
fn text_bundle(text: &str, font: Handle<Font>) -> TextBundle {
    TextBundle {
        style: Style::default(),
        text: Text::from_section(text, TextStyle { font, font_size: 20.0, color: Color::WHITE }),
        background_color: BackgroundColor(Color::NONE),
        focus_policy: FocusPolicy::Pass,
        ..Default::default()
    }
}
