use std::any::TypeId;

use bevy::MinimalPlugins;
use bevy_app::App;
use bevy_ecs::world::World;
use bevy_reflect::Reflect;
use froglight_protocol::versions::v1_21_0::V1_21_0;

use super::{BlockBuilder, BlockStorage, BlockStorageArc, ReflectBlockBuilder};
use crate::{attribute::*, block::*, BlockState, BlockStateExt};

#[test]
fn storage() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Manually register the test builder.
    app.register_type::<TestBuilder>();
    app.register_type_data::<TestBuilder, ReflectBlockBuilder<V1_21_0>>();

    // Initialize the block storage.
    app.init_resource::<BlockStorageArc<V1_21_0>>();

    // Retrieve the block storage.
    let storage = app.world().resource::<BlockStorageArc<V1_21_0>>();
    let storage = storage.read();

    // Test retrieving known block types.
    {
        assert_eq!(storage.get_stored_default_type::<Air>(), Some(&Air));
        assert_eq!(storage.get_stored_default_type::<Stone>(), Some(&Stone));
        assert_eq!(storage.get_stored_default_type::<Granite>(), Some(&Granite));
        assert_eq!(storage.get_stored_default_type::<PolishedGranite>(), Some(&PolishedGranite));
        assert_eq!(storage.get_stored_default_type::<Diorite>(), Some(&Diorite));
        assert_eq!(storage.get_stored_default_type::<PolishedDiorite>(), Some(&PolishedDiorite));
        assert_eq!(storage.get_stored_default_type::<Andesite>(), Some(&Andesite));
        assert_eq!(storage.get_stored_default_type::<PolishedAndesite>(), Some(&PolishedAndesite));
        assert_eq!(storage.get_stored_default_type::<Dirt>(), Some(&Dirt));
        assert_eq!(storage.get_stored_default_type::<CoarseDirt>(), Some(&CoarseDirt));

        if let Some(block) = storage.get_stored_default_type::<GrassBlock>() {
            assert_eq!(BlockState::<V1_21_0>::resource_key(block), "minecraft:grass_block");
            assert_eq!(
                <GrassBlock as BlockStateExt<V1_21_0>>::to_attributes(block),
                SnowyBooleanAttribute(false)
            );
        } else {
            panic!("Grass block not found!");
        }

        if let Some(block) = storage.get_stored_default_type::<Podzol>() {
            assert_eq!(BlockState::<V1_21_0>::resource_key(block), "minecraft:podzol");
            assert_eq!(
                <Podzol as BlockStateExt<V1_21_0>>::to_attributes(block),
                SnowyBooleanAttribute(false)
            );
        } else {
            panic!("Podzol block not found!");
        }
    }

    // Test retrieving dynamic block types.
    {
        if let Some(block) = storage.get_stored_default(0) {
            assert_eq!(block.type_id(), TypeId::of::<Air>());
            assert_eq!(block.resource_key(), "minecraft:air");
        }

        if let Some(block) = storage.get_stored_default(1) {
            assert_eq!(block.type_id(), TypeId::of::<Stone>());
            assert_eq!(block.resource_key(), "minecraft:stone");
        }

        if let Some(block) = storage.get_stored_default(2) {
            assert_eq!(block.type_id(), TypeId::of::<Granite>());
            assert_eq!(block.resource_key(), "minecraft:granite");
        }

        assert_eq!(storage.get_type(3), Some(TypeId::of::<PolishedGranite>()));
        assert_eq!(storage.get_type(4), Some(TypeId::of::<Diorite>()));
        assert_eq!(storage.get_type(5), Some(TypeId::of::<PolishedDiorite>()));
        assert_eq!(storage.get_type(6), Some(TypeId::of::<Andesite>()));
        assert_eq!(storage.get_type(7), Some(TypeId::of::<PolishedAndesite>()));
        assert_eq!(storage.get_type(10), Some(TypeId::of::<Dirt>()));
        assert_eq!(storage.get_type(11), Some(TypeId::of::<CoarseDirt>()));

        assert_eq!(storage.get_block_id(&Air), Some(0));
        assert_eq!(storage.get_block_id(&Stone), Some(1));
        assert_eq!(storage.get_block_id(&Granite), Some(2));
        assert_eq!(storage.get_block_id(&PolishedGranite), Some(3));
        assert_eq!(storage.get_block_id(&Diorite), Some(4));
        assert_eq!(storage.get_block_id(&PolishedDiorite), Some(5));
        assert_eq!(storage.get_block_id(&Andesite), Some(6));
        assert_eq!(storage.get_block_id(&PolishedAndesite), Some(7));
        assert_eq!(storage.get_block_id(&Dirt), Some(10));
        assert_eq!(storage.get_block_id(&CoarseDirt), Some(11));

        assert_eq!(storage.get_type(8), Some(TypeId::of::<GrassBlock>()));
        assert_eq!(storage.get_type(9), Some(TypeId::of::<GrassBlock>()));

        assert_eq!(
            storage.get_block_id(&<GrassBlock as BlockStateExt<V1_21_0>>::from_attributes(
                SnowyBooleanAttribute(true)
            )),
            Some(8)
        );
        assert_eq!(
            storage.get_block_id(&<GrassBlock as BlockStateExt<V1_21_0>>::from_attributes(
                SnowyBooleanAttribute(false)
            )),
            Some(9)
        );

        assert_eq!(storage.get_type(12), Some(TypeId::of::<Podzol>()));
        assert_eq!(storage.get_type(13), Some(TypeId::of::<Podzol>()));

        assert_eq!(
            storage.get_block_id(&<Podzol as BlockStateExt<V1_21_0>>::from_attributes(
                SnowyBooleanAttribute(true)
            )),
            Some(12)
        );
        assert_eq!(
            storage.get_block_id(&<Podzol as BlockStateExt<V1_21_0>>::from_attributes(
                SnowyBooleanAttribute(false)
            )),
            Some(13)
        );
    }
}

#[derive(Reflect)]
struct TestBuilder;

impl BlockBuilder<V1_21_0> for TestBuilder {
    fn build(
        storage: &mut BlockStorage<V1_21_0>,
        _: &mut World,
        _: &[&ReflectBlockBuilder<V1_21_0>],
    ) {
        storage.register::<Air>();
        storage.register::<Stone>();
        storage.register::<Granite>();
        storage.register::<PolishedGranite>();
        storage.register::<Diorite>();
        storage.register::<PolishedDiorite>();
        storage.register::<Andesite>();
        storage.register::<PolishedAndesite>();
        storage.register::<GrassBlock>();
        storage.register::<Dirt>();
        storage.register::<CoarseDirt>();
        storage.register::<Podzol>();
    }
}
