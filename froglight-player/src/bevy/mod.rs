//! TODO

use bevy_app::{App, Plugin};

use crate::prelude::{PlayerProfile, Username};

/// A [`Plugin`] that registers player-related components.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Username>().register_type::<PlayerProfile>();
    }
}
