use bevy::prelude::*;
use mc_rs_core::schedule::{set::LoadingSet, state::ApplicationState};

use crate::resourcepacks::{ResourcePacksFinishReloadEvent, ResourcePacksStartReloadEvent};

use super::{camera::DefaultCamera, InterfaceAssets, InterfaceRoot};

/// The screen that appears when initializing the game
/// and when reloading resourcepacks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct LoadingInterface;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
struct LoadingInterfaceActive;

impl LoadingInterface {
    /// Setup the loading interface systems.
    pub(super) fn setup(app: &mut App) {
        // Build the LoadingInterface on startup.
        app.add_systems(
            Startup,
            (LoadingInterface::build, DefaultCamera::create_camera2d()),
        );

        app.add_systems(
            Update,
            (
                // Show the LoadingInterface when reloading resourcepacks
                LoadingInterface::show.run_if(
                    not(any_with_component::<LoadingInterfaceActive>())
                        .and_then(on_event::<ResourcePacksStartReloadEvent>()),
                ),
                // Change state to MainMenu when resourcepacks are finished reloading,
                // but *do not* show the main menu until all interface assets are loaded.
                LoadingInterface::change_state
                    .run_if(
                        in_state(ApplicationState::Loading)
                            .and_then(on_event::<ResourcePacksFinishReloadEvent>()),
                    )
                    .in_set(LoadingSet),
                // Hide the LoadingInterface when resourcepacks are finished reloading,
                // all interface assets are loaded, and the interface has been built.
                LoadingInterface::hide.run_if(
                    any_with_component::<LoadingInterfaceActive>().and_then(
                        LoadingInterface::finish_event_and_loaded
                            .and_then(any_with_component::<InterfaceRoot>()),
                    ),
                ),
            ),
        );
    }

    /// Build the loading interface.
    // TODO: Make it look nice
    fn build(world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building LoadingInterface");

        // Create the main node
        let node = NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            // TODO: It loads too fast, so we'll just make it blue for now
            background_color: BackgroundColor(Color::BLUE),
            visibility: Visibility::Visible,
            ..Default::default()
        };

        // Add text to the main node
        let text = TextBundle::from_section(
            "Loading...",
            TextStyle {
                font_size: 32.0,
                ..Default::default()
            },
        );

        world
            .spawn((LoadingInterface, LoadingInterfaceActive, node))
            .with_children(|root| {
                root.spawn(text);
            });
    }

    fn change_state(mut state: ResMut<NextState<ApplicationState>>) {
        state.set(ApplicationState::MainMenu);
    }

    /// Returns true if the resourcepacks are finished
    /// reloading and all interface assets are loaded.
    fn finish_event_and_loaded(
        start_event: EventReader<ResourcePacksStartReloadEvent>,
        finish_event: EventReader<ResourcePacksFinishReloadEvent>,

        interface_assets: Res<InterfaceAssets>,
        assets: Res<AssetServer>,

        mut finished: Local<bool>,
        mut loaded: Local<bool>,
    ) -> bool {
        // Reset finished and loaded if start event is received
        if !start_event.is_empty() {
            *finished = false;
            *loaded = false;
        }

        // If not finished and finish event is received, set finished to true
        if !*finished && !finish_event.is_empty() {
            *finished = true;
        }

        // If not loaded and interface assets are loaded, set loaded to true
        if !*loaded && interface_assets.loaded(&assets) {
            *loaded = true;
        }

        // Return true if both finished and all assets are loaded
        *finished && *loaded
    }

    /// Show the loading interface.
    // TODO: Have nice animations and stuff
    fn show(
        mut query: Query<(Entity, &mut Visibility), With<LoadingInterface>>,
        mut commands: Commands,
    ) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Showing LoadingInterface");

        query.for_each_mut(|(entity, mut vis)| {
            commands.entity(entity).insert(LoadingInterfaceActive);

            *vis = Visibility::Visible;
        });
    }

    /// Hide the loading interface.
    // TODO: Have nice animations and stuff
    fn hide(
        mut query: Query<(Entity, &mut Visibility), With<LoadingInterfaceActive>>,
        mut commands: Commands,
    ) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Hiding LoadingInterface");

        query.for_each_mut(|(entity, mut vis)| {
            commands.entity(entity).remove::<LoadingInterfaceActive>();

            *vis = Visibility::Hidden;
        });
    }
}
