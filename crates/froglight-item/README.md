# FrogLight Item

## Overview

FrogLight-Item provides a set of utilities for converting item IDs into items and back.

## Features

- **Dynamic Items**
  - Items are registered at runtime, allowing for adjustable item IDs.
  - Supports custom item types and NBT properties!
- **Multi-Version Support**
  - Supports multiple versions of Minecraft simultaneously.

## Usage

```rust
use froglight_item::prelude::*;
#[cfg(feature = "v1_21_4")]
use froglight_common::{vanilla::Vanilla, version::V1_21_4};

#[cfg(not(feature = "v1_21_4"))]
fn main() {}

#[cfg(feature = "v1_21_4")]
fn main() {
    // Create a new storage container with the default items registered.
    let storage = ItemStorage::<V1_21_4>::new();

    // Item IDs

    // Untyped items can hold any item internally with any NBT data.
    let untyped_item: UntypedItem<V1_21_4> = storage.get_untyped(GlobalItemId::new_unchecked(0), None).unwrap();
    assert_eq!(storage.get_global(&untyped_item), Some(GlobalItemId::new_unchecked(0)));

    // Untyped items have limited functionality, but can be resolved into typed items.
    assert_eq!(untyped_item.identifier(), "minecraft:air");
    assert_eq!(untyped_item.resolve::<Vanilla>(), Ok(Item::<item::Air, V1_21_4>::default().into()));

    let item_id = GlobalItemId::new_unchecked(1);
    assert_eq!(storage.get_typed::<Vanilla>(item_id, None), Some(Ok(Item::<item::Stone, V1_21_4>::default().into())));
    assert_eq!(storage.get_global_type::<item::Stone>(), Some(item_id));

    let item_id = GlobalItemId::new_unchecked(1146);
    assert_eq!(storage.get_typed::<Vanilla>(item_id, None), Some(Ok(Item::<item::BakedPotato, V1_21_4>::default().into())));
    assert_eq!(storage.get_global_type::<item::BakedPotato>(), Some(item_id));

}
```

## TODO

- [x] Item Types
  - [x] Typed Item
  - [x] Untyped Item
- [x] Item Properties
  - [x] Internal NBT Data
  - [ ] Property Types
  - [ ] Property Data
- [x] Code Generation
  - [x] Generate Item Types
  - [x] Generate Item Properties
  - [ ] Generate Property Types
  - [ ] Generate Property Data
