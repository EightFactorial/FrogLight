use std::convert::Infallible;

use froglight_protocol::{common::ResourceKey, traits::Version};

use crate::{
    definitions::{ConvertKey, DefaultRegistry, InitializeRegistry, SimpleRegistry},
    ConvertKeyError,
};

/// A test registry with four values.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum TestRegistryClone {
    First,
    Second,
    Third,
    Fourth,
    Other(ResourceKey),
}

impl ConvertKey for TestRegistryClone {
    type Error = Infallible;
    fn from_key(key: &ResourceKey) -> Result<Self, Self::Error> {
        match key.as_ref() {
            "froglight:first" => Ok(Self::First),
            "froglight:second" => Ok(Self::Second),
            "froglight:third" => Ok(Self::Third),
            "froglight:fourth" => Ok(Self::Fourth),
            _ => Ok(Self::Other(key.clone())),
        }
    }

    fn to_key(&self) -> ResourceKey {
        match self {
            Self::First => ResourceKey::new("froglight:first"),
            Self::Second => ResourceKey::new("froglight:second"),
            Self::Third => ResourceKey::new("froglight:third"),
            Self::Fourth => ResourceKey::new("froglight:fourth"),
            Self::Other(key) => key.clone(),
        }
    }
}

#[test]
fn from_key() {
    // Test known keys.
    assert_eq!(
        TestRegistryClone::from_key(&ResourceKey::new_inline("froglight:first")).unwrap(),
        TestRegistryClone::First
    );
    assert_eq!(
        TestRegistryClone::from_key(&ResourceKey::new_inline("froglight:second")).unwrap(),
        TestRegistryClone::Second
    );
    assert_eq!(
        TestRegistryClone::from_key(&ResourceKey::new_inline("froglight:third")).unwrap(),
        TestRegistryClone::Third
    );
    assert_eq!(
        TestRegistryClone::from_key(&ResourceKey::new_inline("froglight:fourth")).unwrap(),
        TestRegistryClone::Fourth
    );

    // Test an unknown key.
    let fifth = TestRegistryClone::from_key(&ResourceKey::new_inline("froglight:fifth"));
    assert!(matches!(fifth, Ok(TestRegistryClone::Other(_))));

    // Test an invalid key
    let invalid = TestRegistryClone::try_from_key("froglight:invalid:key");
    assert!(matches!(invalid, Err(ConvertKeyError::ResourceKey(_))));
}

#[test]
fn as_key() {
    // Test all variants have the correct keys.
    assert_eq!(TestRegistryClone::First.to_key(), ResourceKey::new_inline("froglight:first"));
    assert_eq!(TestRegistryClone::Second.to_key(), ResourceKey::new_inline("froglight:second"));
    assert_eq!(TestRegistryClone::Third.to_key(), ResourceKey::new_inline("froglight:third"));
    assert_eq!(TestRegistryClone::Fourth.to_key(), ResourceKey::new_inline("froglight:fourth"));
}

/// A test [`Version`] that's registry values are in order.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct InOrderVersion;

impl Version for InOrderVersion {
    const ID: i32 = 0;
}

impl InitializeRegistry<InOrderVersion> for TestRegistryClone {
    fn initialize() -> Vec<Self> { vec![Self::First, Self::Second, Self::Third] }
}

#[test]
fn inorder_registry() {
    let default = DefaultRegistry::<InOrderVersion, TestRegistryClone>::new();

    // Test the DefaultRegistry values.
    {
        assert_eq!(default.get_value_cloned(0), Some(TestRegistryClone::First));
        assert_eq!(default.get_value_cloned(1), Some(TestRegistryClone::Second));
        assert_eq!(default.get_value_cloned(2), Some(TestRegistryClone::Third));
        assert_eq!(default.get_value_cloned(3), None);

        assert_eq!(default.get_id(&TestRegistryClone::First), Some(0));
        assert_eq!(default.get_id(&TestRegistryClone::Second), Some(1));
        assert_eq!(default.get_id(&TestRegistryClone::Third), Some(2));
        assert_eq!(default.get_id(&TestRegistryClone::Fourth), None);
    }

    let mut registry = SimpleRegistry::from_default(&default);

    // Test the RuntimeRegistry values.
    {
        assert_eq!(registry.get_value_cloned(0), Some(TestRegistryClone::First));
        assert_eq!(registry.get_value_cloned(1), Some(TestRegistryClone::Second));
        assert_eq!(registry.get_value_cloned(2), Some(TestRegistryClone::Third));
        assert_eq!(registry.get_value_cloned(3), None);

        assert_eq!(registry.get_id(&TestRegistryClone::First), Some(0));
        assert_eq!(registry.get_id(&TestRegistryClone::Second), Some(1));
        assert_eq!(registry.get_id(&TestRegistryClone::Third), Some(2));
        assert_eq!(registry.get_id(&TestRegistryClone::Fourth), None);
    }

    // Add a new value to the RuntimeRegistry.
    {
        registry.push_value(TestRegistryClone::Fourth);

        assert_eq!(default.get_value_cloned(3), None);
        assert_eq!(registry.get_value_cloned(3), Some(TestRegistryClone::Fourth));

        assert_eq!(default.get_id(&TestRegistryClone::Fourth), None);
        assert_eq!(registry.get_id(&TestRegistryClone::Fourth), Some(3));
    }

    // Overwrite the RuntimeRegistry with the DefaultRegistry.
    {
        registry.clone_default(&default);

        assert_eq!(registry.get_value_cloned(0), Some(TestRegistryClone::First));
        assert_eq!(registry.get_value_cloned(1), Some(TestRegistryClone::Second));
        assert_eq!(registry.get_value_cloned(2), Some(TestRegistryClone::Third));
        assert_eq!(registry.get_value_cloned(3), None);

        assert_eq!(registry.get_id(&TestRegistryClone::First), Some(0));
        assert_eq!(registry.get_id(&TestRegistryClone::Second), Some(1));
        assert_eq!(registry.get_id(&TestRegistryClone::Third), Some(2));
        assert_eq!(registry.get_id(&TestRegistryClone::Fourth), None);
    }
}

