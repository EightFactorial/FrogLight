use bevy_app::{App, Update};
use bevy_ecs::{
    schedule::{
        common_conditions::in_state, BoxedCondition, IntoSystemConfigs, IntoSystemSetConfigs,
        NextState, State, States, SystemSet,
    },
    system::{Res, ResMut},
};
use bevy_log::{debug, warn};
use froglight_core::systemsets::AssetUpdateSet;

use super::AssetPlugin;
use crate::{AssetManager, ReloadAssets, ResourcePackSettings};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Initialize the asset loading state.
    app.init_state::<AssetLoadingState>();

    // Configure AssetLoadingState::Paused
    {
        app.configure_sets(
            Update,
            AssetLoadingState::Paused
                .run_if(in_state(AssetLoadingState::Paused))
                .in_set(AssetUpdateSet),
        );
    }

    // Configure AssetLoadingState::LoadingResources
    {
        app.configure_sets(
            Update,
            AssetLoadingState::LoadingResources
                .run_if(in_state(AssetLoadingState::LoadingResources))
                .in_set(AssetUpdateSet),
        );
        app.add_systems(
            Update,
            AssetLoadingState::enter_next_state
                .after(ResourcePackSettings::load_resourcepack)
                .after(ReloadAssets::reload_assets_event)
                .run_if(ResourcePackSettings::all_loaded)
                .run_if(AssetManager::all_loaded)
                .ambiguous_with_all()
                .in_set(AssetLoadingState::LoadingResources),
        );
    }

    // Configure AssetLoadingState::BuildingAssets
    {
        app.configure_sets(
            Update,
            AssetLoadingState::BuildingAssets
                .run_if(in_state(AssetLoadingState::BuildingAssets))
                .in_set(AssetUpdateSet),
        );
        app.add_systems(
            Update,
            AssetLoadingState::enter_next_state
                .ambiguous_with_all()
                .in_set(AssetLoadingState::BuildingAssets),
        );
    }

    // Configure AssetLoadingState::WaitingForSystems
    {
        app.configure_sets(
            Update,
            AssetLoadingState::WaitingForSystems
                .run_if(in_state(AssetLoadingState::WaitingForSystems))
                .in_set(AssetUpdateSet),
        );

        // `enter_next_state` is added in `finish` to
        // ensure that all conditions are added.
    }

    // Configure AssetLoadingState::Ready
    {
        app.configure_sets(
            Update,
            AssetLoadingState::Ready
                .run_if(in_state(AssetLoadingState::Ready))
                .in_set(AssetUpdateSet),
        );
    }
}

#[doc(hidden)]
pub(super) fn finish(plugin: &AssetPlugin, app: &mut App) {
    // Add the `enter_next_state` system to the `WaitingForSystems` state
    {
        // Get the `next_state` system
        let mut next_state_system = AssetLoadingState::enter_next_state.into_configs();

        // Add the conditions from the plugin
        let conditions: Vec<BoxedCondition> = std::mem::take(&mut *plugin.conditions.lock());
        debug!("AssetLoadingState::WaitingForSystems: {} conditions", conditions.len());

        for c in conditions {
            #[cfg(debug_assertions)]
            debug!("Condition: {}", c.name());

            next_state_system.run_if_dyn(c);
        }

        // Add the system
        app.add_systems(
            Update,
            next_state_system.ambiguous_with_all().in_set(AssetLoadingState::WaitingForSystems),
        );
    }
}

/// The asset loading state.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States, SystemSet)]
pub enum AssetLoadingState {
    /// Paused
    #[default]
    Paused,
    /// Loading resources
    LoadingResources,
    /// Building assets
    BuildingAssets,
    /// Waiting for other systems
    WaitingForSystems,
    /// Ready
    Ready,
}

impl AssetLoadingState {
    fn enter_next_state(state: Res<State<Self>>, mut next_state: ResMut<NextState<Self>>) {
        match **state {
            AssetLoadingState::Paused => next_state.set(AssetLoadingState::LoadingResources),
            AssetLoadingState::LoadingResources => {
                next_state.set(AssetLoadingState::BuildingAssets);
            }
            AssetLoadingState::BuildingAssets => {
                next_state.set(AssetLoadingState::WaitingForSystems);
            }
            AssetLoadingState::WaitingForSystems => next_state.set(AssetLoadingState::Ready),
            AssetLoadingState::Ready => warn!("Already in AssetLoadingState::Ready state?"),
        }

        #[cfg(debug_assertions)]
        debug!("AssetLoadingState: {:?} -> {:?}", **state, next_state.0.unwrap());
    }
}
