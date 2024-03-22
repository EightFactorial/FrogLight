/// Trait for reading and writing integers of a specific size.
pub(super) trait IntegerSize: Sized {
    const BYTES: usize;
}
