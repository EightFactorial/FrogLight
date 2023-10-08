use mc_rs_proto::versions::v1_20_0::V1_20_0;

use crate::systems::world::global_palette::GlobalPalette;

impl GlobalPalette for V1_20_0 {
    fn to_global_block(state_id: u32) -> u32 { state_id }

    fn to_global_biome(biome_id: u32) -> u32 { biome_id }
}
