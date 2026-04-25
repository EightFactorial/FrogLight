//! TODO
#![no_std]

use facet::{Facet, Partial, ReflectError};
use froglight_facet::{
    self as mc,
    facet::FacetTemplate,
    format::{Reader, Writer, WriterError, serialize::SerializeItem},
    to_vec,
};

#[test]
fn variable() {
    #[derive(Facet)]
    struct Variable(#[facet(mc::variable)] u32);

    // Check that `Variable(42)` was serialized correctly.
    let serialized = to_vec(&Variable(42)).unwrap();
    assert_eq!(serialized, [42]);
}

#[test]
fn variable_inner() {
    // Check that `#[facet(mc::variable_inner)]` types pass
    // `#[facet(mc::variable)]` to inner fields.

    #[derive(Facet)]
    struct Outer {
        first: Inner,
        #[facet(mc::variable)]
        second: Ignored,
        #[facet(mc::variable)]
        third: Inner,
    }

    #[derive(Facet)]
    #[facet(mc::variable_inner)]
    struct Inner(u32);

    #[derive(Facet)]
    struct Ignored(u32);

    // Check that `outer` was serialized correctly.
    let outer = Outer { first: Inner(100), second: Ignored(100), third: Inner(100) };
    let serialized = to_vec(&outer).unwrap();
    assert_eq!(serialized, [100, 0, 0, 0, 100, 0, 0, 0, 100]);
}

#[test]
fn template() {
    // Check that `#[facet(mc::with = ...)]` works on types.

    #[derive(Facet)]
    #[facet(mc::with = Templated::WITH)]
    struct Templated(u32);

    impl FacetTemplate for Templated {
        fn serialize(
            item: SerializeItem<'_, '_>,
            writer: &mut Writer<'_>,
        ) -> Result<(), WriterError> {
            let val = item.peek().get::<Templated>().unwrap();
            writer.write_bytes(&val.0.to_be_bytes())
        }

        fn deserialize(
            _partial: Partial<'static, false>,
            _reader: &mut Reader<'_>,
        ) -> Result<Partial<'static, false>, ReflectError> {
            todo!()
        }
    }

    // Check that `Templated(42)` was serialized correctly.
    let serialized = to_vec(&Templated(42)).unwrap();
    assert_eq!(serialized, [0, 0, 0, 42]);
}

#[test]
fn template_field() {
    // Check that `#[facet(mc::with = ...)]` works on fields.

    #[derive(Facet)]
    struct Templated(#[facet(mc::with = Templated::WITH)] u32);

    impl FacetTemplate for Templated {
        fn serialize(
            item: SerializeItem<'_, '_>,
            writer: &mut Writer<'_>,
        ) -> Result<(), WriterError> {
            let val = item.peek().get::<u32>().unwrap();
            writer.write_bytes(&val.to_be_bytes())
        }

        fn deserialize(
            _partial: Partial<'static, false>,
            _reader: &mut Reader<'_>,
        ) -> Result<Partial<'static, false>, ReflectError> {
            todo!()
        }
    }

    // Check that `Templated(42)` was serialized correctly.
    let serialized = to_vec(&Templated(42)).unwrap();
    assert_eq!(serialized, [0, 0, 0, 42]);
}
