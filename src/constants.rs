use crossterm::event::{KeyCode, ModifierKeyCode};
use std::collections::HashMap;

pub const KEYBOARD_PERCENTAGE: u16 = 85;
pub const SEARCH_BOX_PERCENTAGE: u16 = 75;
pub const TEXT_BOX_PERCENTAGE: u16 = 70;
pub const TIMER_BOX_PERCENTAGE: u16 = 10;
pub const SCORE_BOX_PERCENTAGE: u16 = 10;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyLength {
    SHORT,
    MEDIUM,
    LONG,
}

pub type Coord = (u32, u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Key {
    pub key_code: KeyCode,
    pub sec_key_code: Option<KeyCode>,
    pub key_length: KeyLength,
}

impl Key {
    // We will add secondary keycode here
    fn from_values(keycode: KeyCode, s_keycode: Option<KeyCode>, keylength: KeyLength) -> Self {
        Key {
            key_code: keycode,
            sec_key_code: s_keycode,
            key_length: keylength,
        }
    }
}

pub fn initialize_key_vec() -> Vec<Vec<Key>> {
    vec![
        vec![
            Key::from_values(
                KeyCode::Char('1'),
                Some(KeyCode::Char('!')),
                KeyLength::SHORT,
            ),
            Key::from_values(
                KeyCode::Char('2'),
                Some(KeyCode::Char('@')),
                KeyLength::SHORT,
            ),
            Key::from_values(
                KeyCode::Char('3'),
                Some(KeyCode::Char('#')),
                KeyLength::SHORT,
            ),
            Key::from_values(
                KeyCode::Char('4'),
                Some(KeyCode::Char('$')),
                KeyLength::SHORT,
            ),
            Key::from_values(
                KeyCode::Char('5'),
                Some(KeyCode::Char('%')),
                KeyLength::SHORT,
            ),
            Key::from_values(
                KeyCode::Char('6'),
                Some(KeyCode::Char('^')),
                KeyLength::SHORT,
            ),
            Key::from_values(
                KeyCode::Char('7'),
                Some(KeyCode::Char('&')),
                KeyLength::SHORT,
            ),
            Key::from_values(
                KeyCode::Char('8'),
                Some(KeyCode::Char('*')),
                KeyLength::SHORT,
            ),
            Key::from_values(
                KeyCode::Char('9'),
                Some(KeyCode::Char('(')),
                KeyLength::SHORT,
            ),
            Key::from_values(
                KeyCode::Char('0'),
                Some(KeyCode::Char(')')),
                KeyLength::SHORT,
            ),
            Key::from_values(
                KeyCode::Char('-'),
                Some(KeyCode::Char('_')),
                KeyLength::SHORT,
            ),
            Key::from_values(
                KeyCode::Char('='),
                Some(KeyCode::Char('+')),
                KeyLength::SHORT,
            ),
            Key::from_values(
                KeyCode::Char('\\'),
                Some(KeyCode::Char('|')),
                KeyLength::SHORT,
            ),
        ],
        vec![
            Key::from_values(KeyCode::Char('Q'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('W'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('E'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('R'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('T'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('Y'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('U'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('I'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('O'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('P'), None, KeyLength::SHORT),
            Key::from_values(
                KeyCode::Char('['),
                Some(KeyCode::Char('{')),
                KeyLength::SHORT,
            ),
            Key::from_values(
                KeyCode::Char(']'),
                Some(KeyCode::Char('}')),
                KeyLength::SHORT,
            ),
        ],
        vec![
            Key::from_values(KeyCode::Char('A'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('S'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('D'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('F'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('G'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('H'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('J'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('K'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('L'), None, KeyLength::SHORT),
            Key::from_values(
                KeyCode::Char(';'),
                Some(KeyCode::Char(':')),
                KeyLength::SHORT,
            ),
            Key::from_values(
                KeyCode::Char('\''),
                Some(KeyCode::Char('"')),
                KeyLength::SHORT,
            ),
            Key::from_values(KeyCode::Enter, None, KeyLength::MEDIUM),
        ],
        vec![
            Key::from_values(KeyCode::Char('Z'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('X'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('C'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('V'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('B'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('N'), None, KeyLength::SHORT),
            Key::from_values(KeyCode::Char('M'), None, KeyLength::SHORT),
            Key::from_values(
                KeyCode::Char(','),
                Some(KeyCode::Char('<')),
                KeyLength::SHORT,
            ),
            Key::from_values(
                KeyCode::Char('.'),
                Some(KeyCode::Char('>')),
                KeyLength::SHORT,
            ),
            Key::from_values(
                KeyCode::Char('/'),
                Some(KeyCode::Char('?')),
                KeyLength::SHORT,
            ),
        ],
        vec![
            Key::from_values(
                KeyCode::Modifier(ModifierKeyCode::LeftControl),
                None,
                KeyLength::SHORT,
            ),
            Key::from_values(
                KeyCode::Modifier(ModifierKeyCode::LeftSuper),
                None,
                KeyLength::MEDIUM,
            ),
            Key::from_values(
                KeyCode::Modifier(ModifierKeyCode::LeftAlt),
                None,
                KeyLength::SHORT,
            ),
            Key::from_values(KeyCode::Char(' '), None, KeyLength::LONG),
            Key::from_values(
                KeyCode::Modifier(ModifierKeyCode::RightAlt),
                None,
                KeyLength::SHORT,
            ),
            Key::from_values(
                KeyCode::Modifier(ModifierKeyCode::RightSuper),
                None,
                KeyLength::MEDIUM,
            ),
            Key::from_values(
                KeyCode::Modifier(ModifierKeyCode::RightControl),
                None,
                KeyLength::SHORT,
            ),
        ],
    ]
}

// map the secondary keycodes to the specific maps.
pub fn initialize_key_coord_map() -> HashMap<KeyCode, Coord> {
    HashMap::from([
        (KeyCode::Char('1'), (0, 0)),
        (KeyCode::Char('!'), (0, 0)),
        (KeyCode::Char('2'), (0, 1)),
        (KeyCode::Char('@'), (0, 1)),
        (KeyCode::Char('3'), (0, 2)),
        (KeyCode::Char('#'), (0, 2)),
        (KeyCode::Char('4'), (0, 3)),
        (KeyCode::Char('$'), (0, 3)),
        (KeyCode::Char('5'), (0, 4)),
        (KeyCode::Char('%'), (0, 4)),
        (KeyCode::Char('6'), (0, 5)),
        (KeyCode::Char('^'), (0, 5)),
        (KeyCode::Char('7'), (0, 6)),
        (KeyCode::Char('&'), (0, 6)),
        (KeyCode::Char('8'), (0, 7)),
        (KeyCode::Char('*'), (0, 7)),
        (KeyCode::Char('9'), (0, 8)),
        (KeyCode::Char('('), (0, 8)),
        (KeyCode::Char('0'), (0, 9)),
        (KeyCode::Char(')'), (0, 9)),
        // Second row
        (KeyCode::Char('Q'), (1, 0)),
        (KeyCode::Char('W'), (1, 1)),
        (KeyCode::Char('E'), (1, 2)),
        (KeyCode::Char('R'), (1, 3)),
        (KeyCode::Char('T'), (1, 4)),
        (KeyCode::Char('Y'), (1, 5)),
        (KeyCode::Char('U'), (1, 6)),
        (KeyCode::Char('I'), (1, 7)),
        (KeyCode::Char('O'), (1, 8)),
        (KeyCode::Char('P'), (1, 9)),
        (KeyCode::Char('['), (1, 10)),
        (KeyCode::Char('{'), (1, 10)),
        (KeyCode::Char(']'), (1, 11)),
        (KeyCode::Char('}'), (1, 11)),
        // Third row
        (KeyCode::Char('A'), (2, 0)),
        (KeyCode::Char('S'), (2, 1)),
        (KeyCode::Char('D'), (2, 2)),
        (KeyCode::Char('F'), (2, 3)),
        (KeyCode::Char('G'), (2, 4)),
        (KeyCode::Char('H'), (2, 5)),
        (KeyCode::Char('J'), (2, 6)),
        (KeyCode::Char('K'), (2, 7)),
        (KeyCode::Char('L'), (2, 8)),
        (KeyCode::Char(';'), (2, 9)),
        (KeyCode::Char(':'), (2, 9)),
        (KeyCode::Char('\''), (2, 10)),
        (KeyCode::Char('"'), (2, 11)),
        (KeyCode::Enter, (2, 12)),
        // Third row
        (KeyCode::Char('Z'), (3, 0)),
        (KeyCode::Char('X'), (3, 1)),
        (KeyCode::Char('C'), (3, 2)),
        (KeyCode::Char('V'), (3, 3)),
        (KeyCode::Char('B'), (3, 4)),
        (KeyCode::Char('N'), (3, 5)),
        (KeyCode::Char('M'), (3, 6)),
        (KeyCode::Char(','), (3, 7)),
        (KeyCode::Char('<'), (3, 7)),
        (KeyCode::Char('.'), (3, 8)),
        (KeyCode::Char('>'), (3, 8)),
        (KeyCode::Char('/'), (3, 9)),
        (KeyCode::Char('?'), (3, 9)),
        // fourth row
        (KeyCode::Modifier(ModifierKeyCode::LeftControl), (4, 0)),
        (KeyCode::Modifier(ModifierKeyCode::LeftSuper), (4, 1)),
        (KeyCode::Modifier(ModifierKeyCode::LeftAlt), (4, 2)),
        (KeyCode::Char(' '), (4, 3)),
        (KeyCode::Modifier(ModifierKeyCode::RightAlt), (4, 5)),
        (KeyCode::Modifier(ModifierKeyCode::RightSuper), (4, 6)),
        (KeyCode::Modifier(ModifierKeyCode::RightControl), (4, 7)),
    ])
}
