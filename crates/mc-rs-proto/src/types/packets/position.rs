use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[bitset]
pub struct PositionRelativeFlags {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub yaw: bool,
    pub pitch: bool,
}
