use std::marker::PhantomData;

use crate::{states::State, versions::Version};

/// A connection to a Server
///
/// Sends `ServerboundPackets` and receives `ClientboundPackets`
pub struct ServerConnection<V, S>
where
    V: Version,
    S: State<V>,
{
    _version: PhantomData<V>,
    _state: PhantomData<S>,
}

/// A connection to a Client
///
/// Sends `ClientboundPackets` and receives `ServerboundPackets`
pub struct ClientConnection<V, S>
where
    V: Version,
    S: State<V>,
{
    _version: PhantomData<V>,
    _state: PhantomData<S>,
}
