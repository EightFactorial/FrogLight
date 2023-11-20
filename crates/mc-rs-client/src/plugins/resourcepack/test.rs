use std::fs;

use bevy::{app::AppExit, log::LogPlugin, prelude::*};
use mc_rs_resourcepack::pack::{ResourcePackAsset, ResourcePackPlugin};

use crate::dir::config_folder;

use super::ResourcePackSourcePlugin;

#[derive(Debug, PartialEq, Eq, Resource)]
struct ResourcePackPath(String);

#[test]
fn load_resourcepack() {
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
    app.add_plugins((
        MinimalPlugins,
        LogPlugin::default(),
        ResourcePackSourcePlugin,
        AssetPlugin {
            mode: AssetMode::Unprocessed,
            file_path: String::from("../../assets"),
            processed_file_path: String::from("../../imported_assets/Default"),
            watch_for_changes_override: None,
        },
        ResourcePackPlugin,
        ImagePlugin::default(),
    ));

    // Add the resourcepack path and systems.
    app.insert_resource(ResourcePackPath(path));
    app.add_systems(Startup, load);
    app.add_systems(
        Update,
        (
            load_event.run_if(on_event::<AssetEvent<ResourcePackAsset>>()),
            panic_timer,
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

fn panic_timer(time: Res<Time<Real>>) {
    if time.elapsed_seconds() > 5.0 {
        panic!("Failed to load ResourcePack within 5 seconds, panicking!");
    }
}
