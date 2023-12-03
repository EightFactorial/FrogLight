use bevy::{ecs::query::QuerySingleError, prelude::*};

mod traits;
use traits::MenuComponent;

use crate::{menus::resources::MenuResources, resources::camera::DefaultCamera};

use self::states::assets::AssetLoadingState;

pub mod resources;

pub mod states;

pub mod app_ingame;
pub mod app_loading;
pub mod app_menus;
pub mod notifications;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MenusNodeComponent;

impl MenusNodeComponent {
    pub(super) fn setup(app: &mut App) {
        states::setup(app);
        resources::setup(app);

        app.add_systems(Startup, Self::create_cameras);

        app.add_systems(OnEnter(AssetLoadingState::Finished), Self::build);

        app_ingame::AppIngameNodeComponent::setup(app);
        app_loading::AppLoadingNodeComponent::setup(app);
        app_menus::AppMenusNodeComponent::setup(app);
        notifications::NotificationsNodeComponent::setup(app);
    }

    /// Builds the [`MenusNodeComponent`] entity.
    fn build(world: &mut World) {
        // Clear the menu resources.
        world.resource_mut::<MenuResources>().clear();

        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building MenusNodeComponent");

        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            visibility: Visibility::Visible,
            ..Default::default()
        };
        let entity = Self::spawn_or_despawn_children(world).insert(node).id();

        // Build the menus.
        app_ingame::AppIngameNodeComponent::build(entity, world);
        app_menus::AppMenusNodeComponent::build(entity, world);
        notifications::NotificationsNodeComponent::build(entity, world);
    }

    /// Creates a [`MenusNodeComponent`] entity if one does not exist, and despawns all of its
    /// children.
    fn spawn_or_despawn_children(world: &mut World) -> EntityWorldMut {
        let mut query = world.query_filtered::<Entity, With<MenusNodeComponent>>();
        match query.get_single(world) {
            Ok(entity) => {
                let mut entity = world.entity_mut(entity);
                entity.despawn_descendants();

                entity
            }
            Err(err) => {
                if let QuerySingleError::MultipleEntities(_) = err {
                    error!("Multiple MenusNodeComponent entities found!");

                    // Despawn all of the entities.
                    let entities = query.iter(world).collect::<Vec<_>>();
                    entities
                        .into_iter()
                        .for_each(|ent| world.entity_mut(ent).despawn_recursive())
                }

                // Spawn a new entity.
                world.spawn(Self)
            }
        }
    }

    /// Creates the default cameras.
    fn create_cameras(mut commands: Commands) {
        commands.spawn(DefaultCamera::default_camera3d());
        commands.spawn(DefaultCamera::default_camera2d());
    }
}
