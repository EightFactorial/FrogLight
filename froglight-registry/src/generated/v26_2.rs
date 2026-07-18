use froglight_common::prelude::V26_2;

impl crate::version::RegistryVersion for V26_2 {
    const REGISTRY: &'static crate::version::LazyLock<
        crate::version::RwLock<crate::storage::RegistryStorage>,
    > = {
        static STATIC: crate::version::LazyLock<
            crate::version::RwLock<crate::storage::RegistryStorage>,
        > = crate::version::LazyLock::new(|| crate::version::RwLock::new(V26_2::new_registry()));
        &STATIC
    };

    fn new_registry() -> crate::storage::RegistryStorage {
        crate::storage::RegistryStorage::build::<Self>(alloc::vec![], alloc::vec![])
    }
}
