use froglight_protocol::{
    common::GameProfile,
    states::Login,
    traits::{State, Version},
};

use crate::connection::{Connection, ConnectionError, NetworkDirection, Serverbound};

mod v1_20_0;
mod v1_20_2;
mod v1_20_3;

pub(crate) trait LoginState: Version
where
    Login: State<Self>,
    Serverbound: NetworkDirection<Self, Login>,
{
    async fn perform_login(
        conn: &mut Connection<Self, Login>,
    ) -> Result<GameProfile, ConnectionError>;
}
