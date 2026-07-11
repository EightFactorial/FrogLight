#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextInteraction<'a> {
    phantom: core::marker::PhantomData<&'a ()>,
}