/// A test [`Version`] that's registry values are in reverse order.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct ReverseOrderVersion;

impl Version for ReverseOrderVersion {
    const ID: i32 = 0;
}

impl InitializeRegistry<ReverseOrderVersion> for TestRegistryClone {
    fn initialize() -> Vec<Self> {
        vec![TestRegistryClone::Third, TestRegistryClone::Second, TestRegistryClone::First]
    }
}

#[test]
fn reverseorder_registry() {
    let default = DefaultRegistry::<ReverseOrderVersion, TestRegistryClone>::default();

    // Test the DefaultRegistry values.
    {
        assert_eq!(default.get_value_cloned(0), Some(TestRegistryClone::Third));
        assert_eq!(default.get_value_cloned(1), Some(TestRegistryClone::Second));
        assert_eq!(default.get_value_cloned(2), Some(TestRegistryClone::First));
        assert_eq!(default.get_value_cloned(3), None);

        assert_eq!(default.get_id(&TestRegistryClone::Fourth), None);
        assert_eq!(default.get_id(&TestRegistryClone::Third), Some(0));
        assert_eq!(default.get_id(&TestRegistryClone::Second), Some(1));
        assert_eq!(default.get_id(&TestRegistryClone::First), Some(2));
    }

    let mut registry = SimpleRegistry::from_default(&default);

    //  Test the RuntimeRegistry values.
    {
        assert_eq!(registry.get_value_cloned(0), Some(TestRegistryClone::Third));
        assert_eq!(registry.get_value_cloned(1), Some(TestRegistryClone::Second));
        assert_eq!(registry.get_value_cloned(2), Some(TestRegistryClone::First));
        assert_eq!(registry.get_value_cloned(3), None);

        assert_eq!(registry.get_id(&TestRegistryClone::Fourth), None);
        assert_eq!(registry.get_id(&TestRegistryClone::Third), Some(0));
        assert_eq!(registry.get_id(&TestRegistryClone::Second), Some(1));
        assert_eq!(registry.get_id(&TestRegistryClone::First), Some(2));
    }

    // Add a new value to the RuntimeRegistry.
    {
        registry.push_value(TestRegistryClone::Fourth);

        assert_eq!(default.get_value_cloned(3), None);
        assert_eq!(registry.get_value_cloned(3), Some(TestRegistryClone::Fourth));

        assert_eq!(default.get_id(&TestRegistryClone::Fourth), None);
        assert_eq!(registry.get_id(&TestRegistryClone::Fourth), Some(3));
    }

    // Overwrite the RuntimeRegistry with the DefaultRegistry.
    {
        registry.clone_default(&default);

        assert_eq!(registry.get_value_cloned(0), Some(TestRegistryClone::Third));
        assert_eq!(registry.get_value_cloned(1), Some(TestRegistryClone::Second));
        assert_eq!(registry.get_value_cloned(2), Some(TestRegistryClone::First));
        assert_eq!(registry.get_value_cloned(3), None);

        assert_eq!(registry.get_id(&TestRegistryClone::Third), Some(0));
        assert_eq!(registry.get_id(&TestRegistryClone::Second), Some(1));
        assert_eq!(registry.get_id(&TestRegistryClone::First), Some(2));
        assert_eq!(registry.get_id(&TestRegistryClone::Fourth), None);
    }
}
