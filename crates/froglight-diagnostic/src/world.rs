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
        app.register_diagnostic(
            Diagnostic::new(Self::CHUNK_COUNT)
                .with_max_history_length(1)
                .with_smoothing_factor(0.0),
        )
        .add_systems(HalfSecond, Self::world_diagnostics);
    }
}

impl WorldDiagnosticsPlugin {
    /// The [`DiagnosticPath`] for the chunk count.
    pub const CHUNK_COUNT: DiagnosticPath =
        DiagnosticPath::const_new("froglight/world/chunk_count");

    /// A system that updates world diagnostics.
    fn world_diagnostics(query: Query<(), With<Chunk>>, mut diagnostics: Diagnostics) {
        let chunk_count = query.iter().len();

        #[allow(clippy::cast_precision_loss)]
        diagnostics.add_measurement(&Self::CHUNK_COUNT, || chunk_count as f64);
    }
}
