use froglight_common::prelude::V26_2;

crate::version::version_implement! {
    impl crate::version::RegistryVersion => V26_2 {
        const REGISTRY: crate::storage::RegistryStorage;
        fn new_registry() => {
            crate::storage::RegistryStorage::build::<Self>(
                alloc::vec![],
                alloc::vec![],
            )
        }
    }
}
