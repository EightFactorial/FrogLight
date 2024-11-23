use std::any::TypeId;

use derive_more::derive::Into;

use super::{BlockAttribute, ResolvableAttributes};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Into)]
pub struct BooleanAttribute(bool);
impl From<BooleanAttribute> for usize {
    fn from(attr: BooleanAttribute) -> usize { usize::from(!attr.0) }
}
impl BlockAttribute for BooleanAttribute {
    const STATES: &'static [Self] = &[Self(true), Self(false)];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnumAttribute {
    A,
    B,
    C,
}
impl From<EnumAttribute> for usize {
    fn from(attr: EnumAttribute) -> usize {
        match attr {
            EnumAttribute::A => 0,
            EnumAttribute::B => 1,
            EnumAttribute::C => 2,
        }
    }
}
impl BlockAttribute for EnumAttribute {
    const STATES: &'static [Self] = &[Self::A, Self::B, Self::C];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LargerEnumAttribute {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}
impl From<LargerEnumAttribute> for usize {
    fn from(attr: LargerEnumAttribute) -> usize {
        match attr {
            LargerEnumAttribute::A => 0,
            LargerEnumAttribute::B => 1,
            LargerEnumAttribute::C => 2,
            LargerEnumAttribute::D => 3,
            LargerEnumAttribute::E => 4,
            LargerEnumAttribute::F => 5,
            LargerEnumAttribute::G => 6,
            LargerEnumAttribute::H => 7,
            LargerEnumAttribute::I => 8,
            LargerEnumAttribute::J => 9,
            LargerEnumAttribute::K => 10,
            LargerEnumAttribute::L => 11,
            LargerEnumAttribute::M => 12,
            LargerEnumAttribute::N => 13,
            LargerEnumAttribute::O => 14,
            LargerEnumAttribute::P => 15,
            LargerEnumAttribute::Q => 16,
            LargerEnumAttribute::R => 17,
            LargerEnumAttribute::S => 18,
            LargerEnumAttribute::T => 19,
            LargerEnumAttribute::U => 20,
            LargerEnumAttribute::V => 21,
            LargerEnumAttribute::W => 22,
            LargerEnumAttribute::X => 23,
            LargerEnumAttribute::Y => 24,
            LargerEnumAttribute::Z => 25,
        }
    }
}
impl BlockAttribute for LargerEnumAttribute {
    const STATES: &'static [Self] = &[
        Self::A,
        Self::B,
        Self::C,
        Self::D,
        Self::E,
        Self::F,
        Self::G,
        Self::H,
        Self::I,
        Self::J,
        Self::K,
        Self::L,
        Self::M,
        Self::N,
        Self::O,
        Self::P,
        Self::Q,
        Self::R,
        Self::S,
        Self::T,
        Self::U,
        Self::V,
        Self::W,
        Self::X,
        Self::Y,
        Self::Z,
    ];
}

