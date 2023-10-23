use belly::prelude::BellyPlugin;
use bevy::prelude::*;
use bevy_rapier3d::prelude::RapierPhysicsPlugin;
use mc_rs_core::resources::client_information::ClientInformation;

use crate::systems::settings::Settings;

mod default;

#[cfg(feature = "splash")]
pub(crate) mod splash;

#[cfg(feature = "debug")]
mod debug;

/// Add plugins to the [App].
///
/// Plugins added changes depending on the enabled features.
pub(super) fn add_plugins(app: &mut App) {
    let settings = Settings::load();

    // Add default plugins
    default::default_plugins(&settings).finish(app);
    app.insert_resource(settings);
    app.init_resource::<ClientInformation>();

    // Add Belly plugin
    app.add_plugins(BellyPlugin);

    // Add Rapier physics plugins
    app.add_plugins(RapierPhysicsPlugin::<()>::default());

    #[cfg(feature = "rapier_debug")]
    {
        use bevy_rapier3d::render::RapierDebugRenderPlugin;
        app.add_plugins(RapierDebugRenderPlugin::default());
    }

    #[cfg(feature = "splash")]
    {
        use splash::SplashPlugin;
        app.add_plugins(SplashPlugin);
    }

    #[cfg(feature = "debug")]
    {
        use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
        app.add_plugins(FrameTimeDiagnosticsPlugin);

        use debug::DebugPlugin;
        app.add_plugins(DebugPlugin);
    }
}
