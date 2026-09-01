//! TODO

use std::time::Instant;

use bevy_app::{App, Plugin};
use bevy_diagnostic::{Diagnostic, DiagnosticPath, Diagnostics, RegisterDiagnostic};
use bevy_ecs::prelude::*;
use bevy_reflect::{Reflect, std_traits::ReflectDefault};

/// A [`Plugin`] that...
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TickMeasurementPlugin;

impl Plugin for TickMeasurementPlugin {
    fn build(&self, app: &mut App) {
        app.register_diagnostic(Self::create_diagnostic());
        app.init_resource::<TickInstant>();
    }
}

// -------------------------------------------------------------------------------------------------

/// A wrapper around an [`Instant`].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource, Reflect)]
#[reflect(Debug, Default, Clone, PartialEq, Resource)]
pub struct TickInstant(Instant);

impl Default for TickInstant {
    fn default() -> Self { Self(Instant::now()) }
}

impl TickMeasurementPlugin {
    /// The [`DiagnosticPath`] for the "tick_runtime" diagnostic.
    ///
    /// This is the amount of time it took to run the [`TickSchedule`]s in
    /// milliseconds.
    pub const TICK_RUNTIME: DiagnosticPath =
        DiagnosticPath::const_new("froglight_tick/tick_runtime");
    /// The suffix for the [`TICK_RUNTIME`] diagnostic.
    pub const TICK_RUNTIME_SUFFIX: &'static str = "ms";

    /// Creates the default [`TICK_RUNTIME`] [`Diagnostic`].
    #[must_use]
    pub fn create_diagnostic() -> Diagnostic {
        Diagnostic::new(Self::TICK_RUNTIME)
            .with_suffix(Self::TICK_RUNTIME_SUFFIX)
            .with_max_history_length(50)
            .with_smoothing_factor(0.0)
    }

    /// A [`System`] for starting [`TICK_RUNTIME`] measurements.
    pub fn start_measurement(mut instant: ResMut<TickInstant>) { instant.0 = Instant::now(); }

    /// A [`System`] for ending [`TICK_RUNTIME`] measurements.
    pub fn end_measurement(instant: Res<TickInstant>, mut diag: Diagnostics) {
        #[cfg(feature = "nightly")]
        diag.add_measurement(&Self::TICK_RUNTIME, || instant.0.elapsed().as_millis_f64());
        #[cfg(not(feature = "nightly"))]
        diag.add_measurement(&Self::TICK_RUNTIME, || instant.0.elapsed().as_secs_f64() * 1000.0);
    }
}
