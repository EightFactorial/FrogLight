use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Resource)]
pub struct DefaultFonts {
    pub regular: Handle<Font>,
    pub bold: Handle<Font>,
    pub italic: Handle<Font>,
    pub bold_italic: Handle<Font>,
}

impl DefaultFonts {
    #[allow(clippy::needless_pass_by_value)]
    pub(super) fn initialize(assets: Res<AssetServer>, mut commands: Commands) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Initializing DefaultFonts");

        commands.insert_resource(Self {
            regular: assets.load("embedded://mc_rs_gui/resources/font/embedded/Regular.otf"),
            bold: assets.load("embedded://mc_rs_gui/resources/font/embedded/Bold.otf"),
            italic: assets.load("embedded://mc_rs_gui/resources/font/embedded/Italic.otf"),
            bold_italic: assets.load("embedded://mc_rs_gui/resources/font/embedded/BoldItalic.otf"),
        });
    }
}
