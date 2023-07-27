use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Transcode)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self { Self { x, y, z } }
}

impl From<bevy_math::Vec3> for Vec3 {
    fn from(bevy_math::Vec3 { x, y, z }: bevy_math::Vec3) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
}

impl From<Vec3> for bevy_math::DVec3 {
    fn from(Vec3 { x, y, z }: Vec3) -> Self { Self::new(x, y, z) }
}

impl From<bevy_math::DVec3> for Vec3 {
    fn from(bevy_math::DVec3 { x, y, z }: bevy_math::DVec3) -> Self { Self::new(x, y, z) }
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from((x, y, z): (f64, f64, f64)) -> Self { Self { x, y, z } }
}

impl From<Vec3> for (f64, f64, f64) {
    fn from(Vec3 { x, y, z }: Vec3) -> Self { (x, y, z) }
}

impl From<[f64; 3]> for Vec3 {
    fn from([x, y, z]: [f64; 3]) -> Self { Self { x, y, z } }
}

impl From<Vec3> for [f64; 3] {
    fn from(Vec3 { x, y, z }: Vec3) -> Self { [x, y, z] }
}