#[test]
fn attribute() {
    // Test individual attributes
    {
        assert_eq!(BooleanAttribute::from_index(0), Some(BooleanAttribute(true)));
        assert_eq!(BooleanAttribute::from_index(1), Some(BooleanAttribute(false)));
        assert_eq!(BooleanAttribute::from_index(2), None);

        assert_eq!(BooleanAttribute::from_index(0).unwrap().to_index(), 0);
        assert_eq!(BooleanAttribute::from_index(1).unwrap().to_index(), 1);
        assert_eq!(BooleanAttribute::from_index(2), None);
    }
    {
        assert_eq!(EnumAttribute::from_index(0), Some(EnumAttribute::A));
        assert_eq!(EnumAttribute::from_index(1), Some(EnumAttribute::B));
        assert_eq!(EnumAttribute::from_index(2), Some(EnumAttribute::C));
        assert_eq!(EnumAttribute::from_index(3), None);

        assert_eq!(EnumAttribute::from_index(0).unwrap().to_index(), 0);
        assert_eq!(EnumAttribute::from_index(1).unwrap().to_index(), 1);
        assert_eq!(EnumAttribute::from_index(2).unwrap().to_index(), 2);
        assert_eq!(EnumAttribute::from_index(3), None);
    }

    // Test attribute tuples
    {
        type Attributes = (BooleanAttribute,);
        assert_eq!(Attributes::TYPES, &[TypeId::of::<BooleanAttribute>()]);
        assert_eq!(Attributes::STATE_COUNT, 2);

        assert_eq!(Attributes::from_index(0), Some((BooleanAttribute(true),)));
        assert_eq!(Attributes::from_index(1), Some((BooleanAttribute(false),)));
        assert_eq!(Attributes::from_index(2), None);

        assert_eq!(Attributes::from_index(0).unwrap().to_index(), 0);
        assert_eq!(Attributes::from_index(1).unwrap().to_index(), 1);
        assert_eq!(Attributes::from_index(2), None);
    }
    {
        type Attributes = (BooleanAttribute, EnumAttribute);
        assert_eq!(
            Attributes::TYPES,
            &[TypeId::of::<BooleanAttribute>(), TypeId::of::<EnumAttribute>()]
        );
        assert_eq!(Attributes::STATE_COUNT, 6);

        assert_eq!(Attributes::from_index(0), Some((BooleanAttribute(true), EnumAttribute::A)));
        assert_eq!(Attributes::from_index(1), Some((BooleanAttribute(true), EnumAttribute::B)));
        assert_eq!(Attributes::from_index(2), Some((BooleanAttribute(true), EnumAttribute::C)));
        assert_eq!(Attributes::from_index(3), Some((BooleanAttribute(false), EnumAttribute::A)));
        assert_eq!(Attributes::from_index(4), Some((BooleanAttribute(false), EnumAttribute::B)));
        assert_eq!(Attributes::from_index(5), Some((BooleanAttribute(false), EnumAttribute::C)));
        assert_eq!(Attributes::from_index(6), None);

        assert_eq!(Attributes::from_index(0).unwrap().to_index(), 0);
        assert_eq!(Attributes::from_index(1).unwrap().to_index(), 1);
        assert_eq!(Attributes::from_index(2).unwrap().to_index(), 2);
        assert_eq!(Attributes::from_index(3).unwrap().to_index(), 3);
        assert_eq!(Attributes::from_index(4).unwrap().to_index(), 4);
        assert_eq!(Attributes::from_index(5).unwrap().to_index(), 5);
        assert_eq!(Attributes::from_index(6), None);
    }
    {
        type Attributes = (BooleanAttribute, EnumAttribute, LargerEnumAttribute);
        assert_eq!(
            Attributes::TYPES,
            &[
                TypeId::of::<BooleanAttribute>(),
                TypeId::of::<EnumAttribute>(),
                TypeId::of::<LargerEnumAttribute>()
            ]
        );
        assert_eq!(Attributes::STATE_COUNT, 156);

        // Test [true, A, A] to [true, A, Z]
        for (index, attr) in LargerEnumAttribute::STATES.iter().enumerate() {
            let attrs = Attributes::from_index(index).unwrap();
            assert_eq!(attrs, (BooleanAttribute(true), EnumAttribute::A, *attr));
            assert_eq!(attrs.to_index(), index);
        }

        // Test [true, B, A] to [true, B, Z]
        for (index, attr) in LargerEnumAttribute::STATES
            .iter()
            .enumerate()
            .map(|(i, a)| (i + <LargerEnumAttribute as ResolvableAttributes>::STATE_COUNT, a))
        {
            let attrs = Attributes::from_index(index).unwrap();
            assert_eq!(attrs, (BooleanAttribute(true), EnumAttribute::B, *attr));
            assert_eq!(attrs.to_index(), index);
        }

        // Test [false, A, A] to [false, A, Z]
        for (index, attr) in LargerEnumAttribute::STATES.iter().enumerate().map(|(i, a)| {
            (
                i + <EnumAttribute as ResolvableAttributes>::STATE_COUNT
                    * <LargerEnumAttribute as ResolvableAttributes>::STATE_COUNT,
                a,
            )
        }) {
            let attrs = Attributes::from_index(index).unwrap();
            assert_eq!(attrs, (BooleanAttribute(false), EnumAttribute::A, *attr));
            assert_eq!(attrs.to_index(), index);
        }

        // Test [false, B, A] to [false, B, Z]
        for (index, attr) in LargerEnumAttribute::STATES.iter().enumerate().map(|(i, a)| {
            (
                i + <EnumAttribute as ResolvableAttributes>::STATE_COUNT
                    * <LargerEnumAttribute as ResolvableAttributes>::STATE_COUNT
                    + <LargerEnumAttribute as ResolvableAttributes>::STATE_COUNT,
                a,
            )
        }) {
            let attrs = Attributes::from_index(index).unwrap();
            assert_eq!(attrs, (BooleanAttribute(false), EnumAttribute::B, *attr));
            assert_eq!(attrs.to_index(), index);
        }
    }
}
