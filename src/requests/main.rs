use ratatui::{
    crossterm::terminal::enable_raw_mode, layout::Rect, prelude::CrosstermBackend, style::Stylize,
    text::Line, widgets::Paragraph, Terminal,
};
use std::io::stdout;

mod response;

#[tokio::main]
async fn main() {
    if let Err(e) = enable_raw_mode() {
        panic!("Failed to enable raw mode Error: {}", e);
    };

    let mut terminal: Terminal<CrosstermBackend<_>> =
        match Terminal::new(CrosstermBackend::new(stdout())) {
            Ok(t) => t,
            Err(e) => panic!("Failed to make a new terminal, Error {}", e),
        };

    let _ = terminal.clear();

    let _ = terminal.draw(|f| {
        let r: Rect = Rect::new(10, 10, 100, 10);
        f.render_widget(
            Paragraph::new(Line::from(vec!["Hello. ".red(), "world".green()])),
            r,
        );
    });
}
