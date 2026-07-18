use core::num::{
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroU8, NonZeroU16, NonZeroU32,
    NonZeroU64, NonZeroU128,
};

use super::{ArgumentParseError, ArgumentParser};

impl ArgumentParser for bool {
    type Data = ();

    fn parse<'a>(input: &'a str, (): &()) -> Result<(Self, &'a str), ArgumentParseError<'a>> {
        let (input, remainder) = input.split_once(' ').unwrap_or((input, ""));
        match input {
            "true" => Ok((true, remainder)),
            "false" => Ok((false, remainder)),
            _ => Err(ArgumentParseError::InputMismatch),
        }
    }
}

// -------------------------------------------------------------------------------------------------

macro_rules! impl_integer {
    ($($ty:ty),*) => {
        $(
            impl ArgumentParser for $ty {
                type Data = ();
                fn parse<'a>(input: &'a str, (): &()) -> Result<(Self, &'a str), ArgumentParseError<'a>> {
                    let (input, remainder) = input.split_once(' ').unwrap_or((input, ""));
                    match lexical::parse::<$ty, _>(input) {
                        Ok(value) => Ok((value, remainder)),
                        Err(lexical::Error::InvalidDigit(_)) => Err(ArgumentParseError::InputMismatch),
                        #[cfg(feature = "std")]
                        Err(err) => Err(ArgumentParseError::other(err)),
                        #[cfg(not(feature = "std"))]
                        Err(_) => Err(ArgumentParseError::Unknown),
                    }
                }
            }
        )*
    };
    (@nonzero $($ty:ty: $inner:ty),*) => {
        $(
            impl ArgumentParser for $ty {
                type Data = ();
                fn parse<'a>(input: &'a str, (): &()) -> Result<(Self, &'a str), ArgumentParseError<'a>> {
                    let (input, remainder) = input.split_once(' ').unwrap_or((input, ""));
                    match lexical::parse::<$inner, _>(input) {
                        Ok(value) => match <$ty>::new(value) {
                            Some(value) => Ok((value, remainder)),
                            None => Err(ArgumentParseError::InputInvalid),
                        },
                        Err(lexical::Error::InvalidDigit(_)) => Err(ArgumentParseError::InputMismatch),
                        #[cfg(feature = "std")]
                        Err(err) => Err(ArgumentParseError::other(err)),
                        #[cfg(not(feature = "std"))]
                        Err(_) => Err(ArgumentParseError::Unknown),
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
                fn parse<'a>(input: &'a str, (): &()) -> Result<(Self, &'a str), ArgumentParseError<'a>> {
                    let (input, remainder) = input.split_once(' ').unwrap_or((input, ""));
                    match lexical::parse::<$ty, &str>(input) {
                        Ok(value) => Ok((value, remainder)),
                        Err(lexical::Error::InvalidDigit(_)) => Err(ArgumentParseError::InputMismatch),
                        #[cfg(feature = "std")]
                        Err(err) => Err(ArgumentParseError::other(err)),
                        #[cfg(not(feature = "std"))]
                        Err(_) => Err(ArgumentParseError::Unknown),
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

    fn parse<'a>(
        input: &'a str,
        data: &Self::Data,
    ) -> Result<(Self, &'a str), ArgumentParseError<'a>> {
        if input.is_empty() {
            Ok((None, input))
        } else {
            match T::parse(input, data) {
                Ok((value, rest)) => Ok((Some(value), rest)),
                Err(err) => Err(err),
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<T: ArgumentParser, const N: usize> ArgumentParser for [T; N] {
    type Data = T::Data;

    #[cfg(not(feature = "nightly"))]
    fn parse<'a>(
        mut input: &'a str,
        data: &T::Data,
    ) -> Result<(Self, &'a str), ArgumentParseError<'a>> {
        let mut result = alloc::vec::Vec::with_capacity(N);
        for _ in 0..N {
            let (value, rest) = T::parse(input, data)?;
            result.push(value);
            input = rest;
        }
        // SAFETY: Vector is guaranteed to be `N` elements long.
        Ok((unsafe { result.try_into().unwrap_unchecked() }, input))
    }

    #[cfg(feature = "nightly")]
    fn parse<'a>(
        mut input: &'a str,
        data: &T::Data,
    ) -> Result<(Self, &'a str), ArgumentParseError<'a>> {
        core::array::try_from_fn(|_| {
            let (value, rest) = T::parse(input, data)?;
            input = rest;
            Ok(value)
        })
        .map(|arr| (arr, input))
    }
}
