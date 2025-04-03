use std::marker::PhantomData;

use bevy_app::AppLabel;
use bevy_ecs::prelude::*;
use bevy_reflect::{FromType, prelude::*};

use super::SubAppSync;

/// A [`TypeData`](bevy_reflect::TypeData)-compatible
/// container for [`SubAppSync`].
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ReflectSubAppSync<SubApp: AppLabel> {
    sync_fn: fn(&mut World, &mut World),
    _phantom: PhantomData<SubApp>,
}

impl<SubApp: AppLabel> ReflectSubAppSync<SubApp> {
    /// Sync the main [`App`](bevy_app::App)
    /// with the [`SubApp`](bevy_app::SubApp).
    #[inline]
    pub fn sync(&self, app: &mut World, sub: &mut World) { (self.sync_fn)(app, sub); }
}

impl<T: SubAppSync<SubApp> + PartialReflect, SubApp: AppLabel> FromType<T>
    for ReflectSubAppSync<SubApp>
{
    fn from_type() -> Self { Self { sync_fn: T::sync, _phantom: PhantomData } }
}

// Manual implementations to avoid trait bounds on `SubApp`.

impl<SubApp: AppLabel> Clone for ReflectSubAppSync<SubApp> {
    #[allow(clippy::non_canonical_clone_impl)]
    fn clone(&self) -> Self { Self { sync_fn: self.sync_fn, _phantom: PhantomData } }
}
impl<SubApp: AppLabel> Copy for ReflectSubAppSync<SubApp> {}
