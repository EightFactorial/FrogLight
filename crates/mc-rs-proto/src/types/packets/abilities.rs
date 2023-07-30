use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[bitset]
pub struct ClientboundPlayerAbilityFlags {
    pub invulnerable: bool,
    pub flying: bool,
    pub allow_flying: bool,
    pub instant_break: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[bitset]
pub struct ServerboundPlayerAbilityFlags {
    _empty: bool,
    pub flying: bool,
}

impl ServerboundPlayerAbilityFlags {
    pub fn new(flying: bool) -> Self {
        Self {
            _empty: false,
            flying,
        }
    }
}
