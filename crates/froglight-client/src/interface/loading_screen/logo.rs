use bevy::{asset::embedded_asset, prelude::*};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<LoadingScreenLogo>();

    embedded_asset!(app, "assets/logo.png");
}

/// A marker [`Component`] for the [`LoadingScreen`](super::LoadingScreen) logo.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Default, Component)]
pub struct LoadingScreenLogo;

impl LoadingScreenLogo {
    /// Spawns a [`LoadingScreenLogo`], returning the [`Entity`].
    pub fn spawn(world: &mut World) -> Entity {
        // Spawn a new loading screen logo
        let entity = world.spawn_empty().id();
        Self::spawn_at(entity, world);

        // Return the `Entity`
        entity
    }

    /// Spawns a [`LoadingScreenLogo`] at the given [`Entity`].
    pub fn spawn_at(entity: Entity, world: &mut World) {
        debug!("Entity {entity:?} - Spawning a new `LoadingScreenLogo`");
        let Some(mut entity_commands) = world.get_entity_mut(entity) else {
            error!("Failed to spawn `LoadingScreenLogo`, Entity not found!");
            return;
        };

        // Load the embedded logo image
        // let image_handle: Handle<Image> = entity_commands.world_scope(|world| {
        //     world.resource::<AssetServer>().load(
        //         "embedded://froglight_client/interface/loading_screen/assets/logo.
        // png"     )
        //});

        // Create a new NodeBundle
        let node = ImageBundle {
            style: Style {
                position_type: PositionType::Relative,
                margin: UiRect::vertical(Val::Px(2.0)),
                height: Val::Px(100.0),
                width: Val::Px(100.0),
                ..Default::default()
            },
            // image: UiImage::from(image_handle),
            ..Default::default()
        };

        // Insert the marker and bundle
        entity_commands.insert((LoadingScreenLogo, Name::new("LoadingScreenLogo"), node));
    }
}
