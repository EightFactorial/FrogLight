#![allow(missing_docs)]

use froglight_macros::frog_blocks;

use super::attributes::SnowyAttribute;

frog_blocks! {
    Air,
    Stone,
    Granite,
    PolishedGranite,
    Diorite,
    PolishedDiorite,
    Andesite,
    PolishedAndesite,
    GrassBlock {
        pub snowy: SnowyAttribute,
    },
    Dirt,
    CoarseDirt,
    Podzol {
        pub snowy: SnowyAttribute,
    },
    Cobblestone,
}
