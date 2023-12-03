use bevy::prelude::*;
use mc_rs_resourcepack::assets::resourcepacks::AssetFromWorld;

use crate::{
    menus::{
        app_menus::states::MainMenuState,
        states::menus::MenuComponentMenusSet,
        traits::{AddMenuResource, MenuComponent},
    },
    resources::{font::DefaultTextStyle, scale::GuiScaleComponent},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct TitleNodeComponent;

impl MenuComponent for TitleNodeComponent {
    fn setup(app: &mut App) {
        app.add_systems(
            Update,
            TitleTextNodeComponent::text_animation
                .in_set(MenuComponentMenusSet)
                .run_if(
                    in_state(MainMenuState::MainMenu)
                        .and_then(any_with_component::<TitleTextNodeComponent>()),
                ),
        );
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building TitleNodeComponent");

        // Get the title texture
        let title = world
            .get_texture_or_fallback("minecraft:gui/title/minecraft")
            .clone();
        world.add_menu_resource(title.clone().untyped());

        // Get the edition texture
        let edition = world
            .get_texture_or_fallback("minecraft:gui/title/edition")
            .clone();
        world.add_menu_resource(edition.clone().untyped());

        // Spawn the title node
        let node = world
            .spawn((
                TitleNodeComponent,
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        top: Val::Percent(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                #[cfg(any(debug_assertions, feature = "debug"))]
                Outline {
                    color: Color::BLUE,
                    width: Val::Px(1.0),
                    ..Default::default()
                },
            ))
            .set_parent(parent)
            .id();

        // Spawn the title image
        let outer_title = world
            .spawn((
                GuiScaleComponent::new(256, 64),
                ImageBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::End,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    image: title.into(),
                    ..Default::default()
                },
            ))
            .set_parent(node)
            .id();

        // Spawn the edition image
        world
            .spawn((
                GuiScaleComponent::new(128, 16),
                ImageBundle {
                    style: Style {
                        margin: UiRect::bottom(Val::Percent(4.0)),
                        ..Default::default()
                    },
                    image: edition.into(),
                    ..Default::default()
                },
            ))
            .set_parent(outer_title);

        // Spawn the random splash text
        // TODO: Get the random splash text
        let mut style = world.resource::<DefaultTextStyle>().clone();
        style.color = Color::YELLOW;

        world
            .spawn((
                TitleTextNodeComponent,
                // IgnoreDefaultTextStyle,
                TextBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        bottom: Val::Percent(10.0),
                        ..Default::default()
                    },
                    transform: Transform::from_rotation(Quat::from_rotation_z(-20f32.to_radians())),
                    text: Text::from_section("TODO: Random Splash", style.into())
                        .with_alignment(TextAlignment::Center),
                    ..Default::default()
                },
            ))
            .set_parent(node);
    }
}

/// A component that scales the title text.
// TODO: Set the font size based on the length of the text and GuiScale.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct TitleTextNodeComponent;

impl TitleTextNodeComponent {
    const SCALE_SPEED: f32 = 10.0;
    const SCALE: f32 = 1.0 / 20.0;

    fn text_animation(
        mut query: Query<(&mut Transform, &mut Style, &Node), With<TitleTextNodeComponent>>,
        time: Res<Time<Real>>,
    ) {
        let delta =
            ((time.elapsed_seconds_wrapped() * Self::SCALE_SPEED).sin() + 1.0) * Self::SCALE + 1.0;
        query.iter_mut().for_each(|(mut t, mut s, n)| {
            // Scale the textbox
            t.scale = Vec3::splat(delta);

            // Shift the textbox to center the newly scaled text
            s.right = Val::Px((n.size().x * delta / 2.0) - (n.size().x * 0.9));
            s.bottom = Val::Px((n.size().y * 0.9) - (n.size().y * delta / 2.0));
        })
    }
}
