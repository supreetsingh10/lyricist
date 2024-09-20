mod keyboard_event;
use core::panic;
use std::io::{stdout, Result};
use std::rc::Rc;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use keyboard_event::{handle_keyboard_events, KeyPressEvent};
use layout::Layout;
use ratatui::{
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
    },
    prelude::*,
    widgets::*,
};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum KeyLength {
    SHORT,
    MEDIUM,
    LONG,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct Key {
    keychar: char,
    keylength: KeyLength,
}

impl Key {
    fn from_values(l_keychar: char, l_keylength: KeyLength) -> Self {
        Key {
            keychar: l_keychar,
            keylength: l_keylength,
        }
    }
}

fn initialize_keys() -> Vec<Vec<Key>> {
    vec![
        vec![
            Key::from_values('0', KeyLength::SHORT),
            Key::from_values('1', KeyLength::SHORT),
            Key::from_values('2', KeyLength::SHORT),
            Key::from_values('3', KeyLength::SHORT),
            Key::from_values('4', KeyLength::SHORT),
            Key::from_values('5', KeyLength::SHORT),
            Key::from_values('6', KeyLength::SHORT),
            Key::from_values('7', KeyLength::SHORT),
            Key::from_values('8', KeyLength::SHORT),
            Key::from_values('9', KeyLength::SHORT),
        ],
        vec![
            Key::from_values('Q', KeyLength::SHORT),
            Key::from_values('W', KeyLength::SHORT),
            Key::from_values('E', KeyLength::SHORT),
            Key::from_values('R', KeyLength::SHORT),
            Key::from_values('T', KeyLength::SHORT),
            Key::from_values('Y', KeyLength::SHORT),
            Key::from_values('U', KeyLength::SHORT),
            Key::from_values('I', KeyLength::SHORT),
            Key::from_values('O', KeyLength::SHORT),
            Key::from_values('P', KeyLength::SHORT),
        ],
        vec![
            Key::from_values('A', KeyLength::SHORT),
            Key::from_values('S', KeyLength::SHORT),
            Key::from_values('D', KeyLength::SHORT),
            Key::from_values('F', KeyLength::SHORT),
            Key::from_values('G', KeyLength::SHORT),
            Key::from_values('H', KeyLength::SHORT),
            Key::from_values('J', KeyLength::SHORT),
            Key::from_values('K', KeyLength::SHORT),
            Key::from_values('L', KeyLength::SHORT),
        ],
        vec![
            Key::from_values('Z', KeyLength::SHORT),
            Key::from_values('X', KeyLength::SHORT),
            Key::from_values('C', KeyLength::SHORT),
            Key::from_values(' ', KeyLength::LONG),
            Key::from_values('V', KeyLength::SHORT),
            Key::from_values('B', KeyLength::SHORT),
            Key::from_values('N', KeyLength::SHORT),
            Key::from_values('M', KeyLength::SHORT),
        ],
    ]
}

#[derive(Clone, Debug)]
struct TypingState {
    sentence: String,
    index: usize,
    update_color: bool,
    keypressed: Option<char>,
}

impl TypingState {
    fn get_current_char(&mut self) -> char {
        self.sentence.chars().nth(self.index).unwrap()
    }

    fn process_event(&mut self, key_press_event: KeyPressEvent) -> bool {
        let key_event = match key_press_event {
            KeyPressEvent::KeyPress(k) => k,
            KeyPressEvent::NoPress => return false,
        };

        if key_event
            == KeyEvent::new_with_kind(KeyCode::Esc, KeyModifiers::NONE, KeyEventKind::Press)
        {
            return true;
        }

        if key_event.code == KeyCode::from(KeyCode::Char(self.get_current_char())) {
            self.keypressed = Some(self.get_current_char().clone());
            self.update_color = true;
            self.index += 1;
        } else if key_event.code != KeyCode::from(KeyCode::Char(self.get_current_char())) {
            match key_event.code {
                KeyCode::Char(c) => {
                    self.keypressed = Some(c);
                    self.update_color = false;
                }
                _ => {
                    self.keypressed = None;
                    self.update_color = false;
                }
            }
        }

        false
    }
}

fn center_rect(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(layout::Flex::Center)
        .areas(area);

    let [area] = Layout::vertical([vertical])
        .flex(layout::Flex::Center)
        .areas(area);

    area
}

fn generate_keyboard_layout(
    area: Rect,
    horizontal: Constraint,
    vertical: Constraint,
    keys: &Vec<Vec<Key>>,
) -> Vec<Rc<[Rect]>> {
    let [area] = Layout::horizontal([horizontal])
        .flex(layout::Flex::Center)
        .areas(area);

    let [area] = Layout::vertical([vertical])
        .flex(layout::Flex::End)
        .areas(area);

    let rects = Layout::new(
        Direction::Vertical,
        Constraint::from_percentages([25, 25, 25, 25]),
    )
    .split(area);

    generate_key_layout(Rc::clone(&rects), keys)
}

fn generate_key_layout(key_layers: Rc<[Rect]>, keys: &Vec<Vec<Key>>) -> Vec<Rc<[Rect]>> {
    let mut key_layout: Vec<Rc<[Rect]>> = Vec::new();

    let mut current_block: u32 = 0;
    for key_sub_vec in keys.into_iter() {
        let rat_vec: Vec<(u32, u32)> = key_sub_vec
            .into_iter()
            .map(|element| match element.keylength {
                KeyLength::SHORT => (1_u32, 10_u32),
                KeyLength::MEDIUM => (2_u32, 10_u32),
                KeyLength::LONG => (3_u32, 10_u32),
            })
            .collect();

        key_layout.push(
            Layout::new(Direction::Horizontal, Constraint::from_ratios(rat_vec))
                .split(key_layers[current_block as usize]),
        );

        current_block += 1;
    }

    key_layout
}

#[allow(dead_code)]
struct KeyboardLayout(Rect, Vec<Rc<[Rect]>>);

#[allow(dead_code)]
fn render_keyboard_layout(frame: &mut Frame, keys: &Vec<Vec<Key>>) -> KeyboardLayout {
    let text_block = center_rect(
        frame.size(),
        Constraint::Percentage(70),
        Constraint::Length(4),
    );

    let keyboard_layers = generate_keyboard_layout(
        frame.size(),
        Constraint::Percentage(75),
        Constraint::Length(frame.size().height / 3_u16),
        keys,
    );

    frame.render_widget(Block::new().borders(Borders::all()), text_block);

    frame.render_widget(
        Paragraph::new(Text::from(String::from(
            "Welcome to the lyricist, ready to write your songs?",
        )))
        .centered(),
        text_block,
    );

    for key_layer in keyboard_layers.iter() {
        for key_rect in key_layer.iter() {
            frame.render_widget(Block::new().borders(Borders::all()), *key_rect);
        }
    }

    for (i, key_sub_vec) in keys.iter().enumerate() {
        let key_sub_rect = match keyboard_layers.get(i) {
            Some(r) => r,
            None => panic!(
                "Failed to get the required rect, it should exist, please check what is wrong"
            ),
        };

        for (key_index, key) in key_sub_vec.iter().enumerate() {
            let key_rect = key_sub_rect.get(key_index).unwrap();
            let key_char: String = String::from(key.keychar.to_string());

            frame.render_widget(Paragraph::new(key_char).centered(), *key_rect);
        }
    }

    KeyboardLayout(text_block, keyboard_layers)
}

#[allow(dead_code)]
fn render_events() {}

// since text would need to be tracked as it would be continously update as the program grows.
#[allow(dead_code)]
fn render_text() {}

#[async_std::main]
async fn main() -> Result<()> {
    if let Err(e) = enable_raw_mode() {
        panic!("Failed to enable raw mode Error: {}", e);
    };

    let mut terminal: Terminal<CrosstermBackend<_>> =
        match Terminal::new(CrosstermBackend::new(stdout())) {
            Ok(t) => t,
            Err(e) => panic!("Failed to make a new terminal, Error {}", e),
        };

    let mut state_struct = TypingState {
        sentence: String::from("Rock and roll"),
        index: (0 as usize),
        update_color: false,
        keypressed: None,
    };

    let (sn, rc) = async_std::channel::unbounded::<keyboard_event::KeyPressEvent>();

    let _ = terminal.clear();
    let keys = initialize_keys();

    let _ = terminal.draw(|f| {
        let _ = render_keyboard_layout(f, &keys);
    });

    loop {
        async_std::task::spawn(handle_keyboard_events(sn.clone()));

        let quit = match rc.recv().await {
            Ok(rec_eve) => state_struct.process_event(rec_eve),
            Err(e) => panic!("Failed to recieve the keyboard event, {}", e.to_string()),
        };

        let _ = terminal.draw(|f| {
            let _ = render_keyboard_layout(f, &keys);
        });

        if quit {
            break;
        }
    }

    if let Err(e) = execute!(stdout(), EnterAlternateScreen) {
        panic!("Failed to get into Alternate Screen {}", e);
    }

    if let Err(e) = disable_raw_mode() {
        panic!("Failed to disable raw mode Error: {}", e);
    };

    Ok(())
}
