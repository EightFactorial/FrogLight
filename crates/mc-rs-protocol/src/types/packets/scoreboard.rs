#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoreboardUpdate {
    Change(i32),
    Remove,
}
