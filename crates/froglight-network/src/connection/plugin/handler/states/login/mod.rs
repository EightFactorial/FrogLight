use std::future::Future;

use froglight_protocol::{
    common::GameProfile,
    states::Login,
    traits::{State, Version},
};

use crate::connection::{
    plugin::channels::parts::TaskPair, Connection, ConnectionError, LoginPlugins, NetworkDirection,
    Serverbound,
};

mod v1_20_0;
mod v1_20_2;
mod v1_20_3;

/// A trait for handling the [`Login`] state.
pub trait LoginHandler: Version
where
    Serverbound: NetworkDirection<Self, Login>,
    Login: State<Self>,
{
    /// Performs the login for the connection.
    fn perform_login(
        conn: &mut Connection<Self, Login>,
        channel: &TaskPair<Self, Login>,
        plugins: &LoginPlugins,
    ) -> impl Future<Output = Result<GameProfile, ConnectionError>> + Send + Sync;
}
