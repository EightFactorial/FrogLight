use bevy::prelude::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Add and register the AssetLoading states
    app.init_state::<AssetLoading>().register_type::<AssetLoading>();
    app.register_type::<State<AssetLoading>>()
        .register_type_data::<State<AssetLoading>, ReflectResource>();

    // Add the AssetUpdateSet SystemSet
    app.configure_sets(Update, AssetUpdateSet);

    // Add the AssetLoading SystemSets
    app.configure_sets(
        Update,
        AssetLoading::Waiting.run_if(in_state(AssetLoading::Waiting)).in_set(AssetUpdateSet),
    );
    app.configure_sets(
        Update,
        AssetLoading::Loading.run_if(in_state(AssetLoading::Loading)).in_set(AssetUpdateSet),
    );
    app.configure_sets(
        Update,
        AssetLoading::Processing.run_if(in_state(AssetLoading::Processing)).in_set(AssetUpdateSet),
    );
    app.configure_sets(
        Update,
        AssetLoading::Finished.run_if(in_state(AssetLoading::Finished)).in_set(AssetUpdateSet),
    );
}

/// A [`SystemSet`] containing systems for [`AssetLoading`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct AssetUpdateSet;

/// The state of the [`AssetLoading`] system.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect, States, SystemSet,
)]
#[reflect(Default)]
pub enum AssetLoading {
    /// Asset loading has not started.
    #[default]
    Waiting,
    /// Assets are being loaded.
    Loading,
    /// Assets are being processed.
    Processing,
    /// Asset loading has finished.
    Finished,
}
