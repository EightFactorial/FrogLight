use froglight_protocol::{common::ResourceKey, traits::Version};

use crate::definitions::{
    ConvertKey, DefaultRegistry, InitializeRegistry, SimpleRegistry, UnknownKeyError,
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
    fn try_from_key(key: &(impl AsRef<str> + ?Sized)) -> Result<Self, UnknownKeyError> {
        match key.as_ref() {
            "froglight:first" => Ok(Self::First),
            "froglight:second" => Ok(Self::Second),
            "froglight:third" => Ok(Self::Third),
            "froglight:fourth" => Ok(Self::Fourth),
            unk => Err(UnknownKeyError::new::<Self>(unk)),
        }
    }

    fn as_key(&self) -> ResourceKey {
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
    assert_eq!(TestRegistry::try_from_key("froglight:first").unwrap(), TestRegistry::First);
    assert_eq!(TestRegistry::try_from_key("froglight:second").unwrap(), TestRegistry::Second);
    assert_eq!(TestRegistry::try_from_key("froglight:third").unwrap(), TestRegistry::Third);
    assert_eq!(TestRegistry::try_from_key("froglight:fourth").unwrap(), TestRegistry::Fourth);

    // Test an unknown key.
    {
        let fifth = TestRegistry::try_from_key("froglight:fifth");
        let err = fifth.expect_err("Expected an UnknownKeyError");

        assert_eq!(err.type_name, "froglight_registry::tests::simple_registry::TestRegistry");
        assert_eq!(err.key, "froglight:fifth");
    }
}

#[test]
fn as_key() {
    // Test all variants have the correct keys.
    assert_eq!(TestRegistry::First.as_key(), ResourceKey::new("froglight:first"));
    assert_eq!(TestRegistry::Second.as_key(), ResourceKey::new("froglight:second"));
    assert_eq!(TestRegistry::Third.as_key(), ResourceKey::new("froglight:third"));
    assert_eq!(TestRegistry::Fourth.as_key(), ResourceKey::new("froglight:fourth"));
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
    let default = SimpleRegistry::<InOrderVersion, TestRegistry, DefaultRegistry>::default();

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
        registry.insert_value(TestRegistry::Fourth);

        assert_eq!(default.get_value(3), None);
        assert_eq!(registry.get_value(3), Some(TestRegistry::Fourth));

        assert_eq!(default.get_id(&TestRegistry::Fourth), None);
        assert_eq!(registry.get_id(&TestRegistry::Fourth), Some(3));
    }

    // Overwrite the RuntimeRegistry with the DefaultRegistry.
    {
        registry.overwrite_default(&default);

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
    let default = SimpleRegistry::<ReverseOrderVersion, TestRegistry, DefaultRegistry>::default();

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
        registry.insert_value(TestRegistry::Fourth);

        assert_eq!(default.get_value(3), None);
        assert_eq!(registry.get_value(3), Some(TestRegistry::Fourth));

        assert_eq!(default.get_id(&TestRegistry::Fourth), None);
        assert_eq!(registry.get_id(&TestRegistry::Fourth), Some(3));
    }

    // Overwrite the RuntimeRegistry with the DefaultRegistry.
    {
        registry.overwrite_default(&default);

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
