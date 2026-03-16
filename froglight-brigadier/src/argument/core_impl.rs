use core::num::{
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroU8, NonZeroU16, NonZeroU32,
    NonZeroU64, NonZeroU128,
};

use super::{ArgumentParseError, ArgumentParser};

impl ArgumentParser for bool {
    type Data = ();

    fn parse<'a>(input: &'a str, (): &()) -> Result<(Self, &'a str), ArgumentParseError> {
        if let Some(rest) = input.strip_prefix("true") {
            Ok((true, rest))
        } else if let Some(rest) = input.strip_prefix("false") {
            Ok((false, rest))
        } else {
            Err(ArgumentParseError::InputMismatch)
        }
    }
}

// -------------------------------------------------------------------------------------------------

macro_rules! impl_integer {
    ($($ty:ty),*) => {
        $(
            impl ArgumentParser for $ty {
                type Data = ();
                fn parse<'a>(input: &'a str, (): &()) -> Result<(Self, &'a str), ArgumentParseError> {
                    match lexical::parse_partial::<$ty, _>(input) {
                        Ok((value, length)) => Ok((value, &input[length..])),
                        Err(lexical::Error::InvalidDigit(_)) => Err(ArgumentParseError::InputMismatch),
                        Err(err) => Err(ArgumentParseError::other(err)),
                    }
                }
            }
        )*
    };
    (@nonzero $($ty:ty: $inner:ty),*) => {
        $(
            impl ArgumentParser for $ty {
                type Data = ();
                fn parse<'a>(input: &'a str, (): &()) -> Result<(Self, &'a str), ArgumentParseError> {
                    match lexical::parse_partial::<$inner, _>(input) {
                        Ok((value, length)) => match <$ty>::new(value) {
                            Some(value) => Ok((value, &input[length..])),
                            None => Err(ArgumentParseError::InputInvalid),
                        },
                        Err(lexical::Error::InvalidDigit(_)) => Err(ArgumentParseError::InputMismatch),
                        Err(err) => Err(ArgumentParseError::other(err)),
                    }
                }
            }
        )*
    };
}

impl_integer!(u8, u16, u32, u64, u128, usize);
impl_integer!(i8, i16, i32, i64, i128, isize);

impl_integer!(@nonzero NonZeroU8: u8, NonZeroU16: u16, NonZeroU32: u32, NonZeroU64: u64, NonZeroU128: u128);
impl_integer!(@nonzero NonZeroI8: i8, NonZeroI16: i16, NonZeroI32: i32, NonZeroI64: i64, NonZeroI128: i128);

// -------------------------------------------------------------------------------------------------

macro_rules! impl_float {
    ($($ty:ty),*) => {
        $(
            impl ArgumentParser for $ty {
                type Data = ();
                fn parse<'a>(input: &'a str, (): &()) -> Result<(Self, &'a str), ArgumentParseError> {
                    match lexical::parse_partial::<$ty, _>(input) {
                        Ok((value, length)) => Ok((value, &input[length..])),
                        Err(lexical::Error::InvalidDigit(_)) => Err(ArgumentParseError::InputMismatch),
                        Err(err) => Err(ArgumentParseError::other(err)),
                    }
                }
            }
        )*
    };
}

impl_float!(f32, f64);

// -------------------------------------------------------------------------------------------------

impl<T: ArgumentParser> ArgumentParser for Option<T> {
    type Data = T::Data;

    fn parse<'a>(input: &'a str, data: &Self::Data) -> Result<(Self, &'a str), ArgumentParseError> {
        match T::parse(input, data) {
            Ok((value, remaining)) => Ok((Some(value), remaining)),
            Err(ArgumentParseError::InputMismatch) => Ok((None, input)),
            Err(err) => Err(err),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<T: ArgumentParser, const N: usize> ArgumentParser for [T; N] {
    type Data = T::Data;

    fn parse<'a>(
        mut input: &'a str,
        data: &T::Data,
    ) -> Result<(Self, &'a str), ArgumentParseError> {
        let mut result = alloc::vec::Vec::with_capacity(N);
        for _ in 0..N {
            let (value, rest) = T::parse(input, data)?;
            result.push(value);
            input = rest;
        }
        // SAFETY: Vector is guaranteed to be `N` elements long.
        Ok((unsafe { result.try_into().unwrap_unchecked() }, input))
    }
}
