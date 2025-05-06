//! [`FromCompound`] and [`IntoCompound`] implementations for [`FormattedText`]
#[cfg(feature = "io")]
use froglight_io::prelude::*;
use froglight_nbt::{convert::ConvertError, nbt::NbtListTag, prelude::*};

use super::{
    FormattedContent, FormattedText, TextInteraction,
    component::{
        KeybindComponent, ScoreComponent, SelectorComponent, TextComponent, TranslateComponent,
        ValueComponent, ValueComponentSource, ValueSourceKind,
    },
    formatting::{TextColor, TextFormatting},
};

#[cfg(feature = "io")]
impl FrogRead for FormattedText {
    fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, ReadError> {
        NbtTag::frog_read(buffer).and_then(|tag| Self::from_tag(&tag).map_err(|_err| todo!()))
    }
}
#[cfg(feature = "io")]
impl FrogWrite for FormattedText {
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        Self::into_tag(self).map_err(|_err| todo!()).and_then(|tag| tag.frog_write(buffer))
    }

    fn frog_len(&self) -> usize {
        match Self::into_tag(self) {
            Ok(tag) => tag.frog_len(),
            Err(err) => panic!("Failed to convert FormattedText into NbtTag: {err}"),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl FromTag for FormattedText {
    fn from_tag(tag: &NbtTag) -> Result<Self, ConvertError> {
        match tag {
            NbtTag::String(s) => Self::from_mutf8_string(s),
            NbtTag::Compound(c) => Self::from_compound(c),
            _ => Err(ConvertError::MismatchedTag(
                core::any::type_name::<Self>(),
                "String or Compound",
            )),
        }
    }
}

impl FormattedText {
    /// Parse a [`FormattedText`] from an [`NbtCompound`].
    ///
    /// # Errors
    /// Returns an error if the compound is not a valid [`FormattedText`].
    pub fn from_compound(c: &NbtCompound) -> Result<FormattedText, ConvertError> {
        // Parse the content
        let content = FormattedContent::from_compound(c)?;
        let mut text = FormattedText::from_content(content);

        // Parse the children
        match c.get_tag("extra") {
            Some(NbtTag::List(children)) => match children {
                NbtListTag::String(strings) => {
                    for string in strings {
                        text.children.push(Self::from_mutf8_string(string)?);
                    }
                }
                NbtListTag::Compound(compounds) => {
                    for compound in compounds {
                        text.children.push(Self::from_compound(compound)?);
                    }
                }
                _ => {
                    return Err(ConvertError::MismatchedTag(
                        core::any::type_name::<Self>(),
                        "String or Compound",
                    ));
                }
            },
            Some(..) => Err(ConvertError::MismatchedTag(core::any::type_name::<Self>(), "extra"))?,
            None => {}
        }

        // Parse the formatting
        if let Ok(formatting) = TextFormatting::from_compound(c) {
            text = text.with_formatting(formatting);
        }

        // Parse the interaction settings
        if let Ok(interaction) = TextInteraction::from_compound(c) {
            text = text.with_interaction(interaction);
        }

        Ok(text)
    }

    /// Parse a [`FormattedText`] from an [`Mutf8String`].
    ///
    /// # Errors (Never Occurs)
    /// Returns an error if the string is not a valid [`FormattedText`].
    pub fn from_mutf8_string(string: &Mutf8String) -> Result<FormattedText, ConvertError> {
        Ok(FormattedText::from_content(FormattedContent::Text(TextComponent {
            text: string.to_string_lossy().into(),
        })))
    }
}

impl IntoTag for FormattedText {
    fn into_tag(&self) -> Result<NbtTag, ConvertError> {
        // Return a string tag if there is no formatting or children
        if let FormattedContent::Text(text) = &self.content {
            if self.formatting.is_empty() && self.interact.is_empty() && self.children.is_empty() {
                return Ok(NbtTag::String(Mutf8String::from_string(&text)));
            }
        }

        // Start building a compound tag
        let mut compound = NbtCompound::new();

        // Convert all children into `NbtTag`s, tracking their types
        let mut list = Vec::new();
        let mut list_type = 0u8;
        for child in &self.children {
            let child = child.into_tag()?;
            match (&child, list_type) {
                (NbtTag::String(..), 0 | 1) => list_type = 1,
                _ => list_type = 2,
            }
            list.push(child);
        }

        // Add the list of children, if there are any
        match list_type {
            // Empty list, skip adding anything
            0 => {}
            // All children are strings, add them as a list of strings
            1 => {
                let list = list.into_iter().map(|tag| tag.unwrap_string()).collect();
                compound.insert("extra", NbtTag::List(NbtListTag::String(list)));
            }
            // All children are compounds or strings, add them as a list of compounds
            2 => {
                let list = list
                    .into_iter()
                    .map(|tag| match tag {
                        NbtTag::String(str) => NbtCompound::from_iter([("text", str)]),
                        NbtTag::Compound(c) => c,
                        _ => unreachable!(),
                    })
                    .collect();
                compound.insert("extra", NbtTag::List(NbtListTag::Compound(list)));
            }
            _ => unreachable!(),
        }

        // Add the formatting
        compound.extend(self.formatting.into_compound()?);

        // Add the interaction settings
        compound.extend(self.interact.into_compound()?);

        Ok(NbtTag::Compound(compound))
    }
}

// -------------------------------------------------------------------------------------------------

impl FromCompound for FormattedContent {
    fn from_compound(c: &NbtCompound) -> Result<Self, ConvertError> {
        let ttag = c.get_tag("type").and_then(|tag| tag.as_string());
        let ttag = ttag.map(|str| str.to_str_lossy());
        let ttag = ttag.as_ref();

        // Parse the content
        if ttag.is_some_and(|t| t == "text") || c.contains_key("text") {
            Ok(FormattedContent::Text(TextComponent::from_compound(c)?))
        } else if ttag.is_some_and(|t| t == "translatable") || c.contains_key("translate") {
            Ok(FormattedContent::Translation(TranslateComponent::from_compound(c)?))
        } else if ttag.is_some_and(|t| t == "score") || c.contains_key("score") {
            Ok(FormattedContent::Score(ScoreComponent::from_compound(c)?))
        } else if ttag.is_some_and(|t| t == "selector") || c.contains_key("selector") {
            Ok(FormattedContent::Selector(SelectorComponent::from_compound(c)?))
        } else if ttag.is_some_and(|t| t == "keybind") || c.contains_key("keybind") {
            Ok(FormattedContent::Keybind(KeybindComponent::from_compound(c)?))
        } else if ttag.is_some_and(|t| t == "nbt") || c.contains_key("nbt") {
            Ok(FormattedContent::Nbt(ValueComponent::from_compound(c)?))
        } else {
            Err(ConvertError::MissingField(core::any::type_name::<Self>(), "type"))
        }
    }
}
impl IntoCompound for FormattedContent {
    fn into_compound(&self) -> Result<NbtCompound, ConvertError> {
        let mut compound = NbtCompound::new();
        match self {
            FormattedContent::Text(c) => {
                compound.insert("type", "text");
                compound.extend(c.into_compound()?);
            }
            FormattedContent::Translation(c) => {
                compound.insert("type", "translatable");
                compound.extend(c.into_compound()?);
            }
            FormattedContent::Score(c) => {
                compound.insert("type", "score");
                compound.extend(c.into_compound()?);
            }
            FormattedContent::Selector(c) => {
                compound.insert("type", "selector");
                compound.extend(c.into_compound()?);
            }
            FormattedContent::Keybind(c) => {
                compound.insert("type", "keybind");
                compound.extend(c.into_compound()?);
            }
            FormattedContent::Nbt(c) => {
                compound = NbtCompound::new();
                compound.insert("type", "nbt");
                compound.extend(c.into_compound()?);
            }
        }
        Ok(compound)
    }
}

impl FromCompound for TranslateComponent {
    fn from_compound(_: &NbtCompound) -> Result<Self, ConvertError> { todo!() }
}
impl IntoCompound for TranslateComponent {
    fn into_compound(&self) -> Result<NbtCompound, ConvertError> { todo!() }
}

impl FromCompound for ValueComponentSource {
    fn from_compound(_: &NbtCompound) -> Result<Self, ConvertError> { todo!() }
}
impl IntoCompound for ValueComponentSource {
    fn into_compound(&self) -> Result<NbtCompound, ConvertError> { todo!() }
}

impl FromCompound for ValueSourceKind {
    fn from_compound(_: &NbtCompound) -> Result<Self, ConvertError> { todo!() }
}
impl IntoCompound for ValueSourceKind {
    fn into_compound(&self) -> Result<NbtCompound, ConvertError> { todo!() }
}

// -------------------------------------------------------------------------------------------------

impl FromCompound for TextInteraction {
    fn from_compound(_: &NbtCompound) -> Result<Self, ConvertError> { todo!() }
}
impl IntoCompound for TextInteraction {
    fn into_compound(&self) -> Result<NbtCompound, ConvertError> { todo!() }
}

// -------------------------------------------------------------------------------------------------

impl FromCompound for TextFormatting {
    fn from_compound(_: &NbtCompound) -> Result<Self, ConvertError> { todo!() }
}
impl IntoCompound for TextFormatting {
    fn into_compound(&self) -> Result<NbtCompound, ConvertError> { todo!() }
}

impl FromCompound for TextColor {
    fn from_compound(_: &NbtCompound) -> Result<Self, ConvertError> { todo!() }
}
impl IntoCompound for TextColor {
    fn into_compound(&self) -> Result<NbtCompound, ConvertError> { todo!() }
}
