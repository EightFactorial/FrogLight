use bevy_ecs::{component::ComponentInfo, prelude::*};
use bevy_reflect::{TypeRegistry, func::FunctionRegistry};
use froglight_common::{EntityId, EntityUuid};
use uuid::Uuid;

use crate::function::{CommandBuilder, WorldRef};

#[test]
fn execute() {
    use crate::{argument::ReflectArgumentParser, graph::BrigadierGraph};

    // Create a new `World` and `Entity` for testing
    let mut world = World::new();
    let entity = world.spawn((EntityId::from(1), EntityUuid::from(Uuid::nil()))).id();

    // Prepare a `FunctionRegistry` and `TypeRegistry`
    let mut functions = FunctionRegistry::default();
    let mut registry = TypeRegistry::default();
    {
        registry.register::<Entity>();
        registry.register_type_data::<u32, ReflectArgumentParser>();
        registry.register_type_data::<f64, ReflectArgumentParser>();
        registry.register_type_data::<String, ReflectArgumentParser>();
    }

    // Create a new `BrigadierGraph`
    let mut graph = BrigadierGraph::default();

    // Add a basic command with no arguments
    let builder = CommandBuilder::new("test", &mut graph, &mut functions);
    builder.build(|entity, _world| {
        println!("Entity {entity}: Hello, world!");
    });

    // Add a command with multiple arguments
    let builder = CommandBuilder::new("test_2", &mut graph, &mut functions);
    let builder = builder.arg::<u32, _>().arg::<f64, _>().arg::<String, _>().arg::<String, _>();
    builder.build(|entity, num, float, string_1, string_2, _world| {
        assert_eq!(num, 42);
        assert_eq!(float.total_cmp(&float), std::cmp::Ordering::Equal);
        println!("Entity {entity}: {string_1} {string_2}");
    });

    // Add a command with a literal argument, show how to access the `World`
    let builder = CommandBuilder::new("test_3", &mut graph, &mut functions);
    let builder = builder.arg::<u32, _>().literal("literal").arg::<f64, _>();
    builder.build(|entity, num, float, mut world| {
        assert_eq!(num, 1000);
        assert_eq!(float.total_cmp(&40320.0), std::cmp::Ordering::Equal);

        let world = world.value();
        let components = world.inspect_entity(entity).map(ComponentInfo::name);
        println!("Entity {entity}: {}", components.collect::<Vec<_>>().join(", "));
    });

    // Execute the commands
    WorldRef::new().scoped(&mut world, |world| {
        graph.execute(entity, "test", &registry, &functions, world).unwrap();
        graph.execute(entity, "test_2 42 3.14 foo bar", &registry, &functions, world).unwrap();
        graph.execute(entity, "test_3 1000 literal 40320", &registry, &functions, world).unwrap();
    });
}
