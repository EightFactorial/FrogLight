use bevy::{prelude::*, window::WindowResized};

mod scale;
pub use scale::GuiScale;

pub(super) fn setup(app: &mut App) {
    app.add_systems(
        Startup,
        GuiScale::initialize.run_if(not(resource_exists::<GuiScale>())),
    );

    app.add_systems(
        PreUpdate,
        GuiScale::update_scale.run_if(on_event::<WindowResized>()),
    );
}
