use froglight_macros::FrogReadWrite;

use crate::packet::ClientStatusAction;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0])]
pub struct ClientStatusC2SPacket {
    pub action: ClientStatusAction,
}
