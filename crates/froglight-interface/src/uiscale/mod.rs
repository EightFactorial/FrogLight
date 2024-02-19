use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

mod enable;
pub use enable::UiScaleEnable;

mod maximum;
pub use maximum::UiScaleMaximum;

use crate::systemsets::{InterfaceStartupSet, InterfaceUpdateSet};

/// A [`Plugin`] for managing the [`UiScale`].
///
/// Automatically scales the UI based on the window size,
/// optionally respecting a maximum scale.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UiScalePlugin;

impl Plugin for UiScalePlugin {
    fn build(&self, app: &mut App) {
        // Add and register the `UiScaleEnable` resource
        app.init_resource::<UiScaleEnable>().register_type::<UiScaleEnable>();

        // Add and register the `UiScaleMaximum` resource
        app.init_resource::<UiScaleMaximum>().register_type::<UiScaleMaximum>();

        // Add startup systems
        app.add_systems(
            Startup,
            UiScalePlugin::set_scale_on_startup
                .run_if(UiScaleEnable::is_enabled)
                .in_set(InterfaceStartupSet),
        );

        // Add update systems
        app.add_systems(
            Update,
            UiScalePlugin::set_scale_on_resize
                .run_if(on_event::<WindowResized>())
                .run_if(UiScaleEnable::is_enabled)
                .in_set(InterfaceUpdateSet),
        );
    }
}

impl UiScalePlugin {
    /// Set the [`UiScale`] based on the window size when the window is created.
    fn set_scale_on_startup(
        query: Query<&Window, With<PrimaryWindow>>,
        mut scale: ResMut<UiScale>,
        scale_max: Res<UiScaleMaximum>,
    ) {
        // Get the primary window
        let Ok(window) = query.get_single() else {
            return;
        };

        // Update the scale
        Self::update_scale(window, &mut scale, *scale_max);
    }

    /// Set the [`UiScale`] based on the window size when the window is resized.
    fn set_scale_on_resize(
        query: Query<&Window, With<PrimaryWindow>>,
        mut scale: ResMut<UiScale>,
        scale_max: Res<UiScaleMaximum>,

        mut events: EventReader<WindowResized>,
    ) {
        for window in events.read() {
            // Get the primary window
            let Ok(window) = query.get(window.window) else {
                continue;
            };

            // Update the scale
            Self::update_scale(window, &mut scale, *scale_max);
        }
    }

    /// Update the [`UiScale`] based on the window size.
    fn update_scale(window: &Window, scale: &mut UiScale, scale_max: UiScaleMaximum) {
        // Calculate the new scale
        let mut value = Self::calculate(window.physical_width(), window.physical_height());

        // Clamp the scale to the maximum if it exists
        if let Some(max) = scale_max.get() {
            value = value.min(max.into());
        }

        // Update the scale if it has changed
        let value = f64::from(value);
        if (**scale - value).abs() > f64::EPSILON {
            debug!("Setting UIScale to: `{value}`");
            **scale = value;
        }
    }

    // max(1, min(floor(width / 320), floor(height / 240)))
    fn calculate(width: u32, height: u32) -> u32 {
        let width = width / 320;
        let height = height / 240;
        width.max(height).max(1)
    }
}