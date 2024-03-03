//! Creates graphs for various system schedules.
//!
//! Requires the `dot` command to be installed.

use bevy::{
    ecs::schedule::ScheduleLabel,
    log::LogPlugin,
    prelude::*,
    render::{Render, RenderApp},
};
use bevy_mod_debugdump::{
    render_graph::{settings::Style as RenderStyle, Settings as RenderSettings},
    render_graph_dot,
    schedule_graph::{settings::Style as ScheduleStyle, Settings as ScheduleSettings},
    schedule_graph_dot,
};
use froglight_client::plugins::AppPlugins;

fn main() {
    let mut app = App::new();

    // Add and finish the plugins
    app.add_plugins(AppPlugins.build().disable::<LogPlugin>());
    app.finish();

    // Save startup graphs
    save_graph("schedule_prestartup", PreStartup, &mut app);
    save_graph("schedule_startup", Startup, &mut app);
    save_graph("schedule_poststartup", PostStartup, &mut app);

    // Save update graphs
    save_graph("schedule_preupdate", PreUpdate, &mut app);
    save_graph("schedule_update", Update, &mut app);
    save_graph("schedule_postupdate", PostUpdate, &mut app);

    // Save render graphs
    save_render_graph("render_graph", &mut app);
    if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
        save_graph("render_render", Render, render_app);
    }
}

const DIRECTORY: &str = "crates/froglight-client/examples/system-graph";

/// Saves a graph of the given schedule to the given path.
fn save_graph(file_name: &str, schedule: impl ScheduleLabel, app: &mut App) {
    // Create the graph settings
    let settings = ScheduleSettings { style: ScheduleStyle::dark_github(), ..Default::default() };

    // Create the graph
    let dot = schedule_graph_dot(app, schedule, &settings);

    // Save the graph
    let file_path = format!("{DIRECTORY}/{file_name}.dot");
    std::fs::write(&file_path, dot).unwrap();

    // Convert the graph to a SVG image
    std::process::Command::new("dot")
        .arg("-Tsvg")
        .arg(file_path)
        .arg("-o")
        .arg(format!("{DIRECTORY}/{file_name}.svg"))
        .output()
        .unwrap();
}

fn save_render_graph(file_name: &str, app: &mut App) {
    // Create the graph settings
    let settings = RenderSettings { style: RenderStyle::dark_github() };

    // Create the graph
    let dot = render_graph_dot(app, &settings);

    // Save the graph
    let file_path = format!("{DIRECTORY}/{file_name}.dot");
    std::fs::write(&file_path, dot).unwrap();

    // Convert the graph to a SVG image
    std::process::Command::new("dot")
        .arg("-Tsvg")
        .arg(file_path)
        .arg("-o")
        .arg(format!("{DIRECTORY}/{file_name}.svg"))
        .output()
        .unwrap();
}
