//! @generated registry trait implementations

macro_rules! generate {
    // Generate a `Registry` implementation for a given version.
    (@registry $version:ty, $( $default:tt => $name:literal: [ $($item:literal),* ] ),+) => {
        impl Registry for $version {
            fn registry() -> &'static StaticRegistryMap {
                static REGISTRY: Lazy<StaticRegistryMap> = Lazy::new(|| {
                    let mut map = RegistryMap::new();
                    <$version as Registry>::init_registry(&mut map);
                    StaticRegistryMap::new::<$version>(map)
                });

                &REGISTRY
            }

            fn init_registry(map: &mut RegistryMap) {
                let inner = RegistryMap::as_inner_mut(map);
                $(
                    generate!(@item inner, $default => $name: [ $($item),* ]);
                )+
            }
        }
    };

    (@item $inner:expr, None => $name:literal: [ $($item:literal),* ]) => {
        $inner.insert(Identifier::new_static($name), RegistrySet::new(
            None, [
                $(Identifier::new_static($item)),
            +].into_iter().collect()
        ));
    };
    (@item $inner:expr, $default:literal => $name:literal: [ $($item:literal),* ]) => {
        $inner.insert(Identifier::new_static($name), RegistrySet::new(
            Some(Identifier::new_static($default)), [
                $(Identifier::new_static($item)),
            +].into_iter().collect()
        ));
    };
}

// -------------------------------------------------------------------------------------------------
// Note: The following modules are automatically @generated.

mod v1_21_8;
// mod v1_21_9;
