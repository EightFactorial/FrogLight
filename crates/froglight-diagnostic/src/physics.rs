use bevy_app::{App, Plugin};
use bevy_diagnostic::{Diagnostic, DiagnosticPath, Diagnostics, RegisterDiagnostic};
use froglight_utils::schedules::HalfSecond;

/// A [`Plugin`] for adding physics diagnostics.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicsDiagnosticsPlugin;

impl Plugin for PhysicsDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        // Add `OBJECT_COUNT`
        app.add_systems(HalfSecond, Self::object_diagnostics).register_diagnostic(
            Diagnostic::new(Self::OBJECT_COUNT)
                .with_max_history_length(1)
                .with_smoothing_factor(0.0),
        );
    }
}

impl PhysicsDiagnosticsPlugin {
    /// The [`DiagnosticPath`] for the object count.
    pub const OBJECT_COUNT: DiagnosticPath =
        DiagnosticPath::const_new("froglight/physics/object/count");

    /// A system that updates physic object diagnostics.
    ///
    /// TODO: Implement physics objects.
    fn object_diagnostics(mut diagnostics: Diagnostics) {
        diagnostics.add_measurement(&Self::OBJECT_COUNT, || 0f64);
    }
}
