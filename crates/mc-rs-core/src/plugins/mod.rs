use bevy::prelude::*;
use bevy_rapier3d::prelude::RapierPhysicsPlugin;

mod default;
use default::DefaultPlugins;

pub(super) fn setup(app: &mut App) {
    // Add Rapier physics plugins
    app.add_plugins(RapierPhysicsPlugin::<()>::default());

    #[cfg(feature = "debug_rapier")]
    {
        use bevy_rapier3d::render::RapierDebugRenderPlugin;
        app.add_plugins(RapierDebugRenderPlugin::default());
    }

    // Add default plugins
    app.add_plugins(DefaultPlugins);
}
