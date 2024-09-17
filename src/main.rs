mod keyboard_event;
use core::panic;
use std::io::{stdout, Result};
use std::rc::Rc;
use std::u16;

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
#[derive(Clone, Debug)]
enum KeyLength {
    SHORT,
    MEDIUM,
    LONG,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Key {
    keychar: char,
    keylength: KeyLength,
    boxlevel: u16,
}

impl Key {
    fn from_values(l_keychar: char, l_keylength: KeyLength, l_boxlevel: u16) -> Self {
        Key {
            keychar: l_keychar,
            keylength: l_keylength,
            boxlevel: l_boxlevel,
        }
    }
}

fn initialize_keys() -> Vec<Key> {
    vec![
        Key::from_values('0', KeyLength::SHORT, 0),
        Key::from_values('1', KeyLength::SHORT, 0),
        Key::from_values('2', KeyLength::SHORT, 0),
        Key::from_values('3', KeyLength::SHORT, 0),
        Key::from_values('4', KeyLength::SHORT, 0),
        Key::from_values('5', KeyLength::SHORT, 0),
        Key::from_values('6', KeyLength::SHORT, 0),
        Key::from_values('7', KeyLength::SHORT, 0),
        Key::from_values('8', KeyLength::SHORT, 0),
        Key::from_values('9', KeyLength::SHORT, 0),
        Key::from_values('Q', KeyLength::SHORT, 1),
        Key::from_values('W', KeyLength::SHORT, 1),
        Key::from_values('E', KeyLength::SHORT, 1),
        Key::from_values('R', KeyLength::SHORT, 1),
        Key::from_values('T', KeyLength::SHORT, 1),
        Key::from_values('Y', KeyLength::SHORT, 1),
        Key::from_values('U', KeyLength::SHORT, 1),
        Key::from_values('I', KeyLength::SHORT, 1),
        Key::from_values('O', KeyLength::SHORT, 1),
        Key::from_values('P', KeyLength::SHORT, 1),
        Key::from_values('A', KeyLength::SHORT, 2),
        Key::from_values('S', KeyLength::SHORT, 2),
        Key::from_values('D', KeyLength::SHORT, 2),
        Key::from_values('F', KeyLength::SHORT, 2),
        Key::from_values('G', KeyLength::SHORT, 2),
        Key::from_values('H', KeyLength::SHORT, 2),
        Key::from_values('J', KeyLength::SHORT, 2),
        Key::from_values('K', KeyLength::SHORT, 2),
        Key::from_values('L', KeyLength::SHORT, 3),
        Key::from_values('Z', KeyLength::SHORT, 3),
        Key::from_values('X', KeyLength::SHORT, 3),
        Key::from_values('C', KeyLength::SHORT, 3),
        Key::from_values(' ', KeyLength::LONG, 3),
        Key::from_values('V', KeyLength::SHORT, 3),
        Key::from_values('B', KeyLength::SHORT, 3),
        Key::from_values('N', KeyLength::SHORT, 3),
        Key::from_values('M', KeyLength::SHORT, 3),
    ]
}

trait Split {
    fn from_nums(nums: u32, direction: Direction, rect: Rect) -> Rc<[Rect]>;
}

impl Split for Constraint {
    fn from_nums(nums: u32, direction: Direction, rect: Rect) -> Rc<[Rect]> {}
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

    // let us render keys here.
    generate_key_layout(Rc::clone(&rects))
}

fn generate_key_layout(key_layers: Rc<[Rect]>, keys: &Vec<Key>) -> Vec<Rc<[Rect]>> {
    let mut v = vec![];
    for k in {}

    v
}

fn render(frame: &mut Frame, state: &mut TypingState) {
    let area = center_rect(
        frame.size(),
        Constraint::Percentage(70),
        Constraint::Length(4),
    );

    let keyboard_layers = generate_keyboard_layout(
        frame.size(),
        Constraint::Percentage(75),
        Constraint::Length(frame.size().height / 3 as u16),
    );

    if state.update_color {
        frame.render_widget(Block::new().borders(Borders::all()).blue(), area);
    } else {
        frame.render_widget(Clear, area);
        frame.render_widget(Block::new().borders(Borders::all()).white(), area);
    }

    for k in keyboard_layers.into_iter() {
        for kk in k.into_iter() {
            frame.render_widget(Block::new().borders(Borders::all()), *kk);
        }
    }

    frame.render_widget(
        Paragraph::new(state.sentence.clone()).block(Block::bordered().title("Test your speed")),
        area,
    );
}

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

    let keys = initialize_keys();
    let (sn, rc) = async_std::channel::unbounded::<keyboard_event::KeyPressEvent>();
    let _ = terminal.clear();
    loop {
        async_std::task::spawn(handle_keyboard_events(sn.clone()));

        let quit = match rc.recv().await {
            Ok(rec_eve) => state_struct.process_event(rec_eve),
            Err(e) => panic!("Failed to recieve the keyboard event, {}", e.to_string()),
        };

        let _ = terminal.draw(|f| {
            render(f, &mut state_struct);
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
