use bevy::prelude::*;
use froglight_assets::{AssetManager, FallbackImage};

use super::MainMenuSplashText;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<MainMenuLogoNode>()
        .register_type::<MainMenuLogo>()
        .register_type::<MainMenuSubLogo>();
}

/// A marker [`Component`] for the logo container of the main menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct MainMenuLogoNode;

impl MainMenuLogoNode {
    pub(crate) fn build(world: &mut World, parent: Entity) {
        // Create the logo container
        // TODO: Resize based on accurate size
        let bundle = NodeBundle {
            style: Style {
                width: Val::Px(240.0),
                max_width: Val::Vw(80.0),
                height: Val::Px(100.0),
                max_height: Val::Vh(30.0),

                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,

                top: Val::Percent(5.0),
                ..Default::default()
            },
            ..Default::default()
        };

        // Spawn the logo container
        let node =
            world.spawn((Self, Name::new("MainMenuLogoNode"), bundle)).set_parent(parent).id();

        // Build the logo parts
        MainMenuLogo::build(world, node);
        MainMenuSubLogo::build(world, node);
        MainMenuSplashText::build(world, node);
    }
}

/// A marker [`Component`] for the logo of the main menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct MainMenuLogo;

impl MainMenuLogo {
    pub(crate) fn build(world: &mut World, parent: Entity) {
        // Create the logo
        let logo: Handle<Image> = {
            let assets = world.resource::<AssetManager>();
            let assets = assets.textures.read();

            let handle = assets.get("minecraft:gui/title/minecraft");
            if let Some(handle) = handle {
                handle.clone()
            } else {
                warn!("Failed to find the title image, using fallback");
                world.resource::<FallbackImage>().as_ref().clone()
            }
        };
        let bundle = ImageBundle {
            style: Style {
                width: Val::Auto,
                max_width: Val::Percent(100.0),
                height: Val::Auto,
                max_height: Val::Percent(100.0),

                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                ..Default::default()
            },
            image: logo.into(),
            ..Default::default()
        };

        // Spawn the logo
        world
            .spawn(center_node())
            .with_children(|world| {
                world.spawn((Self, Name::new("MainMenuLogo"), bundle));
            })
            .set_parent(parent);
    }
}

/// A marker [`Component`] for the sub-logo of the main menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct MainMenuSubLogo;

impl MainMenuSubLogo {
    pub(crate) fn build(world: &mut World, parent: Entity) {
        // Create the logo
        let logo: Handle<Image> = {
            let assets = world.resource::<AssetManager>();
            let assets = assets.textures.read();

            let handle = assets.get("minecraft:gui/title/edition");
            if let Some(handle) = handle {
                handle.clone()
            } else {
                warn!("Failed to find the edition image, using fallback");
                world.resource::<FallbackImage>().as_ref().clone()
            }
        };

        let sublogo_bundle = ImageBundle {
            style: Style {
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,

                width: Val::Auto,
                max_width: Val::Percent(50.0),
                height: Val::Auto,
                max_height: Val::Percent(50.0),

                top: Val::Px(12.0),
                ..Default::default()
            },
            image: logo.into(),
            ..Default::default()
        };

        // Spawn the sublogo
        world
            .spawn(center_node())
            .with_children(|world| {
                world.spawn((Self, Name::new("MainMenuSubLogo"), sublogo_bundle));
            })
            .set_parent(parent);
    }
}

fn center_node() -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,

            width: Val::Percent(100.0),
            height: Val::Percent(100.0),

            justify_content: JustifyContent::Center,
            align_content: AlignContent::Center,

            top: Val::Px(0.0),
            left: Val::Px(0.0),
            ..Default::default()
        },
        ..Default::default()
    }
}
