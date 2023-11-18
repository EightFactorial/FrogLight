use bevy::{prelude::*, window::WindowResized};

mod comp;
pub use comp::*;

mod res;
pub use res::*;

pub(super) fn setup(app: &mut App) {
    app.add_systems(
        Startup,
        GuiScale::initialize.run_if(not(resource_exists::<GuiScale>())),
    );

    app.add_systems(
        PreUpdate,
        (
            GuiScale::update_scale.run_if(on_event::<WindowResized>()),
            GuiScaleComponent::resize_update.run_if(resource_exists_and_changed::<GuiScale>()),
            GuiScaleComponent::added_update.run_if(GuiScaleComponent::scale_added),
        )
            .chain(),
    );
}
