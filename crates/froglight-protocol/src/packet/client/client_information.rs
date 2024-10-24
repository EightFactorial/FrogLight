#![allow(missing_docs)]

use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0])]
pub enum ChatVisibility {
    #[default]
    All,
    System,
    None,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0])]
pub enum ParticleMode {
    #[default]
    All,
    Decreased,
    Minimal,
}
