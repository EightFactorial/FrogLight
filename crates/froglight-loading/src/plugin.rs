use bevy::prelude::*;

/// A [`Plugin`] that shows a loading screen while assets are being loaded
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoadingPlugin(pub String);

/// The asset path to the loading art
#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource)]
pub(crate) struct LoadingPluginArtPath(pub(crate) String);

impl LoadingPlugin {
    /// The asset path to the default loading art
    pub const DEFAULT_EMBEDDED_ART_PATH: &'static str =
        "embedded://froglight_loading/assets/loading_art.png";

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
    /// let plugin = LoadingPlugin::new("embedded://my_custom_crate/assets/my_custom_art.png");
    ///
    /// // Add the plugin to the App
    /// app.add_plugins(plugin);
    /// ```
    pub fn new(path: impl Into<String>) -> Self { Self(path.into()) }
}

impl Default for LoadingPlugin {
    fn default() -> Self { Self::new(Self::DEFAULT_EMBEDDED_ART_PATH) }
}

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        // Add the loading art asset path
        app.insert_resource(LoadingPluginArtPath(self.0.clone()));

        // Setup the loading screen
        crate::systemsets::setup(app);
        crate::layout::setup(app);
    }

    fn cleanup(&self, app: &mut App) {
        // Setup assets
        crate::assets::setup(app);
    }
}
