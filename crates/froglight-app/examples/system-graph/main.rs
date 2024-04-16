//! Prints graphs of bevy apps and subapps.
//!
//! Useful for visualizing schedule and system ordering
#![feature(lazy_cell)]

use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

use bevy::{
    app::{MainScheduleOrder, RunFixedMainLoop},
    ecs::schedule::{InternedScheduleLabel, ScheduleLabel},
    prelude::*,
};
use bevy_mod_debugdump::schedule_graph::settings::{
    Settings as ScheduleSettings, Style as ScheduleStyle,
};
use froglight_app::AppPlugins;
use froglight_utils::fixed_schedules::{
    FiveSecondSchedule, OneSecondSchedule, OneTickSchedule, RunFixedTimers, ThirtySecondSchedule,
    TwoTickSchedule,
};

/// Schedules that run once every fixed amount of time.
static FIXED_SCHEDULES: LazyLock<[InternedScheduleLabel; 6]> = LazyLock::new(|| {
    [
        RunFixedMainLoop.intern(),
        FixedFirst.intern(),
        FixedPreUpdate.intern(),
        FixedUpdate.intern(),
        FixedPostUpdate.intern(),
        FixedLast.intern(),
    ]
});

/// Schedules that run once every fixed amount of time.
static TIMED_SCHEDULES: LazyLock<[InternedScheduleLabel; 6]> = LazyLock::new(|| {
    [
        RunFixedTimers.intern(),
        OneTickSchedule.intern(),
        TwoTickSchedule.intern(),
        OneSecondSchedule.intern(),
        FiveSecondSchedule.intern(),
        ThirtySecondSchedule.intern(),
    ]
});

fn main() {
    let mut app = App::new();
    app.add_plugins(AppPlugins.build());

    let startup_labels = app.world.resource::<MainScheduleOrder>().startup_labels.clone();
    graph_schedules(&mut app, "startup", &startup_labels);

    let labels = app.world.resource::<MainScheduleOrder>().labels.clone();
    graph_schedules(&mut app, "update", &labels);

    graph_schedules(&mut app, "fixed", &*FIXED_SCHEDULES);
    graph_schedules(&mut app, "timed", &*TIMED_SCHEDULES);
}

fn graph_schedules(app: &mut App, folder: &str, schedules: &[InternedScheduleLabel]) {
    let settings = ScheduleSettings { style: ScheduleStyle::dark_github(), ..Default::default() };

    // Get the path to write the graphs to
    let mut path = PathBuf::from(file!());
    {
        path.pop();
        path.push("graphs");
        path.push(folder);

        if !path.exists() {
            std::fs::create_dir_all(&path).expect("Failed to create directory");
        }
    }

    for label in schedules {
        // Skip schedules that don't exist
        if !app.world.resource::<Schedules>().contains(label.intern()) {
            warn!("Unable to find Schedule `{label:?}`!");
            continue;
        }

        // Generate the graph
        info!("Generating graph for `{label:?}`");
        let graph = bevy_mod_debugdump::schedule_graph_dot(app, label.intern(), &settings);

        // Get the path to write the graph to
        let path = path.join(format!("{label:?}.dot"));
        debug!("Writing `{label:?}` to \"{}\"", truncate_path(&path));

        // Write the graph to a file
        if let Err(err) = std::fs::write(&path, graph) {
            error!("Failed to write `{label:?}`: {err}");
        }

        // Convert the graph to an image
        let output_path = path.with_extension("svg");
        debug!("Converting \"{}\" to \"{}\"", truncate_path(&path), truncate_path(&output_path));

        if let Err(err) = std::process::Command::new("dot")
            .arg("-Tsvg")
            .arg(&path)
            .arg("-o")
            .arg(output_path)
            .output()
        {
            error!("Failed to convert `{label:?}` to an image: {err}");
        }
    }
}

/// Truncates the path to just the file name.
fn truncate_path(path: &Path) -> &str {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_else(|| path.to_str().unwrap_or("unknown"))
}
