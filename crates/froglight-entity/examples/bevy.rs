//! TODO

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use froglight_common::version::V1_21_4;
use froglight_entity::{
    entity_data::EntityDataBundle, entity_type::GlobalEntityTypeId, prelude::*,
};

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, LogPlugin { level: Level::INFO, ..default() }));

    app.init_resource::<AppEntityAttributeStorage<V1_21_4>>();
    app.init_resource::<AppEntityTypeStorage<V1_21_4>>();

    // TODO: Add an `EntityPlugin` that automatically registers these
    {
        let registry = app.world_mut().resource_mut::<AppTypeRegistry>();
        let mut registry = registry.write();
        registry.register::<EntityCollider>();
        registry.register::<EntityEyeHeight>();
        registry.register::<EntityDataBundle>();
        EntityType::register(&mut registry);
        EntityAttribute::register(&mut registry);
    }

    app.add_systems(Startup, spawn_entities);
    app.add_systems(Update, |mut commands: Commands| {
        commands.send_event(AppExit::Success);
    });

    app.run()
}

fn spawn_entities(world: &mut World) {
    let entity_types = world.resource::<AppEntityTypeStorage<V1_21_4>>().clone();
    let entity_types = entity_types.read();

    // Spawn: Arrow Projectile
    let mut entity = world.spawn_empty();
    let arrow_trait = entity_types.get_trait(GlobalEntityTypeId::new_unchecked(6)).unwrap();
    arrow_trait.insert_bundle(&mut entity);
    log_components(&mut entity);

    // Spawn: Axolotl
    let mut entity = world.spawn_empty();
    let axolotl_trait = entity_types.get_trait(GlobalEntityTypeId::new_unchecked(7)).unwrap();
    axolotl_trait.insert_bundle(&mut entity);
    log_components(&mut entity);

    // Spawn: Creeper
    let mut entity = world.spawn_empty();
    EntityTypeTrait::<V1_21_4>::insert_bundle(&entity::Creeper, &mut entity);
    log_components(&mut entity);

    // Spawn: Warden
    let mut entity = world.spawn_empty();
    EntityTypeTrait::<V1_21_4>::insert_bundle(&entity::Warden, &mut entity);
    log_components(&mut entity);

    // Spawn:
}

/// Log the components of an [`Entity`].
fn log_components(entity: &mut EntityWorldMut) {
    let entity_id = entity.id();
    let iter: Vec<_> = entity.archetype().components().collect();
    entity.world_scope(|world| {
        info!(
            "Entity ({entity_id}): {}\n",
            iter.into_iter()
                .map(|c_id| {
                    let reg = world.resource::<AppTypeRegistry>().read();
                    let c_inf = world.components().get_info(c_id).unwrap();
                    let c_ty = c_inf.type_id().unwrap();
                    if let Some(c_ref) = reg.get(c_ty) {
                        let c_fn = c_ref.data::<ReflectComponent>().unwrap();
                        format!("{:?}", c_fn.reflect(world.entity(entity_id)).unwrap())
                    } else {
                        format!("{}(Unknown)", c_inf.name().split("::").last().unwrap())
                    }
                })
                .collect::<Vec<_>>()
                .join(", ")
        );
    });
}
