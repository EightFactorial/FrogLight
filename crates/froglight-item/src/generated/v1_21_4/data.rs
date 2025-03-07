use froglight_macros::FrogNbt;

/// Example
#[derive(FrogNbt)]
pub struct ExampleAttributeData {
    /// Example
    #[frog(ident = "froglight:value", tag = "string")]
    pub value: String,
}
