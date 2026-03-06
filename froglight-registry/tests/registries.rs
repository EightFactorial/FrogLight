//! TODO
#![no_std]

use froglight_common::prelude::*;
use froglight_registry::{implement_registry, prelude::*};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct TestVersion;

impl Version for TestVersion {
    const DATA_VERSION: u32 = u32::MIN;
    const PROTOCOL_ID: u32 = u32::MIN;
    const RESOURCE_VERSION: u32 = u32::MIN;
}

implement_registry! {
    TestVersion => {
        "test:example_a" => [
            "test:example_a_a" => &[],
            "test:example_a_b" => &[],
            "test:example_a_c" => &[],
        ],
        "test:example_b" => [
            "test:example_b_1" => &[],
            "test:example_b_2" => &[],
        ]
    }
}

// -------------------------------------------------------------------------------------------------

#[test]
#[allow(unused_mut, reason = "Used if `alloc` feature is enabled")]
fn entry_a() {
    let mut storage = TestVersion::new_registry();
    let registry = storage.get("test:example_a").unwrap();

    assert_eq!(registry.len(), 3);
    assert!(registry.get_by_name("test:example_a_a").is_some());
    assert_eq!(registry.get(0).unwrap().key(), "test:example_a_a");
    assert!(registry.get_by_name("test:example_a_b").is_some());
    assert_eq!(registry.get(1).unwrap().key(), "test:example_a_b");
    assert!(registry.get_by_name("test:example_a_c").is_some());
    assert_eq!(registry.get(2).unwrap().key(), "test:example_a_c");

    #[cfg(feature = "alloc")]
    {
        use froglight_registry::storage::RegistryValue;

        let registry = storage.get_mut("test:example_a").unwrap();
        registry
            .to_mut()
            .push(RegistryValue::new_static(Identifier::new_static("test:example_a_d"), &[]));

        assert_eq!(registry.len(), 4);
        assert!(registry.get_by_name("test:example_a_a").is_some());
        assert_eq!(registry.get(0).unwrap().key(), "test:example_a_a");
        assert!(registry.get_by_name("test:example_a_b").is_some());
        assert_eq!(registry.get(1).unwrap().key(), "test:example_a_b");
        assert!(registry.get_by_name("test:example_a_c").is_some());
        assert_eq!(registry.get(2).unwrap().key(), "test:example_a_c");
        assert!(registry.get_by_name("test:example_a_d").is_some());
        assert_eq!(registry.get(3).unwrap().key(), "test:example_a_d");
    }
}

#[test]
#[allow(unused_mut, reason = "Used if `alloc` feature is enabled")]
#[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
fn entry_b() {
    let mut storage = TestVersion::registry().write();
    let registry = storage.get("test:example_b").unwrap();

    assert_eq!(registry.len(), 2);
    assert!(registry.get_by_name("test:example_b_1").is_some());
    assert_eq!(registry.get(0).unwrap().key(), "test:example_b_1");
    assert!(registry.get_by_name("test:example_b_2").is_some());
    assert_eq!(registry.get(1).unwrap().key(), "test:example_b_2");

    #[cfg(feature = "alloc")]
    {
        use froglight_registry::storage::RegistryValue;

        let registry = storage.get_mut("test:example_b").unwrap();
        registry
            .to_mut()
            .push(RegistryValue::new_static(Identifier::new_static("test:example_b_3"), &[]));

        assert_eq!(registry.len(), 3);
        assert!(registry.get_by_name("test:example_b_1").is_some());
        assert_eq!(registry.get(0).unwrap().key(), "test:example_b_1");
        assert!(registry.get_by_name("test:example_b_2").is_some());
        assert_eq!(registry.get(1).unwrap().key(), "test:example_b_2");
        assert!(registry.get_by_name("test:example_b_3").is_some());
        assert_eq!(registry.get(2).unwrap().key(), "test:example_b_3");
    }
}
