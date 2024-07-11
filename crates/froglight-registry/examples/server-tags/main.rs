//! This example demonstrates how to use the [`ConvertId`] trait to convert
//! between ids and keys.

use froglight_protocol::{common::ResourceKey, packet::ServerTagData};
use froglight_registry::{definitions::ItemKey, ConvertId};
use hashbrown::HashMap;

fn main() {
    // A set of tags sent from the server to the client.
    // The outer "minecraft:item" tag was cut off for clarity.
    let tag_data = ServerTagData {
        data: {
            let mut data = HashMap::new();
            data.insert(
                ResourceKey::new("minecraft:dampens_vibrations"),
                vec![
                    202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217,
                    446, 447, 448, 449, 450, 451, 452, 453, 454, 455, 456, 457, 458, 459, 460, 461,
                ],
            );
            data.insert(ResourceKey::new("minecraft:rails"), vec![763, 761, 762, 764]);
            data.insert(
                ResourceKey::new("minecraft:enchantable/fire_aspect"),
                vec![838, 823, 828, 843, 818, 833, 1093],
            );
            data.insert(
                ResourceKey::new("minecraft:foot_armor"),
                vec![859, 863, 875, 867, 871, 879],
            );
            data
        },
    };
    println!("{tag_data:#?}\n\n");

    println!("dampens_vibrations:");
    let tag_values = tag_data.get("minecraft:dampens_vibrations").unwrap();
    let keys: Vec<ItemKey> = tag_values.iter().map(|id| ItemKey::from_id(*id).unwrap()).collect();
    println!("   {keys:?}\n");

    println!("rails:");
    let tag_values = tag_data.get("minecraft:rails").unwrap();
    let keys: Vec<ItemKey> = tag_values.iter().map(|id| ItemKey::from_id(*id).unwrap()).collect();
    println!("   {keys:?}\n");

    println!("enchantable/fire_aspect:");
    let tag_values = tag_data.get("minecraft:enchantable/fire_aspect").unwrap();
    let keys: Vec<ItemKey> = tag_values.iter().map(|id| ItemKey::from_id(*id).unwrap()).collect();
    println!("   {keys:?}\n");

    println!("foot_armor:");
    let tag_values = tag_data.get("minecraft:foot_armor").unwrap();
    let keys: Vec<ItemKey> = tag_values.iter().map(|id| ItemKey::from_id(*id).unwrap()).collect();
    println!("   {keys:?}\n");
}
