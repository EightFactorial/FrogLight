use bevy_reflect::{FromType, Reflect};
use froglight_protocol::traits::Version;

use super::BiomeType;

/// A type generated by the #[`reflect_trait`](bevy_reflect::reflect_trait)
/// macro for the `BiomeType` trait.
///
/// This allows casting from `dyn Reflect` to `dyn BiomeType`.
#[derive(Clone)]
#[allow(clippy::type_complexity)]
pub struct ReflectBiomeType<V: Version> {
    get_func: fn(&dyn Reflect) -> Option<&dyn BiomeType<V>>,
    get_mut_func: fn(&mut dyn Reflect) -> Option<&mut dyn BiomeType<V>>,
    get_boxed_func: fn(Box<dyn Reflect>) -> Result<Box<dyn BiomeType<V>>, Box<dyn Reflect>>,
}
impl<V: Version> ReflectBiomeType<V> {
    /// Downcast a `&dyn Reflect` type to `&dyn BiomeType`.
    ///
    /// If the type cannot be downcast, `None` is returned.
    pub fn get<'a>(&self, reflect_value: &'a dyn Reflect) -> Option<&'a dyn BiomeType<V>> {
        (self.get_func)(reflect_value)
    }
    /// Downcast a `&mut dyn Reflect` type to `&mut dyn BiomeType`.
    ///
    /// If the type cannot be downcast, `None` is returned.
    pub fn get_mut<'a>(
        &self,
        reflect_value: &'a mut dyn Reflect,
    ) -> Option<&'a mut dyn BiomeType<V>> {
        (self.get_mut_func)(reflect_value)
    }
    /// Downcast a `Box<dyn Reflect>` type to `Box<dyn BiomeType>`.
    ///
    /// If the type cannot be downcast, this will return `Err(Box<dyn
    /// Reflect>)`.
    #[allow(clippy::missing_errors_doc)]
    pub fn get_boxed(
        &self,
        reflect_value: Box<dyn Reflect>,
    ) -> Result<Box<dyn BiomeType<V>>, Box<dyn Reflect>> {
        (self.get_boxed_func)(reflect_value)
    }
}
impl<T: BiomeType<V> + Reflect, V: Version> FromType<T> for ReflectBiomeType<V> {
    #[allow(trivial_casts)]
    fn from_type() -> Self {
        Self {
            get_func: |reflect_value| {
                <dyn Reflect>::downcast_ref::<T>(reflect_value)
                    .map(|value| value as &dyn BiomeType<V>)
            },
            get_mut_func: |reflect_value| {
                <dyn Reflect>::downcast_mut::<T>(reflect_value)
                    .map(|value| value as &mut dyn BiomeType<V>)
            },
            get_boxed_func: |reflect_value| {
                <dyn Reflect>::downcast::<T>(reflect_value)
                    .map(|value| value as Box<dyn BiomeType<V>>)
            },
        }
    }
}
