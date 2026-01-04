//! TODO
#![no_std]

use core::any::TypeId;

use froglight_block::{
    block::{BlockAttr, BlockMetadata, BlockType},
    implement_storage,
    prelude::*,
    storage::BlockStorage,
};
use froglight_common::{prelude::Identifier, version::Version};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct TestVersion;

impl Version for TestVersion {
    const PROTOCOL_ID: u32 = u32::MIN;
    const RESOURCE_VERSION: u32 = u32::MIN;
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Air;

impl BlockType<TestVersion> for Air {
    type Attributes = ();

    const ATTRDATA: &'static [(&'static str, TypeId)] = &[];
    const METADATA: &'static BlockMetadata = {
        static STATIC: BlockMetadata = unsafe {
            BlockMetadata::new::<Air, TestVersion>(Identifier::new_unchecked("test:air"), 0, 0)
        };
        &STATIC
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Stone;

impl BlockType<TestVersion> for Stone {
    type Attributes = ();

    const ATTRDATA: &'static [(&'static str, TypeId)] = &[];
    const METADATA: &'static BlockMetadata = {
        static STATIC: BlockMetadata = unsafe {
            BlockMetadata::new::<Stone, TestVersion>(Identifier::new_unchecked("test:stone"), 1, 0)
        };
        &STATIC
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Dirt;

impl BlockType<TestVersion> for Dirt {
    type Attributes = ();

    const ATTRDATA: &'static [(&'static str, TypeId)] = &[];
    const METADATA: &'static BlockMetadata = {
        static STATIC: BlockMetadata = unsafe {
            BlockMetadata::new::<Dirt, TestVersion>(Identifier::new_unchecked("test:stone"), 2, 0)
        };
        &STATIC
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Grass;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Snowy(bool);

impl BlockType<TestVersion> for Grass {
    type Attributes = Snowy;

    const ATTRDATA: &'static [(&'static str, TypeId)] = &[("snowy", TypeId::of::<Snowy>())];
    const METADATA: &'static BlockMetadata = {
        static STATIC: BlockMetadata = unsafe {
            BlockMetadata::new::<Grass, TestVersion>(Identifier::new_unchecked("test:grass"), 3, 1)
        };
        &STATIC
    };
}

impl BlockAttr for Snowy {
    const STATES: &'static [(&'static str, Self)] = &[("true", Self(true)), ("false", Self(false))];
}

implement_storage! {
    TestVersion => unsafe {
        BlockStorage::new_static(&[
            Air::METADATA,
            Stone::METADATA,
            Dirt::METADATA,
            Grass::METADATA,
            Grass::METADATA,
        ])
    }
}

// -------------------------------------------------------------------------------------------------

#[test]
fn air() {
    let air = Block::new_default::<Air, TestVersion>();
    assert_eq!(air.global_id(), 0u32);
    assert_eq!(air.state_id(), 0u16);

    let mut attr_iter = air.get_attributes();
    assert_eq!(attr_iter.next(), None);

    #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
    {
        let global = TestVersion::blocks().read();
        assert_eq!(global.get_block(GlobalId::new(0)), Some(air));
    }

    assert!(Block::new_state::<Air, TestVersion>(StateId::new(0)).is_some());
    assert!(Block::new_state::<Air, TestVersion>(StateId::new(1)).is_none());
}

#[test]
fn stone() {
    let stone = Block::new_default::<Stone, TestVersion>();
    assert_eq!(stone.global_id(), 1u32);
    assert_eq!(stone.state_id(), 0u16);

    let mut attr_iter = stone.get_attributes();
    assert_eq!(attr_iter.next(), None);

    #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
    {
        let global = TestVersion::blocks().read();
        assert_eq!(global.get_block(GlobalId::new(1)), Some(stone));
    }

    assert!(Block::new_state::<Stone, TestVersion>(StateId::new(0)).is_some());
    assert!(Block::new_state::<Stone, TestVersion>(StateId::new(1)).is_none());
}

#[test]
fn dirt() {
    let dirt = Block::new_default::<Dirt, TestVersion>();
    assert_eq!(dirt.global_id(), 2u32);
    assert_eq!(dirt.state_id(), 0u16);

    let mut attr_iter = dirt.get_attributes();
    assert_eq!(attr_iter.next(), None);

    #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
    {
        let global = TestVersion::blocks().read();
        assert_eq!(global.get_block(GlobalId::new(2)), Some(dirt));
    }

    assert!(Block::new_state::<Dirt, TestVersion>(StateId::new(0)).is_some());
    assert!(Block::new_state::<Dirt, TestVersion>(StateId::new(1)).is_none());
}

#[test]
fn grass() {
    let grassy = Block::new::<Grass, TestVersion>(Snowy(false));
    assert_eq!(Block::new_state::<Grass, TestVersion>(StateId::new(1)), Some(grassy));
    assert_eq!(Block::new_default::<Grass, TestVersion>(), grassy);
    assert_eq!(grassy.global_id(), 4u32);
    assert_eq!(grassy.state_id(), 1u16);

    assert_eq!(grassy.get_attribute::<Snowy>(), Some(Snowy(false)));
    assert_eq!(grassy.get_attribute_str("snowy"), Some("false"));
    let mut attr_iter = grassy.get_attributes();
    assert_eq!(attr_iter.next(), Some(("snowy", "false")));
    assert_eq!(attr_iter.next(), None);

    let snowy = Block::new::<Grass, TestVersion>(Snowy(true));
    assert_eq!(Block::new_state::<Grass, TestVersion>(StateId::new(0)), Some(snowy));
    assert_eq!(snowy.global_id(), 3u32);
    assert_eq!(snowy.state_id(), 0u16);

    assert_eq!(snowy.get_attribute::<Snowy>(), Some(Snowy(true)));
    assert_eq!(snowy.get_attribute_str("snowy"), Some("true"));
    let mut attr_iter = snowy.get_attributes();
    assert_eq!(attr_iter.next(), Some(("snowy", "true")));
    assert_eq!(attr_iter.next(), None);

    #[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
    {
        let global = TestVersion::blocks().read();
        assert_eq!(global.get_block(GlobalId::new(4)), Some(grassy));
        assert_eq!(global.get_block(GlobalId::new(3)), Some(snowy));
    }

    assert!(Block::new_state::<Grass, TestVersion>(StateId::new(0)).is_some());
    assert!(Block::new_state::<Grass, TestVersion>(StateId::new(1)).is_some());
    assert!(Block::new_state::<Grass, TestVersion>(StateId::new(2)).is_none());
}
