use bevy::prelude::*;

/// A custom loading plugin that displays a GIF instead of a static image.
///
/// This requires [`froglight_client`]'s `default-loading` feature to be enabled
/// **and** the [`LoadingPlugin`](froglight_client::loading::LoadingPlugin) to
/// be
/// set to [`LoadingPlugin::None`](froglight_client::loading::LoadingPlugin::None).
///
///
/// # Note
/// This plugin assumes a lot about the GIF's path and resolution.
#[derive(Debug, Clone, PartialEq)]
pub struct GifLoadingPlugin {
    pub path: String,
    pub frame_height: f32,
    pub frame_width: f32,
    pub frame_count: usize,
}

impl Plugin for GifLoadingPlugin {
    fn build(&self, app: &mut App) {
        // Insert the GIF settings into the app
        app.insert_resource(GifSettings::from(self));

        // TODO: Show the GIF and update the displayed frame
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Resource)]
pub(crate) struct GifSettings {
    pub(crate) frame_height: f32,
    pub(crate) frame_width: f32,
    pub(crate) frame_count: usize,
}

impl From<&GifLoadingPlugin> for GifSettings {
    fn from(plugin: &GifLoadingPlugin) -> Self {
        Self {
            frame_height: plugin.frame_height,
            frame_width: plugin.frame_width,
            frame_count: plugin.frame_count,
        }
    }
}
