use froglight_protocol::{common::ResourceKey, traits::Version};

use crate::{
    definitions::{ConvertKey, DefaultRegistry, InitializeRegistry, SimpleRegistry},
    ConvertKeyError, MissingKeyError,
};

/// A test registry with four values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TestRegistry {
    First,
    Second,
    Third,
    Fourth,
}

impl ConvertKey for TestRegistry {
    type Error = MissingKeyError;
    fn from_key(key: &ResourceKey) -> Result<Self, Self::Error> {
        match key.as_ref() {
            "froglight:first" => Ok(Self::First),
            "froglight:second" => Ok(Self::Second),
            "froglight:third" => Ok(Self::Third),
            "froglight:fourth" => Ok(Self::Fourth),
            _ => Err(MissingKeyError::from(key.clone())),
        }
    }

    fn to_key(&self) -> ResourceKey {
        match self {
            Self::First => ResourceKey::new("froglight:first"),
            Self::Second => ResourceKey::new("froglight:second"),
            Self::Third => ResourceKey::new("froglight:third"),
            Self::Fourth => ResourceKey::new("froglight:fourth"),
        }
    }
}

#[test]
fn from_key() {
    // Test known keys.
    assert_eq!(
        TestRegistry::from_key(&ResourceKey::new_inline("froglight:first")).unwrap(),
        TestRegistry::First
    );
    assert_eq!(
        TestRegistry::from_key(&ResourceKey::new_inline("froglight:second")).unwrap(),
        TestRegistry::Second
    );
    assert_eq!(
        TestRegistry::from_key(&ResourceKey::new_inline("froglight:third")).unwrap(),
        TestRegistry::Third
    );
    assert_eq!(
        TestRegistry::from_key(&ResourceKey::new_inline("froglight:fourth")).unwrap(),
        TestRegistry::Fourth
    );

    // Test an unknown key.
    let fifth = TestRegistry::from_key(&ResourceKey::new_inline("froglight:fifth"));
    assert!(matches!(fifth, Err(MissingKeyError { .. })));

    // Test an invalid key
    let invalid = TestRegistry::try_from_key("froglight:invalid:key");
    assert!(matches!(invalid, Err(ConvertKeyError::ResourceKey(_))));
}

#[test]
fn as_key() {
    // Test all variants have the correct keys.
    assert_eq!(TestRegistry::First.to_key(), ResourceKey::new_inline("froglight:first"));
    assert_eq!(TestRegistry::Second.to_key(), ResourceKey::new_inline("froglight:second"));
    assert_eq!(TestRegistry::Third.to_key(), ResourceKey::new_inline("froglight:third"));
    assert_eq!(TestRegistry::Fourth.to_key(), ResourceKey::new_inline("froglight:fourth"));
}

/// A test [`Version`] that's registry values are in order.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct InOrderVersion;

impl Version for InOrderVersion {
    const ID: i32 = 0;
}

impl InitializeRegistry<InOrderVersion> for TestRegistry {
    fn initialize() -> Vec<Self> { vec![Self::First, Self::Second, Self::Third] }
}

