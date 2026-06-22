//! TODO

use bevy_app::{App, Plugin};

pub mod instance;
use instance::WorldInstance;

mod instance_hook;
use instance_hook::{discard_hook, insert_hook};

pub mod query;

pub mod relationship;
use relationship::EntityOfInstance;

use crate::entity::{EntityId, EntityUuid};

/// A [`Plugin`] that adds entity-world instance relationships.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EntityId>().register_type::<EntityUuid>();
        app.register_type::<EntityOfInstance>().register_type::<WorldInstance>();

        let world = app.world_mut();

        world
            .register_component_hooks::<EntityId>()
            .on_insert(insert_hook::<EntityId>)
            .on_discard(discard_hook::<EntityId>);
        world
            .register_component_hooks::<EntityUuid>()
            .on_insert(insert_hook::<EntityUuid>)
            .on_discard(discard_hook::<EntityUuid>);
    }
}
