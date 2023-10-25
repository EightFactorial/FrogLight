use bevy::prelude::*;
use mc_rs_core::resources::client_information::ClientInformation;
use mc_rs_interface::settings::Settings;

mod default;

/// Add plugins to the [App].
///
/// Plugins added changes depending on the enabled features.
pub(super) fn add_plugins(app: &mut App) {
    let settings = Settings::load();

    // Add default plugins
    default::default_plugins(&settings).finish(app);
    app.insert_resource(settings);
    app.init_resource::<ClientInformation>();
}
