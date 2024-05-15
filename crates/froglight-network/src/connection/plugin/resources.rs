use std::sync::Arc;

use bevy_derive::Deref;
use bevy_ecs::{reflect::ReflectResource, system::Resource};
use bevy_reflect::{std_traits::ReflectDefault, Reflect};
use froglight_protocol::common::ResourceKey;
use hashbrown::HashSet;
use parking_lot::RwLock;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) {
    app.init_resource::<LoginPlugins>()
        .register_type::<LoginPlugins>()
        .init_resource::<ConfigPlugins>()
        .register_type::<ConfigPlugins>()
        .init_resource::<PlayPlugins>()
        .register_type::<PlayPlugins>();
}

/// A [`Resource`] that holds a set of [`ResourceKey`]s that represent
/// plugin channels the connection task should listen to.
///
/// Any plugin that wants to listen to a channel should add its key to this set.
#[derive(Debug, Default, Clone, Deref, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct LoginPlugins(#[reflect(ignore)] Arc<RwLock<HashSet<ResourceKey>>>);

/// A [`Resource`] that holds a set of [`ResourceKey`]s that represent
/// plugin channels the connection task should listen to.
///
/// Any plugin that wants to listen to a channel should add its key to this set.
#[derive(Debug, Default, Clone, Deref, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct ConfigPlugins(#[reflect(ignore)] Arc<RwLock<HashSet<ResourceKey>>>);

/// A [`Resource`] that holds a set of [`ResourceKey`]s that represent
/// plugin channels the connection task should listen to.
///
/// Any plugin that wants to listen to a channel should add its key to this set.
#[derive(Debug, Default, Clone, Deref, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct PlayPlugins(#[reflect(ignore)] Arc<RwLock<HashSet<ResourceKey>>>);
