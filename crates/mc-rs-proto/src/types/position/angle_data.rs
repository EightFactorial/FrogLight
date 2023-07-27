use std::num::TryFromIntError;

use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Transcode)]
pub struct AngleData {
    pub yaw: i8,
    pub pitch: i8,
}

impl AngleData {
    pub const ZERO: Self = Self { yaw: 0, pitch: 0 };

    pub fn new(yaw: i8, pitch: i8) -> Self { Self { yaw, pitch } }
}

impl From<(i8, i8)> for AngleData {
    fn from((yaw, pitch): (i8, i8)) -> Self { Self { yaw, pitch } }
}

impl From<AngleData> for (i8, i8) {
    fn from(AngleData { yaw, pitch }: AngleData) -> Self { (yaw, pitch) }
}

impl From<[i8; 2]> for AngleData {
    fn from([yaw, pitch]: [i8; 2]) -> Self { Self { yaw, pitch } }
}

impl From<AngleData> for [i8; 2] {
    fn from(AngleData { yaw, pitch }: AngleData) -> Self { [yaw, pitch] }
}

impl TryFrom<bevy_math::IVec2> for AngleData {
    type Error = TryFromIntError;

    fn try_from(bevy_math::IVec2 { x, y }: bevy_math::IVec2) -> Result<Self, Self::Error> {
        Ok(Self {
            yaw: i8::try_from(x)?,
            pitch: i8::try_from(y)?,
        })
    }
}

impl From<AngleData> for bevy_math::IVec2 {
    fn from(AngleData { yaw, pitch }: AngleData) -> Self { Self::new(yaw as i32, pitch as i32) }
}
