//! TODO

use froglight_common::prelude::*;
use froglight_registry::{prelude::*, storage::RegistryStorage};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct TestVersion;

impl Version for TestVersion {
    const DATA_VERSION: u32 = u32::MIN;
    const PROTOCOL_ID: u32 = u32::MIN;
    const RESOURCE_VERSION: u32 = u32::MIN;
}

froglight_registry::version::version_implement! {
    impl RegistryVersion => TestVersion {
        const REGISTRY: RegistryStorage;
        fn new_registry() => {
            RegistryStorage::build::<Self>(vec![
                (
                    Identifier::new_static("test:example_a"),
                    vec![
                        (Identifier::new_static("test:example_a_a"), vec![]),
                        (Identifier::new_static("test:example_a_b"), vec![]),
                        (Identifier::new_static("test:example_a_c"), vec![]),
                    ]
                ),
                (
                    Identifier::new_static("test:example_b"),
                    vec![
                        (Identifier::new_static("test:example_b_0"), vec![]),
                        (Identifier::new_static("test:example_b_1"), vec![]),
                    ]
                ),
            ])
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[test]
fn registry() {
    let registry = TestVersion::registry();
    assert_eq!(registry.metadata().len(), 2);

    // "test:example_a"

    let registry_ref = registry.get_registry_by_identifier("test:example_a").unwrap();
    assert_eq!(registry_ref.identifier(), "test:example_a");
    assert_eq!(registry_ref.len(), 3);

    assert!(registry_ref.get_by_identifier("test:example_a_a").is_some());
    assert_eq!(registry_ref.get_by_index(0).unwrap().identifier(), "test:example_a_a");

    assert!(registry_ref.get_by_identifier("test:example_a_b").is_some());
    assert_eq!(registry_ref.get_by_index(1).unwrap().identifier(), "test:example_a_b");

    assert!(registry_ref.get_by_identifier("test:example_a_c").is_some());
    assert_eq!(registry_ref.get_by_index(2).unwrap().identifier(), "test:example_a_c");

    // "test:example_b"

    let registry_ref = registry.get_registry_by_identifier("test:example_b").unwrap();
    assert_eq!(registry_ref.identifier(), "test:example_b");
    assert_eq!(registry_ref.len(), 2);

    assert!(registry_ref.get_by_identifier("test:example_b_0").is_some());
    assert_eq!(registry_ref.get_by_index(0).unwrap().identifier(), "test:example_b_0");

    assert!(registry_ref.get_by_identifier("test:example_b_1").is_some());
    assert_eq!(registry_ref.get_by_index(1).unwrap().identifier(), "test:example_b_1");
}
