use bevy_reflect::TypeRegistry;

mod bool;
pub use bool::BrigadierBool;

mod range;
pub use range::{BrigadierDouble, BrigadierFloat, BrigadierInt, BrigadierLong, BrigadierRange};

mod string;
pub use string::{BrigadierPhrase, BrigadierTail, BrigadierWord};

pub(super) fn register_types(registry: &mut TypeRegistry) {
    registry.register::<BrigadierBool>();
    registry.register::<BrigadierInt>();
    registry.register::<BrigadierLong>();
    registry.register::<BrigadierFloat>();
    registry.register::<BrigadierDouble>();

    registry.register::<BrigadierWord>();
    registry.register::<BrigadierPhrase>();
    registry.register::<BrigadierTail>();
}
