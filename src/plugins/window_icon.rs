use bevy::{prelude::*, winit::WinitWindows};
use image::{imageops::FilterType, ImageFormat};
use winit::{
    raw_window_handle::{HasWindowHandle, RawWindowHandle},
    window::Icon,
};

/// A plugin for setting the application window icon.
///
/// # Note
/// This only works on Windows and Linux using X11.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowIconPlugin;

impl Plugin for WindowIconPlugin {
    fn build(&self, app: &mut App) {
        // Run the `set_window_icon` system during the pre-startup schedule.
        app.add_systems(PreStartup, Self::set_window_icon);
    }
}

impl WindowIconPlugin {
    /// The debug window icon.
    #[cfg(debug_assertions)]
    const WINDOW_ICON: &'static [u8; 3620] = include_bytes!("../assets/pearlescent.png");
    /// The release window icon.
    #[cfg(not(debug_assertions))]
    const WINDOW_ICON: &'static [u8; 3540] = include_bytes!("../assets/verdant.png");

    /// A bevy system that sets all window icons.
    fn set_window_icon(windows: NonSend<WinitWindows>) {
        // Load the icon image.
        let icon_image =
            match image::load_from_memory_with_format(Self::WINDOW_ICON, ImageFormat::Png) {
                Ok(icon_image) => icon_image.resize(32, 32, FilterType::Triangle).into_rgba8(),
                Err(err) => {
                    error!("Failed to load window icon: \"{err}\"");
                    return;
                }
            };

        // Create the window icon.
        let (icon_width, icon_height) = icon_image.dimensions();
        let window_icon = match Icon::from_rgba(icon_image.into_raw(), icon_width, icon_height) {
            Ok(icon) => icon,
            Err(err) => {
                error!("Failed to create window icon: \"{err}\"");
                return;
            }
        };

        // Set the window icon for all windows.
        for window in windows.windows.values() {
            // Log whether setting the window icon is supported.
            if let Ok(handle) = window.window_handle() {
                if matches!(
                    handle.as_raw(),
                    RawWindowHandle::Xcb(_)
                        | RawWindowHandle::Xlib(_)
                        | RawWindowHandle::Win32(_)
                        | RawWindowHandle::WinRt(_)
                ) {
                    debug!("Setting window icon");
                } else {
                    warn!("Setting window icon not supported on this platform");
                }
            }

            // Set the window icon anyway, even if it's not supported.
            window.set_window_icon(Some(window_icon.clone()))
        }
    }
}
