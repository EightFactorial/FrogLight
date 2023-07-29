use mc_rs_macros::Transcode;
use strum::EnumString;

#[derive(Debug, Clone, Transcode)]
pub enum ObjectiveUpdate {
    Add(ObjectiveInfo),
    Remove,
    Change(ObjectiveInfo),
}

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
pub struct ObjectiveInfo {
    pub display_name: String,
    pub render_type: RenderType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Transcode)]
pub enum RenderType {
    Integer,
    Hearts,
}
