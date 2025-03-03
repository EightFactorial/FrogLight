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
        let (mut content, remaining) = read_enclosed(content.trim(), '{', '}')?;

        let mut compound = NbtCompound::new();
        while !content.is_empty() {
            // Read the item key, trimming is already done by `Mutf8String`.
            let (key, remaining) = Mutf8String::read_from_string(content)?;

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
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        if content.starts_with('{') {
            return NbtCompound::read_from_string(content).map(|(c, r)| (NbtTag::Compound(c), r));
        }

        // TODO: Use FIELD_REGEX to attempt to guess the tag type.

        todo!()
    }
}

// -------------------------------------------------------------------------------------------------

impl ReadCompat for NbtListTag {
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        let (content, remaining) = read_enclosed(content.trim(), '[', ']')?;

        // Return an empty list if no content is found.
        if content.is_empty() {
            return Ok((NbtListTag::Empty, remaining));
        }

        // TODO: Guess the list type based on the first item.

        todo!()
    }
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
/// # Note
/// This cannot handle strings!
///
/// # Examples
/// - `(None, None, "[1,2,3]")` -> `[1,2,3]`
/// - `(None, Some('S'), "[1S,2S,3S]")` -> `[1,2,3]`
/// - `(Some('B'), Some('B'), "[B;1B,2B,3B]")` -> `[1,2,3]`
fn read_array<T: From<Vec<I>>, I: std::str::FromStr>(
    prefix: Option<char>,
    suffix: Option<char>,
    content: &str,
) -> Result<(T, &str), ConvertError> {
    let (mut content, remaining) = read_enclosed(content.trim(), '[', ']')?;

    // Strip the array prefix, erroring if one was expected but not found.
    if let Some(prefix) = prefix {
        if let Some(stripped) = content.strip_prefix(prefix).and_then(|s| s.strip_prefix(';')) {
            content = stripped;
        } else {
            return Err(ConvertError::UnexpectedData(content.to_string()));
        }
    }

    // Return an empty array if no content is found.
    if content.is_empty() {
        return Ok((T::from(Vec::new()), remaining));
    }

    // Parse the array content
    let mut array = Vec::new();
    for mut item in content.split(',') {
        if let Some(suffix) = suffix {
            if let Some(stripped) = item.strip_suffix(suffix) {
                item = stripped;
            } else {
                return Err(ConvertError::UnexpectedData(item.to_string()));
            }
        }

        match item.parse::<I>() {
            Ok(value) => array.push(value),
            Err(_) => return Err(ConvertError::UnexpectedData(item.to_string())),
        }
    }

    Ok((T::from(array), remaining))
}

#[test]
#[cfg(test)]
fn test_read_array() {
    let content = String::from("[B;1B,2B,3B]");
    let (array, remaining) = ByteArray::read_from_string(&content).unwrap();
    assert_eq!(&**array, vec![1, 2, 3]);
    assert_eq!(remaining, "");

    let content = String::from("[B;]");
    let (array, remaining) = ByteArray::read_from_string(&content).unwrap();
    assert_eq!(&**array, Vec::<i8>::new());
    assert_eq!(remaining, "");

    let content = String::from("[1S,2S,3S]");
    let (array, remaining) = ShortArray::read_from_string(&content).unwrap();
    assert_eq!(&**array, vec![1, 2, 3]);
    assert_eq!(remaining, "");

    let content = String::from("[]");
    let (array, remaining) = ShortArray::read_from_string(&content).unwrap();
    assert_eq!(&**array, Vec::<i16>::new());
    assert_eq!(remaining, "");

    let content = String::from("[I;1,2,3]");
    let (array, remaining) = IntArray::read_from_string(&content).unwrap();
    assert_eq!(&**array, vec![1, 2, 3]);
    assert_eq!(remaining, "");

    let content = String::from("[L;1L,2L,3L]");
    let (array, remaining) = LongArray::read_from_string(&content).unwrap();
    assert_eq!(&**array, vec![1, 2, 3]);
    assert_eq!(remaining, "");

    let content = String::from("[1.0F,2.0F,3.0F]");
    let (array, remaining) = FloatArray::read_from_string(&content).unwrap();
    assert_eq!(&**array, vec![1.0, 2.0, 3.0]);
    assert_eq!(remaining, "");

    let content = String::from("[1.0,2.0,3.0]");
    let (array, remaining) = DoubleArray::read_from_string(&content).unwrap();
    assert_eq!(&**array, vec![1.0, 2.0, 3.0]);
    assert_eq!(remaining, "");
}

