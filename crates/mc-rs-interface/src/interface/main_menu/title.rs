use bevy::prelude::*;
use mc_rs_core::ResourceLocation;

use crate::{
    interface::InterfaceAssets,
    traits::{interface::SubInterface, textures::GetAssetFromWorld},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct MainMenuTitle;

impl SubInterface for MainMenuTitle {
    fn setup(_app: &mut App) {
        // TODO: Add systems
    }

    fn build(main_menu: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building MainMenuTitle");

        // Create a container for the title
        let container = NodeBundle {
            style: Style {
                min_width: Val::Px(400.0),
                width: Val::Vw(60.0),

                min_height: Val::Px(200.0),
                height: Val::Vh(30.0),

                margin: UiRect::bottom(Val::Vh(10.0)),

                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..Default::default()
        };

        // Get the title image
        let title_image = world
            .get_texture(&ResourceLocation::new("minecraft:gui/title/minecraft"))
            .clone();

        // Add the title image to the interface assets
        let mut interface_assets = world.resource_mut::<InterfaceAssets>();
        interface_assets.push(title_image.clone_weak().untyped());

        // Create the title image node
        let image = ImageBundle {
            style: Style {
                height: Val::Auto,
                max_height: Val::Percent(100.0),

                width: Val::Auto,
                max_width: Val::Percent(100.0),
                ..Default::default()
            },
            image: title_image.into(),
            ..Default::default()
        };

        // Create the title node
        let title = world
            .spawn((
                MainMenuTitle,
                #[cfg(any(debug_assertions, feature = "debug"))]
                Outline {
                    color: Color::BLACK,
                    width: Val::Px(1.0),
                    ..Default::default()
                },
                container,
            ))
            .with_children(|title| {
                title.spawn(image);
            })
            .id();

        // Add the title node as a child of the main menu node
        world.entity_mut(main_menu).add_child(title);
    }
}
