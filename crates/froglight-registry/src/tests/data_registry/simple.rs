use froglight_macros::FrogRegistry;
use hashbrown::HashMap;

use crate::{
    definitions::{ConvertKey, DefaultRegistry, InitializeRegistry},
    tests::TestVersion,
};

/// A test registry with four values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogRegistry)]
enum TestEnum {
    #[frog(key = "froglight:first")]
    First,
    #[frog(key = "froglight:second")]
    Second,
    #[frog(key = "froglight:third")]
    Third,
    #[frog(key = "froglight:fourth")]
    Fourth,
}

impl InitializeRegistry<TestVersion> for TestEnum {
    fn initialize_ids() -> Vec<Self> { vec![Self::First, Self::Second, Self::Third] }

    fn initialize_storage() -> HashMap<Self, serde_json::Value> { HashMap::new() }
}

#[test]
fn from_key() {
    assert_eq!(TestEnum::try_from_key("froglight:first").unwrap(), TestEnum::First);
    assert_eq!(TestEnum::try_from_key("froglight:second").unwrap(), TestEnum::Second);
    assert_eq!(TestEnum::try_from_key("froglight:third").unwrap(), TestEnum::Third);
    assert_eq!(TestEnum::try_from_key("froglight:fourth").unwrap(), TestEnum::Fourth);

    assert!(TestEnum::try_from_key("froglight:fifth").is_err());
}

#[test]
fn to_key() {
    assert_eq!(TestEnum::First.to_key(), "froglight:first");
    assert_eq!(TestEnum::Second.to_key(), "froglight:second");
    assert_eq!(TestEnum::Third.to_key(), "froglight:third");
    assert_eq!(TestEnum::Fourth.to_key(), "froglight:fourth");
}

#[test]
fn registry_inorder() {
    // Create a default registry
    let default: DefaultRegistry<TestVersion, TestEnum> = DefaultRegistry::default();

    // Check the default values
    {
        assert_eq!(default.get_id(&TestEnum::First), Some(0));
        assert_eq!(default.get_id(&TestEnum::Second), Some(1));
        assert_eq!(default.get_id(&TestEnum::Third), Some(2));
        assert_eq!(default.get_id(&TestEnum::Fourth), None);

        assert_eq!(default.get_value(0), Some(TestEnum::First));
        assert_eq!(default.get_value(1), Some(TestEnum::Second));
        assert_eq!(default.get_value(2), Some(TestEnum::Third));
        assert_eq!(default.get_value(3), None);

        assert_eq!(default.get_data(&TestEnum::First), None);
        assert_eq!(default.get_data(&TestEnum::Second), None);
        assert_eq!(default.get_data(&TestEnum::Third), None);
        assert_eq!(default.get_data(&TestEnum::Fourth), None);
    }

    // Create a simple registry
    let mut simple = TestEnumRegistry::new_from_default(&default);

    // Check that new simple registry is the same as the default
    {
        assert_eq!(simple.get_id(&TestEnum::First), Some(0));
        assert_eq!(simple.get_id(&TestEnum::Second), Some(1));
        assert_eq!(simple.get_id(&TestEnum::Third), Some(2));
        assert_eq!(simple.get_id(&TestEnum::Fourth), None);

        assert_eq!(simple.get_value(0), Some(TestEnum::First));
        assert_eq!(simple.get_value(1), Some(TestEnum::Second));
        assert_eq!(simple.get_value(2), Some(TestEnum::Third));
        assert_eq!(simple.get_value(3), None);

        assert_eq!(simple.get_data(&TestEnum::First), None);
        assert_eq!(simple.get_data(&TestEnum::Second), None);
        assert_eq!(simple.get_data(&TestEnum::Third), None);
        assert_eq!(simple.get_data(&TestEnum::Fourth), None);
    }

    // Push a new value into the simple registry
    {
        simple.push_value(TestEnum::Fourth);
        simple.insert_data(TestEnum::Fourth, serde_json::Value::Null);

        // Make sure the value exists in the simple registry
        assert_eq!(simple.get_id(&TestEnum::Fourth), Some(3));
        assert_eq!(simple.get_data(&TestEnum::Fourth), Some(serde_json::Value::Null));
        assert_eq!(simple.get_value(3), Some(TestEnum::Fourth));

        // Make sure the value does not exist in the default registry
        assert_eq!(default.get_id(&TestEnum::Fourth), None);
        assert_eq!(default.get_data(&TestEnum::Fourth), None);
        assert_eq!(default.get_value(3), None);
    }

    // Reset the simple registry
    {
        simple.overwrite_with(&default);

        // Make sure the simple registry is the same as the default
        assert_eq!(simple.get_id(&TestEnum::First), Some(0));
        assert_eq!(simple.get_id(&TestEnum::Second), Some(1));
        assert_eq!(simple.get_id(&TestEnum::Third), Some(2));
        assert_eq!(simple.get_id(&TestEnum::Fourth), None);

        assert_eq!(simple.get_value(0), Some(TestEnum::First));
        assert_eq!(simple.get_value(1), Some(TestEnum::Second));
        assert_eq!(simple.get_value(2), Some(TestEnum::Third));
        assert_eq!(simple.get_value(3), None);

        assert_eq!(simple.get_data(&TestEnum::First), None);
        assert_eq!(simple.get_data(&TestEnum::Second), None);
        assert_eq!(simple.get_data(&TestEnum::Third), None);
        assert_eq!(simple.get_data(&TestEnum::Fourth), None);
    }
}
