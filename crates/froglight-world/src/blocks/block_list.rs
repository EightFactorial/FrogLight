#![allow(missing_docs)]

use froglight_macros::frog_blocks;

use super::attributes::SnowyAttribute;

// TODO: Move `BlockEnum::from_dyn` somewhere else to avoid
// every block needing a where clause
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
