use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0])]
pub enum AnimationAction {
    SwingMainHand,
    Hurt,
    WakeUp,
    SwingOffHand,
    CriticalHit,
    MagicCriticalHit,
}
