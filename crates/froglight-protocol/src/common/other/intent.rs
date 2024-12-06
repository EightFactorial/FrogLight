#[cfg(feature = "bevy")]
use bevy_reflect::std_traits::ReflectDefault;
use froglight_macros::FrogReadWrite;

/// The intent of a new connection.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Default))]
#[frog(tests = ["read_verify", "write_verify"], bytes = [2])]
pub enum ConnectionIntent {
    /// The connection wants to get the status of the server.
    Status = 1,
    /// The connection wants to login to the server.
    #[default]
    Login = 2,
    /// The connection is being transferred from another server.
    Transfer = 3,
}
