#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextFormatting<'a> {
    phantom: core::marker::PhantomData<&'a ()>,
}
