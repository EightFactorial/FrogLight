use bevy::{asset::ReflectHandle, ecs::schedule::BoxedCondition, prelude::*};
use parking_lot::Mutex;

use crate::{manager::ResourcePackManager, ResourcePack, ResourcePackLoader, ResourcePackTracker};

/// The [`Plugin`] for the [`froglight-resourcepack`](crate) crate.
///
/// Adds support for loading resource packs and asset management.
#[derive(Default)]
pub struct ResourcePackPlugin {
    /// A manager for resource packs.
    pub manager: ResourcePackManager,

    ///  A list of conditions that must be met before the plugin can finish.
    pub(crate) conditions: Mutex<Vec<BoxedCondition>>,
}

impl ResourcePackPlugin {
    /// Creates a new [`ResourcePackPlugin`].
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Creates a new [`ResourcePackPlugin`] with the given
    /// [`ResourcePackManager`].
    #[must_use]
    pub fn new_from(manager: ResourcePackManager) -> Self { Self { manager, ..Self::default() } }

    /// Adds a processing condition to the plugin.
    ///
    /// Prevents the plugin from considering [`ResourcePack`]s done
    /// processing until all conditions are met.
    pub fn add_condition<M>(&self, condition: impl Condition<M>) {
        self.conditions.lock().push(Self::new_condition(condition));
    }

    /// Creates a new [`BoxedCondition`] from the given condition.
    ///
    /// Copied from `bevy_ecs::schedule::config::new_condition`.
    fn new_condition<M>(condition: impl Condition<M>) -> BoxedCondition {
        let condition_system = IntoSystem::into_system(condition);
        assert!(
            condition_system.is_send(),
            "Condition `{}` accesses `NonSend` resources. This is not currently supported.",
            condition_system.name()
        );

        Box::new(condition_system)
    }
}

impl Plugin for ResourcePackPlugin {
    fn build(&self, app: &mut App) {
        // Configure the ResourcePackState
        crate::schedule::build(app);

        // Insert the manager into the app's resources
        app.insert_resource(self.manager.clone());

        // Register the ResourcePack type and initialize it as an asset
        app.init_asset::<ResourcePack>()
            .register_type::<ResourcePack>()
            .register_type_data::<Handle<ResourcePack>, ReflectHandle>();

        // Register the ResourcePackLoader
        app.register_asset_loader(ResourcePackLoader);

        // Initialize resources
        app.init_resource::<ResourcePackManager>().register_type::<ResourcePackManager>();
        app.init_resource::<ResourcePackTracker>();

        // Configure the ResourcePackTracker
        crate::tracker::build(app);
    }

    fn finish(&self, app: &mut App) {
        // Finish the ResourcePackState
        crate::schedule::finish(self, app);
    }
}

impl From<ResourcePackManager> for ResourcePackPlugin {
    fn from(manager: ResourcePackManager) -> Self { Self::new_from(manager) }
}

impl std::fmt::Debug for ResourcePackPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResourcePackPlugin")
            .field("manager", &self.manager)
            .field("conditions", &self.conditions.lock().len())
            .finish()
    }
}
