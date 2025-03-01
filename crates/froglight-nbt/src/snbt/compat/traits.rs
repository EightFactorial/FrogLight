use super::regex::STRING_REGEX;
use crate::{
    convert::ConvertError,
    mutf8::Mutf8String,
    nbt::{
        ByteArray, DoubleArray, FloatArray, IntArray, LongArray, NbtCompound, NbtListTag, NbtTag,
        ShortArray,
    },
    snbt::compat::regex::FIELD_REGEX,
};

pub(super) trait ConvertCompat: Sized {
    /// Read the content from a string, returning the remaining content.
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError>;
    /// Write the content to a string.
    fn write_to_string(&self, content: &mut String);
}

impl ConvertCompat for NbtCompound {
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        let (mut content, remaining) = read_enclosed(content, '{', '}')?;

        let mut compound = NbtCompound::new();
        while !content.is_empty() {
            let (key, mut remaining) = Mutf8String::read_from_string(content)?;
            remaining = remaining.trim_start_matches([' ', ':']);

            let (value, remaining) = NbtTag::read_from_string(remaining)?;
            compound.insert(key, value);

            content = remaining.trim_start_matches([' ', ',']);
        }

        Ok((compound, remaining))
    }

    fn write_to_string(&self, content: &mut String) {
        content.push('{');

        let last = self.len().saturating_sub(1);
        for (i, (key, value)) in self.iter().enumerate() {
            key.write_to_string(content);
            content.push(':');
            value.write_to_string(content);

            if i < last {
                content.push(',');
            }
        }

        content.push('}');
    }
}

// -------------------------------------------------------------------------------------------------

impl ConvertCompat for NbtTag {
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        match content.trim_start().chars().next().unwrap() {
            '{' => NbtCompound::read_from_string(content).map(|(c, r)| (NbtTag::Compound(c), r)),
            '[' => {
                todo!("Guess whether *Array or List: {content:?}");
            }
            '\"' | '\'' => {
                Mutf8String::read_from_string(content).map(|(s, r)| (NbtTag::String(s), r))
            }
            _ => {
                let _matches = FIELD_REGEX.matches_at(content, 0);
                todo!("Guess the type of the tag from the content: {content:?}");
            }
        }
    }

    fn write_to_string(&self, content: &mut String) {
        match self {
            NbtTag::Byte(item) => {
                content.push_str(&item.to_string());
                content.push('B');
            }
            NbtTag::Short(item) => {
                content.push_str(&item.to_string());
                content.push('S');
            }
            NbtTag::Int(item) => {
                content.push_str(&item.to_string());
            }
            NbtTag::Long(item) => {
                content.push_str(&item.to_string());
                content.push('L');
            }
            NbtTag::Float(item) => {
                content.push_str(&item.to_string());
                content.push('F');
            }
            NbtTag::Double(item) => {
                content.push_str(&item.to_string());
            }
            NbtTag::String(item) => item.write_to_string(content),
            NbtTag::List(item) => item.write_to_string(content),
            NbtTag::Compound(item) => item.write_to_string(content),
            NbtTag::ByteArray(array) => array.write_to_string(content),
            NbtTag::IntArray(array) => array.write_to_string(content),
            NbtTag::LongArray(array) => array.write_to_string(content),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl ConvertCompat for NbtListTag {
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        let (content, remaining) = read_enclosed(content, '[', ']')?;
        if content.is_empty() {
            return Ok((NbtListTag::Empty, remaining));
        }

        match content.chars().next().unwrap() {
            '{' => {
                todo!("Read a list of compounds: {content:?}");
            }
            '[' => {
                todo!("Guess whether *Array or List: {content:?}");
            }
            '\"' | '\'' => {
                todo!("Read a list of strings: {content:?}");
            }
            _ => {
                let _matches = FIELD_REGEX.matches_at(content, 0);
                todo!("Guess the type of the tag from the content: {content:?}");
            }
        }
    }

    fn write_to_string(&self, content: &mut String) {
        match self {
            NbtListTag::Empty => content.push_str("[]"),
            NbtListTag::Byte(array) => write_array(None, Some('B'), array.iter(), content),
            NbtListTag::Short(array) => write_array(None, Some('S'), array.iter(), content),
            NbtListTag::Int(array) => write_array(None, None, array.iter(), content),
            NbtListTag::Long(array) => write_array(None, Some('L'), array.iter(), content),
            NbtListTag::Float(array) => write_array(None, Some('F'), array.iter(), content),
            NbtListTag::Double(array) => write_array(None, None, array.iter(), content),
            NbtListTag::String(array) => write_list(array.iter(), content),
            NbtListTag::List(array) => write_list(array.iter(), content),
            NbtListTag::Compound(array) => write_list(array.iter(), content),
            NbtListTag::ByteArray(array) => write_list(array.iter(), content),
            NbtListTag::IntArray(array) => write_list(array.iter(), content),
            NbtListTag::LongArray(array) => write_list(array.iter(), content),
        }
    }
}

fn write_list<'a, T: ConvertCompat + 'a>(
    iter: impl ExactSizeIterator<Item = &'a T>,
    content: &mut String,
) {
    content.push('[');

    let last = iter.len().saturating_sub(1);
    for (i, item) in iter.enumerate() {
        item.write_to_string(content);

        if i < last {
            content.push(',');
        }
    }

    content.push(']');
}

// -------------------------------------------------------------------------------------------------

/// ByteArray-specific format: `[B;({BYTE}B)?(,{BYTE}B)*]`
impl ConvertCompat for ByteArray {
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        read_array(Some('B'), Some('B'), content)
    }

    fn write_to_string(&self, content: &mut String) {
        write_array(Some('B'), Some('B'), self.iter(), content);
    }
}

