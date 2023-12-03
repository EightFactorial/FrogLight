use bevy::prelude::*;
use mc_rs_resourcepack::assets::resourcepacks::AssetFromWorld;

use crate::{
    menus::traits::{AddMenuResource, MenuComponent},
    resources::scale::GuiScaleComponent,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct TitleNodeComponent;

impl MenuComponent for TitleNodeComponent {
    fn setup(_app: &mut App) {}

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

        // Inner title
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
    }
}
