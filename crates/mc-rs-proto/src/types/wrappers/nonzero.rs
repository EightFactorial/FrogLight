use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Add, Sub},
};

use derive_more::{Deref, DerefMut};

use crate::buffer::{Decode, DecodeError, Encode, EncodeError, VarDecode, VarEncode};

/// `NonZero` is a wrapper that encodes `None` as 0 and `Some` as `1 + value`.
///
/// ### Examples
/// ```rust
/// use mc_rs_proto::{buffer::FromValue, types::NonZero};
///
/// // A u32 is encoded as 4 bytes.
/// let nonzero_none: NonZero<u32> = NonZero::new(None);
/// assert_eq!(&Vec::from_value(&nonzero_none).unwrap(), &[0, 0, 0, 0]);
/// assert_eq!(&Vec::from_var_value(&nonzero_none).unwrap(), &[0]);
///
/// let nonzero_0u32: NonZero<u32> = NonZero::new(Some(0));
/// assert_eq!(&Vec::from_value(&nonzero_0u32).unwrap(), &[0, 0, 0, 1]);
/// assert_eq!(&Vec::from_var_value(&nonzero_0u32).unwrap(), &[1]);
///
/// let nonzero_1u32: NonZero<u32> = NonZero::new(Some(1));
/// assert_eq!(&Vec::from_value(&nonzero_1u32).unwrap(), &[0, 0, 0, 2]);
/// assert_eq!(&Vec::from_var_value(&nonzero_1u32).unwrap(), &[2]);
///
/// // An option is encoded as a zero if it's None,
/// // or a 1 followed by the value if it's Some.
/// let option_none: Option<NonZero<u16>> = None;
/// assert_eq!(&Vec::from_value(&option_none).unwrap(), &[0]);
/// assert_eq!(&Vec::from_var_value(&option_none).unwrap(), &[0]);
///
/// // A u16 is encoded as 2 bytes.
/// let option_16u16: Option<NonZero<u16>> = Some(NonZero::new_from(16));
/// assert_eq!(&Vec::from_value(&option_16u16).unwrap(), &[1, 0, 17]);
/// assert_eq!(&Vec::from_var_value(&option_16u16).unwrap(), &[1, 17]);
/// ```
#[derive(Deref, DerefMut)]
pub struct NonZero<T>(Option<T>);

impl<T> NonZero<T> {
    /// Creates a new `NonZero` with the given [Option].
    pub const fn new(val: Option<T>) -> Self { Self(val) }

    /// Creates a new `NonZero` with `Some(val)`.
    ///
    /// ### Examples
    /// ```rust
    /// use mc_rs_proto::types::NonZero;
    ///
    /// let nonzero_0u32: NonZero<u32> = NonZero::new_from(0);
    /// assert_eq!(nonzero_0u32.into_inner(), Some(0));
    ///
    /// let nonzero_255u8: NonZero<u8> = NonZero::new_from(255);
    /// assert_eq!(nonzero_255u8.into_inner(), Some(255));
    /// ```
    pub const fn new_from(val: T) -> Self { Self(Some(val)) }

    /// Returns the inner [Option].
    ///
    /// ### Examples
    /// ```rust
    /// use mc_rs_proto::types::NonZero;
    ///
    /// let nonzero_none: NonZero<u32> = NonZero::new(None);
    /// assert_eq!(nonzero_none.into_inner(), None);
    ///
    /// let nonzero_0u32: NonZero<u32> = NonZero::new_from(0);
    /// assert_eq!(nonzero_0u32.into_inner(), Some(0));
    ///
    /// let nonzero_255u8: NonZero<u8> = NonZero::new_from(255);
    /// assert_eq!(nonzero_255u8.into_inner(), Some(255));
    /// ```
    pub fn into_inner(self) -> Option<T> { self.0 }

