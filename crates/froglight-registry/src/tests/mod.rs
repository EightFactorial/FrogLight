use froglight_protocol::traits::Version;

mod simple_registry;
mod simple_registry_other;

/// A test [`Version`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct TestVersion;

impl Version for TestVersion {
    const ID: i32 = 0;
}
