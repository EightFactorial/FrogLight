use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [2])]
pub enum SuggestionAction {
    Add,
    Remove,
    Set,
}
