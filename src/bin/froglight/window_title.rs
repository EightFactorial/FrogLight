use bevy::{prelude::*, winit::WinitWindows};

/// A plugin that sets a random window title.
///
/// # Note
/// Does not work for Android or iOS.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct WindowTitlePlugin;

impl Plugin for WindowTitlePlugin {
    fn build(&self, app: &mut App) { app.add_systems(PreStartup, Self::set_window_title); }
}

impl WindowTitlePlugin {
    /// The window title with placeholders for the version and a random saying.
    const WINDOW_TITLE: &'static str = "Froglight v{version} - {random}";

    /// The error message when choosing a random saying fails.
    const ERROR_SAYING: &'static str = "Null Pointer Exception";

    /// A list of random sayings to choose from.
    ///
    /// Feel free to suggest more!
    const RANDOM_SAYING: [&'static str; 15] = [
        Self::ERROR_SAYING,
        "Hello, World!",
        "Blazingly Fast!",
        "Open Source!",
        "Some assembly required!",
        "Batteries not included!",
        "Works on my machine!",
        "Just add water!",
        "It's what plants crave!",
        "I'm a teapot!",
        "Your mileage may vary!",
        "It's super effective!",
        "Your princess is in another castle!",
        "Objects in mirror are closer than they appear!",
        "Do not pass go, do not collect $200!",
    ];

    /// A bevy system that sets a random window title.
    fn set_window_title(windows: NonSend<WinitWindows>) {
        // Get the window title with the version.
        let mut window_title = Self::WINDOW_TITLE.replace("{version}", env!("CARGO_PKG_VERSION"));

        // Get a random saying, or fallback to the error message.
        let random = fastrand::choice(Self::RANDOM_SAYING).unwrap_or(Self::ERROR_SAYING);
        window_title = window_title.replace("{random}", random);

        // Set the window title for all windows.
        debug!("Setting window title: \"{window_title}\"");
        for window in windows.windows.values() {
            window.set_title(window_title.as_str());
        }
    }
}
