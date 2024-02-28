use std::{
    any::Any,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use bevy_asset::Handle;
use bevy_audio::AudioSource;
#[cfg(feature = "inspector")]
use bevy_inspector_egui::{
    egui::{Button, Id, RichText, Ui},
    inspector_egui_impls::InspectorEguiImpl,
    reflect_inspector::InspectorUi,
};
use bevy_log::warn;
use bevy_reflect::Reflect;
use bevy_render::texture::Image;
use froglight_core::data::ResourceKey;
use hashbrown::HashMap;
use parking_lot::RwLock;

use crate::ResourcePack;

/// A manager for resource packs and their assets.
#[derive(Debug, Reflect)]
pub struct AssetManagerInner {
    #[reflect(ignore)]
    /// A list of loaded resource packs.
    pub handles: RwLock<Vec<Handle<ResourcePack>>>,

    /// A collection of loaded texture assets.
    #[reflect(ignore)]
    pub textures: RwLock<HashMap<ResourceKey, Handle<Image>>>,

    /// A collection of loaded audio assets.
    #[reflect(ignore)]
    pub sounds: RwLock<HashMap<ResourceKey, Handle<AudioSource>>>,
}

static MANAGER_CREATED: AtomicBool = AtomicBool::new(false);

impl Default for AssetManagerInner {
    fn default() -> Self {
        // Log a warning if more than one `AssetManagerInner` is created.
        if MANAGER_CREATED.load(Ordering::Relaxed) {
            warn!("AssetManagerInner::default() called more than once!");
        } else {
            MANAGER_CREATED.store(true, Ordering::Relaxed);
        }

        Self {
            handles: RwLock::new(Vec::with_capacity(2)),
            textures: RwLock::new(HashMap::with_capacity(512)),
            sounds: RwLock::new(HashMap::with_capacity(512)),
        }
    }
}

#[cfg(feature = "inspector")]
impl AssetManagerInner {
    pub(crate) fn egui_impl() -> InspectorEguiImpl {
        InspectorEguiImpl::new(Self::fn_mut, Self::fn_readonly, Self::fn_many)
    }

    fn fn_mut(
        value: &mut dyn Any,
        ui: &mut Ui,
        _: &dyn Any,
        _: Id,
        _: InspectorUi<'_, '_>,
    ) -> bool {
        let value = value.downcast_mut::<Arc<Self>>().unwrap();
        let mut changed = false;

        ui.horizontal(|ui| {
            ui.label("Resource Packs");
            ui.add_enabled_ui(false, |ui| {
                let string = format!("{}", value.handles.read().len());
                changed |= ui.add(Button::new(RichText::new(string))).changed();
            });
        });

        ui.horizontal(|ui| {
            ui.label("Textures");
            ui.add_enabled_ui(false, |ui| {
                let string = format!("{}", value.textures.read().len());
                changed |= ui.add(Button::new(RichText::new(string))).changed();
            });
        });

        ui.horizontal(|ui| {
            ui.label("Sounds");
            ui.add_enabled_ui(false, |ui| {
                let string = format!("{}", value.sounds.read().len());
                changed |= ui.add(Button::new(RichText::new(string))).changed();
            });
        });

        changed
    }

    fn fn_readonly(
        value: &dyn Any,
        ui: &mut Ui,
        options: &dyn Any,
        id: Id,
        env: InspectorUi<'_, '_>,
    ) {
        let mut value = value.downcast_ref::<Arc<Self>>().unwrap().clone();
        Self::fn_mut(&mut value, ui, options, id, env);
    }

    fn fn_many(
        _: &mut Ui,
        _: &dyn Any,
        _: Id,
        _: InspectorUi<'_, '_>,
        _: &mut [&mut dyn Reflect],
        _: &dyn Fn(&mut dyn Reflect) -> &mut dyn Reflect,
    ) -> bool {
        false
    }
}
