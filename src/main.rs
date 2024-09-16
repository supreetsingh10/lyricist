mod keyboard_event;
use core::panic;
use std::io::{stdout, Result};
use std::rc::Rc;
use std::u16;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
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

#[derive(Clone, Debug)]
struct Keys {
    keychar: char,
    keylength: KeyLength,
    boxlevel: u16,
}

impl Keys {
    fn from_values(l_keychar: char, l_keylength: KeyLength, l_boxlevel: u16) -> Self {
        Keys {
            keychar: l_keychar,
            keylength: l_keylength,
            boxlevel: l_boxlevel,
        }
    }
}

fn initialize_keys() -> Vec<Keys> {
    vec![
        Keys::from_values('Q', KeyLength::SHORT, 0),
        Keys::from_values('W', KeyLength::SHORT, 0),
        Keys::from_values('E', KeyLength::SHORT, 0),
        Keys::from_values('R', KeyLength::SHORT, 0),
        Keys::from_values('T', KeyLength::SHORT, 0),
        Keys::from_values('Y', KeyLength::SHORT, 0),
        Keys::from_values('U', KeyLength::SHORT, 0),
        Keys::from_values('I', KeyLength::SHORT, 0),
        Keys::from_values('O', KeyLength::SHORT, 0),
        Keys::from_values('P', KeyLength::SHORT, 0),
        Keys::from_values('A', KeyLength::SHORT, 1),
        Keys::from_values('S', KeyLength::SHORT, 1),
        Keys::from_values('D', KeyLength::SHORT, 1),
        Keys::from_values('F', KeyLength::SHORT, 1),
        Keys::from_values('G', KeyLength::SHORT, 1),
        Keys::from_values('H', KeyLength::SHORT, 1),
        Keys::from_values('J', KeyLength::SHORT, 1),
        Keys::from_values('K', KeyLength::SHORT, 1),
        Keys::from_values('L', KeyLength::SHORT, 2),
        Keys::from_values('Z', KeyLength::SHORT, 2),
        Keys::from_values('X', KeyLength::SHORT, 2),
        Keys::from_values('C', KeyLength::SHORT, 2),
        Keys::from_values(' ', KeyLength::LONG, 2),
        Keys::from_values('V', KeyLength::SHORT, 2),
        Keys::from_values('B', KeyLength::SHORT, 2),
        Keys::from_values('N', KeyLength::SHORT, 2),
        Keys::from_values('M', KeyLength::SHORT, 2),
    ]
}

#[derive(Clone, Debug)]
struct TypingState {
    sentence: String,
    index: usize,
    update_color: bool,
    keypressed: Option<char>,
}

trait Split {
    fn from_num(num: u16, rect: Rect, direction: Direction) -> Rc<[Rect]>;
}

impl Split for Constraint {
    fn from_num(num: u16, rect: Rect, direction: Direction) -> Rc<[Rect]> {
        match direction {
            Direction::Horizontal => Layout::new(
                Direction::Horizontal,
                Constraint::from_ratios([
                    (1, 10),
                    (1, 10),
                    (1, 10),
                    (1, 10),
                    (1, 10),
                    (1, 10),
                    (1, 10),
                    (1, 10),
                    (1, 10),
                    (1, 10),
                ]),
            )
            .split(rect),
            Direction::Vertical => Layout::new(
                Direction::Vertical,
                Constraint::from_lengths([rect.height / num]),
            )
            .split(rect),
        }
    }
}

impl TypingState {
    fn get_current_char(&mut self) -> char {
        self.sentence.chars().nth(self.index).unwrap()
    }

    fn process_event(&mut self, key_press_event: KeyPressEvent) -> bool {
        if KeyPressEvent::KeyPress(KeyEvent {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
            state: KeyEventState::NONE,
            kind: KeyEventKind::Press,
        }) == key_press_event
        {
            return true;
        } else if KeyPressEvent::KeyPress(KeyEvent {
            code: KeyCode::Char(self.get_current_char()),
            modifiers: KeyModifiers::NONE,
            state: KeyEventState::NONE,
            kind: KeyEventKind::Press,
        }) == key_press_event
        {
            self.index += 1;
            self.update_color = true;
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

fn render_keyboard(area: Rect, horizontal: Constraint, vertical: Constraint) -> Vec<Rc<[Rect]>> {
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
    render_keys(Rc::clone(&rects))
}

fn render_keys(keyboard: Rc<[Rect]>) -> Vec<Rc<[Rect]>> {
    let mut v = vec![];
    for k in keyboard.into_iter() {
        v.push(Constraint::from_num(10 as u16, *k, Direction::Horizontal));
    }

    v
}

fn render(frame: &mut Frame, state: &mut TypingState) {
    let area = center_rect(
        frame.size(),
        Constraint::Percentage(70),
        Constraint::Length(4),
    );

    let keyboard = render_keyboard(
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

    for k in keyboard.into_iter() {
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

    let _keys = initialize_keys();
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
