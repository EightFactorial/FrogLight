#![allow(missing_docs)]

#[cfg(feature = "bevy")]
use bevy_reflect::std_traits::ReflectDefault;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Default))]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0])]
pub enum ChatVisibility {
    #[default]
    All,
    System,
    None,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Default))]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0])]
pub enum ParticleMode {
    #[default]
    All,
    Decreased,
    Minimal,
}
