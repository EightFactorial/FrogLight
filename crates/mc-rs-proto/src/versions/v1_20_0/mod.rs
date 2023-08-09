use crate::Version;

pub mod connection;
pub mod handshake;
pub mod login;
pub mod play;
pub mod status;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct V1_20_0;

impl Version for V1_20_0 {
    const ID: i32 = 763;
}
