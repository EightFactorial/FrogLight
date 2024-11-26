//! A few basic examples of using type reflection.
//!
//! Enabling reflection *really* increases compile times and binary size,
//! so it's not recommended to use it.

use bevy::prelude::*;
use froglight::prelude::HeadlessPlugins;

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(HeadlessPlugins);

    // Run `print_pale_blocks`, `print_entities` and `exit` in sequence.
    app.add_systems(Update, (print_pale_blocks, print_entities, exit).chain());

    app.run()
}

/// Use the [`AppTypeRegistry`] to find and print the names of all blocks
/// that contain "Pale" in their name.
///
/// # Note
/// This doesn't have any knowledge of the versions these
/// blocks are supported by, only that they exist.
fn print_pale_blocks(registry: Res<AppTypeRegistry>) {
    let registry = registry.read();

    // Collect all Pale blocks.
    let mut pale_blocks = Vec::new();
    for registration in registry.iter() {
        let path = registration.type_info().type_path();
        let name = registration.type_info().type_path_table().short_path();
        if path.starts_with("froglight_block::generated::block") && name.contains("Pale") {
            pale_blocks.push(name);
        }
    }

    // Sort the blocks by name and print them.
    pale_blocks.sort_unstable();
    info!("\"Pale\" Blocks:");
    for block_name in pale_blocks {
        info!("    {block_name}");
    }
}

/// Use the [`AppTypeRegistry`] to find and print the names of all entities.
///
/// # Note
/// This doesn't have any knowledge of the versions these
/// entities are supported by, only that they exist.
fn print_entities(registry: Res<AppTypeRegistry>) {
    let registry = registry.read();

    // Collect all entities.
    let mut entities = Vec::new();
    for registration in registry.iter() {
        let path = registration.type_info().type_path();
        let name = registration.type_info().type_path_table().short_path();
        if path.starts_with("froglight_entity::generated::entity") {
            entities.push(name);
        }
    }

    // Sort the entities by name and print them.
    entities.sort_unstable();
    info!("Entities:");
    for entity_name in entities {
        info!("    {entity_name}");
    }
}

fn exit(mut events: EventWriter<AppExit>) {
    info!("Exiting...");
    events.send(AppExit::Success);
}
