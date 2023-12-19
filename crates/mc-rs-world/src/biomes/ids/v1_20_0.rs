use mc_rs_protocol::versions::v1_20_0::V1_20_0;

use crate::biomes::traits::VersionBiomeIds;

impl VersionBiomeIds for V1_20_0 {
    fn biome_id_to_name(_id: &u32) -> Option<&'static str> { None }

    fn biome_name_to_id(_name: &str) -> Option<&'static u32> { None }
}
