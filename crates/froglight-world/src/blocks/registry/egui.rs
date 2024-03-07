use std::any::Any;

use bevy_app::App;
use bevy_inspector_egui::{
    egui::{Button, Id, RichText, Ui},
    inspector_egui_impls::InspectorEguiImpl,
    reflect_inspector::InspectorUi,
};
use bevy_reflect::Reflect;
use froglight_protocol::traits::Version;

use super::{inner::InnerRegistry, BlockRegistry};

impl<V: Version> BlockRegistry<V>
where
    InnerRegistry<V>: Default,
{
    /// Registers an `InspectorEguiImpl` for the `BlockRegistry` type.
    pub(super) fn egui_register(app: &mut App) {
        let registry = app.world.resource::<bevy_ecs::prelude::AppTypeRegistry>();
        let mut registry = registry.write();

        let arc_manager = registry.get_mut(std::any::TypeId::of::<BlockRegistry<V>>()).unwrap();
        arc_manager.insert(BlockRegistry::<V>::egui_impl());
    }

    /// Creates an `InspectorEguiImpl` for the `BlockRegistry` type.
    fn egui_impl() -> InspectorEguiImpl {
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

        for (state_range, block_id) in value.read().block_states.iter() {
            if let Some(block) = value.read().blocks.get(*block_id) {
                ui.horizontal(|ui| {
                    ui.label(block.resource_key().as_str());
                    ui.add_enabled_ui(false, |ui| {
                        let string = format!("{state_range:?}");
                        changed |= ui.add(Button::new(RichText::new(string))).changed();
                    });
                });
            } else {
                ui.horizontal(|ui| {
                    ui.label("Unknown");
                    ui.add_enabled_ui(false, |ui| {
                        let string = format!("{state_range:?}");
                        changed |= ui.add(Button::new(RichText::new(string))).changed();
                    });
                });
            }
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
