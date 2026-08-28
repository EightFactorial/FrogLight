//! TODO

facet::define_attr_grammar! {
    ns "snbt";
    crate_path ::froglight_snbt::facet::attr;

    /// Attributes for customizing serialization and deserialization.
    #[derive(Copy)]
    pub enum Attr {
        /// TODO
        Tag(&'static str),
    }
}

// -------------------------------------------------------------------------------------------------
