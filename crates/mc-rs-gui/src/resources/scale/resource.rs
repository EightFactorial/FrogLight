use bevy::{prelude::*, window::PrimaryWindow};

/// An event that is sent when the [`GuiScale`] changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Event)]
pub struct GuiScaleEvent;

/// The GUI scale.
///
/// This automatically updates when the window is resized, unless it is set to [`GuiScale::Fixed`].
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
    #[allow(clippy::needless_pass_by_value)]
    pub(super) fn initialize(window: Query<&Window, With<PrimaryWindow>>, mut commands: Commands) {
        if let Ok(window) = window.get_single() {
            #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            let scale = Self::new_auto(window.width() as u32, window.height() as u32);

            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Initializing GuiScale at {scale:?}");
            commands.insert_resource(scale);
        } else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Window not found, setting GuiScale to default");
            commands.insert_resource(Self::default());
        }
    }

    /// Update the `GuiScale` resource if the window size has changed.
    #[allow(clippy::needless_pass_by_value)]
    pub(super) fn update_scale(
        window: Query<&Window, With<PrimaryWindow>>,
        mut scale: ResMut<GuiScale>,
        mut events: EventWriter<GuiScaleEvent>,
    ) {
        let Ok(window) = window.get_single() else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Window resized but not found, not updating gui scale");
            return;
        };

        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let new_scale = Self::scale(window.width() as u32, window.height() as u32);

        match &mut *scale {
            // Set the scale to the new scale
            GuiScale::Auto(scale) => {
                if new_scale != *scale {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    debug!("Window resize set GuiScale to {new_scale:?}");
                    *scale = new_scale;

                    events.send(GuiScaleEvent);
                }
            }
            // Set the scale to at most the minimum of the new scale and the maximum scale
            GuiScale::Fixed { max, actual } => {
                let max = std::cmp::min(new_scale, *max);

                if max != *actual {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    debug!("Window resize set GuiScale to {max:?}");
                    *actual = max;

                    events.send(GuiScaleEvent);
                }
            }
        }
    }

    /// Calculate the scale value for the given width and height.
    fn scale(width: u32, height: u32) -> u32 {
        std::cmp::max(1, std::cmp::min(width / 320, height / 240))
    }

    /// Get the scale value.
    #[must_use]
    pub fn value(&self) -> u32 {
        match self {
            Self::Auto(scale) => *scale,
            Self::Fixed { actual, .. } => *actual,
        }
    }

    /// Create a new [`GuiScale::Auto`] with the given width and height.
    #[must_use]
    pub fn new_auto(width: u32, height: u32) -> Self { Self::Auto(Self::scale(width, height)) }

    /// Create a new [`GuiScale::Fixed`] with the given maximum scale, width and height.
    #[must_use]
    pub fn new_fixed(max: u32, width: u32, height: u32) -> Self {
        Self::Fixed {
            max,
            actual: std::cmp::min(max, Self::scale(width, height)),
        }
    }
}
