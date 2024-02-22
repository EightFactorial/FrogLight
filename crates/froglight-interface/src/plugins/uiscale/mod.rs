//! A plugin for managing the UI scale.

use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

mod enable;
pub use enable::UiScaleEnable;

mod maximum;
pub use maximum::UiScaleMaximum;

use self::systemset::{UiScaleStartupSet, UiScaleUpdateSet};

pub(crate) mod systemset;

/// A [`Plugin`] for managing the [`UiScale`].
///
/// Automatically scales the UI based on the window size,
/// optionally respecting a maximum scale.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UiScalePlugin;

impl Plugin for UiScalePlugin {
    fn build(&self, app: &mut App) {
        // Add the `SystemSet`s
        systemset::build(app);

        // Register the `UiScale` resource
        app.register_type_data::<UiScale, ReflectResource>();

        // Add and register the `UiScaleEnable` resource
        app.init_resource::<UiScaleEnable>().register_type::<UiScaleEnable>();

        // Add and register the `UiScaleMaximum` resource
        app.init_resource::<UiScaleMaximum>().register_type::<UiScaleMaximum>();

        // Add startup systems
        app.add_systems(
            Startup,
            UiScalePlugin::set_uiscale.run_if(UiScaleEnable::is_enabled).in_set(UiScaleStartupSet),
        );

        // Add update systems
        app.add_systems(
            Update,
            UiScalePlugin::set_uiscale
                .ambiguous_with(UiScalePlugin::set_uiscale_on_resize)
                .run_if(UiScaleEnable::is_enabled)
                .run_if(resource_exists_and_changed::<UiScaleMaximum>)
                .in_set(UiScaleUpdateSet),
        );
        app.add_systems(
            Update,
            UiScalePlugin::set_uiscale_on_resize
                .ambiguous_with(UiScalePlugin::set_uiscale)
                .run_if(on_event::<WindowResized>())
                .run_if(UiScaleEnable::is_enabled)
                .in_set(UiScaleUpdateSet),
        );
    }
}

impl UiScalePlugin {
    /// Set the [`UiScale`] based on the window size.
    fn set_uiscale(
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
    fn set_uiscale_on_resize(
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
    #[allow(clippy::cast_precision_loss)]
    fn update_scale(window: &Window, scale: &mut UiScale, scale_max: UiScaleMaximum) {
        // Calculate the new scale
        let mut value = Self::calculate(window.physical_width(), window.physical_height());

        // Clamp the scale to the maximum if it exists
        if let Some(max) = scale_max.get() {
            value = value.min(max.into());
        }

        // Update the scale if it has changed
        let value = value as f32;
        if (**scale - value).abs() > f32::EPSILON {
            debug!("Setting UIScale to: `{value}`");
            **scale = value;
        }
    }

    /// max(1, min(floor(width / 320), floor(height / 240)))
    ///
    /// A 300x225 rectangle in the center seems to never go off the screen?
    fn calculate(width: u32, height: u32) -> u32 {
        let w_scale = width / 320;
        let h_scale = height / 240;

        w_scale.min(h_scale).max(1)
    }
}
