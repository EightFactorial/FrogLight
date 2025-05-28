#[cfg(not(feature = "std"))]
use alloc::string::String;
use core::any::type_name;

use froglight_nbt::prelude::*;

use super::{
    FormattedTextRef,
    content::{
        KeybindComponent, ScoreComponent, SelectorComponent, TextComponent, TranslateComponent,
        ValueComponent,
    },
};
use crate::prelude::*;

impl FromTag for FormattedText {
    fn from_tag(tag: &NbtTag) -> Result<Self, NbtError> {
        match tag {
            // Use the string directly as the text.
            NbtTag::String(string) => Ok(Self::from_string(string.to_string_lossy())),
            // Use the first string as the text, and the rest as children.
            NbtTag::List(NbtListTag::String(strings)) => {
                let mut strings = strings.iter();
                match strings.next() {
                    Some(first) => Ok(Self::from_string(first.to_string_lossy())
                        .with_children(strings.map(|s| Self::from_string(s.to_string_lossy())))),
                    None => Ok(Self::from_string("")),
                }
            }
            // Parse the compound as a formatted text.
            NbtTag::Compound(compound) => Self::from_compound(compound),
            _ => Err(NbtError::MismatchedTag(type_name::<Self>(), "String, List, or Compound")),
        }
    }
}

impl FormattedText {
    /// Parse the type from an [`NbtCompound`].
    ///
    /// # Errors
    /// Returns an error if the type fails to parse.
    pub fn from_compound(nbt: &NbtCompound) -> Result<Self, NbtError> {
        // Parse the type of content and the content itself.
        let content: TextContent = match nbt.get_tag("type") {
            // Use the "type" field to determine the content type.
            Some(NbtTag::String(kind)) => match kind.to_str_lossy().as_ref() {
                "text" => TextComponent::from_compound(nbt)?.into(),
                "translatable" => TranslateComponent::from_compound(nbt)?.into(),
                "score" => ScoreComponent::from_compound(nbt)?.into(),
                "selector" => SelectorComponent::from_compound(nbt)?.into(),
                "keybind" => KeybindComponent::from_compound(nbt)?.into(),
                "nbt" => ValueComponent::from_compound(nbt)?.into(),
                unknown => todo!("Unknown type: {unknown}"),
            },
            Some(..) => Err(NbtError::MismatchedTag(type_name::<Self>(), "String"))?,
            // Guess the content type based on the fields present.
            None => {
                if nbt.contains_key("text") {
                    TextComponent::from_compound(nbt)?.into()
                } else if nbt.contains_key("translate") {
                    TranslateComponent::from_compound(nbt)?.into()
                } else if nbt.contains_key("score") {
                    ScoreComponent::from_compound(nbt)?.into()
                } else if nbt.contains_key("selector") {
                    SelectorComponent::from_compound(nbt)?.into()
                } else if nbt.contains_key("keybind") {
                    KeybindComponent::from_compound(nbt)?.into()
                } else if nbt.contains_key("nbt") {
                    ValueComponent::from_compound(nbt)?.into()
                } else {
                    todo!("Missing all identifying fields in compound: {nbt:?}")
                }
            }
        };

        // Parse the style and interaction components.
        let style = TextStyle::from_compound(nbt).unwrap_or(TextStyle::EMPTY);
        let interaction = TextInteraction::from_compound(nbt).unwrap_or(TextInteraction::DEFAULT);

        // Parse and collect any children.
        let mut children = Vec::new();
        match nbt.get_tag("extra") {
            Some(NbtTag::List(NbtListTag::String(strings))) => {
                for string in strings {
                    let string = string.try_as_string().map_err(|err| {
                        NbtError::ConversionError(type_name::<String>(), Box::new(err))
                    })?;
                    children.push(Self::from_string(string));
                }
            }
            Some(NbtTag::List(NbtListTag::Compound(compounds))) => {
                for compound in compounds {
                    children.push(FormattedText::from_compound(compound)?);
                }
            }
            Some(..) => {
                return Err(NbtError::MismatchedTag(
                    type_name::<Self>(),
                    "List of Strings or Compounds",
                ));
            }
            None => {}
        }

        Ok(FormattedText { content, style, interaction, children })
    }
}

// -------------------------------------------------------------------------------------------------

