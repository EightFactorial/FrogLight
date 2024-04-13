use froglight_protocol::traits::Version;

mod block_registry;
mod id_registry;

/// A test [`Version`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct TestVersion;

impl Version for TestVersion {
    const ID: i32 = 0;
}
