use core::panic;
use crossterm::event::KeyCode;
use std::time::Duration;

use crossterm::event::{Event, EventStream, KeyEvent, KeyModifiers};
use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;

#[allow(dead_code)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Actions {
    EXIT,
    SEARCH,
    TYPE,
    PAUSE,
}

#[derive(Eq, Clone, Copy, PartialEq, Debug)]
pub struct KeyboardActions {
    pub key_event: KeyEvent,
    pub action: Actions,
}

impl KeyboardActions {
    fn new(kkey_event: KeyEvent, k_action: Actions) -> Self {
        KeyboardActions {
            key_event: kkey_event,
            action: k_action,
        }
    }

    fn process_keyevent_for_actions(key_event: &KeyEvent) -> Self {
        if key_event.eq(&KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL)) {
            return KeyboardActions::new(key_event.to_owned(), Actions::SEARCH);
        } else if key_event.eq(&KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)) {
            return KeyboardActions::new(key_event.to_owned(), Actions::EXIT);
        } else if key_event.eq(&KeyEvent::new(KeyCode::Char('p'), KeyModifiers::CONTROL)) {
            return KeyboardActions::new(key_event.to_owned(), Actions::PAUSE);
        } else {
            return KeyboardActions::new(key_event.to_owned(), Actions::TYPE);
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum KeyboardEvent {
    KeyPress(KeyboardActions),
    NoPress,
}

pub async fn handle_keyboard_events(sn: async_std::channel::Sender<KeyboardEvent>) {
    let mut event_tapper = EventStream::new();

    loop {
        let mut delay = Delay::new(Duration::from_millis(1_000)).fuse();
        let mut event = event_tapper.next().fuse();

        select! {
            _ = delay => {
                let _ = sn.send(KeyboardEvent::NoPress);
            },
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        let _ = match event {
                            Event::Key(k) => {
                                if let Err(e) = sn.send(KeyboardEvent::KeyPress(KeyboardActions::process_keyevent_for_actions(&k))).await {
                                    panic!("Failed to send {}", e.to_string());
                                }
                            }
                            _ => {
                                if let Err(e) = sn.send(KeyboardEvent::NoPress).await {
                                    panic!("Failed to send {}", e.to_string());
                                }
                            },
                        };
                    },
                    Some(Err(e)) => println!("Error {:?}", e),
                    None => break,
                }
            }
        }
    }
}
