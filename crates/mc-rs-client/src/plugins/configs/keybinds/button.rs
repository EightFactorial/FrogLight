use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// An enum representing a button on a keyboard *or* mouse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u32)]
pub enum Button {
    /// The `1` key over the letters.
    Key1,
    /// The `2` key over the letters.
    Key2,
    /// The `3` key over the letters.
    Key3,
    /// The `4` key over the letters.
    Key4,
    /// The `5` key over the letters.
    Key5,
    /// The `6` key over the letters.
    Key6,
    /// The `7` key over the letters.
    Key7,
    /// The `8` key over the letters.
    Key8,
    /// The `9` key over the letters.
    Key9,
    /// The `0` key over the letters.
    Key0,

    /// The `A` key.
    A,
    /// The `B` key.
    B,
    /// The `C` key.
    C,
    /// The `D` key.
    D,
    /// The `E` key.
    E,
    /// The `F` key.
    F,
    /// The `G` key.
    G,
    /// The `H` key.
    H,
    /// The `I` key.
    I,
    /// The `J` key.
    J,
    /// The `K` key.
    K,
    /// The `L` key.
    L,
    /// The `M` key.
    M,
    /// The `N` key.
    N,
    /// The `O` key.
    O,
    /// The `P` key.
    P,
    /// The `Q` key.
    Q,
    /// The `R` key.
    R,
    /// The `S` key.
    S,
    /// The `T` key.
    T,
    /// The `U` key.
    U,
    /// The `V` key.
    V,
    /// The `W` key.
    W,
    /// The `X` key.
    X,
    /// The `Y` key.
    Y,
    /// The `Z` key.
    Z,

    /// The `Escape` / `ESC` key, next to the `F1` key.
    Escape,

    /// The `F1` key.
    F1,
    /// The `F2` key.
    F2,
    /// The `F3` key.
    F3,
    /// The `F4` key.
    F4,
    /// The `F5` key.
    F5,
    /// The `F6` key.
    F6,
    /// The `F7` key.
    F7,
    /// The `F8` key.
    F8,
    /// The `F9` key.
    F9,
    /// The `F10` key.
    F10,
    /// The `F11` key.
    F11,
    /// The `F12` key.
    F12,
    /// The `F13` key.
    F13,
    /// The `F14` key.
    F14,
    /// The `F15` key.
    F15,
    /// The `F16` key.
    F16,
    /// The `F17` key.
    F17,
    /// The `F18` key.
    F18,
    /// The `F19` key.
    F19,
    /// The `F20` key.
    F20,
    /// The `F21` key.
    F21,
    /// The `F22` key.
    F22,
    /// The `F23` key.
    F23,
    /// The `F24` key.
    F24,

    /// The `Snapshot` / `Print Screen` key.
    Snapshot,
    /// The `Scroll` / `Scroll Lock` key.
    Scroll,
    /// The `Pause` / `Break` key, next to the `Scroll` key.
    Pause,

    /// The `Insert` key, next to the `Backspace` key.
    Insert,
    /// The `Home` key.
    Home,
    /// The `Delete` key.
    Delete,
    /// The `End` key.
    End,
    /// The `PageDown` key.
    PageDown,
    /// The `PageUp` key.
    PageUp,

    /// The `Left` / `Left Arrow` key.
    Left,
    /// The `Up` / `Up Arrow` key.
    Up,
    /// The `Right` / `Right Arrow` key.
    Right,
    /// The `Down` / `Down Arrow` key.
    Down,

    /// The `Back` / `Backspace` key.
    Back,
    /// The `Return` / `Enter` key.
    Return,
    /// The `Space` / `Spacebar` / ` ` key.
    Space,

    /// The `Compose` key on Linux.
    Compose,
    /// The `Caret` / `^` key.
    Caret,

