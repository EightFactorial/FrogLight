//! @generated by `froglight-generator` #ecfea09

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [])]
pub struct EnterCombatPacket;