    /// Similar to [Default::default], but uses the
    /// inner type's [Default] instead of [None].
    ///
    /// ### Examples
    /// ```rust
    /// use mc_rs_proto::types::NonZero;
    ///
    /// let nonzero_none: NonZero<u32> = NonZero::default();
    /// assert_eq!(nonzero_none.into_inner(), None);
    ///
    /// let nonzero_default: NonZero<u32> = NonZero::default_some();
    /// assert_eq!(nonzero_default.into_inner(), Some(0));
    /// ```
    pub fn default_some() -> Self
    where
        T: Default,
    {
        Self::new_from(T::default())
    }
}

impl<T> Default for NonZero<T> {
    fn default() -> Self { Self(None) }
}

impl<T: Debug> Debug for NonZero<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(val) => write!(f, "NonZero({:?})", val),
            None => write!(f, "NonZero(None)"),
        }
    }
}

impl<T: Display> Display for NonZero<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "None"),
        }
    }
}

impl<T: Clone> Clone for NonZero<T> {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl<T: Copy> Copy for NonZero<T> {}

impl<T: PartialEq> PartialEq for NonZero<T> {
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}

impl<T: Eq> Eq for NonZero<T> {}

impl<T> From<T> for NonZero<T> {
    fn from(val: T) -> Self { Self(Some(val)) }
}

impl<T> From<Option<T>> for NonZero<T> {
    fn from(val: Option<T>) -> Self { Self(val) }
}

impl<T> From<NonZero<T>> for Option<T> {
    fn from(val: NonZero<T>) -> Self { val.0 }
}

impl<T: Encode + Clone + PartialEq + Add<Output = T> + From<u8>> Encode for NonZero<T> {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        match &self.0 {
            Some(val) => val.clone().add(T::from(1)).encode(buf),
            None => 0u32.encode(buf),
        }
    }
}

impl<T: VarEncode + Clone + PartialEq + Add<Output = T> + From<u8>> VarEncode for NonZero<T> {
    fn var_encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        match &self.0 {
            Some(val) => val.clone().add(T::from(1)).var_encode(buf),
            None => 0u32.var_encode(buf),
        }
    }
}

impl<T: Decode + PartialEq + Sub<Output = T> + From<u8>> Decode for NonZero<T> {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let val = T::decode(buf)?;

        if val == T::from(0) {
            Ok(Self(None))
        } else {
            Ok(Self(Some(val.sub(T::from(1)))))
        }
    }
}

impl<T: VarDecode + PartialEq + Sub<Output = T> + From<u8>> VarDecode for NonZero<T> {
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
fn nonzero_u8() {
    let numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut buf: Vec<u8> = Vec::with_capacity(10);

    // Turn all the numbers into NonZeros and encode them
    for num in numbers.iter() {
        assert!(NonZero::<u8>::new_from(*num).encode(&mut buf).is_ok());
    }

    // Decode them back into the same numbers
    let mut cursor = std::io::Cursor::new(buf);
    for num in numbers.iter() {
        assert_eq!(NonZero::<u8>::decode(&mut cursor).unwrap().unwrap(), *num);
    }
}

#[test]
fn nonzero_u32() {
    let numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut buf: Vec<u8> = Vec::with_capacity(40);

    // Turn all the numbers into NonZeros and encode them
    for num in numbers.iter() {
        assert!(NonZero::<u32>::new_from(*num).encode(&mut buf).is_ok());
    }

    // Decode them back into the same numbers
    let mut cursor = std::io::Cursor::new(buf);
    for num in numbers.iter() {
        assert_eq!(NonZero::<u32>::decode(&mut cursor).unwrap().unwrap(), *num);
    }
}

#[test]
fn nonzero_var_u32() {
    let numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut buf: Vec<u8> = Vec::with_capacity(10);

    // Turn all the numbers into NonZeros and encode them
    for num in numbers.iter() {
        assert!(NonZero::<u32>::new_from(*num).var_encode(&mut buf).is_ok());
    }

    // Decode them back into the same numbers
    let mut cursor = std::io::Cursor::new(buf);
    for num in numbers.iter() {
        assert_eq!(
            NonZero::<u32>::var_decode(&mut cursor).unwrap().unwrap(),
            *num
        );
    }
}
