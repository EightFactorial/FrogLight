use core::ops::Range;

use crate::parse::{ArgumentParseError, CommandArgument};

macro_rules! impl_integer {
    ($($ty:ty),*) => {
        $(
            impl CommandArgument for $ty {
                type Output = $ty;

                fn parse_argument<'a>(
                    &self,
                    input: &'a str,
                ) -> Result<(Self::Output, &'a str), ArgumentParseError> {
                    match lexical::parse_partial::<$ty, _>(input) {
                        Ok((value, length)) => Ok((value, &input[length..])),
                        Err(lexical::Error::InvalidDigit(_)) => Err(ArgumentParseError::InputMismatch),
                        Err(err) => Err(ArgumentParseError::other(err)),
                    }
                }
            }

            impl CommandArgument for Range<$ty> {
                type Output = $ty;

                fn parse_argument<'a>(
                    &self,
                    input: &'a str,
                ) -> Result<(Self::Output, &'a str), ArgumentParseError> {
                    match lexical::parse_partial::<$ty, _>(input) {
                        Ok((value, length)) if self.contains(&value) => Ok((value, &input[length..])),
                        Ok(_) | Err(lexical::Error::InvalidDigit(_)) => Err(ArgumentParseError::InputMismatch),
                        Err(err) => Err(ArgumentParseError::other(err)),
                    }
                }
            }
        )*
    };
}

impl_integer!(u8, u16, u32, u64, u128, usize);
impl_integer!(i8, i16, i32, i64, i128, isize);
