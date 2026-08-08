//! TODO

use std::time::Instant;

use bevy_app::{App, Plugin};
use bevy_diagnostic::{Diagnostic, DiagnosticPath, Diagnostics, RegisterDiagnostic};
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;

use crate::prelude::*;

/// A [`Plugin`] that...
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TickMeasurementPlugin;

impl Plugin for TickMeasurementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TickInstant(Instant::now()));
        app.register_diagnostic(Diagnostic::new(Self::TICK_RUNTIME).with_suffix("ms"));

        app.add_systems(TickFirst, Self::tick_start_instant);
        app.add_systems(TickLast, Self::tick_end_duration);
    }
}

// -------------------------------------------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Resource, Reflect)]
#[reflect(Debug, Clone, PartialEq, Resource)]
struct TickInstant(Instant);

impl TickMeasurementPlugin {
    /// The [`DiagnosticPath`] for the "tick_runtime" diagnostic.
    ///
    /// This is the amount of time it took to run the [`TickSchedule`]s in
    /// milliseconds.
    pub const TICK_RUNTIME: DiagnosticPath =
        DiagnosticPath::const_new("froglight_tick/tick_runtime");

    /// A [`System`] for adding [`TICK_RUNTIME`] measurements.
    fn tick_start_instant(mut instant: ResMut<TickInstant>) { instant.0 = Instant::now(); }

    /// A [`System`] for adding [`TICK_RUNTIME`] measurements.
    fn tick_end_duration(instant: Res<TickInstant>, mut diag: Diagnostics) {
        const MILLIS_PER_SEC: f64 = 1_000.0;
        const NANOS_PER_MILLI: f64 = 1_000_000.0;

        diag.add_measurement(&Self::TICK_RUNTIME, || {
            let elapsed = instant.0.elapsed();

            // Mimic `Duration::as_millis_f64`
            (elapsed.as_secs_f64() * MILLIS_PER_SEC)
                + (f64::from(elapsed.subsec_nanos()) / NANOS_PER_MILLI)
        });
    }
}
