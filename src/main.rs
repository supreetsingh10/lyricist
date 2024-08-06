use std::io::{stdout, Result};

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
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

    fn update_state(&mut self) {
        match event::poll(std::time::Duration::from_millis(10)) {
            Ok(flag) => {
                if flag {
                    if let Ok(Event::Key(key)) = event::read() {
                        if key.kind == event::KeyEventKind::Press
                            && key.code == KeyCode::Char(self.get_current_char())
                        {
                            self.index += 1;
                            self.update_color = true;
                        }
                    }
                }
            }
            Err(e) => {
                panic!("Failed to poll {}", e);
            }
        };
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

    state.update_state();
    frame.render_widget(Clear, area);

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
fn handle_exit() -> bool {
    match event::poll(std::time::Duration::from_millis(10)) {
        Ok(flag) => {
            if flag {
                if let Ok(Event::Key(key)) = event::read() {
                    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Esc {
                        return true;
                    }

                    return false;
                }
            }
        }
        Err(e) => {
            panic!("Failed to poll {}", e);
        }
    };

    false
}

fn main() -> Result<()> {
    if let Err(e) = enable_raw_mode() {
        panic!("Failed to enable raw mode Error: {}", e);
    };

    let mut terminal: Terminal<CrosstermBackend<_>> =
        match Terminal::new(CrosstermBackend::new(stdout())) {
            Ok(t) => t,
            Err(e) => panic!("Failed to make a new terminal, Error {}", e),
        };

    let mut state_struct = TypingState {
        sentence: String::from("fuck the world"),
        index: (0 as usize),
        update_color: false,
    };

    let _ = terminal.clear();
    loop {
        if handle_exit() {
            break;
        }

        let _ = terminal.draw(|f| {
            render(f, &mut state_struct);
        });
    }

    if let Err(e) = execute!(stdout(), EnterAlternateScreen) {
        panic!("Failed to get into Alternate Screen {}", e);
    }

    if let Err(e) = disable_raw_mode() {
        panic!("Failed to disable raw mode Error: {}", e);
    };

    Ok(())
}
