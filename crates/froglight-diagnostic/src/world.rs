use bevy_app::{App, Plugin};
use bevy_diagnostic::{Diagnostic, DiagnosticPath, Diagnostics, RegisterDiagnostic};
use bevy_ecs::{query::With, system::Query};
use froglight_utils::schedules::HalfSecond;
use froglight_world::Chunk;

/// A [`Plugin`] for adding world diagnostics.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorldDiagnosticsPlugin;

impl Plugin for WorldDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        // Add `CHUNK_COUNT`
        app.add_systems(HalfSecond, Self::world_diagnostics).register_diagnostic(
            Diagnostic::new(Self::CHUNK_COUNT)
                .with_max_history_length(1)
                .with_smoothing_factor(0.0),
        );
    }
}

impl WorldDiagnosticsPlugin {
    /// The [`DiagnosticPath`] for the chunk count.
    pub const CHUNK_COUNT: DiagnosticPath =
        DiagnosticPath::const_new("froglight/world/chunk/count");

    /// A system that updates world diagnostics.
    #[expect(clippy::cast_precision_loss)]
    fn world_diagnostics(query: Query<(), With<Chunk>>, mut diagnostics: Diagnostics) {
        diagnostics.add_measurement(&Self::CHUNK_COUNT, || query.iter().len() as f64);
    }
}
