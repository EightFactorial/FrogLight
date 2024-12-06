use froglight_macros::FrogReadWrite;

/// A command sent from the client to the server.
///
/// Used to signal that the client has performed a certain action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [7])]
pub enum ClientPlayerCommand {
    /// The client has started sneaking.
    PressShiftKey,
    /// The client has stopped sneaking.
    ReleaseShiftKey,
    /// The client has started sleeping.
    StopSleeping,
    /// The client has started sprinting.
    StartSprinting,
    /// The client has stopped sprinting.
    StopSprinting,
    /// The client has started charging a horse's jump.
    StartRidingJump,
    /// The client has stopped charging a horse's jump.
    StopRidingJump,
    /// The client has opened an inventory.
    OpenInventory,
    /// The client has started using an elytra.
    StartFallFlying,
}