impl IntoTag for FormattedText {
    fn into_tag(&self) -> Result<NbtTag, NbtError> {
        Child(FormattedTextRef::new(self), &TextStyle::EMPTY).into_tag()
    }
}
impl IntoTag for FormattedTextRef<'_, '_> {
    fn into_tag(&self) -> Result<NbtTag, NbtError> { Child(*self, &TextStyle::EMPTY).into_tag() }
}

/// Serialize a slice of [`FormattedText`] children while inheriting formatting.
struct Children<'a>(&'a [FormattedText], &'a TextStyle);
impl IntoTag for Children<'_> {
    fn into_tag(&self) -> Result<NbtTag, NbtError> {
        let mut tags = Vec::new();
        for child in self.0 {
            tags.push(Child(child.into(), self.1).into_tag()?);
        }

        // Based on the type of all tags,
        // return either a list of strings or a list of compounds.
        if tags.iter().any(|t| !matches!(t, NbtTag::String(..))) {
            let mut compounds = Vec::new();
            for tag in tags {
                match tag {
                    tag @ NbtTag::String(..) => compounds
                        .push(NbtCompound::from_iter([(Mutf8String::from_string("text"), tag)])),
                    NbtTag::Compound(compound) => compounds.push(compound),
                    _ => unreachable!("Tags can only be `String` or `Compound`"),
                }
            }

            Ok(NbtTag::List(NbtListTag::Compound(compounds)))
        } else {
            let mut strings = Vec::new();
            for tag in tags {
                match tag {
                    NbtTag::String(string) => strings.push(string),
                    _ => unreachable!("All tags were already checked to be `String`s"),
                }
            }

            Ok(NbtTag::List(NbtListTag::String(strings)))
        }
    }
}

/// Serialize a [`FormattedText`] while inheriting formatting.
struct Child<'a>(FormattedTextRef<'a, 'a>, &'a TextStyle);
impl IntoTag for Child<'_> {
    fn into_tag(&self) -> Result<NbtTag, NbtError> {
        let mut compound: NbtCompound;

        // Prepare formatting arguments
        let inherit = self.0.style.inherit(self.1);
        let diff = inherit.difference(self.1);

        match &self.0.content {
            TextContent::Text(c) => {
                if diff.is_empty() && self.0.interaction.is_empty() && self.0.children.is_empty() {
                    return Ok(NbtTag::String(Mutf8String::from_string(c)));
                }

                compound = NbtCompound::from_iter([(
                    Mutf8String::from_string("type"),
                    Mutf8String::from_string("text"),
                )]);
                compound.extend(c.into_compound()?);
            }
            TextContent::Translation(c) => {
                compound = NbtCompound::from_iter([(
                    Mutf8String::from_string("type"),
                    Mutf8String::from_string("translatable"),
                )]);
                compound.extend(c.into_compound()?);
            }
            TextContent::Score(c) => {
                compound = NbtCompound::from_iter([(
                    Mutf8String::from_string("type"),
                    Mutf8String::from_string("score"),
                )]);
                compound.extend(c.into_compound()?);
            }
            TextContent::Selector(c) => {
                compound = NbtCompound::from_iter([(
                    Mutf8String::from_string("type"),
                    Mutf8String::from_string("selector"),
                )]);
                compound.extend(c.into_compound()?);
            }
            TextContent::Keybind(c) => {
                compound = NbtCompound::from_iter([(
                    Mutf8String::from_string("type"),
                    Mutf8String::from_string("keybind"),
                )]);
                compound.extend(c.into_compound()?);
            }
            TextContent::Nbt(c) => {
                compound = NbtCompound::from_iter([(
                    Mutf8String::from_string("type"),
                    Mutf8String::from_string("nbt"),
                )]);
                compound.extend(c.into_compound()?);
            }
        }

        // Serialize the text's children recursively
        if !self.0.children.is_empty() {
            compound.insert(
                Mutf8String::from_string("extra"),
                Children(&self.0.children, &inherit).into_tag()?,
            );
        }

        // Serialize the differences from the parent text's formatting
        compound.extend(diff.into_compound()?);

        // Serialize the text's interaction settings
        compound.extend(self.0.interaction.into_compound()?);

        Ok(NbtTag::Compound(compound))
    }
}
