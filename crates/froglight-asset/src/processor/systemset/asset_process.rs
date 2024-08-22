use bevy_app::{App, Update};
use bevy_ecs::{
    reflect::ReflectResource,
    schedule::{IntoSystemSetConfigs, SystemSet},
};
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_state::{
    app::AppExtStates,
    prelude::in_state,
    state::{State, States},
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(Update, AssetProcessSet);

    app.register_type::<AssetProcess>();
    app.init_state::<AssetProcess>();

    app.register_type::<State<AssetProcess>>()
        .register_type_data::<State<AssetProcess>, ReflectResource>();

    // Configure `AssetProcess::Waiting`
    app.configure_sets(
        Update,
        AssetProcess::Waiting.run_if(in_state(AssetProcess::Waiting)).in_set(AssetProcessSet),
    );

    // Configure `AssetProcess::Loading`
    app.configure_sets(
        Update,
        AssetProcess::Loading
            .run_if(in_state(AssetProcess::Loading))
            .after(AssetProcess::Waiting)
            .in_set(AssetProcessSet),
    );

    // Configure `AssetProcess::Processing`
    app.configure_sets(
        Update,
        AssetProcess::Processing
            .run_if(in_state(AssetProcess::Processing))
            .after(AssetProcess::Loading)
            .in_set(AssetProcessSet),
    );

    // Configure `AssetProcess::Spawning`
    app.configure_sets(
        Update,
        AssetProcess::Spawning
            .run_if(in_state(AssetProcess::Spawning))
            .after(AssetProcess::Processing)
            .in_set(AssetProcessSet),
    );

    // Configure `AssetProcess::Finished`
    app.configure_sets(
        Update,
        AssetProcess::Finished
            .run_if(in_state(AssetProcess::Finished))
            .after(AssetProcess::Spawning)
            .in_set(AssetProcessSet),
    );
}

/// A set of systems that process assets.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Reflect, SystemSet)]
pub struct AssetProcessSet;

/// The state of the asset loading process.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SystemSet, States, Reflect,
)]
#[reflect(Default)]
pub enum AssetProcess {
    /// Waiting for assets to be loaded.
    #[default]
    Waiting,
    /// Loading assets.
    Loading,
    /// Processing loaded assets.
    Processing,
    /// Spawning entities from loaded assets.
    Spawning,
    /// Finished loading assets.
    Finished,
}
