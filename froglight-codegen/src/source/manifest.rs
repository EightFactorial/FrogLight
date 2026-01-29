use std::sync::LazyLock;

/// A version manifest, containing information about all available versions.
pub struct Manifest {}

static MANIFEST: LazyLock<Manifest> = LazyLock::new(|| todo!());

impl Manifest {
    /// Get the global [`Manifest`].
    pub fn get() -> &'static Manifest { &MANIFEST }
}
