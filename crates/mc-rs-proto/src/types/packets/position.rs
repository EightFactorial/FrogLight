use mc_rs_macros::Transcode;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0])]
#[bitset]
pub struct PositionRelativeFlags {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub yaw: bool,
    pub pitch: bool,
}