    /// The `Numlock` key.
    Numlock,
    /// The `Numpad0` / `0` key.
    Numpad0,
    /// The `Numpad1` / `1` key.
    Numpad1,
    /// The `Numpad2` / `2` key.
    Numpad2,
    /// The `Numpad3` / `3` key.
    Numpad3,
    /// The `Numpad4` / `4` key.
    Numpad4,
    /// The `Numpad5` / `5` key.
    Numpad5,
    /// The `Numpad6` / `6` key.
    Numpad6,
    /// The `Numpad7` / `7` key.
    Numpad7,
    /// The `Numpad8` / `8` key.
    Numpad8,
    /// The `Numpad9` / `9` key.
    Numpad9,

    /// The `AbntC1` key.
    AbntC1,
    /// The `AbntC2` key.
    AbntC2,

    /// The `NumpadAdd` / `+` key.
    NumpadAdd,
    /// The `Apostrophe` / `'` key.
    Apostrophe,
    /// The `Apps` key.
    Apps,
    /// The `Asterisk` / `*` key.
    Asterisk,
    /// The `Plus` / `+` key.
    Plus,
    /// The `At` / `@` key.
    At,
    /// The `Ax` key.
    Ax,
    /// The `Backslash` / `\` key.
    Backslash,
    /// The `Calculator` key.
    Calculator,
    /// The `Capital` key.
    Capital,
    /// The `Colon` / `:` key.
    Colon,
    /// The `Comma` / `,` key.
    Comma,
    /// The `Convert` key.
    Convert,
    /// The `NumpadDecimal` / `.` key.
    NumpadDecimal,
    /// The `NumpadDivide` / `/` key.
    NumpadDivide,
    /// The `Equals` / `=` key.
    Equals,
    /// The `Grave` / `Backtick` / `` ` `` key.
    Grave,
    /// The `Kana` key.
    Kana,
    /// The `Kanji` key.
    Kanji,

    /// The `Left Alt` key. Maps to `Left Option` on Mac.
    AltLeft,
    /// The `Left Bracket` / `[` key.
    BracketLeft,
    /// The `Left Control` key.
    ControlLeft,
    /// The `Left Shift` key.
    ShiftLeft,
    /// The `Left Super` key.
    /// Generic keyboards usually display this key with the *Microsoft Windows* logo.
    /// Apple keyboards call this key the *Command Key* and display it using the ⌘ character.
    #[doc(alias("LWin", "LMeta", "LLogo"))]
    SuperLeft,

    /// The `Mail` key.
    Mail,
    /// The `MediaSelect` key.
    MediaSelect,
    /// The `MediaStop` key.
    MediaStop,
    /// The `Minus` / `-` key.
    Minus,
    /// The `NumpadMultiply` / `*` key.
    NumpadMultiply,
    /// The `Mute` key.
    Mute,
    /// The `MyComputer` key.
    MyComputer,
    /// The `NavigateForward` / `Prior` key.
    NavigateForward,
    /// The `NavigateBackward` / `Next` key.
    NavigateBackward,
    /// The `NextTrack` key.
    NextTrack,
    /// The `NoConvert` key.
    NoConvert,
    /// The `NumpadComma` / `,` key.
    NumpadComma,
    /// The `NumpadEnter` key.
    NumpadEnter,
    /// The `NumpadEquals` / `=` key.
    NumpadEquals,
    /// The `Oem102` key.
    Oem102,
    /// The `Period` / `.` key.
    Period,
    /// The `PlayPause` key.
    PlayPause,
    /// The `Power` key.
    Power,
    /// The `PrevTrack` key.
    PrevTrack,

    /// The `Right Alt` key. Maps to `Right Option` on Mac.
    AltRight,
    /// The `Right Bracket` / `]` key.
    BracketRight,
    /// The `Right Control` key.
    ControlRight,
    /// The `Right Shift` key.
    ShiftRight,
    /// The `Right Super` key.
    /// Generic keyboards usually display this key with the *Microsoft Windows* logo.
    /// Apple keyboards call this key the *Command Key* and display it using the ⌘ character.
    #[doc(alias("RWin", "RMeta", "RLogo"))]
    SuperRight,

