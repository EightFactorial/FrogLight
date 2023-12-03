use bevy::{asset::embedded_asset, prelude::*};

mod default_style;
pub use default_style::{DefaultTextStyle, IgnoreDefaultTextStyle};

mod fonts;
pub use fonts::DefaultFonts;

pub mod shadows;

use super::scale::GuiScaleEvent;

pub(super) fn setup(app: &mut App) {
    embedded_asset!(app, "embedded/Regular.otf");
    embedded_asset!(app, "embedded/Bold.otf");
    embedded_asset!(app, "embedded/Italic.otf");
    embedded_asset!(app, "embedded/BoldItalic.otf");

    app.add_systems(
        PreStartup,
        DefaultFonts::initialize.run_if(not(resource_exists::<DefaultFonts>())),
    );
    app.add_systems(
        Startup,
        DefaultTextStyle::initialize.run_if(not(resource_exists::<DefaultTextStyle>())),
    );

    app.add_systems(
        Update,
        (
            DefaultTextStyle::resize_font.run_if(on_event::<GuiScaleEvent>()),
            DefaultTextStyle::update_styles.run_if(
                resource_exists_and_changed::<DefaultTextStyle>()
                    .or_else(DefaultTextStyle::any_added_texts),
            ),
        )
            .chain(),
    );

    shadows::setup(app);
}
