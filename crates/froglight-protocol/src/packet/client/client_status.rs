use froglight_macros::FrogReadWrite;

/// A client status action.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0])]
pub enum ClientStatusAction {
    /// Perform a respawn.
    #[default]
    PerformRespawn,
    /// Request statistics.
    RequestStats,
}
