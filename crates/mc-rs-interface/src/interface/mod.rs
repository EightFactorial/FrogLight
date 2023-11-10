use bevy::{
    core_pipeline::clear_color::ClearColorConfig, ecs::schedule::SystemConfigs, prelude::*,
};
use mc_rs_core::schedule::state::ApplicationState;

mod loading;
use loading::LoadingInterface;

mod main_menu;
use main_menu::MainMenuInterface;

pub mod set;
// use set::*;

pub mod state;
use state::*;

use crate::{configs::settings::Settings, resourcepacks::ResourcePacksStartReloadEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref, DerefMut, Resource)]
pub struct InterfaceRoot(Entity);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct ShowInterfaceEvent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct HideInterfaceEvent;

pub(super) fn setup(app: &mut App) {
    // Register states.
    app.add_state::<MainMenuState>()
        .add_state::<SettingsState>()
        .add_state::<GuiState>();

    // Register events.
    app.add_event::<ShowInterfaceEvent>()
        .add_event::<HideInterfaceEvent>();

    // Add systems to show/hide/destroy the interface.
    app.add_systems(
        Update,
        (
            InterfaceRoot::show.run_if(InterfaceRoot::show_event),
            InterfaceRoot::hide.run_if(InterfaceRoot::hide_event),
            InterfaceRoot::destroy.run_if(
                InterfaceRoot::start_reload_event
                    .and_then(not(in_state(ApplicationState::Loading))),
            ),
        )
            .run_if(resource_exists::<InterfaceRoot>()),
    );

    // Add systems to spawn the interface.
    app.add_systems(
        OnExit(ApplicationState::Loading),
        InterfaceRoot::spawn_systems(),
    );
    app.add_systems(
        Update,
        InterfaceRoot::spawn_systems().run_if(
            InterfaceRoot::finish_reload_event.and_then(not(in_state(ApplicationState::Loading))),
        ),
    );

    LoadingInterface::setup(app);
    MainMenuInterface::setup(app);
}

impl InterfaceRoot {
    pub fn no_camera2d(query: Query<(), With<Camera2d>>) -> bool { query.is_empty() }

    pub fn default_camera2d(mut commands: Commands) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Spawning Camera2d");

        commands.spawn(Camera2dBundle {
            camera: Camera {
                // Put the camera in front of almost everything.
                order: isize::MAX - 8,
                is_active: true,
                ..Default::default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            ..Default::default()
        });
    }

    pub fn no_camera3d(query: Query<(), With<Camera3d>>) -> bool { query.is_empty() }

    pub fn default_camera3d(settings: Res<Settings>, mut commands: Commands) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Spawning Camera3d");

        commands.spawn(Camera3dBundle {
            camera: Camera {
                is_active: true,
                ..Default::default()
            },
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..Default::default()
            },
            projection: Projection::Perspective(PerspectiveProjection {
                fov: settings.camera.fov,
                ..Default::default()
            }),
            ..Default::default()
        });
    }

    fn finish_reload_event(events: EventReader<ResourcePacksStartReloadEvent>) -> bool {
        !events.is_empty()
    }

    /// Spawn the interface root.
    #[allow(clippy::too_many_arguments)]
    fn spawn(root: Option<Res<Self>>, mut commands: Commands) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Spawning InterfaceRoot");

        if root.is_none() {
            Self::build(&mut commands);
        }
    }

    /// Build the interface root.
    fn build(commands: &mut Commands) {
        let entity = commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                visibility: Visibility::Visible,
                background_color: Color::NONE.into(),
                ..Default::default()
            })
            .id();

        commands.insert_resource(InterfaceRoot(entity));
    }

    /// Systems to spawn the interface.
    fn spawn_systems() -> SystemConfigs {
        (
            InterfaceRoot::spawn,
            apply_deferred,
            MainMenuInterface::spawn,
            InterfaceRoot::default_camera2d.run_if(InterfaceRoot::no_camera2d),
            apply_deferred,
        )
            .chain()
    }

    fn show_event(events: EventReader<ShowInterfaceEvent>) -> bool { !events.is_empty() }

    fn show(mut query: Query<&mut Visibility, With<Node>>, root: Res<Self>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Showing InterfaceRoot");

        if let Ok(mut vis) = query.get_mut(**root) {
            *vis = Visibility::Visible;
        }
    }

    fn hide_event(events: EventReader<HideInterfaceEvent>) -> bool { !events.is_empty() }

    fn hide(mut query: Query<&mut Visibility, With<Node>>, root: Res<Self>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Hiding InterfaceRoot");

        if let Ok(mut vis) = query.get_mut(**root) {
            *vis = Visibility::Hidden;
        }
    }

    fn start_reload_event(events: EventReader<ResourcePacksStartReloadEvent>) -> bool {
        !events.is_empty()
    }

    fn destroy(root: Res<Self>, mut commands: Commands) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Destroying InterfaceRoot");

        commands.entity(**root).despawn_descendants();
    }
}
