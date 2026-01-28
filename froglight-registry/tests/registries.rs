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
            "test:example_a_a",
            "test:example_a_b",
            "test:example_a_c",
        ],
        "test:example_b" => [
            "test:example_b_1",
            "test:example_b_2",
        ]
    }
}

// -------------------------------------------------------------------------------------------------

#[test]
#[allow(unused_mut, reason = "Used if `alloc` feature is enabled")]
fn entry_a() {
    let mut storage = TestVersion::new_registries();
    let registry = storage.get("test:example_a").unwrap();

    assert_eq!(registry.len(), 3);
    assert_eq!(registry.get_index("test:example_a_a"), Some(0));
    assert_eq!(registry.get_name(0).unwrap(), "test:example_a_a");
    assert_eq!(registry.get_index("test:example_a_b"), Some(1));
    assert_eq!(registry.get_name(1).unwrap(), "test:example_a_b");
    assert_eq!(registry.get_index("test:example_a_c"), Some(2));
    assert_eq!(registry.get_name(2).unwrap(), "test:example_a_c");

    #[cfg(feature = "alloc")]
    {
        let registry = storage.get_mut("test:example_a").unwrap();
        registry.to_mut().push(Identifier::new_static("test:example_a_d"));

        assert_eq!(registry.len(), 4);
        assert_eq!(registry.get_index("test:example_a_a"), Some(0));
        assert_eq!(registry.get_name(0).unwrap(), "test:example_a_a");
        assert_eq!(registry.get_index("test:example_a_b"), Some(1));
        assert_eq!(registry.get_name(1).unwrap(), "test:example_a_b");
        assert_eq!(registry.get_index("test:example_a_c"), Some(2));
        assert_eq!(registry.get_name(2).unwrap(), "test:example_a_c");
        assert_eq!(registry.get_index("test:example_a_d"), Some(3));
        assert_eq!(registry.get_name(3).unwrap(), "test:example_a_d");
    }
}

#[test]
#[allow(unused_mut, reason = "Used if `alloc` feature is enabled")]
#[cfg(any(feature = "async", feature = "parking_lot", feature = "std"))]
fn entry_b() {
    let mut storage = TestVersion::registries().write();
    let registry = storage.get("test:example_b").unwrap();

    assert_eq!(registry.len(), 2);
    assert_eq!(registry.get_index("test:example_b_1"), Some(0));
    assert_eq!(registry.get_name(0).unwrap(), "test:example_b_1");
    assert_eq!(registry.get_index("test:example_b_2"), Some(1));
    assert_eq!(registry.get_name(1).unwrap(), "test:example_b_2");

    #[cfg(feature = "alloc")]
    {
        let registry = storage.get_mut("test:example_b").unwrap();
        registry.to_mut().push(Identifier::new_static("test:example_b_3"));

        assert_eq!(registry.len(), 3);
        assert_eq!(registry.get_index("test:example_b_1"), Some(0));
        assert_eq!(registry.get_name(0).unwrap(), "test:example_b_1");
        assert_eq!(registry.get_index("test:example_b_2"), Some(1));
        assert_eq!(registry.get_name(1).unwrap(), "test:example_b_2");
        assert_eq!(registry.get_index("test:example_b_3"), Some(2));
        assert_eq!(registry.get_name(2).unwrap(), "test:example_b_3");
    }
}
