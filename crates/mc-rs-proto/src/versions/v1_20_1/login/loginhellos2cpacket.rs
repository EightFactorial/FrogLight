#[derive(Debug, Clone, PartialEq)]
pub struct LoginHelloS2CPacket {
    pub a: String,
    pub b: Vec<u8>,
    pub c: Vec<u8>,
}
