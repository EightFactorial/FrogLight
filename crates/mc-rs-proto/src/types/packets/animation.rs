use crate::buffer::{Decode, Encode, VarDecode, VarEncode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationAction {
    SwingMainHand = 0,
    Hurt = 1,
    WakeUp = 2,
    SwingOffHand = 3,
    CriticalHit = 4,
    MagicCriticalHit = 5,
}

impl Encode for AnimationAction {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        match self {
            AnimationAction::SwingMainHand => 0u32.var_encode(buf),
            AnimationAction::Hurt => 1u32.var_encode(buf),
            AnimationAction::WakeUp => 2u32.var_encode(buf),
            AnimationAction::SwingOffHand => 3u32.var_encode(buf),
            AnimationAction::CriticalHit => 4u32.var_encode(buf),
            AnimationAction::MagicCriticalHit => 5u32.var_encode(buf),
        }
    }
}

impl Decode for AnimationAction {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        match u32::var_decode(buf)? {
            0 => Ok(AnimationAction::SwingMainHand),
            1 => Ok(AnimationAction::Hurt),
            2 => Ok(AnimationAction::WakeUp),
            3 => Ok(AnimationAction::SwingOffHand),
            4 => Ok(AnimationAction::CriticalHit),
            5 => Ok(AnimationAction::MagicCriticalHit),
            id => Err(crate::buffer::DecodeError::InvalidEnumId(id)),
        }
    }
}
