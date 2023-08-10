use mc_rs_macros::Transcode;
use uuid::Uuid;

use crate::types::ResourceLocation;

#[derive(Debug, Clone, Transcode)]
pub struct EntityAttribute {
    pub attribute: ResourceLocation,
    pub base: f64,
    pub modifiers: Vec<AttributeModifier>,
}

#[derive(Debug, Clone, Transcode)]
pub struct AttributeModifier {
    pub uuid: Uuid,
    pub amount: f64,
    pub operation: ModifierOperation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
pub enum ModifierOperation {
    Add,
    MultiplyBase,
    MultiplyTotal,
}