    /// The `Semicolon` / `;` key.
    Semicolon,
    /// The `Slash` / `/` key.
    Slash,
    /// The `Sleep` key.
    Sleep,
    /// The `Stop` key.
    Stop,
    /// The `NumpadSubtract` / `-` key.
    NumpadSubtract,
    /// The `Sysrq` key.
    Sysrq,
    /// The `Tab` / `   ` key.
    Tab,
    /// The `Underline` / `_` key.
    Underline,
    /// The `Unlabeled` key.
    Unlabeled,

    /// The `VolumeDown` key.
    VolumeDown,
    /// The `VolumeUp` key.
    VolumeUp,

    /// The `Wake` key.
    Wake,

    /// The `WebBack` key.
    WebBack,
    /// The `WebFavorites` key.
    WebFavorites,
    /// The `WebForward` key.
    WebForward,
    /// The `WebHome` key.
    WebHome,
    /// The `WebRefresh` key.
    WebRefresh,
    /// The `WebSearch` key.
    WebSearch,
    /// The `WebStop` key.
    WebStop,

    /// The `Yen` key.
    Yen,

    /// The `Copy` key.
    Copy,
    /// The `Paste` key.
    Paste,
    /// The `Cut` key.
    Cut,

    /// The left mouse button.
    MouseLeft,
    /// The right mouse button.
    MouseRight,
    /// The middle mouse button.
    MouseMiddle,
    /// Another mouse button with the associated number.
    MouseOther(u16),
}

impl Button {
    pub fn is_keyboard(&self) -> bool { !self.is_mouse() }

    pub fn is_mouse(&self) -> bool {
        matches!(
            self,
            Self::MouseLeft | Self::MouseRight | Self::MouseMiddle | Self::MouseOther(_)
        )
    }

    pub fn is_modifier(&self) -> bool {
        matches!(
            self,
            Self::AltLeft
                | Self::AltRight
                | Self::ControlLeft
                | Self::ControlRight
                | Self::ShiftLeft
                | Self::ShiftRight
                | Self::SuperLeft
                | Self::SuperRight
        )
    }
}

