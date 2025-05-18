#[cfg(not(feature = "std"))]
use alloc::{borrow::Cow, vec, vec::Vec};
#[cfg(feature = "std")]
use std::borrow::Cow;

use crate::{
    prelude::*,
    text::{content::TextContent, interaction::TextInteraction},
    translate::TextTranslations,
};

#[test]
fn chat_message() {
    let t = TextTranslations::default();

    assert_eq!(
        FormattedText::from_string("Hello, World!").as_message(&t).unwrap(),
        "Hello, World!"
    );

    assert_eq!(
        FormattedText::from_string("Hello, World!").as_message(&t).unwrap(),
        "Hello, World!"
    );

    assert_eq!(
        FormattedText::from_string("Hello, World!")
            .with_style(TextStyle::default().with_color(PresetColor::Blue))
            .as_message(&t)
            .unwrap(),
        "Hello, World!"
    );

    assert_eq!(
        FormattedText {
            content: TextContent::Text(Cow::Borrowed("Hello, ").into()),
            style: TextStyle::default(),
            interaction: TextInteraction::default(),
            children: vec![FormattedText {
                content: TextContent::Text(Cow::Borrowed("World!").into()),
                style: TextStyle::default(),
                interaction: TextInteraction::default(),
                children: Vec::new(),
            }],
        }
        .as_message(&t)
        .unwrap(),
        "Hello, World!"
    );

    assert_eq!(
        FormattedText {
            content: TextContent::Text(Cow::Borrowed("Hello, ").into()),
            style: TextStyle::default().with_underline(true),
            interaction: TextInteraction::default(),
            children: vec![FormattedText {
                content: TextContent::Text(Cow::Borrowed("World!").into()),
                style: TextStyle::default().with_underline(false).with_italic(true),
                interaction: TextInteraction::default(),
                children: Vec::new(),
            }],
        }
        .as_message(&t)
        .unwrap(),
        "Hello, World!"
    );
}

#[test]
#[cfg(feature = "ansi")]
fn chat_message_ansi() {
    let t = TextTranslations::default();

    assert_eq!(
        FormattedText::from_string("Hello, World!").as_message_ansi(&t).unwrap().to_string(),
        "Hello, World!"
    );

    assert_eq!(
        FormattedText::from_string("Hello, World!")
            .with_style(TextStyle::default())
            .as_message_ansi(&t)
            .unwrap()
            .to_string(),
        "Hello, World!"
    );

    let text = FormattedText::from_string("Hello, World!")
        .with_style(TextStyle::default().with_color(PresetColor::Blue));
    let message = text.as_message_ansi(&t).unwrap().to_string();
    println!("{message}");
    assert_eq!(message, "\u{1b}[38;2;85;85;255mHello, World!\u{1b}[0m");

    let text = FormattedText::from_string("Hello, World!")
        .with_style(TextStyle::default().with_color(IntegerColor::new(0x999999)));
    let message = text.as_message_ansi(&t).unwrap().to_string();
    println!("{message}");
    assert_eq!(message, "\u{1b}[38;2;153;153;153mHello, World!\u{1b}[0m");

    let text = FormattedText {
        content: TextContent::Text(Cow::Borrowed("Hello, ").into()),
        style: TextStyle::default(),
        interaction: TextInteraction::default(),
        children: vec![FormattedText {
            content: TextContent::Text(Cow::Borrowed("World!").into()),
            style: TextStyle::default(),
            interaction: TextInteraction::default(),
            children: Vec::new(),
        }],
    };
    let message = text.as_message_ansi(&t).unwrap().to_string();
    println!("{message}");
    assert_eq!(message, "Hello, World!");

    let text = FormattedText {
        content: TextContent::Text(Cow::Borrowed("Hello, ").into()),
        style: TextStyle::default().with_underline(true),
        interaction: TextInteraction::default(),
        children: vec![FormattedText {
            content: TextContent::Text(Cow::Borrowed("World!").into()),
            style: TextStyle::default().with_underline(false).with_bold(true),
            interaction: TextInteraction::default(),
            children: Vec::new(),
        }],
    };

    let message = text.as_message_ansi(&t).unwrap().to_string();
    println!("{message}");
    assert_eq!(message, "\u{1b}[4mHello, \u{1b}[0m\u{1b}[1mWorld!\u{1b}[0m");

    let text = FormattedText {
        content: TextContent::Text(Cow::Borrowed("Hello, ").into()),
        style: TextStyle::default().with_underline(true),
        interaction: TextInteraction::default(),
        children: vec![FormattedText {
            content: TextContent::Text(Cow::Borrowed("World!").into()),
            style: TextStyle::default().with_bold(true),
            interaction: TextInteraction::default(),
            children: Vec::new(),
        }],
    };

    let message = text.as_message_ansi(&t).unwrap().to_string();
    println!("{message}");
    assert_eq!(message, "\u{1b}[4mHello, \u{1b}[0m\u{1b}[1;4mWorld!\u{1b}[0m");

    let text = FormattedText {
        content: TextContent::Text(Cow::Borrowed("Hello, ").into()),
        style: TextStyle::default().with_strikethrough(true),
        interaction: TextInteraction::default(),
        children: vec![FormattedText {
            content: TextContent::Text(Cow::Borrowed("World").into()),
            style: TextStyle::default().with_underline(true).with_strikethrough(false),
            interaction: TextInteraction::default(),
            children: vec![FormattedText {
                content: TextContent::Text(Cow::Borrowed("!").into()),
                style: TextStyle::default().with_color(PresetColor::Red).with_strikethrough(true),
                interaction: TextInteraction::default(),
                children: Vec::new(),
            }],
        }],
    };

    let message = text.as_message_ansi(&t).unwrap().to_string();
    println!("{message}");
    assert_eq!(
        message,
        "\u{1b}[9mHello, \u{1b}[0m\u{1b}[4mWorld\u{1b}[0m\u{1b}[4;9;38;2;255;85;85m!\u{1b}[0m"
    );

    let text = FormattedText::new("Hello, ")
        .with_style(TextStyle::default().with_strikethrough(true))
        .with_children(vec![
            FormattedText::new("World")
                .with_style(TextStyle::default().with_underline(true).with_strikethrough(false))
                .with_children(vec![FormattedText::new("!").with_style(
                    TextStyle::default().with_color(PresetColor::Red).with_strikethrough(true),
                )]),
        ]);

    let message = text.as_message_ansi(&t).unwrap().to_string();
    println!("{message}");
    assert_eq!(
        message,
        "\u{1b}[9mHello, \u{1b}[0m\u{1b}[4mWorld\u{1b}[0m\u{1b}[4;9;38;2;255;85;85m!\u{1b}[0m"
    );
}
