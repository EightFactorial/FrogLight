use mc_rs_macros::Transcode;
use uuid::Uuid;

use crate::types::ResourceLocation;

#[derive(Debug, Clone, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct EntityAttribute {
    pub attribute: ResourceLocation,
    pub base: f64,
    pub modifiers: Vec<AttributeModifier>,
}

#[derive(Debug, Clone, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct AttributeModifier {
    pub uuid: Uuid,
    pub amount: f64,
    pub operation: ModifierOperation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0])]
pub enum ModifierOperation {
    Add,
    MultiplyBase,
    MultiplyTotal,
}
