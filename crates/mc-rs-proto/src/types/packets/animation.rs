use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
pub enum AnimationAction {
    SwingMainHand,
    Hurt,
    WakeUp,
    SwingOffHand,
    CriticalHit,
    MagicCriticalHit,
}
