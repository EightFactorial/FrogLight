use mc_rs_macros::Transcode;

#[derive(Debug, Default, Clone, Copy, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);

    pub const fn new(x: f64, y: f64, z: f64) -> Self { Self { x, y, z } }
}

impl From<bevy_math::Vec3> for Vec3 {
    fn from(vec: bevy_math::Vec3) -> Self {
        Self {
            x: vec.x.into(),
            y: vec.y.into(),
            z: vec.z.into(),
        }
    }
}

impl From<Vec3> for bevy_math::Vec3 {
    fn from(vec: Vec3) -> Self { Self::new(vec.x as f32, vec.y as f32, vec.z as f32) }
}
