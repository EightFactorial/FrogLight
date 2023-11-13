#![allow(dead_code)]

use bevy::{app::AppExit, prelude::*};

use super::ResourcePackAsset;

#[derive(Debug, PartialEq, Eq, Resource)]
struct ResourcePackPath(String);

#[test]
fn load_resourcepack() {
    use bevy::asset::AssetPlugin;
    use std::fs;

    use super::{init_assets, register_assets};
    use crate::util::dir::config_folder;

    // Pass the test if the resourcepacks folder doesn't exist.
    let dir = config_folder().join("resourcepacks");
    if !dir.exists() || !dir.is_dir() {
        return;
    }

    // Pass the test if the resourcepacks folder is empty.
    let mut read_dir = fs::read_dir(dir).unwrap();
    let path = match read_dir.next() {
        Some(Ok(entry)) => entry.file_name().to_string_lossy().to_string(),
        _ => return,
    };

    // Create the app, register types, and add plugins.
    let mut app = App::new();
    register_assets(&mut app);
    app.add_plugins((
        MinimalPlugins,
        AssetPlugin::default(),
        ImagePlugin::default(),
    ));
    init_assets(&mut app);

    // Add the resourcepack path and systems.
    app.insert_resource(ResourcePackPath(path));
    app.add_systems(Startup, load);
    app.add_systems(
        Update,
        (
            load_event.run_if(on_event::<AssetEvent<ResourcePackAsset>>()),
            abort_timer,
        )
            .chain(),
    );

    // Run the app.
    app.run();
}

/// Load the resourcepack on startup.
fn load(path: Res<ResourcePackPath>, assets: Res<AssetServer>) {
    let _: Handle<ResourcePackAsset> = assets.load(format!("resourcepack://{}", path.0));
}

/// Exit the app if the resourcepack loaded successfully.
fn load_event(
    mut asset_events: EventReader<AssetEvent<ResourcePackAsset>>,
    mut exit_events: EventWriter<AppExit>,
) {
    if asset_events
        .read()
        .any(|event| matches!(event, AssetEvent::LoadedWithDependencies { .. }))
    {
        exit_events.send(AppExit);
    }
}

/// Abort the test if the resourcepack fails to load in 5 seconds.
fn abort_timer(time: Res<Time<Real>>) {
    if time.elapsed_seconds() > 5.0 {
        panic!("Resourcepack failed to load in 5 seconds, aborting.");
    }
}
