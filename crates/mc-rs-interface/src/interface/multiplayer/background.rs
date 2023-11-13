use bevy::prelude::*;
use mc_rs_core::ResourceLocation;

use crate::{
    interface::InterfaceAssets,
    traits::{interface::InterfaceComponent, world::AssetFromWorld},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct MultiplayerBackground;

impl InterfaceComponent for MultiplayerBackground {
    fn setup(_app: &mut App) {}

    fn build(multiplayer: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building MultiplayerBackground");

        let dirt_background = world
            .get_texture(&ResourceLocation::new(
                "minecraft:gui/light_dirt_background",
            ))
            .clone();

        let mut interface_assets = world.resource_mut::<InterfaceAssets>();
        interface_assets.push(dirt_background.clone().untyped());

        let image = ImageBundle {
            style: Style {
                width: Val::Auto,
                height: Val::Auto,
                ..Default::default()
            },
            image: dirt_background.into(),
            ..Default::default()
        };

        let background = world.spawn(image).id();
        world.entity_mut(multiplayer).add_child(background);
    }
}

impl MultiplayerBackground {}
