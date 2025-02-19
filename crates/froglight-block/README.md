# FrogLight Block

## Overview

FrogLight-Block provides a set of utilities for converting between block IDs, states, and attributes.

## Features

- **Dynamic Blocks**
  - Blocks are registered at runtime, allowing for adjustable block IDs.
  - Supports custom block types and attributes!
- **Multi-Version Support**
  - Supports multiple versions of Minecraft simultaneously.
  - Convert blocks between versions.

## Usage

```rust
use froglight_block::prelude::*;
#[cfg(feature = "v1_21_4")]
use froglight_common::version::V1_21_4;

#[cfg(not(feature = "v1_21_4"))]
fn main() {}

#[cfg(feature = "v1_21_4")]
fn main() {
    // Create a new storage container with the default blocks registered.
    let storage = BlockStorage::<V1_21_4>::new();

    // Block IDs

    // Untyped blocks can hold any block state internally
    let untyped_block: UntypedBlock<V1_21_4> = storage.get_untyped(GlobalBlockId::new_unchecked(0)).unwrap();
    assert_eq!(storage.get_global(untyped_block), Some(GlobalBlockId::new_unchecked(0)));

    // Untyped blocks have limited functionality, but can be resolved into typed blocks
    assert_eq!(untyped_block.identifier().as_str(), "minecraft:air");
    assert_eq!(untyped_block.resolve::<Vanilla>(), Some(Block::<block::Air, V1_21_4>::default().into()));

    let state_id = GlobalBlockId::new_unchecked(1);
    assert_eq!(storage.get_typed::<Vanilla>(state_id), Some(Block::<block::Stone, V1_21_4>::default().into()));
    assert_eq!(storage.get_global(Block::<block::Stone, V1_21_4>::default()), Some(state_id));

    let state_id = GlobalBlockId::new_unchecked(27865);
    assert_eq!(storage.get_typed::<Vanilla>(state_id), Some(Block::<block::PottedClosedEyeblossom, V1_21_4>::default().into()));
    assert_eq!(storage.get_global(Block::<block::PottedClosedEyeblossom, V1_21_4>::default()), Some(state_id));

    // Block Attributes

    // Typed blocks can have their attributes accessed and modified
    let mut normal_grass = Block::<block::GrassBlock, V1_21_4>::default();
    assert_eq!(normal_grass.into_attr(), attribute::SnowyBool::False);

    // Attributes can be set using strict types
    normal_grass.scoped_attr(|_snowy| attribute::SnowyBool::True); // Short-hand for `into_attr` and `from_attr`
    assert_eq!(normal_grass.get_attr::<attribute::SnowyBool>(), Some(attribute::SnowyBool::True));
    assert_eq!(normal_grass.get_attr_str("snowy"), Some("true"));

    // Or by using strings
    assert_eq!(normal_grass.set_attr_str("snowy", "false"), Some("true")); // Returns the previous value
    assert_eq!(normal_grass.get_attr_str("snowy"), Some("false"));
    assert_eq!(normal_grass.get_attr::<attribute::SnowyBool>(), Some(attribute::SnowyBool::False));

    // Blocks without attributes will return a unit
    assert_eq!(Block::<block::Stone, V1_21_4>::default().into_attr(), ());

    // Block with multiple attributes will return a tuple
    let attributes = Block::<block::OakLeaves, V1_21_4>::default().into_attr();
    assert_eq!(attributes, (attribute::DistanceInt1To7::_7, attribute::PersistentBool::False, attribute::WaterloggedBool::False));


    // Block Versions
    // let stone_a: Block<block::Stone, V1_21_4> = Block::default();
    // let stone_b: Block<block::Stone, V1_21_5> = stone_a.into_version();
}
```

## TODO

- [x] Block Types
  - [x] Typed Blocks
  - [x] Untyped Blocks
- [x] Block Attributes
  - [x] From/Into Blocks
  - [x] Accessible via strings
- [x] Code Generation
  - [x] Generate Block Types
  - [x] Generate Block Attributes
  - [ ] Generate Block Upgrades/Downgrades
    - [x] Conversions with matching attributes
