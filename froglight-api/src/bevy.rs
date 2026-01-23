//! TODO

use bevy_app::{App, Plugin};

use crate::{agent::Agent, resolver::Resolver};

/// A [`Plugin`] that ...
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApiPlugin;

impl Plugin for ApiPlugin {
    fn build(&self, app: &mut App) { app.register_type::<Agent>().register_type::<Resolver>(); }
}
