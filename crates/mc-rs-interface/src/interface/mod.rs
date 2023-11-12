use bevy::{asset::RecursiveDependencyLoadState, prelude::*};

mod loading;
use loading::LoadingInterface;

mod main_menu;
use main_menu::MainMenuInterface;
use mc_rs_core::schedule::state::ApplicationState;

pub mod set;
pub mod state;

pub mod camera;

use crate::{
    resourcepacks::{ResourcePacksFinishReloadEvent, ResourcePacksStartReloadEvent},
    traits::interface::SubInterface,
};

pub(super) fn setup(app: &mut App) {
    state::setup(app);
    set::setup(app);

    LoadingInterface::setup(app);
    InterfaceRoot::setup(app);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct InterfaceRoot;

impl InterfaceRoot {
    /// Setup the interface root systems.
    fn setup(app: &mut App) {
        // Initialize interface assets tracker
        app.init_resource::<InterfaceAssets>();

        // Build the InterfaceRoot on startup.
        app.add_systems(
            OnExit(ApplicationState::Loading),
            InterfaceRoot::build.run_if(not(any_with_component::<InterfaceRoot>())),
        );

        // Destroy the InterfaceRoot when reloading
        // resourcepacks, and rebuild it when finished.
        app.add_systems(
            Update,
            (
                InterfaceRoot::destroy.run_if(on_event::<ResourcePacksStartReloadEvent>()),
                InterfaceRoot::build.run_if(
                    on_event::<ResourcePacksFinishReloadEvent>()
                        .and_then(not(any_with_component::<InterfaceRoot>())),
                ),
            )
                .run_if(not(in_state(ApplicationState::Loading))),
        );

        // Setup systems for sub-interfaces
        MainMenuInterface::setup(app);
    }

    /// Build the interface root.
    fn build(world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building InterfaceRoot");

        // Clear interface assets, as new assets will be loaded
        let mut assets = world.get_resource_mut::<InterfaceAssets>().unwrap();
        assets.clear();

        // Create the interface root ui node
        let root_node = NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            visibility: Visibility::Visible,
            ..Default::default()
        };

        // Spawn the interface root
        let root = world.spawn((InterfaceRoot, root_node)).id();

        // Build sub-interfaces
        MainMenuInterface::build(root, world);
    }

    /// Destroy the interface root.
    fn destroy(query: Query<Entity, With<InterfaceRoot>>, mut commands: Commands) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Destroying InterfaceRoot");

        query.iter().for_each(|entity| {
            commands.entity(entity).despawn_recursive();
        });
    }
}

/// Contains all assets to be loaded for the interface.
///
/// This is used to track the loading state of interface assets,
/// and to ensure that all required interface assets are loaded
/// before the loading screen is removed and the menus are shown.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource)]
pub struct InterfaceAssets(pub Vec<UntypedHandle>);

impl InterfaceAssets {
    /// Returns true if all interface assets are loaded.
    pub fn loaded(&self, asset_server: &AssetServer) -> bool {
        self.iter().all(|handle| {
            let state = asset_server.get_recursive_dependency_load_state(handle.id());

            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Asset {} is {:?}", handle.id(), state);

            matches!(state, Some(RecursiveDependencyLoadState::Loaded) | None)
        })
    }
}
