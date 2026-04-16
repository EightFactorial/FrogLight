//! TODO

facet::define_attr_grammar! {
    ns "frog";
    crate_path ::froglight_facet::facet;

    /// Attributes for customizing serialization and deserialization.
    #[derive(Copy)]
    pub enum Attr {
        /// Mark a field as using variable-length encoding.
        ///
        /// See the [Minecraft Wiki](https://minecraft.wiki/w/Java_Edition_protocol/VarInt_and_VarLong)
        /// or [Wikipedia](https://en.wikipedia.org/wiki/LEB128) for more details.
        Variable,
    }
}
