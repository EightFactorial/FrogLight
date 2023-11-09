use bevy::prelude::*;
use mc_rs_core::schedule::state::ApplicationState;

pub mod loading;
pub mod set;

use crate::resourcepacks::ResourcePacksStartReloadEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref, DerefMut, Resource)]
pub struct InterfaceRoot(Entity);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct ShowInterfaceEvent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct HideInterfaceEvent;

pub(super) fn setup(app: &mut App) {
    app.add_event::<ShowInterfaceEvent>()
        .add_event::<HideInterfaceEvent>();

    app.add_systems(OnExit(ApplicationState::Loading), InterfaceRoot::spawn);

    app.add_systems(
        Update,
        (
            InterfaceRoot::show.run_if(InterfaceRoot::show_event),
            InterfaceRoot::hide.run_if(InterfaceRoot::hide_event),
            (
                InterfaceRoot::spawn.run_if(InterfaceRoot::finish_reload_event),
                InterfaceRoot::destroy.run_if(InterfaceRoot::start_reload_event),
            )
                .run_if(not(in_state(ApplicationState::Loading))),
        )
            .run_if(resource_exists::<InterfaceRoot>()),
    );

    loading::setup(app);
}

impl InterfaceRoot {
    fn finish_reload_event(events: EventReader<ResourcePacksStartReloadEvent>) -> bool {
        !events.is_empty()
    }

    /// Spawn the interface root.
    fn spawn(
        root: Option<Res<Self>>,
        camera: Query<(), With<Camera>>,

        _state: Res<State<ApplicationState>>,
        assets: Res<AssetServer>,
        mut commands: Commands,
    ) {
        let _root = match root {
            Some(root) => **root,
            None => Self::build(&assets, &mut commands),
        };

        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Spawning InterfaceRoot");

        // TODO: Spawn all the sub-interfaces.

        // Create the camera if it doesn't exist.
        if camera.is_empty() {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Spawning Camera2d");

            commands.spawn(Camera2dBundle::default());
        }
    }

    /// Build the interface root.
    fn build(_assets: &AssetServer, commands: &mut Commands) -> Entity {
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

        entity
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
