use froglight_protocol::{
    states::Play,
    traits::{State, Version},
};

use crate::connection::{NetworkDirection, Serverbound};

mod v1_20_0;
mod v1_20_2;
mod v1_20_3;

pub(crate) trait PlayState: Version
where
    Play: State<Self>,
    Serverbound: NetworkDirection<Self, Play>,
{
}
