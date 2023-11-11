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
                    any_with_component::<LoadingInterfaceActive>()
                        .and_then(LoadingInterface::finish_event_and_loaded),
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
            z_index: ZIndex::Global(i32::MAX - 32),
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
    #[allow(clippy::too_many_arguments)]
    fn finish_event_and_loaded(
        start_event: EventReader<ResourcePacksStartReloadEvent>,
        finish_event: EventReader<ResourcePacksFinishReloadEvent>,

        interface_assets: Res<InterfaceAssets>,
        assets: Res<AssetServer>,

        interface: Query<(), With<InterfaceRoot>>,

        mut finished: Local<bool>,
        mut exists: Local<bool>,
        mut loaded: Local<bool>,
    ) -> bool {
        // Reset finished and loaded if start event is received
        if !start_event.is_empty() {
            *finished = false;
            *exists = false;
            *loaded = false;
        }

        // If not finished and finish event is received, set finished to true
        if !*finished {
            match finish_event.is_empty() {
                true => return false,
                false => {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    debug!("ResourcePacks finished reloading");

                    *finished = true;
                }
            }
        }

        // If exists is false and interface exists, set exists to true
        if !*exists {
            match interface.is_empty() {
                true => return false,
                false => {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    debug!("InterfaceRoot exists");

                    *exists = true;
                }
            }
        }

        // If not loaded and interface assets are loaded, set loaded to true
        if !*loaded {
            match interface_assets.loaded(&assets) {
                false => return false,
                true => {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    debug!("InterfaceAssets loaded");

                    *loaded = true;
                }
            }
        }

        // Return true if resourcepacks are finished loading,
        // all interface assets are loaded,
        // and the interface has been built.
        true
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
