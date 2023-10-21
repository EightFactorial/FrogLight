use compact_str::CompactString;
use mc_rs_macros::Test;

use crate::{
    buffer::{Decode, Encode, VarDecode, VarEncode},
    types::packets::scoreboard::ScoreboardUpdate,
};

#[derive(Debug, Clone, PartialEq, Eq, Test)]
#[mctest(tests = ["transcode", "decode"], bytes = [4, 84, 101, 115, 116, 1, 1, 4, 84, 101, 115, 116, 64])]
pub struct ClientboundScoreboardPlayerUpdatePacket {
    pub entity_name: CompactString,
    pub update_method: ScoreboardUpdate,
    pub objective_name: Option<CompactString>,
}

impl Encode for ClientboundScoreboardPlayerUpdatePacket {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        self.entity_name.encode(buf)?;

        match &self.update_method {
            ScoreboardUpdate::Change(_) => {
                1u8.encode(buf)?;
            }
            ScoreboardUpdate::Remove => {
                0u8.encode(buf)?;
            }
        }

        self.objective_name.encode(buf)?;

        if let ScoreboardUpdate::Change(value) = &self.update_method {
            value.var_encode(buf)?;
        }

        Ok(())
    }
}

impl Decode for ClientboundScoreboardPlayerUpdatePacket {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        let entity_name = CompactString::decode(buf)?;
        let update_method = u8::decode(buf)?;
        let objective_name = Option::<CompactString>::decode(buf)?;

        Ok(Self {
            entity_name,
            update_method: match update_method {
                0 => ScoreboardUpdate::Remove,
                1 => ScoreboardUpdate::Change(i32::var_decode(buf)?),
                id => return Err(crate::buffer::DecodeError::InvalidEnumId(id.into())),
            },
            objective_name,
        })
    }
}
