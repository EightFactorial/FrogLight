use std::{
    fmt::{Debug, Formatter},
    ops::{Add, Sub},
};

use derive_more::{Deref, DerefMut};

use crate::buffer::{Decode, DecodeError, Encode, EncodeError, VarDecode, VarEncode};

/// `NonZeroOption` is a wrapper that encodes `None` as 0 and `Some` as `1 + value`.
///
/// For example:
/// ```text
/// None -> [0u8]
/// Some(0u8) -> [1u8]
/// Some(1u8) -> [2u8]
/// Some(2u32) -> [0u8, 0u8, 0u8, 3u8]
/// #[var] Some(3u32) -> [4u8]
/// ```
///
/// etc.
#[derive(Deref, DerefMut)]
pub struct NonZeroOption<T>(Option<T>);

impl<T> NonZeroOption<T> {
    /// Creates a new `NonZeroOption` with the given [Option].
    pub fn new(val: Option<T>) -> Self { Self(val) }

    /// Creates a new `NonZeroOption` with `Some(val)`.
    pub fn new_with(val: T) -> Self { Self(Some(val)) }
}

impl<T> Default for NonZeroOption<T> {
    fn default() -> Self { Self(None) }
}

impl<T: Debug> Debug for NonZeroOption<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(val) => write!(f, "NonZeroOption({:?})", val),
            None => write!(f, "NonZeroOption(None)"),
        }
    }
}

impl<T: Clone> Clone for NonZeroOption<T> {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl<T: Copy> Copy for NonZeroOption<T> {}

impl<T: PartialEq> PartialEq for NonZeroOption<T> {
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}

impl<T: Eq> Eq for NonZeroOption<T> {}

impl<T> From<T> for NonZeroOption<T> {
    fn from(val: T) -> Self { Self(Some(val)) }
}

impl<T> From<Option<T>> for NonZeroOption<T> {
    fn from(val: Option<T>) -> Self { Self(val) }
}

impl<T> From<NonZeroOption<T>> for Option<T> {
    fn from(val: NonZeroOption<T>) -> Self { val.0 }
}

impl<T: Encode + Clone + PartialEq + Add<Output = T> + From<u8>> Encode for NonZeroOption<T> {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        match &self.0 {
            Some(val) => val.clone().add(T::from(1)).encode(buf),
            None => 0u32.var_encode(buf).map_err(EncodeError::from),
        }
    }
}

impl<T: VarEncode + Clone + PartialEq + Add<Output = T> + From<u8>> VarEncode for NonZeroOption<T> {
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        match &self.0 {
            Some(val) => val.clone().add(T::from(1)).var_encode(buf),
            None => 0u32.var_encode(buf),
        }
    }
}

impl<T: Decode + PartialEq + Sub<Output = T> + From<u8>> Decode for NonZeroOption<T> {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let val = T::decode(buf)?;

        if val == T::from(0) {
            Ok(Self(None))
        } else {
            Ok(Self(Some(val.sub(T::from(1)))))
        }
    }
}

impl<T: VarDecode + PartialEq + Sub<Output = T> + From<u8>> VarDecode for NonZeroOption<T> {
    fn var_decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let val = T::var_decode(buf)?;

        if val == T::from(0) {
            Ok(Self(None))
        } else {
            Ok(Self(Some(val.sub(T::from(1)))))
        }
    }
}

#[test]
fn nonzero_option_u8() {
    let numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut buf: Vec<u8> = Vec::with_capacity(10);

    // Turn all the numbers into NonZeroOptions and encode them
    for num in numbers.iter() {
        assert!(NonZeroOption::<u8>::new_with(*num).encode(&mut buf).is_ok());
    }

    // Decode them back into the same numbers
    let mut cursor = std::io::Cursor::new(buf);
    for num in numbers.iter() {
        assert_eq!(
            NonZeroOption::<u8>::decode(&mut cursor).unwrap().unwrap(),
            *num
        );
    }
}

#[test]
fn nonzero_option_u32() {
    let numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut buf: Vec<u8> = Vec::with_capacity(40);

    // Turn all the numbers into NonZeroOptions and encode them
    for num in numbers.iter() {
        assert!(NonZeroOption::<u32>::new_with(*num)
            .encode(&mut buf)
            .is_ok());
    }

    // Decode them back into the same numbers
    let mut cursor = std::io::Cursor::new(buf);
    for num in numbers.iter() {
        assert_eq!(
            NonZeroOption::<u32>::decode(&mut cursor).unwrap().unwrap(),
            *num
        );
    }
}

#[test]
fn nonzero_option_var_u32() {
    let numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut buf: Vec<u8> = Vec::with_capacity(10);

    // Turn all the numbers into NonZeroOptions and encode them
    for num in numbers.iter() {
        assert!(NonZeroOption::<u32>::new_with(*num)
            .var_encode(&mut buf)
            .is_ok());
    }

    // Decode them back into the same numbers
    let mut cursor = std::io::Cursor::new(buf);
    for num in numbers.iter() {
        assert_eq!(
            NonZeroOption::<u32>::var_decode(&mut cursor)
                .unwrap()
                .unwrap(),
            *num
        );
    }
}
