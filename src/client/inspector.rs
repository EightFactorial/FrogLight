use std::any::{type_name, Any, TypeId};

use bevy::{
    app::{App, Plugin},
    asset::Handle,
    ecs::system::{Local, Res},
    input::{keyboard::KeyCode, ButtonInput},
    log::info,
    prelude::{AppTypeRegistry, Image, Mesh},
    reflect::Reflect,
};
use bevy_inspector_egui::{
    egui::{text::LayoutJob, FontId, Id, TextFormat, Ui},
    inspector_egui_impls::{InspectorEguiImpl, InspectorPrimitive},
    quick::WorldInspectorPlugin,
    reflect_inspector::InspectorUi,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        Self::add_of_with_many::<Handle<Image>>(app);
        Self::add_of_with_many::<Handle<Mesh>>(app);

        info!("Use F3 + I to toggle the inspector!");
        app.add_plugins(WorldInspectorPlugin::new().run_if(Self::input_toggle));
    }
}

impl InspectorPlugin {
    /// Toggle the `bevy-inspector-egui` inspector with F3 + I
    fn input_toggle(input: Res<ButtonInput<KeyCode>>, mut state: Local<bool>) -> bool {
        if input.just_pressed(KeyCode::KeyI) && input.pressed(KeyCode::F3) {
            *state = !*state;
        }
        *state
    }
}

impl InspectorPlugin {
    /// Add an `InspectorEguiImpl` for a type
    fn add_of_with_many<T: InspectorPrimitive>(app: &mut App) {
        app.world_mut()
            .resource_mut::<AppTypeRegistry>()
            .write()
            .get_mut(TypeId::of::<T>())
            .unwrap()
            .insert(InspectorEguiImpl::of_with_many::<T>(Self::many_unimplemented::<T>));
    }

    fn many_unimplemented<T: InspectorPrimitive>(
        ui: &mut Ui,
        _: &dyn Any,
        _: Id,
        _: InspectorUi<'_, '_>,
        _: &mut [&mut dyn Reflect],
        _: &dyn Fn(&mut dyn Reflect) -> &mut dyn Reflect,
    ) -> bool {
        Self::no_multiedit(ui, type_name::<T>());
        false
    }

    fn no_multiedit(ui: &mut Ui, type_name: &str) {
        let job = Self::layout_job(&[
            (FontId::monospace(12.0), type_name),
            (FontId::proportional(13.0), " doesn't support multi-editing."),
        ]);
        ui.label(job);
    }

    fn layout_job(text: &[(FontId, &str)]) -> LayoutJob {
        let mut job = LayoutJob::default();
        for (font_id, text) in text {
            job.append(text, 0.0, TextFormat { font_id: font_id.clone(), ..Default::default() });
        }
        job
    }
}
