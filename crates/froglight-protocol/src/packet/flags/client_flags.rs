use froglight_macros::FrogReadWrite;

/// The player's current state sent from the client.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(bitset, tests = ["read_verify", "write_verify"], bytes = [0])]
pub struct ClientPlayerAbilityFlags {
    /// An empty flag.
    empty: bool,
    /// Whether the player is flying.
    pub flying: bool,
}
