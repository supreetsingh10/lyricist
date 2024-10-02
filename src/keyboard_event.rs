use core::panic;
use std::time::Duration;

use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyModifiers};
use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum States {
    EXIT,
    PAUSE,
    SEARCH,
    SEARCHOFF,
    START,
    TYPE,
}

#[derive(Eq, Clone, Copy, PartialEq, Debug)]
pub struct KeyboardActions {
    pub key_event: KeyEvent,
    pub state: States,
}

impl KeyboardActions {
    fn new(kkey_event: KeyEvent, k_action: States) -> Self {
        KeyboardActions {
            key_event: kkey_event,
            state: k_action,
        }
    }

    pub fn from_char(c: char) -> Self {
        let k = KeyEvent::from(KeyCode::Char(c));
        KeyboardActions {
            key_event: k,
            state: States::TYPE,
        }
    }

    fn process_keyevent_for_actions(key_event: &KeyEvent, state: &mut States) -> Self {
        if *state == States::SEARCH {
            if key_event.eq(&KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)) {
                *state = States::SEARCHOFF;
                return KeyboardActions::new(key_event.to_owned(), *state);
            } else {
                *state = States::SEARCH;
                println!("{:?}", key_event);
                return KeyboardActions::new(key_event.to_owned(), *state);
            }
        } else {
            if key_event.eq(&KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL)) {
                *state = States::SEARCH;
                return KeyboardActions::new(key_event.to_owned(), *state);
            } else if key_event.eq(&KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)) {
                return KeyboardActions::new(key_event.to_owned(), States::EXIT);
            } else if key_event.eq(&KeyEvent::new(KeyCode::Char('p'), KeyModifiers::CONTROL)) {
                *state = States::PAUSE;
                return KeyboardActions::new(key_event.to_owned(), *state);
            } else if key_event.eq(&KeyEvent::new(KeyCode::Char('g'), KeyModifiers::CONTROL)) {
                *state = States::START;
                return KeyboardActions::new(key_event.to_owned(), *state);
            } else {
                return KeyboardActions::new(key_event.to_owned(), States::TYPE);
            }
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

    let mut state: States = States::START;
    loop {
        // we can have a mutable state thing here. Which will persist between state calls.
        let mut delay = Delay::new(Duration::from_millis(600)).fuse();
        let mut event = event_tapper.next().fuse();

        select! {
            _ = delay => {
                let _ = sn.send(KeyboardEvent::NoPress).await;
            },
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        let _ = match event {
                            Event::Key(k) => {
                                if let Err(e) = sn.send(KeyboardEvent::KeyPress(KeyboardActions::process_keyevent_for_actions(&k, &mut state))).await {
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
