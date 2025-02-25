use bevy_ecs::prelude::*;
use bevy_reflect::{TypeRegistry, func::FunctionRegistry};

use crate::function::CommandBuilder;

#[test]
fn build_and_execute() {
    use crate::{argument::ReflectArgumentParser, graph::BrigadierGraph};

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

    // Add some commands
    let builder = CommandBuilder::new("test", &mut graph, &mut functions);
    builder.build(|entity, _world| {
        println!("Entity {entity}: Hello, world!");
    });

    let builder = CommandBuilder::new("test_2", &mut graph, &mut functions);
    let builder = builder.arg::<u32, _>().arg::<f64, _>().arg::<String, _>().arg::<String, _>();
    builder.build(|entity, num, float, string_1, string_2, _world| {
        assert_eq!(num, 42);
        assert_eq!(float.total_cmp(&float), std::cmp::Ordering::Equal);
        println!("Entity {entity}: {string_1} {string_2}");
    });

    let builder = CommandBuilder::new("test_3", &mut graph, &mut functions);
    let builder = builder.arg::<u32, _>().literal("literal").arg::<f64, _>();
    builder.build(|_entity, num, float, _world| {
        assert_eq!(num, 1000);
        assert_eq!(float.total_cmp(&40320.0), std::cmp::Ordering::Equal);
    });

    // Create a new `World` for testing
    let mut world = World::new();
    let entity = world.spawn_empty().id();

    // Execute the commands
    graph.execute(entity, "test", &registry, &functions, &mut world).unwrap();
    graph.execute(entity, "test_2 42 3.14 foo bar", &registry, &functions, &mut world).unwrap();
    graph.execute(entity, "test_3 1000 literal 40320", &registry, &functions, &mut world).unwrap();
}
