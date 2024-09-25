mod keyboard_event;
use core::panic;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use keyboard_event::{handle_keyboard_events, Actions, KeyboardActions, KeyboardEvent};
use layout::Layout;
use ratatui::{
    crossterm::{
        execute,
        style::Color,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
    },
    prelude::*,
    widgets::*,
};
use std::rc::Rc;
use std::{
    collections::HashMap,
    io::{stdout, Result},
    usize,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum KeyLength {
    SHORT,
    LONG,
}

type Coord = (u32, u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Key {
    key_code: KeyCode,
    key_length: KeyLength,
}

impl Key {
    fn from_values(keycode: KeyCode, keylength: KeyLength) -> Self {
        Key {
            key_code: keycode,
            key_length: keylength,
        }
    }
}

fn initialize_key_vec() -> Vec<Vec<Key>> {
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
            Key::from_values(KeyCode::Char('_'), KeyLength::LONG),
            Key::from_values(KeyCode::Char('V'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('B'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('N'), KeyLength::SHORT),
            Key::from_values(KeyCode::Char('M'), KeyLength::SHORT),
        ],
    ]
}

fn initialize_key_coord_map() -> HashMap<KeyCode, Coord> {
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
        (KeyCode::Char('V'), (3, 3)),
        (KeyCode::Char('_'), (3, 4)),
        (KeyCode::Char('B'), (3, 5)),
        (KeyCode::Char('N'), (3, 6)),
        (KeyCode::Char('M'), (3, 7)),
    ])
}

#[derive(Clone, Debug)]
struct TypingState {
    sentence: String,
    index: usize,
    update_text_color: bool,
    keyboard_event: Option<KeyboardActions>,
    correct_hit: bool,
}

impl TypingState {
    fn get_current_char(&mut self) -> char {
        self.sentence.chars().nth(self.index).unwrap()
    }

