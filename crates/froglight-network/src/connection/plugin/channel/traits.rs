use bevy_ecs::component::Component;

#[allow(unreachable_pub)]
pub trait ChannelType: Component + Sized {
    type TaskHalf;
    fn new_pair() -> (Self, Self::TaskHalf);
}
