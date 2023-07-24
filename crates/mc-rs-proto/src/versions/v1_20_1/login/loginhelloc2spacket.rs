use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct LoginHelloC2SPacket {
    pub a: String,
    pub b: Option<Uuid>,
}
