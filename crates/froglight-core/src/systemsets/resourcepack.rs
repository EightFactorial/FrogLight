//! `SystemSets` used for resource packs

use bevy::prelude::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<ResourcePackUpdateSet>()
        .configure_sets(Update, ResourcePackUpdateSet.ambiguous_with_all());
}

/// A [`SystemSet`] that runs resource pack
/// systems during the [`Update`] schedule.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet, Reflect)]
pub struct ResourcePackUpdateSet;
