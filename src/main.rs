mod constants;
mod keyboard_event;
mod renderer;
mod state;

use constants::*;
use renderer::*;
use state::TypingState;
use std::io::{stdout, Result};

use core::panic;
use keyboard_event::handle_keyboard_events;
use ratatui::{
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
    },
    prelude::*,
};

use libreq::{
    generate_client,
    response::{Root, Song},
    Lyrics,
};

#[tokio::main]
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
        update_text_color: false,
        keyboard_actions: None,
        correct_hit: false,
        search_request_build: None,
        search_completed: None,
        correct_hits: 0,
        total_hits: 0,
        song: None,
    };

    let (sn, rc) = async_std::channel::unbounded::<keyboard_event::KeyboardEvent>();

    let _ = terminal.clear();

    let keys = initialize_key_vec();
    let key_map = initialize_key_coord_map();

    let client = match generate_client() {
        Ok(c) => c,
        Err(e) => panic!("Failed to create the client {}", e),
    };

    let app_layout: AppLayout = generate_app_layout(&mut terminal.get_frame(), &keys);
    async_std::task::spawn(handle_keyboard_events(sn));

    loop {
        if let Some(req) = state_struct.search_completed.take() {
            match client.get_lyrics(req.to_owned()).await {
                // Get the body and the data and then parse it in the ROOT.
                Ok(resp) => match resp.json::<Root>().await {
                    Ok(root) => {
                        state_struct.song = Some(Song::new(root));
                    }
                    Err(e) => panic!("Failed to deserialize the struct {}", e.to_string()),
                },
                Err(e) => {
                    panic!("Failed to get the request {}", e.to_string());
                }
            };
        }

        let quit = match rc.recv().await {
            Ok(rec_eve) => state_struct.process_events_or_exit(rec_eve),
            Err(e) => panic!("Failed to recieve the keyboard event, {}", e.to_string()),
        };

        // this is where the requests will be made from the requester code.
        let _ = terminal.draw(|f| {
            let _ = render_app_layout(f, &app_layout, &keys.clone());
            let _ = render_events(f, &state_struct, &app_layout, &key_map);
            let _ = render_text(f, &state_struct, &app_layout);
        });

        if quit {
            break;
        }
    }

    if !DEBUG {
        if let Err(e) = execute!(stdout(), EnterAlternateScreen) {
            panic!("Failed to get into alternate Screen {}", e);
        }
    }

    if let Err(e) = disable_raw_mode() {
        panic!("Failed to disable raw mode Error: {}", e);
    };

    Ok(())
}
