use bevy_reflect::TypeRegistry;

mod bool;
pub use bool::BrigadierBool;

mod double;
pub use double::BrigadierDouble;

pub(super) fn register_types(registry: &mut TypeRegistry) {
    registry.register::<BrigadierBool>();
    registry.register::<BrigadierDouble>();
}
