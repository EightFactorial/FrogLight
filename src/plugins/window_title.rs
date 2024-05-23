use bevy::{prelude::*, winit::WinitWindows};

/// A plugin that sets a random window title.
///
/// # Note
/// Does not work for Android or iOS.
///
/// No, I don't plan on adding support for mobile platforms.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowTitlePlugin;

impl Plugin for WindowTitlePlugin {
    fn build(&self, app: &mut App) {
        // Run the `set_window_title` system during the pre-startup schedule.
        app.add_systems(PreStartup, Self::set_window_title);
    }
}

impl WindowTitlePlugin {
    /// The window title with placeholders for the version and a random saying.
    const WINDOW_TITLE: &'static str = "FrogLight v{version} - {random}";

    /// The error message when choosing a random saying fails.
    const ERROR_SAYING: &'static str = "Null Pointer Exception";

    /// A list of random sayings to choose from.
    ///
    /// Feel free to suggest more!
    const RANDOM_SAYING: [&'static str; 17] = [
        Self::ERROR_SAYING,
        "Hello, World!",
        "Blazingly Fast!",
        "Don't panic!",
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
        "This statement is false!",
        "Is this thing on?",
    ];

    /// A bevy system that sets a random window title.
    fn set_window_title(windows: NonSend<WinitWindows>) {
        // Get the window title with the version.
        let mut window_title = Self::WINDOW_TITLE.replace("{version}", env!("CARGO_PKG_VERSION"));

        // Get a random saying, or fallback to the error message.
        let random = fastrand::choice(Self::RANDOM_SAYING).unwrap_or(Self::ERROR_SAYING);
        window_title = window_title.replace("{random}", random);

        // Set the window title for all windows.
        info!("Setting window title: \"{window_title}\"");
        for window in windows.windows.values() {
            window.set_title(window_title.as_str());
        }
    }
}
