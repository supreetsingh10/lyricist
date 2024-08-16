mod keyboard_event;
use core::panic;
use std::io::{stdout, Result};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use keyboard_event::{handle_keyboard_events, KeyPressEvent};
use ratatui::{
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
    },
    prelude::*,
    widgets::*,
};

#[derive(Clone, Debug)]
struct TypingState {
    sentence: String,
    index: usize,
    update_color: bool,
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

fn render_keyboard(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(layout::Flex::Center)
        .areas(area);

    let [area] = Layout::vertical([vertical])
        .flex(layout::Flex::End)
        .areas(area);

    area
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

    // frame.render_widget(Clear, area);
    if state.update_color {
        frame.render_widget(Block::new().borders(Borders::all()), keyboard);
        state.update_color = false;
    } else {
        frame.render_widget(Block::new().borders(Borders::all()).blue(), keyboard);
    }

    frame.render_widget(
        Paragraph::new(state.sentence.clone()).block(Block::bordered().title("Test your speed")),
        area,
    );
}

// handles closing of the typing application.
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
    };

    let _ = terminal.draw(|f| {
        render(f, &mut state_struct);
    });

    let (sn, rc) = async_std::channel::unbounded::<keyboard_event::KeyPressEvent>();
    loop {
        async_std::task::spawn(handle_keyboard_events(sn.clone()));

        let quit = match rc.recv().await {
            Ok(rec_eve) => {
                let quit = state_struct.process_event(rec_eve);
                let _ = terminal.draw(|f| {
                    render(f, &mut state_struct);
                });

                quit
            }
            Err(e) => panic!("Failed to recieve the keyboard event, {}", e.to_string()),
        };

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
