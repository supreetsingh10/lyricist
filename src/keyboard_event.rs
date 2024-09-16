use core::panic;
use std::time::Duration;

use crossterm::event::{Event, EventStream, KeyEvent};
use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;

#[derive(PartialEq, Debug)]
pub enum KeyPressEvent {
    KeyPress(KeyEvent),
    NoPress,
}

pub async fn handle_keyboard_events(sn: async_std::channel::Sender<KeyPressEvent>) {
    let mut event_tapper = EventStream::new();

    loop {
        let mut delay = Delay::new(Duration::from_millis(1_000)).fuse();
        let mut event = event_tapper.next().fuse();

        select! {
            _ = delay => {
                let _ = sn.send(KeyPressEvent::NoPress);
            },
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        let _ = match event {
                            Event::Key(k) => {
                                if let Err(e) = sn.send(KeyPressEvent::KeyPress(k)).await {
                                    panic!("Failed to send {}", e.to_string());
                                }
                            }
                            _ => {
                                if let Err(e) = sn.send(KeyPressEvent::NoPress).await {
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
