use mc_rs_macros::Transcode;
use strum::{Display, EnumString};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [2])]
pub enum Difficulty {
    Peaceful,
    Easy,
    #[default]
    Normal,
    Hard,
}
