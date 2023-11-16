use bevy::{prelude::*, window::PrimaryWindow};

/// The GUI scale.
///
/// This automatically updates when the window is resized, unless it is set to [GuiScale::Fixed].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource)]
pub enum GuiScale {
    Auto(u32),
    Fixed { max: u32, actual: u32 },
}

impl Default for GuiScale {
    fn default() -> Self { Self::Auto(1) }
}

impl GuiScale {
    /// Initialize the `GuiScale` resource with the current window size if it doesn't exist.
    pub(super) fn initialize(window: Query<&Window, With<PrimaryWindow>>, mut commands: Commands) {
        let scale = match window.get_single() {
            Ok(window) => {
                let scale = Self::new_auto(window.width() as u32, window.height() as u32);

                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Initializing GuiScale at {scale:?}");
                scale
            }
            Err(_) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Window not found, setting GuiScale to default");
                Self::default()
            }
        };

        commands.insert_resource(scale);
    }

    /// Update the `GuiScale` resource if the window size has changed.
    pub(super) fn update_scale(
        window: Query<&Window, With<PrimaryWindow>>,
        mut scale: ResMut<GuiScale>,
    ) {
        let Ok(window) = window.get_single() else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Window resized but not found, not updating gui scale");
            return;
        };

        let new_scale = Self::scale(window.width() as u32, window.height() as u32);

        match &mut *scale {
            // Set the scale to the new scale
            GuiScale::Auto(scale) => {
                if new_scale != *scale {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    debug!("Window resize set GuiScale to {new_scale:?}");
                    *scale = new_scale;
                }
            }
            // Set the scale to at most the minimum of the new scale and the maximum scale
            GuiScale::Fixed { max, actual } => {
                let max = std::cmp::min(new_scale, *max);

                if max != *actual {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    debug!("Window resize set GuiScale to {max:?}");
                    *actual = max;
                }
            }
        }
    }

    /// Calculate the scale value for the given width and height.
    fn scale(width: u32, height: u32) -> u32 {
        std::cmp::max(1, std::cmp::min(width / 320, height / 240))
    }

    /// Get the scale value.
    pub fn value(&self) -> u32 {
        match self {
            Self::Auto(scale) => *scale,
            Self::Fixed { actual, .. } => *actual,
        }
    }

    /// Create a new [GuiScale::Auto] with the given width and height.
    pub fn new_auto(width: u32, height: u32) -> Self { Self::Auto(Self::scale(width, height)) }

    /// Create a new [GuiScale::Fixed] with the given maximum scale, width and height.
    pub fn new_fixed(max: u32, width: u32, height: u32) -> Self {
        Self::Fixed {
            max,
            actual: std::cmp::min(max, Self::scale(width, height)),
        }
    }
}
