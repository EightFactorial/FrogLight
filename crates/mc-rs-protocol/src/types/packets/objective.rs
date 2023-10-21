use mc_rs_macros::Transcode;
use strum::EnumString;

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [2, 5, 77, 67, 45, 82, 83, 0])]
pub enum ObjectiveUpdate {
    Add(ObjectiveInfo),
    Remove,
    Change(ObjectiveInfo),
}

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [5, 77, 67, 45, 82, 83, 0])]
pub struct ObjectiveInfo {
    pub display_name: String,
    pub render_type: RenderType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [1])]
pub enum RenderType {
    Integer,
    Hearts,
}
