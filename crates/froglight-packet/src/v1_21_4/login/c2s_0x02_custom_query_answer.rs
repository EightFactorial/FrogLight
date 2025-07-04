//! This file is auto-generated. Disable this by adding an `@manual` tag.
//!
//! @manual @generated by {COMMIT_HASH}

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub struct LoginQueryResponseC2SPacket {
    #[cfg_attr(feature = "io", frog(var))]
    pub query_id: u32,
    pub payload: Option<UnsizedBuffer<[u8; 16]>>,
}