impl From<KeyCode> for Button {
    fn from(value: KeyCode) -> Self {
        match value {
            KeyCode::Key1 => Self::Key1,
            KeyCode::Key2 => Self::Key2,
            KeyCode::Key3 => Self::Key3,
            KeyCode::Key4 => Self::Key4,
            KeyCode::Key5 => Self::Key5,
            KeyCode::Key6 => Self::Key6,
            KeyCode::Key7 => Self::Key7,
            KeyCode::Key8 => Self::Key8,
            KeyCode::Key9 => Self::Key9,
            KeyCode::Key0 => Self::Key0,

            KeyCode::A => Self::A,
            KeyCode::B => Self::B,
            KeyCode::C => Self::C,
            KeyCode::D => Self::D,
            KeyCode::E => Self::E,
            KeyCode::F => Self::F,
            KeyCode::G => Self::G,
            KeyCode::H => Self::H,
            KeyCode::I => Self::I,
            KeyCode::J => Self::J,
            KeyCode::K => Self::K,
            KeyCode::L => Self::L,
            KeyCode::M => Self::M,
            KeyCode::N => Self::N,
            KeyCode::O => Self::O,
            KeyCode::P => Self::P,
            KeyCode::Q => Self::Q,
            KeyCode::R => Self::R,
            KeyCode::S => Self::S,
            KeyCode::T => Self::T,
            KeyCode::U => Self::U,
            KeyCode::V => Self::V,
            KeyCode::W => Self::W,
            KeyCode::X => Self::X,
            KeyCode::Y => Self::Y,
            KeyCode::Z => Self::Z,

            KeyCode::Escape => Self::Escape,

            KeyCode::F1 => Self::F1,
            KeyCode::F2 => Self::F2,
            KeyCode::F3 => Self::F3,
            KeyCode::F4 => Self::F4,
            KeyCode::F5 => Self::F5,
            KeyCode::F6 => Self::F6,
            KeyCode::F7 => Self::F7,
            KeyCode::F8 => Self::F8,
            KeyCode::F9 => Self::F9,
            KeyCode::F10 => Self::F10,
            KeyCode::F11 => Self::F11,
            KeyCode::F12 => Self::F12,
            KeyCode::F13 => Self::F13,
            KeyCode::F14 => Self::F14,
            KeyCode::F15 => Self::F15,
            KeyCode::F16 => Self::F16,
            KeyCode::F17 => Self::F17,
            KeyCode::F18 => Self::F18,
            KeyCode::F19 => Self::F19,
            KeyCode::F20 => Self::F20,
            KeyCode::F21 => Self::F21,
            KeyCode::F22 => Self::F22,
            KeyCode::F23 => Self::F23,
            KeyCode::F24 => Self::F24,

            KeyCode::Snapshot => Self::Snapshot,
            KeyCode::Scroll => Self::Scroll,
            KeyCode::Pause => Self::Pause,

            KeyCode::Insert => Self::Insert,
            KeyCode::Home => Self::Home,
            KeyCode::Delete => Self::Delete,
            KeyCode::End => Self::End,
            KeyCode::PageDown => Self::PageDown,
            KeyCode::PageUp => Self::PageUp,

            KeyCode::Left => Self::Left,
            KeyCode::Up => Self::Up,
            KeyCode::Right => Self::Right,
            KeyCode::Down => Self::Down,

            KeyCode::Back => Self::Back,
            KeyCode::Return => Self::Return,
            KeyCode::Space => Self::Space,

            KeyCode::Compose => Self::Compose,
            KeyCode::Caret => Self::Caret,

            KeyCode::Numlock => Self::Numlock,
            KeyCode::Numpad0 => Self::Numpad0,
            KeyCode::Numpad1 => Self::Numpad1,
            KeyCode::Numpad2 => Self::Numpad2,
            KeyCode::Numpad3 => Self::Numpad3,
            KeyCode::Numpad4 => Self::Numpad4,
            KeyCode::Numpad5 => Self::Numpad5,
            KeyCode::Numpad6 => Self::Numpad6,
            KeyCode::Numpad7 => Self::Numpad7,
            KeyCode::Numpad8 => Self::Numpad8,
            KeyCode::Numpad9 => Self::Numpad9,

            KeyCode::AbntC1 => Self::AbntC1,
            KeyCode::AbntC2 => Self::AbntC2,

            KeyCode::NumpadAdd => Self::NumpadAdd,
            KeyCode::Apostrophe => Self::Apostrophe,
            KeyCode::Apps => Self::Apps,
            KeyCode::Asterisk => Self::Asterisk,
            KeyCode::Plus => Self::Plus,
            KeyCode::At => Self::At,
            KeyCode::Ax => Self::Ax,
            KeyCode::Backslash => Self::Backslash,
            KeyCode::Calculator => Self::Calculator,
            KeyCode::Capital => Self::Capital,
            KeyCode::Colon => Self::Colon,
            KeyCode::Comma => Self::Comma,
            KeyCode::Convert => Self::Convert,
            KeyCode::NumpadDecimal => Self::NumpadDecimal,
            KeyCode::NumpadDivide => Self::NumpadDivide,
            KeyCode::Equals => Self::Equals,
            KeyCode::Grave => Self::Grave,
            KeyCode::Kana => Self::Kana,
            KeyCode::Kanji => Self::Kanji,

            KeyCode::AltLeft => Self::AltLeft,
            KeyCode::BracketLeft => Self::BracketLeft,
            KeyCode::ControlLeft => Self::ControlLeft,
            KeyCode::ShiftLeft => Self::ShiftLeft,
            KeyCode::SuperLeft => Self::SuperLeft,

            KeyCode::Mail => Self::Mail,
            KeyCode::MediaSelect => Self::MediaSelect,
            KeyCode::MediaStop => Self::MediaStop,
            KeyCode::Minus => Self::Minus,
            KeyCode::NumpadMultiply => Self::NumpadMultiply,
            KeyCode::Mute => Self::Mute,
            KeyCode::MyComputer => Self::MyComputer,
            KeyCode::NavigateForward => Self::NavigateForward,
            KeyCode::NavigateBackward => Self::NavigateBackward,
            KeyCode::NextTrack => Self::NextTrack,
            KeyCode::NoConvert => Self::NoConvert,
            KeyCode::NumpadComma => Self::NumpadComma,
            KeyCode::NumpadEnter => Self::NumpadEnter,
            KeyCode::NumpadEquals => Self::NumpadEquals,
            KeyCode::Oem102 => Self::Oem102,
            KeyCode::Period => Self::Period,
            KeyCode::PlayPause => Self::PlayPause,
            KeyCode::Power => Self::Power,
            KeyCode::PrevTrack => Self::PrevTrack,

            KeyCode::AltRight => Self::AltRight,
            KeyCode::BracketRight => Self::BracketRight,
            KeyCode::ControlRight => Self::ControlRight,
            KeyCode::ShiftRight => Self::ShiftRight,
            KeyCode::SuperRight => Self::SuperRight,

            KeyCode::Semicolon => Self::Semicolon,
            KeyCode::Slash => Self::Slash,
            KeyCode::Sleep => Self::Sleep,
            KeyCode::Stop => Self::Stop,
            KeyCode::NumpadSubtract => Self::NumpadSubtract,
            KeyCode::Sysrq => Self::Sysrq,
            KeyCode::Tab => Self::Tab,
            KeyCode::Underline => Self::Underline,
            KeyCode::Unlabeled => Self::Unlabeled,

            KeyCode::VolumeDown => Self::VolumeDown,
            KeyCode::VolumeUp => Self::VolumeUp,

            KeyCode::Wake => Self::Wake,

            KeyCode::WebBack => Self::WebBack,
            KeyCode::WebFavorites => Self::WebFavorites,
            KeyCode::WebForward => Self::WebForward,
            KeyCode::WebHome => Self::WebHome,
            KeyCode::WebRefresh => Self::WebRefresh,
            KeyCode::WebSearch => Self::WebSearch,
            KeyCode::WebStop => Self::WebStop,

            KeyCode::Yen => Self::Yen,

            KeyCode::Copy => Self::Copy,
            KeyCode::Paste => Self::Paste,
            KeyCode::Cut => Self::Cut,
        }
    }
}

