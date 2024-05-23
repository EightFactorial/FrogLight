use bevy::app::{PluginGroup, PluginGroupBuilder};

#[cfg(feature = "inspector")]
pub mod inspector;

#[cfg(feature = "mimalloc")]
pub mod mimalloc;

#[cfg(any(target_os = "windows", target_os = "linux"))]
pub mod window_icon;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub mod window_title;

/// A [`PluginGroup`] with extra plugins.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtraPlugins;

impl PluginGroup for ExtraPlugins {
    #[allow(unused_mut)]
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();

        // Add the `InspectorPlugin` if the `inspector` feature is enabled.
        #[cfg(feature = "inspector")]
        {
            builder = builder.add(inspector::InspectorPlugin);
        }

        // Add the `MiMallocPlugin` if the `mimalloc` feature is enabled.
        #[cfg(feature = "mimalloc")]
        {
            builder = builder.add(mimalloc::MiMallocPlugin);
        }

        // Add the `WindowIconPlugin` if the target OS is Windows or Linux.
        //
        // Window icons are only supported on Windows and Linux using X11.
        #[cfg(any(target_os = "windows", target_os = "linux"))]
        {
            builder = builder.add(window_icon::WindowIconPlugin);
        }

        // Add the `WindowTitlePlugin` if the target OS is not Android or iOS.
        //
        // Window titles are not supported on mobile platforms.
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        {
            builder = builder.add(window_title::WindowTitlePlugin);
        }

        builder
    }
}
