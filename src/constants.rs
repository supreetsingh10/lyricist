use std::collections::HashMap;

use crossterm::event::KeyCode;

pub const DEBUG: bool = false;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyLength {
    SHORT,
    LONG,
}

pub type Coord = (u32, u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Key {
    pub key_code: KeyCode,
    pub key_length: KeyLength,
}

impl Key {
    fn from_values(keycode: KeyCode, keylength: KeyLength) -> Self {
        Key {
            key_code: keycode,
            key_length: keylength,
        }
    }
}

pub fn initialize_key_vec() -> Vec<Vec<Key>> {
    vec![
        vec![
            Key::from_values(KeyCode::Char('0'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('1'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('2'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('3'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('4'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('5'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('6'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('7'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('8'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('9'), KeyLength::SHORT),
        ],
        vec![
            Key::from_values(KeyCode::Char('Q'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('W'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('E'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('R'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('T'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('Y'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('U'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('I'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('O'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('P'), KeyLength::SHORT),
        ],
        vec![
            Key::from_values(KeyCode::Char('A'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('S'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('D'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('F'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('G'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('H'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('J'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('K'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('L'), KeyLength::SHORT),
            Key::from_values(KeyCode::Enter, KeyLength::LONG),
        ],
        vec![
            Key::from_values(KeyCode::Char('Z'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('X'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('C'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char(' '), KeyLength::LONG),
            Key::from_values(KeyCode::Char('V'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('B'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('N'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('M'), KeyLength::SHORT),
        ],
    ]
}

pub fn initialize_key_coord_map() -> HashMap<KeyCode, Coord> {
    HashMap::from([
        (KeyCode::Char('0'), (0, 0)),
        (KeyCode::Char('1'), (0, 1)),
        (KeyCode::Char('2'), (0, 2)),
        (KeyCode::Char('3'), (0, 3)),
        (KeyCode::Char('4'), (0, 4)),
        (KeyCode::Char('5'), (0, 5)),
        (KeyCode::Char('6'), (0, 6)),
        (KeyCode::Char('7'), (0, 7)),
        (KeyCode::Char('8'), (0, 8)),
        (KeyCode::Char('9'), (0, 9)),
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
        (KeyCode::Enter, (2, 9)),
        // Third row
        (KeyCode::Char('Z'), (3, 0)),
        (KeyCode::Char('X'), (3, 1)),
        (KeyCode::Char('C'), (3, 2)),
        (KeyCode::Char(' '), (3, 3)),
        (KeyCode::Char('V'), (3, 4)),
        (KeyCode::Char('B'), (3, 5)),
        (KeyCode::Char('N'), (3, 6)),
        (KeyCode::Char('M'), (3, 7)),
    ])
}
