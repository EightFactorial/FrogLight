use std::ffi::OsStr;

use bevy::{
    diagnostic::{Diagnostic, DiagnosticPath, Diagnostics, DiagnosticsStore},
    prelude::*,
};
use froglight_utils::schedules::OneSecond;
use sysinfo::{ProcessRefreshKind, RefreshKind, System, UpdateKind};

use super::ClientStartupDiagnosticsSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.add_systems(Startup, setup_memory_diagnostic.in_set(ClientStartupDiagnosticsSet));
    app.add_systems(OneSecond, memory_usage_diagnostic);
}

/// Memory usage of the current application in `GiB` (gibibytes).
pub const MEMORY_USAGE_APP: DiagnosticPath = DiagnosticPath::const_new("system/memory_usage_app");

/// Setup the memory usage diagnostic.
fn setup_memory_diagnostic(mut diag: ResMut<DiagnosticsStore>) {
    diag.add(Diagnostic::new(MEMORY_USAGE_APP).with_max_history_length(16).with_suffix("GiB"));
}

/// Conversion factor from bytes to gibibytes.
const BYTES_TO_GIB: f64 = 1.0 / 1024.0 / 1024.0 / 1024.0;

/// Update the memory usage diagnostic.
fn memory_usage_diagnostic(
    mut diag: Diagnostics,
    mut exe: Local<Option<String>>,
    mut sysinfo: Local<Option<System>>,
) {
    // Set the executable name if it is not already set
    if exe.is_none() {
        match std::env::current_exe() {
            Ok(path) => {
                let name =
                    path.file_name().map(OsStr::to_string_lossy).unwrap_or_default().to_string();

                #[cfg(debug_assertions)]
                debug!("Monitoring: \"{name}\"");

                *exe = Some(name);
            }
            Err(err) => {
                error_once!("Failed to get executable path: {err}");
            }
        }
    }

    // Update the process list and memory usage
    if sysinfo.is_none() {
        *sysinfo =
            Some(System::new_with_specifics(RefreshKind::new().with_processes(
                ProcessRefreshKind::new().with_memory().with_exe(UpdateKind::Always),
            )));
    }
    let Some(sys) = sysinfo.as_mut() else {
        return;
    };
    sys.refresh_processes_specifics(
        ProcessRefreshKind::new().with_memory().with_exe(UpdateKind::Always),
    );

    // Get the memory usage of the current application
    #[allow(clippy::cast_precision_loss)]
    if let Some(process) = sys.processes_by_exact_name(exe.as_ref().unwrap()).next() {
        let memory_usage = process.memory() as f64 * BYTES_TO_GIB;

        // Log the memory usage
        #[cfg(debug_assertions)]
        trace!("Memory usage: {memory_usage:.2} GiB");

        // Update the memory usage diagnostic
        diag.add_measurement(&MEMORY_USAGE_APP, || memory_usage);
    } else {
        warn_once!("Failed to find process by name: \"{}\"", exe.as_ref().unwrap());
    }
}