/// Read the content enclosed by a pair of characters.
fn read_enclosed(content: &str, open: char, close: char) -> Result<(&str, &str), ConvertError> {
    debug_assert_eq!(content.chars().next().unwrap(), open);

    let mut count = 1u32;
    for (index, char) in content[1..].chars().enumerate() {
        match char {
            c if c == open => count += 1,
            c if c == close => {
                count -= 1;

                if count == 0 {
                    let (content, remaining) = content.split_at(index + 1);
                    return Ok((&content[1..], &remaining[1..]));
                }
            }
            _ => (),
        }
    }

    Err(ConvertError::UnexpectedData(content.to_string()))
}

#[test]
#[cfg(test)]
fn test_read_enclosed() {
    let content = String::from("{}");
    let (content, remaining) = read_enclosed(&content, '{', '}').unwrap();
    assert_eq!(content, "");
    assert_eq!(remaining, "");

    let content = String::from("{hello: world}");
    let (content, remaining) = read_enclosed(&content, '{', '}').unwrap();
    assert_eq!(content, "hello: world");
    assert_eq!(remaining, "");

    let content = String::from("{{hello: world}}");
    let (content, remaining) = read_enclosed(&content, '{', '}').unwrap();
    assert_eq!(content, "{hello: world}");
    assert_eq!(remaining, "");

    let content = String::from("[hello: {world: 123}]");
    let (content, remaining) = read_enclosed(&content, '[', ']').unwrap();
    assert_eq!(content, "hello: {world: 123}");
    assert_eq!(remaining, "");
}

// -------------------------------------------------------------------------------------------------

impl ReadCompat for Mutf8String {
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        if content.is_empty() {
            return Ok((Self::from_string(content), ""));
        }

        #[expect(clippy::single_match_else)]
        match content.chars().next().unwrap() {
            char @ ('"' | '\'') => {
                // Find the next unescaped matching quote.
                let mut last = false;
                let index = content[1..].find(|c| {
                    if last && (c == char) {
                        true
                    } else {
                        last = c != '\\';
                        false
                    }
                });

                if let Some(index) = index {
                    // Split the content and unescape any matching quotes.
                    let (content, remaining) = content.split_at(index + 2);
                    let content = if char == '"' {
                        content.replace("\\\"", "\"")
                    } else {
                        content.replace("\\'", "'")
                    };

                    // Create the string, trimming any excess characters from the remaining content.
                    Ok((
                        Self::from_string(&content[1..content.len() - 1]),
                        remaining.trim_start_matches([':', ',', ' ']),
                    ))
                } else {
                    Err(ConvertError::UnexpectedData(content.to_string()))
                }
            }
            _ => {
                // Split the content by the first non-string character,
                // with the character included in the remaining content.
                let (mut content, mut remaining) = (content, "");
                if let Some(index) = content.find([':', ',', ' ', ']', '}']) {
                    (content, remaining) = content.split_at(index);
                }

                // Create the string, trimming any excess characters from the remaining content.
                Ok((Self::from_string(content), remaining.trim_start_matches([':', ',', ' '])))
            }
        }
    }
}

#[test]
#[cfg(test)]
fn test_read_string() {
    let content = String::from("hello123");
    let string = Mutf8String::read_from_string(&content);
    assert_eq!(string.unwrap().0.to_str_lossy(), "hello123");

    let content = String::from("test-1");
    let string = Mutf8String::read_from_string(&content);
    assert_eq!(string.unwrap().0.to_str_lossy(), "test-1");

    let content = String::from(r#""123 \" 456""#);
    let string = Mutf8String::read_from_string(&content);
    assert_eq!(string.unwrap().0.to_str_lossy(), r#"123 " 456"#);

    let content = String::from(r"'123 \' 456'");
    let string = Mutf8String::read_from_string(&content);
    assert_eq!(string.unwrap().0.to_str_lossy(), r"123 ' 456");

    let content = String::from("tag: data");
    let (tag, rem) = Mutf8String::read_from_string(&content).unwrap();
    let (data, rem) = Mutf8String::read_from_string(rem).unwrap();
    assert_eq!(tag.to_str_lossy(), "tag");
    assert_eq!(data.to_str_lossy(), "data");
    assert_eq!(rem, "");

    let content = String::from("tag: data}");
    let (tag, rem) = Mutf8String::read_from_string(&content).unwrap();
    let (data, rem) = Mutf8String::read_from_string(rem).unwrap();
    assert_eq!(tag.to_str_lossy(), "tag");
    assert_eq!(data.to_str_lossy(), "data");
    assert_eq!(rem, "}");

    let content = String::from(r#""hello world": "test message""#);
    let (tag, rem) = Mutf8String::read_from_string(&content).unwrap();
    let (data, rem) = Mutf8String::read_from_string(rem).unwrap();
    assert_eq!(tag.to_str_lossy(), "hello world");
    assert_eq!(data.to_str_lossy(), "test message");
    assert_eq!(rem, "");
}
