use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, Transcode)]
pub enum SoundSource {
    Master,
    Music,
    Records,
    Weather,
    Blocks,
    Hostile,
    Neutral,
    Players,
    Ambient,
    Voice,
}
