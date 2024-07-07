//! Prints graphs of bevy apps and subapps.
//!
//! Useful for visualizing schedule and system ordering

use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

use bevy::{
    app::MainScheduleOrder,
    ecs::schedule::{InternedScheduleLabel, ScheduleLabel},
    prelude::*,
};
use bevy_mod_debugdump::schedule_graph::settings::{
    Settings as ScheduleSettings, Style as ScheduleStyle,
};
use froglight_internal::HeadlessPlugins;
use froglight_utils::schedules::{FiveSeconds, OneSecond, OneTick, TenTicks, TwoTicks};

/// Bevy's fixed [`bevy::app::main_schedule`].
///
/// Run every fixed period of time.
static BEVY_FIXED_SCHEDULES: LazyLock<[InternedScheduleLabel; 5]> = LazyLock::new(|| {
    [
        FixedFirst.intern(),
        FixedPreUpdate.intern(),
        FixedUpdate.intern(),
        FixedPostUpdate.intern(),
        FixedLast.intern(),
    ]
});

/// Schedules defined in the [`froglight-utils`] crate.
///
/// Run every fixed period of time.
static UTIL_FIXED_SCHEDULES: LazyLock<[InternedScheduleLabel; 5]> = LazyLock::new(|| {
    [
        OneTick.intern(),
        TwoTicks.intern(),
        TenTicks.intern(),
        OneSecond.intern(),
        FiveSeconds.intern(),
    ]
});

fn main() {
    let mut app = App::new();
    app.add_plugins(HeadlessPlugins.build());

    // Generate schedule graphs
    graph_schedules(&mut app, "main", &[Main.intern()]);

    let startup_labels = app.world().resource::<MainScheduleOrder>().startup_labels.clone();
    graph_schedules(&mut app, "startup", &startup_labels);

    let labels = app.world().resource::<MainScheduleOrder>().labels.clone();
    graph_schedules(&mut app, "update", &labels);

    graph_schedules(&mut app, "fixed", &*BEVY_FIXED_SCHEDULES);
    graph_schedules(&mut app, "fixed", &*UTIL_FIXED_SCHEDULES);
}

/// Get the path to write the graphs to.
fn graph_path(folder: &str) -> PathBuf {
    // Get the path to write the graphs to
    let mut path = PathBuf::from(file!());

    path.pop();
    path.push("graphs");
    path.push(folder);

    if !path.exists() {
        std::fs::create_dir_all(&path).expect("Failed to create directory");
    }

    path
}

/// Generate graphs for the given schedules.
fn graph_schedules(app: &mut App, folder: &str, schedules: &[InternedScheduleLabel]) {
    let settings = ScheduleSettings { style: ScheduleStyle::dark_github(), ..Default::default() };

    for label in schedules {
        // Skip schedules that don't exist
        if !app.world().resource::<Schedules>().contains(label.intern()) {
            warn!("Unable to find Schedule `{label:?}`!");
            continue;
        }

        // Generate the graph
        info!("Generating graph for `{label:?}`");
        let graph = bevy_mod_debugdump::schedule_graph_dot(app, label.intern(), &settings);

        // Write the graph to a file
        write_dot_and_convert(graph, &format!("{label:?}"), &graph_path(folder));
    }
}

/// Write the graph to a dot file and convert it to an svg.
fn write_dot_and_convert(graph: String, label: &str, path: &Path) {
    // Get the path to write the graph to
    let path = path.join(format!("{label}.dot"));
    debug!("Writing `{label}` to \"{}\"", truncate_path(&path));

    // Write the graph to a file
    if let Err(err) = std::fs::write(&path, graph) {
        error!("Failed to write `{label}`: {err}");
    }

    // Convert the graph to an image
    let output_path = path.with_extension("svg");
    debug!("Converting \"{}\" to \"{}\"", truncate_path(&path), truncate_path(&output_path));

    if let Err(err) = std::process::Command::new("dot")
        .arg("-Tsvg")
        .arg(&path)
        .arg("-o")
        .arg(&output_path)
        .output()
    {
        error!("Failed to convert \"{}\" to \"{}\": {err}", path.display(), output_path.display());
    }
}

/// Truncate the path to just the file name.
fn truncate_path(path: &Path) -> &str {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_else(|| path.to_str().unwrap_or("unknown"))
}