impl From<Button> for Option<KeyCode> {
    fn from(value: Button) -> Self {
        match value {
            Button::Key1 => Some(KeyCode::Key1),
            Button::Key2 => Some(KeyCode::Key2),
            Button::Key3 => Some(KeyCode::Key3),
            Button::Key4 => Some(KeyCode::Key4),
            Button::Key5 => Some(KeyCode::Key5),
            Button::Key6 => Some(KeyCode::Key6),
            Button::Key7 => Some(KeyCode::Key7),
            Button::Key8 => Some(KeyCode::Key8),
            Button::Key9 => Some(KeyCode::Key9),
            Button::Key0 => Some(KeyCode::Key0),

            Button::A => Some(KeyCode::A),
            Button::B => Some(KeyCode::B),
            Button::C => Some(KeyCode::C),
            Button::D => Some(KeyCode::D),
            Button::E => Some(KeyCode::E),
            Button::F => Some(KeyCode::F),
            Button::G => Some(KeyCode::G),
            Button::H => Some(KeyCode::H),
            Button::I => Some(KeyCode::I),
            Button::J => Some(KeyCode::J),
            Button::K => Some(KeyCode::K),
            Button::L => Some(KeyCode::L),
            Button::M => Some(KeyCode::M),
            Button::N => Some(KeyCode::N),
            Button::O => Some(KeyCode::O),
            Button::P => Some(KeyCode::P),
            Button::Q => Some(KeyCode::Q),
            Button::R => Some(KeyCode::R),
            Button::S => Some(KeyCode::S),
            Button::T => Some(KeyCode::T),
            Button::U => Some(KeyCode::U),
            Button::V => Some(KeyCode::V),
            Button::W => Some(KeyCode::W),
            Button::X => Some(KeyCode::X),
            Button::Y => Some(KeyCode::Y),
            Button::Z => Some(KeyCode::Z),

            Button::Escape => Some(KeyCode::Escape),

            Button::F1 => Some(KeyCode::F1),
            Button::F2 => Some(KeyCode::F2),
            Button::F3 => Some(KeyCode::F3),
            Button::F4 => Some(KeyCode::F4),
            Button::F5 => Some(KeyCode::F5),
            Button::F6 => Some(KeyCode::F6),
            Button::F7 => Some(KeyCode::F7),
            Button::F8 => Some(KeyCode::F8),
            Button::F9 => Some(KeyCode::F9),
            Button::F10 => Some(KeyCode::F10),
            Button::F11 => Some(KeyCode::F11),
            Button::F12 => Some(KeyCode::F12),
            Button::F13 => Some(KeyCode::F13),
            Button::F14 => Some(KeyCode::F14),
            Button::F15 => Some(KeyCode::F15),
            Button::F16 => Some(KeyCode::F16),
            Button::F17 => Some(KeyCode::F17),
            Button::F18 => Some(KeyCode::F18),
            Button::F19 => Some(KeyCode::F19),
            Button::F20 => Some(KeyCode::F20),
            Button::F21 => Some(KeyCode::F21),
            Button::F22 => Some(KeyCode::F22),
            Button::F23 => Some(KeyCode::F23),
            Button::F24 => Some(KeyCode::F24),

            Button::Snapshot => Some(KeyCode::Snapshot),
            Button::Scroll => Some(KeyCode::Scroll),
            Button::Pause => Some(KeyCode::Pause),

            Button::Insert => Some(KeyCode::Insert),
            Button::Home => Some(KeyCode::Home),
            Button::Delete => Some(KeyCode::Delete),
            Button::End => Some(KeyCode::End),
            Button::PageDown => Some(KeyCode::PageDown),
            Button::PageUp => Some(KeyCode::PageUp),

            Button::Left => Some(KeyCode::Left),
            Button::Up => Some(KeyCode::Up),
            Button::Right => Some(KeyCode::Right),
            Button::Down => Some(KeyCode::Down),

            Button::Back => Some(KeyCode::Back),
            Button::Return => Some(KeyCode::Return),
            Button::Space => Some(KeyCode::Space),

            Button::Compose => Some(KeyCode::Compose),
            Button::Caret => Some(KeyCode::Caret),

            Button::Numlock => Some(KeyCode::Numlock),
            Button::Numpad0 => Some(KeyCode::Numpad0),
            Button::Numpad1 => Some(KeyCode::Numpad1),
            Button::Numpad2 => Some(KeyCode::Numpad2),
            Button::Numpad3 => Some(KeyCode::Numpad3),
            Button::Numpad4 => Some(KeyCode::Numpad4),
            Button::Numpad5 => Some(KeyCode::Numpad5),
            Button::Numpad6 => Some(KeyCode::Numpad6),
            Button::Numpad7 => Some(KeyCode::Numpad7),
            Button::Numpad8 => Some(KeyCode::Numpad8),
            Button::Numpad9 => Some(KeyCode::Numpad9),

            Button::AbntC1 => Some(KeyCode::AbntC1),
            Button::AbntC2 => Some(KeyCode::AbntC2),

            Button::NumpadAdd => Some(KeyCode::NumpadAdd),
            Button::Apostrophe => Some(KeyCode::Apostrophe),
            Button::Apps => Some(KeyCode::Apps),
            Button::Asterisk => Some(KeyCode::Asterisk),
            Button::Plus => Some(KeyCode::Plus),
            Button::At => Some(KeyCode::At),
            Button::Ax => Some(KeyCode::Ax),
            Button::Backslash => Some(KeyCode::Backslash),
            Button::Calculator => Some(KeyCode::Calculator),
            Button::Capital => Some(KeyCode::Capital),
            Button::Colon => Some(KeyCode::Colon),
            Button::Comma => Some(KeyCode::Comma),
            Button::Convert => Some(KeyCode::Convert),
            Button::NumpadDecimal => Some(KeyCode::NumpadDecimal),
            Button::NumpadDivide => Some(KeyCode::NumpadDivide),
            Button::Equals => Some(KeyCode::Equals),
            Button::Grave => Some(KeyCode::Grave),
            Button::Kana => Some(KeyCode::Kana),
            Button::Kanji => Some(KeyCode::Kanji),

            Button::AltLeft => Some(KeyCode::AltLeft),
            Button::BracketLeft => Some(KeyCode::BracketLeft),
            Button::ControlLeft => Some(KeyCode::ControlLeft),
            Button::ShiftLeft => Some(KeyCode::ShiftLeft),
            Button::SuperLeft => Some(KeyCode::SuperLeft),

            Button::Mail => Some(KeyCode::Mail),
            Button::MediaSelect => Some(KeyCode::MediaSelect),
            Button::MediaStop => Some(KeyCode::MediaStop),
            Button::Minus => Some(KeyCode::Minus),
            Button::NumpadMultiply => Some(KeyCode::NumpadMultiply),
            Button::Mute => Some(KeyCode::Mute),
            Button::MyComputer => Some(KeyCode::MyComputer),
            Button::NavigateForward => Some(KeyCode::NavigateForward),
            Button::NavigateBackward => Some(KeyCode::NavigateBackward),
            Button::NextTrack => Some(KeyCode::NextTrack),
            Button::NoConvert => Some(KeyCode::NoConvert),
            Button::NumpadComma => Some(KeyCode::NumpadComma),
            Button::NumpadEnter => Some(KeyCode::NumpadEnter),
            Button::NumpadEquals => Some(KeyCode::NumpadEquals),
            Button::Oem102 => Some(KeyCode::Oem102),
            Button::Period => Some(KeyCode::Period),
            Button::PlayPause => Some(KeyCode::PlayPause),
            Button::Power => Some(KeyCode::Power),
            Button::PrevTrack => Some(KeyCode::PrevTrack),

            Button::AltRight => Some(KeyCode::AltRight),
            Button::BracketRight => Some(KeyCode::BracketRight),
            Button::ControlRight => Some(KeyCode::ControlRight),
            Button::ShiftRight => Some(KeyCode::ShiftRight),
            Button::SuperRight => Some(KeyCode::SuperRight),

            Button::Semicolon => Some(KeyCode::Semicolon),
            Button::Slash => Some(KeyCode::Slash),
            Button::Sleep => Some(KeyCode::Sleep),
            Button::Stop => Some(KeyCode::Stop),
            Button::NumpadSubtract => Some(KeyCode::NumpadSubtract),
            Button::Sysrq => Some(KeyCode::Sysrq),
            Button::Tab => Some(KeyCode::Tab),
            Button::Underline => Some(KeyCode::Underline),
            Button::Unlabeled => Some(KeyCode::Unlabeled),

            Button::VolumeDown => Some(KeyCode::VolumeDown),
            Button::VolumeUp => Some(KeyCode::VolumeUp),

            Button::Wake => Some(KeyCode::Wake),

            Button::WebBack => Some(KeyCode::WebBack),
            Button::WebFavorites => Some(KeyCode::WebFavorites),
            Button::WebForward => Some(KeyCode::WebForward),
            Button::WebHome => Some(KeyCode::WebHome),
            Button::WebRefresh => Some(KeyCode::WebRefresh),
            Button::WebSearch => Some(KeyCode::WebSearch),
            Button::WebStop => Some(KeyCode::WebStop),

            Button::Yen => Some(KeyCode::Yen),

            Button::Copy => Some(KeyCode::Copy),
            Button::Paste => Some(KeyCode::Paste),
            Button::Cut => Some(KeyCode::Cut),

            _ => None,
        }
    }
}

impl From<MouseButton> for Button {
    fn from(value: MouseButton) -> Self {
        match value {
            MouseButton::Left => Self::MouseLeft,
            MouseButton::Right => Self::MouseRight,
            MouseButton::Middle => Self::MouseMiddle,
            MouseButton::Other(code) => Self::MouseOther(code),
        }
    }
}

impl From<Button> for Option<MouseButton> {
    fn from(value: Button) -> Self {
        match value {
            Button::MouseLeft => Some(MouseButton::Left),
            Button::MouseRight => Some(MouseButton::Right),
            Button::MouseMiddle => Some(MouseButton::Middle),
            Button::MouseOther(code) => Some(MouseButton::Other(code)),
            _ => None,
        }
    }
}
