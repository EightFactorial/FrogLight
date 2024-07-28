use bevy::{
    app::{App, PreStartup},
    core::Name,
    ecs::{
        component::{ComponentHooks, ComponentId, StorageType},
        world::DeferredWorld,
    },
    log::warn,
    prelude::{
        any_with_component, not, Bundle, Commands, Component, DespawnRecursiveExt, Entity,
        IntoSystemConfigs, NodeBundle, OnEnter, Parent, Query, ReflectComponent, ReflectDefault,
        Visibility, With,
    },
    reflect::Reflect,
    ui::{Display, FocusPolicy, PositionType, Style, Val, ZIndex},
};
use froglight_asset::AssetState;

use super::{child::LoadingScreenChild, elements};
use crate::camera::{OverlayCameraLayer, RecursiveCameraLayer};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<LoadingScreen>().register_type::<LoadingScreenChild>();

    // Create a `LoadingScreen` if one does not already exist
    app.add_systems(
        PreStartup,
        LoadingScreen::create_loading_screen.run_if(not(any_with_component::<LoadingScreen>)),
    );

    // Show the loading screen when entering `AssetState::Unloaded`
    app.add_systems(OnEnter(AssetState::Unloaded), LoadingScreen::show_loading_screen);
    // Hide the loading screen when entering `AssetState::Loaded`
    app.add_systems(OnEnter(AssetState::Loaded), LoadingScreen::hide_loading_screen);
}

/// A marker [`Component`] that represents a loading screen.
///
/// When this component is added to an entity, a loading screen is attached.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Default, Component)]
pub struct LoadingScreen;

impl LoadingScreen {
    const NAME: &'static str = "Loading Screen";

    /// Creates a new [`LoadingScreen`] entity.
    fn create_loading_screen(mut commands: Commands) { commands.spawn(LoadingBundle::default()); }

    /// Show the [`LoadingScreen`] entity.
    fn show_loading_screen(mut query: Query<&mut Visibility, With<Self>>) {
        #[cfg(debug_assertions)]
        bevy::log::debug!("LoadingScreen: Showing");

        for mut vis in &mut query {
            *vis = Visibility::Visible;
        }
    }

    /// Hide the [`LoadingScreen`] entity or inherit visibility from a parent.
    fn hide_loading_screen(mut query: Query<(&mut Visibility, Option<&Parent>), With<Self>>) {
        #[cfg(debug_assertions)]
        bevy::log::debug!("LoadingScreen: Inheriting");

        for (mut vis, parent) in &mut query {
            if parent.is_some() {
                // If the loading screen has a parent, inherit its visibility
                *vis = Visibility::Inherited;
            } else {
                // Otherwise, hide the loading screen
                *vis = Visibility::Hidden;
            }
        }
    }
}

impl Component for LoadingScreen {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_insert(Self::on_insert).on_remove(Self::on_remove);
    }
}

impl LoadingScreen {
    /// Add a [`LoadingScreenChild`] and create the loading screen.
    fn on_insert(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        if world.get::<LoadingScreenChild>(entity).is_some() {
            warn!("LoadingScreen: Entity already has a LoadingScreenChild?");
        } else {
            // Create a new `Child` entity
            let child = LoadingScreenChild::construct(&mut world, entity);
            world.commands().entity(entity).insert(LoadingScreenChild(child));

            // Construct the loading screen elements
            let _background = elements::LoadingScreenBackground::construct(&mut world, child);
            // TODO: Add elements

            // Lastly, add a `RecursiveCameraLayer`.
            // This must be last to ensure all children are spawned first.
            world
                .commands()
                .entity(child)
                .insert(RecursiveCameraLayer::<OverlayCameraLayer>::default());
        }
    }

    /// Remove the [`LoadingScreenChild`] and despawn the loading screen.
    fn on_remove(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        if let Some(LoadingScreenChild(child)) = world.get::<LoadingScreenChild>(entity).copied() {
            // Remove the `LoadingScreenChild` component
            world.commands().entity(entity).remove::<LoadingScreenChild>();
            // Despawn the `Child` entity and all of its children
            world.commands().entity(child).despawn_recursive();
        } else {
            warn!("LoadingScreen: Entity does not have a LoadingScreenChild?");
        }
    }
}

#[derive(Bundle)]
struct LoadingBundle {
    loading_screen: LoadingScreen,
    node_bundle: NodeBundle,
    name: Name,
}

impl Default for LoadingBundle {
    fn default() -> Self {
        Self {
            loading_screen: LoadingScreen,
            node_bundle: NodeBundle {
                // Fill the parent `Node` or `Window`
                style: Style {
                    display: Display::Flex,
                    position_type: PositionType::Relative,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                // Block input to any menus underneath
                focus_policy: FocusPolicy::Block,
                // Ensure the `LoadingScreen` is always on top
                z_index: ZIndex::Global(i32::MAX / 2),
                ..Default::default()
            },
            name: Name::new(LoadingScreen::NAME),
        }
    }
}
