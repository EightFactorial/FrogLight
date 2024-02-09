use std::{
    fmt::{Debug, Display, Formatter},
    num::Wrapping,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use bevy_reflect::Reflect;
use derive_more::{Deref, DerefMut};

use crate::io::{FrogRead, FrogVarRead, FrogVarWrite, FrogWrite, ReadError, WriteError};

/// `NonZero` is a wrapper that writes `None` as `0` and `Some` as `1 +
/// value`.
///
/// # Examples
/// ```rust
/// use froglight_protocol::{data::NonZero, io::FrogWrite};
///
/// let nonzero = NonZero::<u8>::new(None);
/// assert_eq!(nonzero.fg_to_bytes(), vec![0u8]);
///
/// let nonzero = NonZero::new_some(0u8);
/// assert_eq!(nonzero.fg_to_bytes(), vec![1u8]);
///
/// let nonzero = NonZero::new_some(5u8);
/// assert_eq!(nonzero.fg_to_bytes(), vec![6u8]);
///
/// let nonzero = NonZero::new_some(0u32);
/// assert_eq!(nonzero.fg_to_bytes(), vec![0u8, 0u8, 0u8, 1u8]);
///
/// let nonzero = NonZero::new_some(5u32);
/// assert_eq!(nonzero.fg_to_bytes(), vec![0u8, 0u8, 0u8, 6u8]);
/// ```
///
/// # Warning
/// Be careful when using values that are close to the minimum or maximum
/// value of the inner type. For example, if the inner type is `u8`, then
/// `NonZero::new_some(255u8)` will overflow and be written as `0`.
#[derive(Deref, DerefMut, Reflect)]
pub struct NonZero<T>(Option<T>);

impl<T> NonZero<T> {
    /// Creates a new `NonZero` with the given [Option].
    ///
    /// # Examples
    /// ```rust
    /// use froglight_protocol::data::NonZero;
    ///
    /// let nonzero = NonZero::new(Some(5u8));
    /// assert_eq!(nonzero.into_inner(), Some(5u8));
    /// ```
    #[must_use]
    pub const fn new(val: Option<T>) -> Self { Self(val) }

    /// Creates a new `NonZero` with `Some(val)`.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_protocol::data::NonZero;
    ///
    /// let nonzero = NonZero::new_some(5u8);
    /// assert_eq!(nonzero.into_inner(), Some(5u8));
    /// ```
    #[must_use]
    pub const fn new_some(val: T) -> Self { Self(Some(val)) }

    /// Creates a new `NonZero` with `None`.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_protocol::data::NonZero;
    ///
    /// let nonzero = NonZero::<u8>::new_none();
    /// assert_eq!(nonzero.into_inner(), None);
    /// ```
    #[must_use]
    pub const fn new_none() -> Self { Self(None) }

    /// Returns the inner [Option].
    ///
    /// ### Examples
    /// ```rust
    /// use froglight_protocol::data::NonZero;
    ///
    /// let nonzero = NonZero::new_some(5u8);
    /// assert_eq!(nonzero.into_inner(), Some(5u8));
    /// ```
    #[must_use]
    pub fn into_inner(self) -> Option<T> { self.0 }

    /// Similar to [`Default::default`], but uses the
    /// inner type's [Default] instead of [None].
    ///
    /// ### Examples
    /// ```rust
    /// use froglight_protocol::data::NonZero;
    ///
    /// let nonzero = NonZero::<u8>::default_some();
    /// assert_eq!(nonzero.into_inner(), Some(0u8));
    ///
    /// let nonzero = NonZero::<u32>::default_some();
    /// assert_eq!(nonzero.into_inner(), Some(0u32));
    /// ```
    #[must_use]
    pub fn default_some() -> Self
    where
        T: Default,
    {
        Self::new_some(T::default())
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

#[allow(clippy::expl_impl_clone_on_copy)]
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

impl<T: FrogWrite + Copy + PartialEq + Add<Output = T> + From<u8>> FrogWrite for NonZero<T>
where
    Wrapping<T>: AddAssign,
{
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        match &self.0 {
            Some(val) => {
                let mut val = Wrapping(*val);
                val += Wrapping(T::from(1));
                val.0.fg_write(buf)
            }
            None => T::from(0).fg_write(buf),
        }
    }
}

impl<T: FrogVarWrite + Copy + PartialEq + Add<Output = T> + From<u8>> FrogVarWrite for NonZero<T>
where
    Wrapping<T>: AddAssign,
{
    fn fg_var_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        match &self.0 {
            Some(val) => {
                let mut val = Wrapping(*val);
                val += Wrapping(T::from(1));
                val.0.fg_var_write(buf)
            }
            None => T::from(0).fg_var_write(buf),
        }
    }
}

impl<T: FrogRead + PartialEq + Sub<Output = T> + From<u8>> FrogRead for NonZero<T>
where
    Wrapping<T>: SubAssign,
{
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError> {
        let val = T::fg_read(buf)?;

        if val == T::from(0) {
            Ok(Self(None))
        } else {
            let mut val = Wrapping(val);
            val -= Wrapping(T::from(1));

            Ok(Self(Some(val.0)))
        }
    }
}

impl<T: FrogVarRead + PartialEq + Sub<Output = T> + From<u8>> FrogVarRead for NonZero<T>
where
    Wrapping<T>: SubAssign,
{
    fn fg_var_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError> {
        let val = T::fg_var_read(buf)?;

        if val == T::from(0) {
            Ok(Self(None))
        } else {
            let mut val = Wrapping(val);
            val -= Wrapping(T::from(1));

            Ok(Self(Some(val.0)))
        }
    }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(256))]

    #[test]
    fn nonzero_u8_read(val in 0u8..u8::MAX) {
        let mut cursor = std::io::Cursor::new(core::slice::from_ref(&val));
        let nonzero = NonZero::fg_read(&mut cursor).unwrap();

        if val == 0 {
            assert_eq!(nonzero.into_inner(), None);
        } else {
            assert_eq!(nonzero.into_inner(), Some(val.wrapping_sub(1)));
        }

    }

    #[test]
    fn nonzero_u8_write(val in 1u8..=u8::MAX) {
        let nonzero = NonZero::new_some(val);
        let buf = nonzero.fg_to_bytes();

        if val == 0 {
            assert_eq!(buf, vec![0]);
        } else {
            assert_eq!(buf, vec![val.wrapping_add(1)]);
        }
    }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(1024))]

    #[test]
    fn nonzero_u32_read(val in 0u32..u32::MAX) {
        let bytes = val.fg_to_bytes();
        let mut cursor = std::io::Cursor::new(bytes.as_slice());
        let nonzero = NonZero::fg_read(&mut cursor).unwrap();

        if val == 0 {
            assert_eq!(nonzero.into_inner(), None);
        } else {
            assert_eq!(nonzero.into_inner(), Some(val.wrapping_sub(1)));
        }
    }

    #[test]
    fn nonzero_u32_write(val in 1u32..=u32::MAX) {
        let nonzero = NonZero::new_some(val);
        let buf = nonzero.fg_to_bytes();

        if val == 0 {
            assert_eq!(buf, vec![0, 0, 0, 1]);
        } else {
            assert_eq!(buf, val.wrapping_add(1).fg_to_bytes());
        }
    }

    #[test]
    fn nonzero_u32_var_read(val in 0u32..u32::MAX) {
        let bytes = val.fg_var_to_bytes();
        let mut cursor = std::io::Cursor::new(bytes.as_slice());
        let nonzero = NonZero::fg_var_read(&mut cursor).unwrap();

        if val == 0 {
            assert_eq!(nonzero.into_inner(), None);
        } else {
            assert_eq!(nonzero.into_inner(), Some(val.wrapping_sub(1)));
        }
    }

    #[test]
    fn nonzero_u32_var_write(val in 1u32..=u32::MAX) {
        let nonzero = NonZero::new_some(val);
        let buf = nonzero.fg_var_to_bytes();

        if val == 0 {
            assert_eq!(buf, vec![1]);
        } else {
            assert_eq!(buf, val.wrapping_add(1).fg_var_to_bytes());
        }
    }
}