    fn process_events_or_exit(&mut self, key_press_event: KeyboardEvent) -> bool {
        match key_press_event {
            KeyboardEvent::KeyPress(keyboard_actions) => {
                if keyboard_actions.action == Actions::EXIT {
                    return true;
                }

                if keyboard_actions.action == Actions::TYPE {
                    if keyboard_actions.key_event.eq(&KeyEvent::new(
                        KeyCode::Char(self.get_current_char()),
                        KeyModifiers::NONE,
                    )) {
                        self.correct_hit = true;
                        self.update_text_color = true;
                        self.index += 1;
                        self.keyboard_event = Some(keyboard_actions);
                    } else if keyboard_actions.key_event.eq(&KeyEvent::new(
                        KeyCode::Char(self.get_current_char()),
                        KeyModifiers::SHIFT,
                    )) {
                        self.correct_hit = true;
                        self.update_text_color = true;
                        self.index += 1;
                        self.keyboard_event = Some(keyboard_actions);
                    } else {
                        self.correct_hit = false;
                        self.update_text_color = true;
                        self.keyboard_event = Some(keyboard_actions);
                    }
                } else {
                    self.correct_hit = false;
                    self.update_text_color = false;
                    self.keyboard_event = Some(keyboard_actions);
                }
            }
            KeyboardEvent::NoPress => {
                self.correct_hit = false;
                self.update_text_color = false;
                self.keyboard_event = None;
            }
        }

        return false;
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

fn generate_keyboard(
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
            .map(|element| match element.key_length {
                KeyLength::SHORT => (1_u32, 10_u32),
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

fn generate_keyboard_layout(frame: &mut Frame, keys: &Vec<Vec<Key>>) -> KeyboardLayout {
    let text_block = center_rect(
        frame.size(),
        Constraint::Percentage(70),
        Constraint::Length(4),
    );

    let keyboard_layers = generate_keyboard(
        frame.size(),
        Constraint::Percentage(75),
        Constraint::Length(frame.size().height / 3_u16),
        keys,
    );

    KeyboardLayout(text_block, keyboard_layers)
}

struct KeyboardLayout(Rect, Vec<Rc<[Rect]>>);

fn render_keyboard_layout(
    frame: &mut Frame,
    key_board_layout: &KeyboardLayout,
    keys: &Vec<Vec<Key>>,
) {
    frame.render_widget(Block::new().borders(Borders::all()), key_board_layout.0);

    frame.render_widget(
        Paragraph::new("Welcome to lyricist, ready to rock and roll?")
            .block(Block::new().padding(Padding::top(key_board_layout.0.height / 2)))
            .centered(),
        key_board_layout.0,
    );

    for key_layer in key_board_layout.1.iter() {
        for key_rect in key_layer.iter() {
            frame.render_widget(Block::new().borders(Borders::all()), *key_rect);
        }
    }

    for (i, key_sub_vec) in keys.iter().enumerate() {
        let key_sub_rect = match key_board_layout.1.get(i) {
            Some(r) => r,
            None => panic!(
                "Failed to get the required rect, it should exist, please check what is wrong"
            ),
        };

        for (key_index, key) in key_sub_vec.iter().enumerate() {
            let key_rect = key_sub_rect.get(key_index).unwrap();
            let key_char: String = String::from(key.key_code.to_string());

            frame.render_widget(
                Paragraph::new(key_char)
                    .block(Block::new().padding(Padding::top(key_rect.height / 2)))
                    .centered(),
                *key_rect,
            );
        }
    }
}

fn render_events(
    frame: &mut Frame,
    state_struct: &TypingState,
    keyboard_layout: &KeyboardLayout,
    key_map: &HashMap<KeyCode, Coord>,
) {
    if let Some(l_key_event) = state_struct.keyboard_event {
        match l_key_event.action {
            Actions::EXIT => todo!(),
            Actions::PAUSE => todo!(),
            Actions::SEARCH => todo!(),
            Actions::START => todo!(),
            Actions::TYPE => {
                if let Some(l_coord) = key_map.get(&l_key_event.key_event.code) {
                    let r = match keyboard_layout.1.get(l_coord.0 as usize) {
                        Some(vec_rc) => match vec_rc.get(l_coord.1 as usize) {
                            Some(r) => r,
                            None => panic!("Failed to get the key rect the parent vector exists, please check the index"),
                        }
                        None => panic!("The Key map is existing but the rect vector layout does not, please check the code"),
                    };

                    let b = Block::bordered().border_type(BorderType::QuadrantInside);

                    if state_struct.correct_hit {
                        frame.render_widget(b.style(Style::new()).fg(Color::Green), *r);
                    } else {
                        frame.render_widget(b.style(Style::new()).fg(Color::Red), *r);
                    }
                }
            }
        };
    }
}

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
        update_text_color: false,
        keyboard_event: None,
        correct_hit: false,
    };

    let (sn, rc) = async_std::channel::unbounded::<keyboard_event::KeyboardEvent>();

    let _ = terminal.clear();

    let keys = initialize_key_vec();
    let key_map = initialize_key_coord_map();

    let keyboard_layout: KeyboardLayout =
        generate_keyboard_layout(&mut terminal.get_frame(), &keys);

    let _ = terminal.draw(|f| {
        let _ = render_keyboard_layout(f, &keyboard_layout, &keys.clone());
    });

    loop {
        async_std::task::spawn(handle_keyboard_events(sn.clone()));

        let quit = match rc.recv().await {
            Ok(rec_eve) => state_struct.process_events_or_exit(rec_eve),
            Err(e) => panic!("Failed to recieve the keyboard event, {}", e.to_string()),
        };

        let _ = terminal.draw(|f| {
            render_events(f, &state_struct, &keyboard_layout, &key_map);
        });

        if quit {
            break;
        }
    }

    if let Err(e) = execute!(stdout(), EnterAlternateScreen) {
        panic!("Failed to get into alternate Screen {}", e);
    }

    if let Err(e) = disable_raw_mode() {
        panic!("Failed to disable raw mode Error: {}", e);
    };

    Ok(())
}
