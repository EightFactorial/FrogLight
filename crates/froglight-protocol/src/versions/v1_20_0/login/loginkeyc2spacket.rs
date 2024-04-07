use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [8, 8, 7, 6, 5, 4, 3, 2, 1, 8, 1, 2, 3, 4, 5, 6, 7, 8])]
pub struct LoginKeyC2SPacket {
    pub encrypted_secret_key: Vec<u8>,
    pub nonce: Vec<u8>,
}
