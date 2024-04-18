use froglight_protocol::{
    states::Configuration,
    traits::{State, Version},
};

use crate::connection::{NetworkDirection, Serverbound};

mod v1_20_2;
mod v1_20_3;

pub(crate) trait ConfigurationState: Version
where
    Configuration: State<Self>,
    Serverbound: NetworkDirection<Self, Configuration>,
{
}
