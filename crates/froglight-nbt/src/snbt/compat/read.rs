use crate::{
    convert::ConvertError,
    mutf8::Mutf8String,
    nbt::{
        ByteArray, DoubleArray, FloatArray, IntArray, LongArray, NbtCompound, NbtListTag, NbtTag,
        ShortArray,
    },
    snbt::compat::regex::FIELD_REGEX,
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

#[test]
#[cfg(test)]
fn test_read_compound() {
    let content = String::from("{hello: world}");
    let (compound, remaining) = NbtCompound::read_from_string(&content).unwrap();
    assert_eq!(
        compound,
        NbtCompound::from(vec![(
            Mutf8String::from_string("hello"),
            NbtTag::String(Mutf8String::from_string("world"))
        )])
    );
    assert_eq!(remaining, "");

    let content = String::from(r#"{"": world}"#);
    let (compound, remaining) = NbtCompound::read_from_string(&content).unwrap();
    assert_eq!(
        compound,
        NbtCompound::from(vec![(
            Mutf8String::from_string(""),
            NbtTag::String(Mutf8String::from_string("world"))
        )])
    );
    assert_eq!(remaining, "");

    let content = String::from("{hello: world, test: 123}");
    let (compound, remaining) = NbtCompound::read_from_string(&content).unwrap();
    assert_eq!(
        compound,
        NbtCompound::from(vec![
            (Mutf8String::from_string("hello"), NbtTag::String(Mutf8String::from_string("world"))),
            (Mutf8String::from_string("test"), NbtTag::Int(123))
        ])
    );
    assert_eq!(remaining, "");

    let content = String::from("{\"hello world\": true, value: false}");
    let (compound, remaining) = NbtCompound::read_from_string(&content).unwrap();
    assert_eq!(
        compound,
        NbtCompound::from(vec![
            (Mutf8String::from_string("hello world"), NbtTag::Byte(1)),
            (Mutf8String::from_string("value"), NbtTag::Byte(0))
        ])
    );
    assert_eq!(remaining, "");
}

// -------------------------------------------------------------------------------------------------

impl ReadCompat for NbtTag {
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        let mut chars = content.chars();
        match chars.next().unwrap() {
            '\'' | '"' => {
                return Mutf8String::read_from_string(content).map(|(s, r)| (NbtTag::String(s), r));
            }
            '{' => {
                return NbtCompound::read_from_string(content)
                    .map(|(c, r)| (NbtTag::Compound(c), r));
            }
            '[' => match chars.next().unwrap() {
                'B' => {
                    return ByteArray::read_from_string(content)
                        .map(|(a, r)| (NbtTag::ByteArray(a), r));
                }
                'I' => {
                    return IntArray::read_from_string(content)
                        .map(|(a, r)| (NbtTag::IntArray(a), r));
                }
                'L' => {
                    return LongArray::read_from_string(content)
                        .map(|(a, r)| (NbtTag::LongArray(a), r));
                }
                _ => {
                    return NbtListTag::read_from_string(content)
                        .map(|(l, r)| (NbtTag::List(l), r));
                }
            },
            _ => {}
        }

        // Split the content by the end of the tag.
        // Guaranteed OK because only Numbers and unquoted Strings are left.
        let index = content.find([' ', ',', '}', ']']).unwrap_or(content.len());
        let (content, remaining) = content.split_at(index);

        let matches = FIELD_REGEX.matches_at(content, 0);
        match (
            matches.matched(0), // STRING
            matches.matched(1), // BYTE
            matches.matched(2), // SHORT
            matches.matched(3), // INT
            matches.matched(4), // LONG
            matches.matched(5), // FLOAT
            matches.matched(6), // DOUBLE
        ) {
            (_, true, ..) => match content {
                "true" => Ok((NbtTag::Byte(1), remaining)),
                "false" => Ok((NbtTag::Byte(0), remaining)),
                content => match content.trim_end_matches('B').parse::<i8>() {
                    Ok(value) => Ok((NbtTag::Byte(value), remaining)),
                    Err(_) => Err(ConvertError::UnexpectedData(content.to_string())),
                },
            },
            (_, _, true, ..) => match content.trim_end_matches('S').parse::<i16>() {
                Ok(value) => Ok((NbtTag::Short(value), remaining)),
                Err(_) => Err(ConvertError::UnexpectedData(content.to_string())),
            },
            (_, _, _, _, true, ..) => match content.trim_end_matches('L').parse::<i64>() {
                Ok(value) => Ok((NbtTag::Long(value), remaining)),
                Err(_) => Err(ConvertError::UnexpectedData(content.to_string())),
            },
            (_, _, _, true, ..) => match content.parse::<i32>() {
                Ok(value) => Ok((NbtTag::Int(value), remaining)),
                Err(_) => Err(ConvertError::UnexpectedData(content.to_string())),
            },
            (_, _, _, _, _, true, _) => match content.trim_end_matches('F').parse::<f32>() {
                Ok(value) => Ok((NbtTag::Float(value), remaining)),
                Err(_) => Err(ConvertError::UnexpectedData(content.to_string())),
            },
            (_, _, _, _, _, _, true) => match content.parse::<f64>() {
                Ok(value) => Ok((NbtTag::Double(value), remaining)),
                Err(_) => Err(ConvertError::UnexpectedData(content.to_string())),
            },
            (true, ..) => {
                let (string, _) = Mutf8String::read_from_string(content)?;
                Ok((NbtTag::String(string), remaining))
            }
            _ => Err(ConvertError::UnexpectedData(content.to_string())),
        }
    }
}

#[test]
#[cfg(test)]
fn test_read_tag() {
    let content = String::from("123B");
    let (tag, remaining) = NbtTag::read_from_string(&content).unwrap();
    assert_eq!(tag, NbtTag::Byte(123));
    assert_eq!(remaining, "");

    let content = String::from("true");
    let (tag, remaining) = NbtTag::read_from_string(&content).unwrap();
    assert_eq!(tag, NbtTag::Byte(1));
    assert_eq!(remaining, "");

    let content = String::from("123S");
    let (tag, remaining) = NbtTag::read_from_string(&content).unwrap();
    assert_eq!(tag, NbtTag::Short(123));
    assert_eq!(remaining, "");

    let content = String::from("123");
    let (tag, remaining) = NbtTag::read_from_string(&content).unwrap();
    assert_eq!(tag, NbtTag::Int(123));
    assert_eq!(remaining, "");

    let content = String::from("123L");
    let (tag, remaining) = NbtTag::read_from_string(&content).unwrap();
    assert_eq!(tag, NbtTag::Long(123));
    assert_eq!(remaining, "");

    let content = String::from("123.0F");
    let (tag, remaining) = NbtTag::read_from_string(&content).unwrap();
    assert_eq!(tag, NbtTag::Float(123.0));
    assert_eq!(remaining, "");

    let content = String::from("123.0");
    let (tag, remaining) = NbtTag::read_from_string(&content).unwrap();
    assert_eq!(tag, NbtTag::Double(123.0));
    assert_eq!(remaining, "");

    let content = String::from("hello");
    let (tag, remaining) = NbtTag::read_from_string(&content).unwrap();
    assert_eq!(tag, NbtTag::String(Mutf8String::from_string("hello")));
    assert_eq!(remaining, "");

    let content = String::from(r#""hello world""#);
    let (tag, remaining) = NbtTag::read_from_string(&content).unwrap();
    assert_eq!(tag, NbtTag::String(Mutf8String::from_string("hello world")));
    assert_eq!(remaining, "");

    let content = String::from("{hello: world}");
    let (tag, remaining) = NbtTag::read_from_string(&content).unwrap();
    assert_eq!(
        tag,
        NbtTag::Compound(NbtCompound::from(vec![(
            Mutf8String::from_string("hello"),
            NbtTag::String(Mutf8String::from_string("world"))
        )]))
    );
    assert_eq!(remaining, "");

    let content = String::from("[B;1B,2B,-3B,true,false]");
    let (tag, remaining) = NbtTag::read_from_string(&content).unwrap();
    assert_eq!(tag, NbtTag::ByteArray(ByteArray::from(vec![1, 2, -3, 1, 0])));
    assert_eq!(remaining, "");

    let content = String::from("[I;1,2,-3]");
    let (tag, remaining) = NbtTag::read_from_string(&content).unwrap();
    assert_eq!(tag, NbtTag::IntArray(IntArray::from(vec![1, 2, -3])));
    assert_eq!(remaining, "");

    let content = String::from("[L;1L,2L,-3L]");
    let (tag, remaining) = NbtTag::read_from_string(&content).unwrap();
    assert_eq!(tag, NbtTag::LongArray(LongArray::from(vec![1, 2, -3])));
    assert_eq!(remaining, "");
}

// -------------------------------------------------------------------------------------------------

impl ReadCompat for NbtListTag {
    #[expect(clippy::too_many_lines)]
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        let (content, remaining) = read_enclosed(content.trim(), '[', ']')?;

        // Return an empty list if no content is found.
        if content.is_empty() {
            return Ok((NbtListTag::Empty, remaining));
        }

        let mut chars = content.chars();
        match chars.next().unwrap() {
            '\'' | '"' => {
                return read_list::<Mutf8String>(content)
                    .map(|list| (NbtListTag::String(list), remaining));
            }
            '{' => {
                return read_list::<NbtCompound>(content)
                    .map(|list| (NbtListTag::Compound(list), remaining));
            }
            '[' => match chars.next().unwrap() {
                'B' => {
                    return read_list::<ByteArray>(content)
                        .map(|list| (NbtListTag::ByteArray(list), remaining));
                }
                'I' => {
                    return read_list::<IntArray>(content)
                        .map(|list| (NbtListTag::IntArray(list), remaining));
                }
                'L' => {
                    return read_list::<LongArray>(content)
                        .map(|list| (NbtListTag::LongArray(list), remaining));
                }
                _ => {
                    return read_list::<NbtListTag>(content)
                        .map(|list| (NbtListTag::List(list), remaining));
                }
            },
            _ => {}
        }

        // Get the first item in the list.
        // Guaranteed OK because only Numbers and unquoted Strings are left.
        let index = content.find([' ', ',', '}', ']']).unwrap_or(content.len());
        let matches = FIELD_REGEX.matches_at(&content[..index], 0);

        match (
            matches.matched(0), // STRING
            matches.matched(1), // BYTE
            matches.matched(2), // SHORT
            matches.matched(3), // INT
            matches.matched(4), // LONG
            matches.matched(5), // FLOAT
            matches.matched(6), // DOUBLE
        ) {
            (_, true, ..) => {
                let list = content.split(',').try_fold(Vec::new(), |mut acc, item| {
                    match item {
                        "true" => acc.push(1),
                        "false" => acc.push(0),
                        item => match item.trim_end_matches('B').parse::<i8>() {
                            Ok(value) => acc.push(value),
                            Err(_) => {
                                return Err(ConvertError::UnexpectedData(item.to_string()));
                            }
                        },
                    }
                    Ok(acc)
                })?;
                Ok((NbtListTag::Byte(ByteArray::from(list)), remaining))
            }
            (_, _, true, ..) => {
                let list = content.split(',').try_fold(Vec::new(), |mut acc, item| {
                    match item.trim_end_matches('S').parse::<i16>() {
                        Ok(value) => acc.push(value),
                        Err(_) => return Err(ConvertError::UnexpectedData(item.to_string())),
                    }
                    Ok(acc)
                })?;
                Ok((NbtListTag::Short(ShortArray::from(list)), remaining))
            }
            (_, _, _, _, true, ..) => {
                let list = content.split(',').try_fold(Vec::new(), |mut acc, item| {
                    match item.trim_end_matches('L').parse::<i64>() {
                        Ok(value) => acc.push(value),
                        Err(_) => return Err(ConvertError::UnexpectedData(content.to_string())),
                    }
                    Ok(acc)
                })?;
                Ok((NbtListTag::Long(LongArray::from(list)), remaining))
            }
            (_, _, _, true, ..) => {
                let list = content.split(',').try_fold(Vec::new(), |mut acc, item| {
                    match item.parse::<i32>() {
                        Ok(value) => acc.push(value),
                        Err(_) => return Err(ConvertError::UnexpectedData(item.to_string())),
                    }
                    Ok(acc)
                })?;
                Ok((NbtListTag::Int(IntArray::from(list)), remaining))
            }
            (_, _, _, _, _, true, _) => {
                let list = content.split(',').try_fold(Vec::new(), |mut acc, item| {
                    match item.trim_end_matches('F').parse::<f32>() {
                        Ok(value) => acc.push(value),
                        Err(_) => return Err(ConvertError::UnexpectedData(item.to_string())),
                    }
                    Ok(acc)
                })?;
                Ok((NbtListTag::Float(FloatArray::from(list)), remaining))
            }
            (_, _, _, _, _, _, true) => {
                let list = content.split(',').try_fold(Vec::new(), |mut acc, item| {
                    match item.parse::<f64>() {
                        Ok(value) => acc.push(value),
                        Err(_) => return Err(ConvertError::UnexpectedData(item.to_string())),
                    }
                    Ok(acc)
                })?;
                Ok((NbtListTag::Double(DoubleArray::from(list)), remaining))
            }
            (true, ..) => {
                let mut content = content;
                let mut list = Vec::new();
                while !content.is_empty() {
                    let (string, remaining) = Mutf8String::read_from_string(content)?;
                    content = remaining.trim_start_matches([' ', ',']);
                    list.push(string);
                }
                Ok((NbtListTag::String(list), remaining))
            }
            _ => Err(ConvertError::UnexpectedData(content.to_string())),
        }
    }
}

fn read_list<T: ReadCompat>(mut content: &str) -> Result<Vec<T>, ConvertError> {
    let mut list = Vec::new();
    while !content.is_empty() {
        let (item, remaining) = T::read_from_string(content)?;
        content = remaining.trim_start_matches([' ', ',']);
        list.push(item);
    }
    Ok(list)
}

#[test]
#[cfg(test)]
fn test_read_list() {
    let content = String::from("[]");
    let (list, remaining) = NbtListTag::read_from_string(&content).unwrap();
    assert_eq!(list, NbtListTag::Empty);
    assert_eq!(remaining, "");

    let content = String::from("[1B,2B,3B]");
    let (list, remaining) = NbtListTag::read_from_string(&content).unwrap();
    assert_eq!(list, NbtListTag::Byte(ByteArray::from(vec![1, 2, 3])));
    assert_eq!(remaining, "");

    let content = String::from("[1S,2S,3S]");
    let (list, remaining) = NbtListTag::read_from_string(&content).unwrap();
    assert_eq!(list, NbtListTag::Short(ShortArray::from(vec![1, 2, 3])));
    assert_eq!(remaining, "");

    let content = String::from("[1,2,3]");
    let (list, remaining) = NbtListTag::read_from_string(&content).unwrap();
    assert_eq!(list, NbtListTag::Int(IntArray::from(vec![1, 2, 3])));
    assert_eq!(remaining, "");

    let content = String::from("[1L,2L,3L]");
    let (list, remaining) = NbtListTag::read_from_string(&content).unwrap();
    assert_eq!(list, NbtListTag::Long(LongArray::from(vec![1, 2, 3])));
    assert_eq!(remaining, "");

    let content = String::from("[1.0F,2.0F,3.0F]");
    let (list, remaining) = NbtListTag::read_from_string(&content).unwrap();
    assert_eq!(list, NbtListTag::Float(FloatArray::from(vec![1.0, 2.0, 3.0])));
    assert_eq!(remaining, "");

    let content = String::from("[1.0,2.0,3.0]");
    let (list, remaining) = NbtListTag::read_from_string(&content).unwrap();
    assert_eq!(list, NbtListTag::Double(DoubleArray::from(vec![1.0, 2.0, 3.0])));
    assert_eq!(remaining, "");

    let content = String::from("[hello, world]");
    let (list, remaining) = NbtListTag::read_from_string(&content).unwrap();
    assert_eq!(
        list,
        NbtListTag::String(vec![
            Mutf8String::from_string("hello"),
            Mutf8String::from_string("world")
        ])
    );
    assert_eq!(remaining, "");

    let content = String::from("[{hello: world}, {test: 123}]");
    let (list, remaining) = NbtListTag::read_from_string(&content).unwrap();
    assert_eq!(
        list,
        NbtListTag::Compound(vec![
            NbtCompound::from(vec![(
                Mutf8String::from_string("hello"),
                NbtTag::String(Mutf8String::from_string("world"))
            )]),
            NbtCompound::from(vec![(Mutf8String::from_string("test"), NbtTag::Int(123))])
        ])
    );
    assert_eq!(remaining, "");

    let content = String::from("[[1B,2B,3B],[4B,5B,6B]]");
    let (list, remaining) = NbtListTag::read_from_string(&content).unwrap();
    assert_eq!(
        list,
        NbtListTag::List(vec![
            NbtListTag::Byte(ByteArray::from(vec![1, 2, 3])),
            NbtListTag::Byte(ByteArray::from(vec![4, 5, 6]))
        ])
    );
    assert_eq!(remaining, "");

    let content = String::from("[[B;1B,2B,3B],[B;4B,5B,6B]]");
    let (list, remaining) = NbtListTag::read_from_string(&content).unwrap();
    assert_eq!(
        list,
        NbtListTag::ByteArray(vec![ByteArray::from(vec![1, 2, 3]), ByteArray::from(vec![4, 5, 6])])
    );
    assert_eq!(remaining, "");

    let content = String::from("[[I;1,2,3],[I;4,5,6]]");
    let (list, remaining) = NbtListTag::read_from_string(&content).unwrap();
    assert_eq!(
        list,
        NbtListTag::IntArray(vec![IntArray::from(vec![1, 2, 3]), IntArray::from(vec![4, 5, 6])])
    );
    assert_eq!(remaining, "");

    let content = String::from("[[L;1L,2L,3L],[L;4L,5L,6L]]");
    let (list, remaining) = NbtListTag::read_from_string(&content).unwrap();
    assert_eq!(
        list,
        NbtListTag::LongArray(vec![LongArray::from(vec![1, 2, 3]), LongArray::from(vec![4, 5, 6])])
    );
    assert_eq!(remaining, "");
}

// -------------------------------------------------------------------------------------------------

/// ByteArray-specific format: `[B;({BYTE}B)?(,{BYTE}B)*]`
impl ReadCompat for ByteArray {
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        let (mut content, remaining) = read_enclosed(content.trim(), '[', ']')?;

        // Strip the array prefix
        if let Some(stripped) = content.strip_prefix("B;") {
            content = stripped;
        } else {
            return Err(ConvertError::UnexpectedData(content.to_string()));
        }

        // Return an empty array if no content is found.
        if content.is_empty() {
            return Ok((ByteArray::from(Vec::new()), remaining));
        }

        // Parse the array content
        let mut array = Vec::new();
        for item in content.split(',') {
            match item {
                "true" => array.push(1),
                "false" => array.push(0),
                item => match item.strip_suffix('B').and_then(|s| s.parse::<i8>().ok()) {
                    Some(value) => array.push(value),
                    None => return Err(ConvertError::UnexpectedData(item.to_string())),
                },
            }
        }
        Ok((ByteArray::from(array), remaining))
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

    let content = String::from("[B;1B,2B,3B,true,false,-10B]");
    let (array, remaining) = ByteArray::read_from_string(&content).unwrap();
    assert_eq!(&**array, vec![1, 2, 3, 1, 0, -10]);
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
            return Ok((Self::from_string(""), content));
        }

        #[expect(clippy::single_match_else)]
        match content.chars().next().unwrap() {
            char @ ('"' | '\'') => {
                // Find the next unescaped matching quote.
                let mut last = true;
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
                let index = content.find([':', ',', ' ', ']', '}']).unwrap_or(content.len());
                let (content, remaining) = content.split_at(index);

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

    let content = String::from("tag: data]");
    let (tag, rem) = Mutf8String::read_from_string(&content).unwrap();
    let (data, rem) = Mutf8String::read_from_string(rem).unwrap();
    assert_eq!(tag.to_str_lossy(), "tag");
    assert_eq!(data.to_str_lossy(), "data");
    assert_eq!(rem, "]");

    let content = String::from(r#""hello world": "test message""#);
    let (tag, rem) = Mutf8String::read_from_string(&content).unwrap();
    let (data, rem) = Mutf8String::read_from_string(rem).unwrap();
    assert_eq!(tag.to_str_lossy(), "hello world");
    assert_eq!(data.to_str_lossy(), "test message");
    assert_eq!(rem, "");
}
