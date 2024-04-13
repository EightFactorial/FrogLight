use froglight_macros::FrogRegistry;
use froglight_protocol::common::ResourceKey;

use super::TestVersion;
use crate::definitions::convert::{ConvertKey, DefaultIdRegistry, InitializeIdRegistry};

/// A test registry with four values.
#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogRegistry)]
enum TestEnumOther {
    #[frog(key = "froglight:first")]
    First,
    #[frog(key = "froglight:second")]
    Second,
    #[frog(key = "froglight:third")]
    Third,
    #[frog(key = "froglight:fourth")]
    Fourth,
    #[frog(other)]
    Other(ResourceKey),
}

impl InitializeIdRegistry<TestVersion> for TestEnumOther {
    fn initialize() -> Vec<Self> {
        vec![
            Self::First,
            Self::Second,
            Self::Third,
            Self::Other(ResourceKey::new("froglight:fifth")),
        ]
    }
}

#[test]
fn from_key() {
    assert_eq!(TestEnumOther::try_from_key("froglight:first").unwrap(), TestEnumOther::First);
    assert_eq!(TestEnumOther::try_from_key("froglight:second").unwrap(), TestEnumOther::Second);
    assert_eq!(TestEnumOther::try_from_key("froglight:third").unwrap(), TestEnumOther::Third);
    assert_eq!(TestEnumOther::try_from_key("froglight:fourth").unwrap(), TestEnumOther::Fourth);
    assert_eq!(
        TestEnumOther::try_from_key("froglight:fifth").unwrap(),
        TestEnumOther::Other(ResourceKey::new_inline("froglight:fifth"))
    );
}

#[test]
fn to_key() {
    assert_eq!(TestEnumOther::First.to_key(), "froglight:first");
    assert_eq!(TestEnumOther::Second.to_key(), "froglight:second");
    assert_eq!(TestEnumOther::Third.to_key(), "froglight:third");
    assert_eq!(TestEnumOther::Fourth.to_key(), "froglight:fourth");
    assert_eq!(
        TestEnumOther::Other(ResourceKey::new("froglight:fifth")).to_key(),
        "froglight:fifth"
    );
}

#[test]
fn registry_inorder() {
    // Create a default registry
    let default: DefaultIdRegistry<TestVersion, TestEnumOther> = DefaultIdRegistry::default();

    // Check the default values
    {
        assert_eq!(default.get_id(&TestEnumOther::First), Some(0));
        assert_eq!(default.get_id(&TestEnumOther::Second), Some(1));
        assert_eq!(default.get_id(&TestEnumOther::Third), Some(2));
        assert_eq!(default.get_id(&TestEnumOther::Fourth), None);
        assert_eq!(
            default.get_id(&TestEnumOther::Other(ResourceKey::new("froglight:fifth"))),
            Some(3)
        );

        assert_eq!(default.get_value_cloned(0), Some(TestEnumOther::First));
        assert_eq!(default.get_value_cloned(1), Some(TestEnumOther::Second));
        assert_eq!(default.get_value_cloned(2), Some(TestEnumOther::Third));
        assert_eq!(
            default.get_value_cloned(3),
            Some(TestEnumOther::Other(ResourceKey::new("froglight:fifth")))
        );
    }

    // Create a simple registry
    let mut simple = TestEnumOtherRegistry::new_from_default(&default);

    // Check that new simple registry is the same as the default
    {
        assert_eq!(simple.get_id(&TestEnumOther::First), Some(0));
        assert_eq!(simple.get_id(&TestEnumOther::Second), Some(1));
        assert_eq!(simple.get_id(&TestEnumOther::Third), Some(2));
        assert_eq!(simple.get_id(&TestEnumOther::Fourth), None);
        assert_eq!(
            simple.get_id(&TestEnumOther::Other(ResourceKey::new("froglight:fifth"))),
            Some(3)
        );

        assert_eq!(simple.get_value_cloned(0), Some(TestEnumOther::First));
        assert_eq!(simple.get_value_cloned(1), Some(TestEnumOther::Second));
        assert_eq!(simple.get_value_cloned(2), Some(TestEnumOther::Third));
        assert_eq!(
            simple.get_value_cloned(3),
            Some(TestEnumOther::Other(ResourceKey::new("froglight:fifth")))
        );
    }

    // Push a new value into the simple registry
    {
        simple.push_value(TestEnumOther::Fourth);

        // Make sure the value exists in the simple registry
        assert_eq!(simple.get_id(&TestEnumOther::Fourth), Some(4));
        assert_eq!(simple.get_value_cloned(4), Some(TestEnumOther::Fourth));

        // Make sure the value does not exist in the default registry
        assert_eq!(default.get_id(&TestEnumOther::Fourth), None);
        assert_eq!(default.get_value_cloned(4), None);
    }

    // Reset the simple registry
    {
        simple.overwrite_with(&default);

        // Make sure the simple registry is the same as the default
        assert_eq!(simple.get_id(&TestEnumOther::First), Some(0));
        assert_eq!(simple.get_id(&TestEnumOther::Second), Some(1));
        assert_eq!(simple.get_id(&TestEnumOther::Third), Some(2));
        assert_eq!(simple.get_id(&TestEnumOther::Fourth), None);
        assert_eq!(
            simple.get_id(&TestEnumOther::Other(ResourceKey::new("froglight:fifth"))),
            Some(3)
        );

        assert_eq!(simple.get_value_cloned(0), Some(TestEnumOther::First));
        assert_eq!(simple.get_value_cloned(1), Some(TestEnumOther::Second));
        assert_eq!(simple.get_value_cloned(2), Some(TestEnumOther::Third));
        assert_eq!(
            simple.get_value_cloned(3),
            Some(TestEnumOther::Other(ResourceKey::new("froglight:fifth")))
        );
    }
}
