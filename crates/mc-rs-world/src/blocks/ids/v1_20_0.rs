use crate::blocks::list::*;
use mc_rs_protocol::versions::v1_20_0::V1_20_0;

use crate::blocks::traits::VersionBlockIds;

impl VersionBlockIds for V1_20_0 {
    fn block_id_to_name(id: &u32) -> Option<&'static str> {
        match id {
            BLOCK_AIR_ID => Some(BLOCK_AIR),
            BLOCK_ERROR_ID => Some(BLOCK_ERROR),
            _ => None,
        }
    }

    fn block_name_to_id(name: &str) -> Option<&'static u32> {
        match name {
            BLOCK_AIR => Some(BLOCK_AIR_ID),
            BLOCK_ERROR => Some(BLOCK_ERROR_ID),
            _ => None,
        }
    }
}

pub const BLOCK_AIR_ID: &u32 = &0;
pub const BLOCK_ERROR_ID: &u32 = &u32::MAX;