#[test]
fn inorder_registry() {
    let default = DefaultRegistry::<InOrderVersion, TestRegistry>::new();

    // Test the DefaultRegistry values.
    {
        assert_eq!(default.get_value(0), Some(TestRegistry::First));
        assert_eq!(default.get_value(1), Some(TestRegistry::Second));
        assert_eq!(default.get_value(2), Some(TestRegistry::Third));
        assert_eq!(default.get_value(3), None);

        assert_eq!(default.get_id(&TestRegistry::First), Some(0));
        assert_eq!(default.get_id(&TestRegistry::Second), Some(1));
        assert_eq!(default.get_id(&TestRegistry::Third), Some(2));
        assert_eq!(default.get_id(&TestRegistry::Fourth), None);
    }

    let mut registry = SimpleRegistry::from_default(&default);

    // Test the RuntimeRegistry values.
    {
        assert_eq!(registry.get_value(0), Some(TestRegistry::First));
        assert_eq!(registry.get_value(1), Some(TestRegistry::Second));
        assert_eq!(registry.get_value(2), Some(TestRegistry::Third));
        assert_eq!(registry.get_value(3), None);

        assert_eq!(registry.get_id(&TestRegistry::First), Some(0));
        assert_eq!(registry.get_id(&TestRegistry::Second), Some(1));
        assert_eq!(registry.get_id(&TestRegistry::Third), Some(2));
        assert_eq!(registry.get_id(&TestRegistry::Fourth), None);
    }

    // Add a new value to the RuntimeRegistry.
    {
        registry.push_value(TestRegistry::Fourth);

        assert_eq!(default.get_value(3), None);
        assert_eq!(registry.get_value(3), Some(TestRegistry::Fourth));

        assert_eq!(default.get_id(&TestRegistry::Fourth), None);
        assert_eq!(registry.get_id(&TestRegistry::Fourth), Some(3));
    }

    // Overwrite the RuntimeRegistry with the DefaultRegistry.
    {
        registry.clone_default(&default);

        assert_eq!(registry.get_value(0), Some(TestRegistry::First));
        assert_eq!(registry.get_value(1), Some(TestRegistry::Second));
        assert_eq!(registry.get_value(2), Some(TestRegistry::Third));
        assert_eq!(registry.get_value(3), None);

        assert_eq!(registry.get_id(&TestRegistry::First), Some(0));
        assert_eq!(registry.get_id(&TestRegistry::Second), Some(1));
        assert_eq!(registry.get_id(&TestRegistry::Third), Some(2));
        assert_eq!(registry.get_id(&TestRegistry::Fourth), None);
    }
}

/// A test [`Version`] that's registry values are in reverse order.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct ReverseOrderVersion;

impl Version for ReverseOrderVersion {
    const ID: i32 = 0;
}

impl InitializeRegistry<ReverseOrderVersion> for TestRegistry {
    fn initialize() -> Vec<Self> {
        vec![TestRegistry::Third, TestRegistry::Second, TestRegistry::First]
    }
}

#[test]
fn reverseorder_registry() {
    let default = DefaultRegistry::<ReverseOrderVersion, TestRegistry>::default();

    // Test the DefaultRegistry values.
    {
        assert_eq!(default.get_value(0), Some(TestRegistry::Third));
        assert_eq!(default.get_value(1), Some(TestRegistry::Second));
        assert_eq!(default.get_value(2), Some(TestRegistry::First));
        assert_eq!(default.get_value(3), None);

        assert_eq!(default.get_id(&TestRegistry::Fourth), None);
        assert_eq!(default.get_id(&TestRegistry::Third), Some(0));
        assert_eq!(default.get_id(&TestRegistry::Second), Some(1));
        assert_eq!(default.get_id(&TestRegistry::First), Some(2));
    }

    let mut registry = SimpleRegistry::from_default(&default);

    //  Test the RuntimeRegistry values.
    {
        assert_eq!(registry.get_value(0), Some(TestRegistry::Third));
        assert_eq!(registry.get_value(1), Some(TestRegistry::Second));
        assert_eq!(registry.get_value(2), Some(TestRegistry::First));
        assert_eq!(registry.get_value(3), None);

        assert_eq!(registry.get_id(&TestRegistry::Fourth), None);
        assert_eq!(registry.get_id(&TestRegistry::Third), Some(0));
        assert_eq!(registry.get_id(&TestRegistry::Second), Some(1));
        assert_eq!(registry.get_id(&TestRegistry::First), Some(2));
    }

    // Add a new value to the RuntimeRegistry.
    {
        registry.push_value(TestRegistry::Fourth);

        assert_eq!(default.get_value(3), None);
        assert_eq!(registry.get_value(3), Some(TestRegistry::Fourth));

        assert_eq!(default.get_id(&TestRegistry::Fourth), None);
        assert_eq!(registry.get_id(&TestRegistry::Fourth), Some(3));
    }

    // Overwrite the RuntimeRegistry with the DefaultRegistry.
    {
        registry.clone_default(&default);

        assert_eq!(registry.get_value(0), Some(TestRegistry::Third));
        assert_eq!(registry.get_value(1), Some(TestRegistry::Second));
        assert_eq!(registry.get_value(2), Some(TestRegistry::First));
        assert_eq!(registry.get_value(3), None);

        assert_eq!(registry.get_id(&TestRegistry::Third), Some(0));
        assert_eq!(registry.get_id(&TestRegistry::Second), Some(1));
        assert_eq!(registry.get_id(&TestRegistry::First), Some(2));
        assert_eq!(registry.get_id(&TestRegistry::Fourth), None);
    }
}
