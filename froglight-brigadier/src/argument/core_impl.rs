use super::{ArgumentParseError, ArgumentParser};

macro_rules! impl_integer {
    ($($ty:ty),*) => {
        $(
            impl ArgumentParser for $ty {
                fn parse(
                    input: & str,
                ) -> Result<(Self, & str), ArgumentParseError> {
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

impl_integer!(u8, u16, u32, u64, u128, usize);
impl_integer!(i8, i16, i32, i64, i128, isize);
