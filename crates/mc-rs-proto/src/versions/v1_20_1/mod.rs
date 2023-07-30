use crate::Version;

pub mod handshake;
pub mod login;
pub mod play;
pub mod status;

/// The 1.20.1 protocol version.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct V1_20_1;

impl Version for V1_20_1 {
    const ID: i32 = 763;
}
