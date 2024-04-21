use std::sync::Arc;

use bevy_derive::Deref;
use bevy_ecs::system::Resource;
use froglight_protocol::common::ResourceKey;
use hashbrown::HashSet;
use parking_lot::RwLock;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) {
    app.init_resource::<LoginPlugins>()
        .init_resource::<ConfigPlugins>()
        .init_resource::<PlayPlugins>();
}

/// A [`Resource`] that holds a set of [`ResourceKey`]s that represent
/// plugin channels the connection task should listen to.
///
/// Any plugin that wants to listen to a channel should add its key to this set.
#[derive(Debug, Default, Clone, Deref, Resource)]
pub struct LoginPlugins(Arc<RwLock<HashSet<ResourceKey>>>);

/// A [`Resource`] that holds a set of [`ResourceKey`]s that represent
/// plugin channels the connection task should listen to.
///
/// Any plugin that wants to listen to a channel should add its key to this set.
#[derive(Debug, Default, Clone, Deref, Resource)]
pub struct ConfigPlugins(Arc<RwLock<HashSet<ResourceKey>>>);

/// A [`Resource`] that holds a set of [`ResourceKey`]s that represent
/// plugin channels the connection task should listen to.
///
/// Any plugin that wants to listen to a channel should add its key to this set.
#[derive(Debug, Default, Clone, Deref, Resource)]
pub struct PlayPlugins(Arc<RwLock<HashSet<ResourceKey>>>);
