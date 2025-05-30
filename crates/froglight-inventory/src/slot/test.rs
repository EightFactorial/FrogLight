#![allow(unused_imports, dead_code)]

use std::num::NonZeroU8;

use froglight_common::vanilla::Vanilla;
use froglight_io::version::{FrogReadVersion, FrogWriteVersion};
use froglight_item::{prelude::*, resolve::ItemResolver};
use paste::paste;

use crate::slot::{InventorySlot, component::VersionComponents, network::RawInventorySlot};

#[allow(unused_macros)]
macro_rules! generate_tests {
    // Generate `empty` and `full` tests
    ($version:ident, $($item:ty),*) => {
        paste! { generate_tests!(@empty [<$version:lower _empty>], $version); }
        paste! { generate_tests!(@full [<$version:lower _full>], $version, $($item),*); }
    };


    // Empty test
    (@empty $test:ident, $version:ident) => {
            #[test]
            fn $test() {
                use froglight_common::version::$version;
                use froglight_item::storage::ItemStorage;

                let storage = ItemStorage::<$version>::new();

                let slot = InventorySlot::<$version>::new_empty();
                let roundtrip = roundtrip(&slot, &storage);

                assert_eq!(roundtrip.item().is_none(), slot.item().is_none());
                assert_eq!(roundtrip.item(), slot.item());
                assert_eq!(roundtrip, slot);
            }
    };
    // Full test
    (@full $test:ident, $version:ident, $($item:ty),*) => {
        #[test]
        fn $test() {
            use froglight_common::version::$version;
            use froglight_item::generated::item::*;

            $(
                template::<$item, $version>();
            )*
        }
    };
}

fn template<I: froglight_item::item::ItemTypeExt<V>, V: VersionComponents>()
where
    Vanilla: ItemResolver<V>,
    RawInventorySlot<V>: FrogReadVersion<V> + FrogWriteVersion<V>,
{
    let storage = ItemStorage::<V>::new();

    let item = Item::<I, V>::default().into_untyped();
    let slot = InventorySlot::<V>::new_from(NonZeroU8::new(8).unwrap(), item);
    let roundtrip = roundtrip(&slot, &storage);

    assert_eq!(roundtrip.item().unwrap().identifier(), slot.item().unwrap().identifier());
    assert_eq!(roundtrip.item(), slot.item());
    assert_eq!(roundtrip, slot);
}

fn roundtrip<V: VersionComponents>(
    slot: &InventorySlot<V>,
    storage: &ItemStorage<V>,
) -> InventorySlot<V>
where
    RawInventorySlot<V>: FrogReadVersion<V> + FrogWriteVersion<V>,
{
    let original = RawInventorySlot::<V>::from_slot(slot, storage).unwrap();

    let mut buffer = Vec::new();
    FrogWriteVersion::<V>::frog_write(&original, &mut buffer).unwrap();
    assert_eq!(FrogWriteVersion::<V>::frog_len(&original), buffer.len());

    let mut cursor = std::io::Cursor::new(buffer);
    RawInventorySlot::<V>::frog_read(&mut cursor).unwrap().into_slot(storage).unwrap()
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "v1_21_4")]
generate_tests!(
    V1_21_4,
    Air,
    Stone,
    GrassBlock,
    Anvil,
    AcaciaSign,
    OakSlab,
    OminousBottle,
    TotemOfUndying,
    Shield,
    BambooRaft,
    SpruceBoat,
    SpruceChestBoat,
    GoldenShovel,
    IronSword,
    DiamondPickaxe,
    LeatherCap,
    LeatherBoots,
    LeatherHorseArmor,
    NetheriteHoe,
    Netherrack,
    NetherWart,
    Wheat,
    WheatSeeds,
    CarvedPumpkin,
    PumpkinPie,
    MelonSlice,
    Beetroot,
    BeetrootSeeds,
    GoldIngot,
    RawGold,
    BlockOfGold,
    BlockOfRawGold,
    Emerald,
    BlockOfEmerald,
    EmeraldOre,
    DeepslateEmeraldOre,
    Beacon
);

#[cfg(feature = "v1_21_5")]
generate_tests!(
    V1_21_5,
    Air,
    Stone,
    GrassBlock,
    Anvil,
    AcaciaSign,
    OakSlab,
    OminousBottle,
    TotemOfUndying,
    Shield,
    BambooRaft,
    SpruceBoat,
    SpruceChestBoat,
    GoldenShovel,
    IronSword,
    DiamondPickaxe,
    LeatherCap,
    LeatherBoots,
    LeatherHorseArmor,
    NetheriteHoe,
    Netherrack,
    NetherWart,
    Wheat,
    WheatSeeds,
    CarvedPumpkin,
    PumpkinPie,
    MelonSlice,
    Beetroot,
    BeetrootSeeds,
    GoldIngot,
    RawGold,
    BlockOfGold,
    BlockOfRawGold,
    Emerald,
    BlockOfEmerald,
    EmeraldOre,
    DeepslateEmeraldOre,
    Beacon
);
