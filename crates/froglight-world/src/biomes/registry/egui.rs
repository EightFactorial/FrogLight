use std::any::Any;

use bevy_inspector_egui::{
    egui::{Button, Id, RichText, Ui},
    inspector_egui_impls::InspectorEguiImpl,
    reflect_inspector::InspectorUi,
};
use bevy_reflect::Reflect;

use super::BiomeRegistry;
use crate::biomes::traits::BiomeRegistration;

impl<V: BiomeRegistration> BiomeRegistry<V> {
    /// Creates an `InspectorEguiImpl` for the `BiomeRegistry` type.
    pub(super) fn inspector_egui_impl() -> InspectorEguiImpl {
        InspectorEguiImpl::new(Self::fn_mut, Self::fn_readonly, Self::fn_many)
    }

    fn fn_mut(
        value: &mut dyn Any,
        ui: &mut Ui,
        _: &dyn Any,
        _: Id,
        _: InspectorUi<'_, '_>,
    ) -> bool {
        let value = value.downcast_mut::<Self>().unwrap();
        let mut changed = false;

        for (biome_id, biome) in value.read().dyn_biomes.iter().enumerate() {
            ui.horizontal(|ui| {
                ui.label(biome.resource_key().as_str());
                ui.add_enabled_ui(false, |ui| {
                    let string = format!("{biome_id:?}");
                    changed |= ui.add(Button::new(RichText::new(string))).changed();
                });
            });
        }

        changed
    }

    fn fn_readonly(
        value: &dyn Any,
        ui: &mut Ui,
        options: &dyn Any,
        id: Id,
        env: InspectorUi<'_, '_>,
    ) {
        let mut value = value.downcast_ref::<Self>().unwrap().clone();
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