/// IntArray-specific format: `[I;{INT}?(,{INT})*]`
impl ConvertCompat for IntArray {
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        read_array(Some('I'), None, content)
    }

    fn write_to_string(&self, content: &mut String) {
        write_array(Some('I'), None, self.iter(), content);
    }
}

/// LongArray-specific format: `[L;({LONG}L)?(,{LONG}L)*]`
impl ConvertCompat for LongArray {
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        read_array(Some('L'), Some('L'), content)
    }

    fn write_to_string(&self, content: &mut String) {
        write_array(Some('L'), Some('L'), self.iter(), content);
    }
}

/// List-based format: `[({SHORT}S?)(,{SHORT}S)*]`
impl ConvertCompat for ShortArray {
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        read_array(None, Some('S'), content)
    }

    fn write_to_string(&self, content: &mut String) {
        write_array(None, Some('S'), self.iter(), content);
    }
}

/// List-based format: `[({FLOAT}F)?(,{FLOAT}F)*]`
impl ConvertCompat for FloatArray {
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        read_array(None, Some('F'), content)
    }

    fn write_to_string(&self, content: &mut String) {
        write_array(None, Some('F'), self.iter(), content);
    }
}

/// List-based format: `[{DOUBLE}?(,{DOUBLE})*]`
impl ConvertCompat for DoubleArray {
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        read_array(None, None, content)
    }

    fn write_to_string(&self, content: &mut String) {
        write_array(None, None, self.iter(), content);
    }
}

/// Read an array from a string.
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
    // Read the content enclosed by square brackets.
    let (mut content, remaining) = read_enclosed(content, '[', ']')?;

    // Return early if the array is empty.
    if content.is_empty() {
        return Ok((T::from(Vec::new()), remaining));
    }

    // Strip the array prefix, making sure it matches the expected format.
    if let Some(prefix) = prefix {
        if content.starts_with(&format!("{prefix};")) {
            content = &content[2..];
        } else {
            let found = content.chars().next().unwrap();
            return Err(ConvertError::InvalidFormat(prefix, found));
        }
    }

    // Parse the array items.
    let mut items: Vec<I> = Vec::new();
    while !content.is_empty() {
        // Strip the array separator.
        let Some((mut item, remainder)) = content.split_once(',') else {
            return Err(ConvertError::UnexpectedData(content.to_string()));
        };
        content = remainder;

        // Strip the item suffix, making sure it matches the expected format.
        if let Some(suffix) = suffix {
            if let Some(stripped) = item.strip_suffix(suffix) {
                item = stripped;
            } else {
                let found = item.chars().next_back().unwrap();
                return Err(ConvertError::InvalidFormat(suffix, found));
            }
        }

        // Parse the item and add it to the list.
        let item =
            item.parse::<I>().map_err(|_| ConvertError::FromString(std::any::type_name::<I>()))?;
        items.push(item);
    }

    Ok((T::from(items), remaining))
}

/// Read the content enclosed by a pair of characters.
fn read_enclosed(content: &str, open: char, close: char) -> Result<(&str, &str), ConvertError> {
    let mut counter = 0;
    for (i, char) in content.chars().enumerate() {
        match char {
            c if c == open => counter += 1,
            c if c == close => {
                counter -= 1;
                if counter == 0 {
                    return Ok(content.split_at(i));
                }
            }
            _ => {}
        }
    }
    Err(ConvertError::UnexpectedData(content.to_string()))
}

/// Write an array to a string.
///
/// # Examples
/// - `(None, None, [1, 2, 3])` -> `"[1,2,3]"`
/// - `(None, Some('S'), [1, 2, 3])` -> `"[1S,2S,3S]"`
/// - `(Some('B'), Some('B'), [1, 2, 3])` -> `"[B;1B,2B,3B]"`
fn write_array<T: ToString>(
    prefix: Option<char>,
    suffix: Option<char>,
    iter: impl ExactSizeIterator<Item = T>,
    content: &mut String,
) {
    content.push('[');

    // Push the array prefix.
    if let Some(prefix) = prefix {
        content.push(prefix);
        content.push(';');
    }

    // Push the array items, separated by commas.
    let last = iter.len().saturating_sub(1);
    for (i, item) in iter.enumerate() {
        content.push_str(&item.to_string());
        if let Some(char) = suffix {
            content.push(char);
        }

        if i < last {
            content.push(',');
        }
    }

    content.push(']');
}

// -------------------------------------------------------------------------------------------------

impl ConvertCompat for Mutf8String {
    fn read_from_string(_content: &str) -> Result<(Self, &str), ConvertError> {
        todo!()
        // if content.starts_with('\"') && content.ends_with('\"') {
        //     // Un-escape double quotes.
        //     let content = &content[1..content.len() - 1].replace("\\\"",
        // "\"");     Ok((Mutf8String::from_string(content), ""))
        // } else if content.starts_with('\'') && content.ends_with('\'') {
        //     // Un-escape single quotes.
        //     let content = &content[1..content.len() - 1].replace("\\'", "'");
        //     Ok((Mutf8String::from_string(content), ""))
        // } else {
        //     Ok((Mutf8String::from_string(content), ""))
        // }
    }

    /// If a string only contains basic characters,
    /// it can be written without quotes.
    fn write_to_string(&self, content: &mut String) {
        let string = self.try_as_string().unwrap();
        if STRING_REGEX.is_match_at(&string, 0) {
            content.push_str(&string);
        } else {
            content.push('"');
            // Escape quotes.
            content.push_str(&string.replace('"', "\\\""));
            content.push('"');
        }
    }
}
