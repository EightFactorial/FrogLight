#[allow(dead_code, unreachable_pub)]
mod sealed {
    use froglight_protocol::{
        states::Configuration,
        traits::{State, Version},
    };

    /// A trait that indicates whether a version
    /// has a [`Configuration`] state.
    pub trait HasConfiguration: Version {
        const CONFIGURATION: bool;
    }
    impl<V: Version> HasConfiguration for V {
        const CONFIGURATION: bool = false;
    }

    /// A struct that checks whether a version implements states.
    pub struct StateChecker<V: Version>(std::marker::PhantomData<V>);
    impl<V: Version> StateChecker<V>
    where
        Configuration: State<V>,
    {
        const CONFIGURATION: bool = true;
    }
}
