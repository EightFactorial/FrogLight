//! TODO

/// The intent a client has when connecting to a server.
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum ConnectionIntent {
    /// The client wants the status of the server.
    Status = 1,
    /// The client wants to login to the server.
    #[default]
    Login = 2,
    /// The client is being transferred from another server.
    Transfer = 3,
}
