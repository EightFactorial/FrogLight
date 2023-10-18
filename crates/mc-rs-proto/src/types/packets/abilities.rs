use mc_rs_macros::Transcode;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0])]
#[bitset]
pub struct ClientboundPlayerAbilityFlags {
    pub invulnerable: bool,
    pub flying: bool,
    pub allow_flying: bool,
    pub instant_break: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0])]
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
