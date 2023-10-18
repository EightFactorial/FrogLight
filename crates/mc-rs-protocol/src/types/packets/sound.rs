use mc_rs_macros::Transcode;

#[derive(Debug, Default, Clone, Copy, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0])]
pub enum SoundSource {
    #[default]
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
