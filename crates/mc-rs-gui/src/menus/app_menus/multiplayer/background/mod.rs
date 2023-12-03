use bevy::{prelude::*, window::PrimaryWindow};
use mc_rs_resourcepack::assets::resourcepacks::AssetFromWorld;

use crate::{
    menus::{
        app_menus::states::MainMenuState,
        shaders::block_background::BlockBackgroundMaterial,
        states::menus::MenuComponentMenusSet,
        traits::{AddMenuResource, MenuComponent},
    },
    resources::scale::GuiScale,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct BackgroundNodeComponent;

impl MenuComponent for BackgroundNodeComponent {
    fn setup(app: &mut App) {
        app.add_systems(
            Update,
            Self::pressed_escape
                .in_set(MenuComponentMenusSet)
                .run_if(in_state(MainMenuState::Multiplayer)),
        );
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building BackgroundNodeComponent");

        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            ..Default::default()
        };

        let node = world
            .spawn((BackgroundNodeComponent, node))
            .set_parent(parent)
            .id();

        // Get the menu background texture.
        let block = world
            .get_texture_or_fallback("minecraft:gui/light_dirt_background")
            .clone();
        world.add_menu_resource(block.clone().untyped());

        // Get the primary window and its dimensions to scale the background.
        let window = world
            .query_filtered::<&Window, With<PrimaryWindow>>()
            .single(world);
        let (width, height) = (window.width(), window.height());
        let scaler = BlockBackgroundMaterial::get_scale(world.resource::<GuiScale>().value());

        // Create the material.
        let material = BlockBackgroundMaterial::with_scale(block, width / scaler, height / scaler);
        let material = world
            .resource_mut::<Assets<BlockBackgroundMaterial>>()
            .add(material);
        world.add_menu_resource(material.clone().untyped());

        // Create the node.
        world
            .spawn(MaterialNodeBundle::<BlockBackgroundMaterial> {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                material,
                ..Default::default()
            })
            .set_parent(node);
    }
}

impl BackgroundNodeComponent {
    fn pressed_escape(input: Res<Input<KeyCode>>, mut state: ResMut<NextState<MainMenuState>>) {
        if input.just_pressed(KeyCode::Escape) {
            #[cfg(any(debug_assertions, feature = "debug"))]
            trace!("Pressed Escape");

            state.set(MainMenuState::MainMenu);
        }
    }
}
