use std::fmt::Debug;

use bevy::{asset::RecursiveDependencyLoadState, ecs::query::QuerySingleError, prelude::*};

pub mod game;

pub mod loading;

pub mod main_menu;
use main_menu::MainMenuRoot;

pub mod settings;
use settings::SettingsMenuRoot;

pub mod state;

mod traits;
use traits::MenuComponent;

use self::loading::LoadingMenuRoot;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct MenuRoot;

impl MenuRoot {
    /// Setup the [MenuRoot] and all of its submenus's systems.
    pub(super) fn setup(app: &mut App) {
        state::setup(app);
        app.init_resource::<MenuResources>();

        // Add submenu systems
        LoadingMenuRoot::add_systems(app);
        MainMenuRoot::add_systems(app);
        SettingsMenuRoot::add_systems(app);
    }

    /// Build the [MenuRoot] and all of its submenus.
    fn build(world: &mut World) {
        let entity = Self::get_or_spawn(world);
        world.entity_mut(entity).despawn_descendants();

        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building MenuRoot");

        // Build submenus
        LoadingMenuRoot::build(entity, world);
        MainMenuRoot::build(entity, world);
        SettingsMenuRoot::build(entity, world);
    }

    /// Get the [MenuRoot] [Entity], or spawn one if it doesn't exist.
    fn get_or_spawn(world: &mut World) -> Entity {
        match world
            .query_filtered::<Entity, With<MenuRoot>>()
            .get_single(world)
        {
            Ok(entity) => entity,
            Err(err) => {
                // If there are multiple MenuRoot entities, despawn them all and spawn a new one
                if let QuerySingleError::MultipleEntities(_) = err {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    error!("Multiple MenuRoot entities found, despawning all");

                    let entities = world
                        .query_filtered::<Entity, With<MenuRoot>>()
                        .iter(world)
                        .collect::<Vec<_>>();

                    entities.into_iter().for_each(|entity| {
                        world.entity_mut(entity).despawn_recursive();
                    });
                }

                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Spawning MenuRoot");
                world.spawn(MenuRoot).id()
            }
        }
    }
}

/// A collection of handles to resources used by the menus.
///
/// This is used to ensure that all of the resources are loaded
/// before the menus are built and shown.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource)]
pub struct MenuResources(pub Vec<UntypedHandle>);

impl MenuResources {
    /// Returns true if all of the resources are loaded.
    fn loaded(res: Res<MenuResources>, assets: Res<AssetServer>) -> bool {
        res.iter().all(|handle| {
            let state = assets.get_recursive_dependency_load_state(handle.id());

            matches!(state, None | Some(RecursiveDependencyLoadState::Loaded))
        })
    }
}
