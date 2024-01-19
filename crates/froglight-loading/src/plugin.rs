use bevy::prelude::*;

/// A [`Plugin`] that shows a loading screen while assets are being loaded
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoadingPlugin(pub String);

/// The asset path to the loading icon
#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource)]
pub(crate) struct LoadingPluginIconPath(pub(crate) String);

impl LoadingPlugin {
    /// The asset path to the default loading icon
    pub const DEFAULT_ICON_PATH: &'static str =
        "embedded://froglight_loading/assets/loading_icon.png";

    /// Creates a new [`LoadingPlugin`] with the given asset path
    ///
    /// # Example
    /// ```rust
    /// use bevy::prelude::*;
    /// use froglight_loading::LoadingPlugin;
    ///
    /// // Create a new App
    /// let mut app = App::new();
    ///
    /// // Create a new LoadingPlugin with an embedded asset path
    /// let plugin = LoadingPlugin::new("embedded://my_custom_crate/assets/my_custom_icon.png");
    ///
    /// // Add the plugin to the App
    /// app.add_plugins(plugin);
    /// ```
    pub fn new(path: impl Into<String>) -> Self { Self(path.into()) }
}

impl Default for LoadingPlugin {
    fn default() -> Self { Self::new(Self::DEFAULT_ICON_PATH) }
}

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        // Add the loading icon asset path
        app.insert_resource(LoadingPluginIconPath(self.0.clone()));

        // Setup the loading screen
        crate::assets::setup(app);
        crate::layout::setup(app);
    }

    fn finish(&self, app: &mut App) {
        // Create the loading screen
        crate::layout::finish(app);
    }
}
