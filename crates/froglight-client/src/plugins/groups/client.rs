use bevy::app::{PluginGroup, PluginGroupBuilder};

/// A [`PluginGroup`] that includes most [`FrogLight`](crate) plugins.
///
/// This group does not include [`bevy's`](bevy)
/// [`DefaultPlugins`](bevy::DefaultPlugins), use with caution.
///
/// For most use cases, [`AppPlugins`](super::app::AppPlugins) is recommended.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientPlugins;

impl PluginGroup for ClientPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        // Add FrogLight Client plugins.
        group = self.build_group(group);

        // Add Client specific plugins.

        group
    }
}

impl ClientPlugins {
    #[allow(clippy::unused_self)]
    pub(super) fn build_group(self, group: PluginGroupBuilder) -> PluginGroupBuilder {
        group.add(froglight_world::WorldPlugin)
    }
}

#[test]
fn test_build() { bevy::app::App::new().add_plugins(ClientPlugins); }

#[test]
fn test_run() {
    use bevy::prelude::*;

    // Create the app.
    let mut app = App::new();

    // Create the ClientPlugins group.
    let group = ClientPlugins::build(ClientPlugins);

    // TODO: Remove any plugins that won't work in this test.

    // Add the MinimalPlugins and ClientPlugins to the app.
    app.add_plugins((MinimalPlugins, group));

    // Create a runner that runs for 75ms.
    app.set_runner(|mut app| {
        let now = std::time::Instant::now();
        let mut counter = 0u32;

        while now.elapsed().as_millis() < 75 {
            app.update();
            counter += 1;
        }

        println!("Ran {counter} updates in 75ms");
    });

    // Run the app.
    app.run();
}
