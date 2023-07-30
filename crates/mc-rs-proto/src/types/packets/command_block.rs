use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
pub enum CommandBlockMode {
    Sequence,
    Auto,
    Redstone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[bitset]
pub struct CommandBlockFlags {
    pub track_output: bool,
    pub conditional: bool,
    pub automatic: bool,
}
