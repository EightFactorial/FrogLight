use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Transcode)]
pub enum SuggestionAction {
    Add,
    Remove,
    Set,
}
