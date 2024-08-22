use bevy_app::{App, Update};
use bevy_ecs::{
    reflect::ReflectResource,
    schedule::{IntoSystemSetConfigs, SystemSet},
};
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_state::{
    app::AppExtStates,
    prelude::in_state,
    state::{ComputedStates, State},
};

use super::AssetProcess;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(Update, AssetStateSet);

    app.register_type::<AssetState>();
    app.add_computed_state::<AssetState>().enable_state_scoped_entities::<AssetState>();

    app.register_type::<State<AssetState>>()
        .register_type_data::<State<AssetState>, ReflectResource>();

    // Configure `AssetState::Unloaded`
    app.configure_sets(
        Update,
        AssetState::Unloaded.run_if(in_state(AssetState::Unloaded)).in_set(AssetStateSet),
    );

    // Configure `AssetState::Loaded`
    app.configure_sets(
        Update,
        AssetState::Loaded
            .run_if(in_state(AssetState::Loaded))
            .after(AssetState::Unloaded)
            .in_set(AssetStateSet),
    );
}

/// A set of systems that rely on the state of assets.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Reflect, SystemSet)]
pub struct AssetStateSet;

/// The state of all assets.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SystemSet, Reflect)]
#[reflect(Default)]
pub enum AssetState {
    /// Assets are not loaded.
    #[default]
    Unloaded,
    /// Assets are loaded.
    Loaded,
}

impl ComputedStates for AssetState {
    type SourceStates = AssetProcess;
    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            AssetProcess::Spawning | AssetProcess::Finished => Some(Self::Loaded),
            _ => Some(Self::Unloaded),
        }
    }
}
