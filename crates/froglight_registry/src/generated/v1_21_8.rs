use froglight_common::identifier::Identifier;
use once_cell::sync::Lazy;

use crate::storage::{Registry, RegistryMap, RegistrySet, StaticRegistryMap};

generate! {
    @registry froglight_common::version::V1_21_8,
    None => "minecraft:placeholder": ["minecraft:air"],
    "minecraft:air" => "minecraft:placeholder_blocks": [
        "minecraft:air", "minecraft:stone", "minecraft:granite", "minecraft:polished_granite"
    ]
}
