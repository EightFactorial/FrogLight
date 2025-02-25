use std::ops::{Deref, DerefMut};

use bevy_ecs::world::World;
use bevy_reflect::{
    TypePath,
    func::{
        ArgError, ArgValue,
        args::{Arg, FromArg, GetOwnership, Ownership},
    },
};

/// A [`World`] reference, used as a function argument.
#[derive(TypePath)]
pub struct WorldRef<'a>(&'a mut World);

impl Deref for WorldRef<'_> {
    type Target = World;
    fn deref(&self) -> &Self::Target { self.0 }
}
impl DerefMut for WorldRef<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target { self.0 }
}

impl FromArg for WorldRef<'_> {
    type This<'a> = WorldRef<'a>;

    fn from_arg(arg: Arg<'_>) -> Result<Self::This<'_>, ArgError> {
        if !matches!(arg.value(), ArgValue::Mut(_)) {
            return Err(ArgError::InvalidOwnership {
                index: arg.index(),
                expected: Ownership::Mut,
                received: match arg.value() {
                    ArgValue::Owned(..) => Ownership::Owned,
                    ArgValue::Ref(..) => Ownership::Ref,
                    ArgValue::Mut(..) => Ownership::Mut,
                },
            });
        }

        let ArgValue::Mut(value) = arg.take_value() else { unreachable!() };

        value.try_downcast_mut::<World>().map_or_else(|| todo!(), |world| Ok(WorldRef(world)))
    }
}

impl GetOwnership for WorldRef<'_> {
    fn ownership() -> Ownership { Ownership::Owned }
}
