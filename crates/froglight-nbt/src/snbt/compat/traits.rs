use super::regex::STRING_REGEX;
use crate::{
    convert::ConvertError,
    mutf8::Mutf8String,
    nbt::{
        ByteArray, DoubleArray, FloatArray, IntArray, LongArray, NbtCompound, NbtListTag, NbtTag,
        ShortArray,
    },
};

pub(super) trait ConvertCompat: Sized {
    /// Read the content from a string, returning the remaining content.
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError>;
    /// Write the content to a string.
    fn write_to_string(&self, content: &mut String);
}

impl ConvertCompat for NbtCompound {
    fn read_from_string(_content: &str) -> Result<(Self, &str), ConvertError> { todo!() }

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
    fn read_from_string(_content: &str) -> Result<(Self, &str), ConvertError> { todo!() }

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
    fn read_from_string(_content: &str) -> Result<(Self, &str), ConvertError> { todo!() }

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

impl ConvertCompat for ByteArray {
    fn read_from_string(_content: &str) -> Result<(Self, &str), ConvertError> { todo!() }

    /// ByteArray-specific format: `[B;{BYTE}B(,{BYTE}B)*]`
    fn write_to_string(&self, content: &mut String) {
        write_array(Some('B'), Some('B'), self.iter(), content);
    }
}

impl ConvertCompat for IntArray {
    fn read_from_string(_content: &str) -> Result<(Self, &str), ConvertError> { todo!() }

    /// IntArray-specific format: `[I;{INT}(,{INT})*]`
    fn write_to_string(&self, content: &mut String) {
        write_array(Some('I'), None, self.iter(), content);
    }
}

impl ConvertCompat for LongArray {
    fn read_from_string(_content: &str) -> Result<(Self, &str), ConvertError> { todo!() }

    /// LongArray-specific format: `[L;{LONG}L(,{LONG}L)*]`
    fn write_to_string(&self, content: &mut String) {
        write_array(Some('L'), Some('L'), self.iter(), content);
    }
}

impl ConvertCompat for ShortArray {
    fn read_from_string(_content: &str) -> Result<(Self, &str), ConvertError> { todo!() }

    /// List-based format: `[{SHORT}S(,{SHORT}S)*]`
    fn write_to_string(&self, content: &mut String) {
        write_array(None, Some('S'), self.iter(), content);
    }
}

impl ConvertCompat for FloatArray {
    fn read_from_string(_content: &str) -> Result<(Self, &str), ConvertError> { todo!() }

    /// List-based format: `[{FLOAT}F(,{FLOAT}F)*]`
    fn write_to_string(&self, content: &mut String) {
        write_array(None, Some('F'), self.iter(), content);
    }
}

impl ConvertCompat for DoubleArray {
    fn read_from_string(_content: &str) -> Result<(Self, &str), ConvertError> { todo!() }

    /// List-based format: `[{DOUBLE}(,{DOUBLE})*]`
    fn write_to_string(&self, content: &mut String) {
        write_array(None, None, self.iter(), content);
    }
}

fn write_array<T: ToString>(
    prefix: Option<char>,
    char: Option<char>,
    iter: impl ExactSizeIterator<Item = T>,
    content: &mut String,
) {
    content.push('[');
    if let Some(prefix) = prefix {
        content.push(prefix);
        content.push(';');
    }

    let last = iter.len().saturating_sub(1);
    for (i, item) in iter.enumerate() {
        content.push_str(&item.to_string());
        if let Some(char) = char {
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
    fn read_from_string(content: &str) -> Result<(Self, &str), ConvertError> {
        Ok((Mutf8String::from_string(content), ""))
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
