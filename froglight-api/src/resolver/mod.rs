//! TODO

use std::{ops::Deref, sync::Arc};

#[cfg(feature = "bevy")]
use bevy_ecs::{reflect::ReflectResource, resource::Resource};
#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;

/// A DNS resolver for performing record lookups.
#[repr(transparent)]
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Resource, Reflect))]
#[cfg_attr(feature = "bevy", reflect(opaque, Clone, Resource))]
pub struct Resolver(Arc<dyn NetworkResolver>);

impl Resolver {
    /// Creates a new [`Resolver`] from a [`NetworkResolver`].
    #[inline]
    #[must_use]
    pub fn new<T: NetworkResolver>(agent: T) -> Self { Self::new_arc(Arc::new(agent)) }

    /// Creates a new [`Resolver`] from an [`Arc<dyn NetworkResolver>`].
    #[inline]
    #[must_use]
    pub const fn new_arc(agent: Arc<dyn NetworkResolver>) -> Self { Self(agent) }

    /// Returns a reference to the inner [`Arc<dyn NetworkResolver>`].
    #[inline]
    #[must_use]
    pub const fn as_arc(&self) -> &Arc<dyn NetworkResolver> { &self.0 }
}

impl AsRef<dyn NetworkResolver> for Resolver {
    #[inline]
    fn as_ref(&self) -> &dyn NetworkResolver { &*self.0 }
}

impl Deref for Resolver {
    type Target = dyn NetworkResolver;

    #[inline]
    fn deref(&self) -> &Self::Target { &*self.0 }
}

// -------------------------------------------------------------------------------------------------

/// A trait for types that can act as network agents.
pub trait NetworkResolver: Send + Sync + 'static {}
