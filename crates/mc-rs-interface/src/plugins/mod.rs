use belly::prelude::BellyPlugin;
use bevy::prelude::*;

#[cfg(feature = "debug")]
mod debug;

#[cfg(feature = "splash")]
mod splash;

pub(super) fn add_plugins(app: &mut App) {
    // Add Belly plugin
    app.add_plugins(BellyPlugin);

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
