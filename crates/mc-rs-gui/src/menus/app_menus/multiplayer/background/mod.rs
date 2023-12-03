use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};
use mc_rs_resourcepack::assets::resourcepacks::AssetFromWorld;

use crate::{
    menus::{
        app_menus::states::MainMenuState,
        shaders::block_background::BlockBackgroundMaterial,
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
            (
                Self::pressed_escape,
                Self::scale_background.run_if(on_event::<WindowResized>()),
            )
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
        let scaler = Self::get_scale(world.resource::<GuiScale>().value());

        // Create the material.
        let material = BlockBackgroundMaterial {
            scale_x: width / scaler,
            scale_y: height / scaler,
            texture: block,
        };
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

    /// Scales the background to fit the window, accounting for the GUI scale.
    fn scale_background(
        query: Query<&Window, With<PrimaryWindow>>,
        gui_scale: Res<GuiScale>,

        mut materials: ResMut<Assets<BlockBackgroundMaterial>>,
    ) {
        let Ok(window) = query.get_single() else {
            error!("Failed to get the primary window");
            return;
        };

        // Calculate the scaling factor.
        let scaler = Self::get_scale(gui_scale.value());
        let scale_width = window.width() / scaler;
        let scale_height = window.height() / scaler;

        // Update the materials.
        for (_, mat) in materials.iter_mut() {
            mat.scale_x = scale_width;
            mat.scale_y = scale_height;
        }
    }

    /// Calculates the scale of the background based on the GUI scale.
    fn get_scale(gui_scale: u32) -> f32 { 32.0 + (gui_scale as f32 - 1.0) * 16.0 }
}
