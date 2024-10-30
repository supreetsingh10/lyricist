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
        correct_hit: false,
        correct_hits: 0,
        error_string: None,
        keyboard_actions: None,
        search_request_build: None,
        search_completed: None,
        song: None,
        total_hits: 0,
        update_text_color: false,
    };

    let (sn, rc) = async_std::channel::unbounded::<keyboard_event::KeyboardEvent>();

    let _ = terminal.clear();

    let keys = initialize_key_vec();
    let key_map = initialize_key_coord_map();

    let client = match generate_client() {
        Ok(c) => c,
        Err(e) => panic!("Failed to create the client, also make sure you have the required environemnt variables. Erro -> {}", e),
    };

    let app_layout: AppLayout = generate_app_layout(&mut terminal.get_frame(), &keys);
    async_std::task::spawn(handle_keyboard_events(sn));

    loop {
        if let Some(req) = state_struct.search_completed.take() {
            match client.get_lyrics(req.to_owned()).await {
                Ok(resp) => match resp.json::<Root>().await {
                    Ok(root) => state_struct.song = Some(Song::new(root)),
                    Err(e) => {
                        state_struct.error_string =
                            Some(format!("Failed to deserialize {}", e.to_string()))
                    }
                },
                Err(e) => state_struct.error_string = Some(format!("Could not get the song you requested, please search a different song, probably some black sabbath: Error ->  {}", e.to_string())),
            };
        }

        let quit = match rc.recv().await {
            Ok(rec_eve) => state_struct.process_events_or_exit(rec_eve),
            Err(e) => panic!("Failed to recieve the keyboard event, {}", e.to_string()),
        };

        let _ = terminal.draw(|f| {
            render_app_layout(f, &app_layout, &keys.clone());
            render_events(f, &state_struct, &app_layout, &key_map);
            render_text(f, &state_struct, &app_layout);
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
