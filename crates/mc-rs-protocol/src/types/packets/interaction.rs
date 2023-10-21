use mc_rs_macros::{Test, Transcode};

use crate::{
    buffer::{Decode, Encode},
    types::{enums::Direction, position::BlockPos, Vec3},
};

#[derive(Debug, Clone, Copy, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0])]
pub enum InteractionAction {
    Interact(InteractionHand),
    Attack,
    InteractAt(Vec3, InteractionHand),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0])]
pub enum InteractionHand {
    #[default]
    MainHand,
    OffHand,
}

#[derive(Debug, Clone, Copy, PartialEq, Test)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct BlockHitResult {
    pub position: BlockPos,
    pub direction: Direction,
    pub location: Vec3,
    pub inside: bool,
}

impl Encode for BlockHitResult {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        self.position.encode(buf)?;
        self.direction.encode(buf)?;

        f32::encode(
            &((self.location.x - f64::from(self.position.x)) as f32),
            buf,
        )?;
        f32::encode(
            &((self.location.y - f64::from(self.position.y)) as f32),
            buf,
        )?;
        f32::encode(
            &((self.location.z - f64::from(self.position.z)) as f32),
            buf,
        )?;

        self.inside.encode(buf)
    }
}

impl Decode for BlockHitResult {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        let position = BlockPos::decode(buf)?;
        let direction = Direction::decode(buf)?;

        let x = f32::decode(buf)?;
        let y = f32::decode(buf)?;
        let z = f32::decode(buf)?;

        let location = Vec3::new(
            f64::from(position.x) + f64::from(x),
            f64::from(position.y) + f64::from(y),
            f64::from(position.z) + f64::from(z),
        );

        Ok(Self {
            position,
            direction,
            location,
            inside: bool::decode(buf)?,
        })
    }
}
