use froglight_macros::FrogReadWrite;

use crate::packet::DebugSampleType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct DebugSamplePacket {
    pub sample: Vec<i64>,
    pub sample_type: DebugSampleType,
}
