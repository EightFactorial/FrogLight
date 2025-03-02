use crate::{
    convert::ConvertError,
    mutf8::Mutf8String,
    nbt::{
        ByteArray, DoubleArray, FloatArray, IntArray, LongArray, NbtCompound, NbtListTag, NbtTag,
        ShortArray,
    },
};

pub(super) trait ReadCompat: Sized {
    /// Read the content from a string, returning the remaining content.
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError>;
}

impl ReadCompat for NbtCompound {
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        let (mut content, remaining) = read_enclosed(content, '{', '}')?;

        let mut compound = NbtCompound::new();
        while !content.is_empty() {
            // Read the item key, trimming away any whitespace and colons.
            let (key, mut remaining) = Mutf8String::read_from_string(content)?;
            remaining = remaining.trim_start_matches([' ', ':']);

            // Read the item data, trimming away any whitespace and commas.
            let (value, remaining) = NbtTag::read_from_string(remaining)?;
            content = remaining.trim_start_matches([' ', ',']);

            compound.insert(key, value);
        }

        Ok((compound, remaining))
    }
}

// -------------------------------------------------------------------------------------------------

impl ReadCompat for NbtTag {
    fn read_from_string(_content: &str) -> Result<(Self, &str), ConvertError> { todo!() }
}

// -------------------------------------------------------------------------------------------------

impl ReadCompat for NbtListTag {
    fn read_from_string(_content: &str) -> Result<(Self, &str), ConvertError> { todo!() }
}

// -------------------------------------------------------------------------------------------------

/// ByteArray-specific format: `[B;({BYTE}B)?(,{BYTE}B)*]`
impl ReadCompat for ByteArray {
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        read_array(Some('B'), Some('B'), content)
    }
}

/// IntArray-specific format: `[I;{INT}?(,{INT})*]`
impl ReadCompat for IntArray {
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        read_array(Some('I'), None, content)
    }
}

/// LongArray-specific format: `[L;({LONG}L)?(,{LONG}L)*]`
impl ReadCompat for LongArray {
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        read_array(Some('L'), Some('L'), content)
    }
}

/// List-based format: `[({SHORT}S?)(,{SHORT}S)*]`
impl ReadCompat for ShortArray {
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        read_array(None, Some('S'), content)
    }
}

/// List-based format: `[({FLOAT}F)?(,{FLOAT}F)*]`
impl ReadCompat for FloatArray {
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        read_array(None, Some('F'), content)
    }
}

/// List-based format: `[{DOUBLE}?(,{DOUBLE})*]`
impl ReadCompat for DoubleArray {
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        read_array(None, None, content)
    }
}

/// Read an array from a string.
///
/// # Examples
/// - `(None, None, "[1,2,3]")` -> `[1,2,3]`
/// - `(None, Some('S'), "[1S,2S,3S]")` -> `[1,2,3]`
/// - `(Some('B'), Some('B'), "[B;1B,2B,3B]")` -> `[1,2,3]`
fn read_array<T: From<Vec<I>>, I: std::str::FromStr>(
    _prefix: Option<char>,
    _suffix: Option<char>,
    _content: &str,
) -> Result<(T, &str), ConvertError> {
    todo!()
}

/// Read the content enclosed by a pair of characters.
fn read_enclosed(_content: &str, _open: char, _close: char) -> Result<(&str, &str), ConvertError> {
    todo!()
}

// -------------------------------------------------------------------------------------------------

impl ReadCompat for Mutf8String {
    fn read_from_string(_content: &str) -> Result<(Self, &str), ConvertError> { todo!() }
}

// #[test]
// #[cfg(test)]
// fn test_read_string() {
//     let mut content = String::from("hello123");
//     let string = Mutf8String::read_from_string(&content);
//     assert_eq!(string.unwrap().0, "hello123");
//
//     let mut content = String::from("test-1");
//     let string = Mutf8String::read_from_string(&content);
//     assert_eq!(string.unwrap().0, "test-1");
//
//     let mut content = String::from(r#""123 \" 456""#);
//     let string = Mutf8String::read_from_string(&content);
//     assert_eq!(string.unwrap().0, "123 \" 456");
// }
