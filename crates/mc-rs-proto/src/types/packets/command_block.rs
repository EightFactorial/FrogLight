use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [1])]
pub enum CommandBlockMode {
    Sequence,
    Auto,
    Redstone,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0])]
#[bitset]
pub struct CommandBlockFlags {
    pub track_output: bool,
    pub conditional: bool,
    pub automatic: bool,
}
