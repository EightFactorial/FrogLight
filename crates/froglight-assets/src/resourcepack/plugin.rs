use std::sync::Arc;

use bevy::{ecs::schedule::BoxedCondition, prelude::*};
use parking_lot::Mutex;

use crate::ResourcePack;

/// Adds the [`ResourcePack`] asset type and
/// the [`ResourcePackLoader`](crate::ResourcePackLoader) asset loader.
#[derive(Default)]
pub struct ResourcePackPlugin {
    ///  A list of conditions that must be met before
    /// `ResourcePack`s are considered finished processing.
    pub(crate) conditions: Arc<Mutex<Vec<BoxedCondition>>>,
}

impl ResourcePackPlugin {
    /// Create a new `ResourcePackPlugin`.
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Create a new `ResourcePackPlugin` with the given conditions.
    #[must_use]
    pub fn from_conditions(conditions: Vec<BoxedCondition>) -> Self {
        Self { conditions: Arc::new(Mutex::new(conditions)) }
    }

    /// Create a new `ResourcePackPlugin` with the given conditions.
    #[must_use]
    pub(crate) fn from_conditions_arc(conditions: Arc<Mutex<Vec<BoxedCondition>>>) -> Self {
        Self { conditions }
    }

    /// Add a condition that must be met before `ResourcePack`s are considered
    /// finished processing.
    pub fn add_condition<M>(&self, condition: impl Condition<M>) {
        let condition = IntoSystem::into_system(condition);
        self.conditions.lock().push(Box::new(condition));
    }
}

impl Plugin for ResourcePackPlugin {
    fn build(&self, app: &mut App) {
        // Add `SystemSet`s
        crate::systemset::build(app);

        // Add the `ResourcePack` asset type and register it with the asset server
        {
            debug!("Initializing ResourcePack asset");
            app.init_asset::<ResourcePack>();
            app.register_asset_reflect::<ResourcePack>();
        }

        // Register the asset loader
        #[cfg(not(feature = "asset_manager"))]
        {
            debug!("Initializing ResourcePackLoader");
            app.init_asset_loader::<ResourcePackLoader>();
        }
    }

    fn finish(&self, app: &mut App) {
        // Add the conditions to the resource pack system
        crate::systemset::finish(&self.conditions, app);
    }
}
