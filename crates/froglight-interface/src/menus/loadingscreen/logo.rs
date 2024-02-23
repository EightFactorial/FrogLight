use bevy::{asset::embedded_asset, prelude::*};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    embedded_asset!(app, "logo.png");

    app.register_type::<LoadingScreenLogoNode>().register_type::<LoadingScreenLogo>();
}

/// A marker [`Component`] for the loading screen logo.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct LoadingScreenLogoNode;

impl LoadingScreenLogoNode {
    pub(super) fn build(world: &mut World, parent: Entity) {
        // Create the logo node
        let logo_node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,

                height: Val::Px(200.0),
                max_height: Val::Vh(50.0),
                width: Val::Px(200.0),
                max_width: Val::Vw(80.0),

                top: Val::Percent(15.0),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..Default::default()
        };

        // Spawn the logo node
        let node = world
            .spawn((Self, logo_node, Name::new("LoadingScreenLogoNode")))
            .set_parent(parent)
            .id();

        // Build the logo
        LoadingScreenLogo::build(world, node);
    }
}

/// A marker [`Component`] for the loading screen logo.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct LoadingScreenLogo;

impl LoadingScreenLogo {
    pub(super) fn build(world: &mut World, parent: Entity) {
        // Get the embedded logo asset
        let assets = world.resource::<AssetServer>();
        let image = assets.load("embedded://froglight_interface/menus/loadingscreen/logo.png");

        // Create the logo imagebundle
        let logo = ImageBundle {
            style: Style {
                height: Val::Auto,
                max_height: Val::Percent(100.0),
                width: Val::Auto,
                max_width: Val::Percent(100.0),
                ..Default::default()
            },
            image: image.into(),
            ..Default::default()
        };

        // Spawn a centering node
        world
            .spawn(super::center_node())
            .with_children(|center| {
                // Spawn the logo node
                center.spawn((Self, logo, Name::new("LoadingScreenLogo")));
            })
            .set_parent(parent);
    }
}
