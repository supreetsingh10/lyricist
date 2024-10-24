use std::char;

use crate::keyboard_event::{KeyboardActions, KeyboardEvent, States};
use crate::DEBUG;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use libreq::response::Song;

#[derive(Clone, Debug)]
pub struct TypingState {
    pub update_text_color: bool,
    pub keyboard_actions: Option<KeyboardActions>,
    pub correct_hit: bool,
    pub search_request_build: Option<String>,
    pub search_completed: Option<String>,
    pub total_hits: u32,
    pub correct_hits: u32,

    pub song: Option<Song>,
}

impl TypingState {
    pub fn get_sentence(&self) -> Option<&str> {
        self.song
            .as_ref()
            .map_or(None, |s| Some(s.get_sentence_ref()))
    }

    pub fn get_current_char(&self) -> Option<char> {
        if let Some(s) = self.song.as_ref() {
            return s.get_current_char();
        }

        None
    }

    fn build_search_request(&mut self, c: char) {
        let mut s = match &self.search_request_build {
            Some(t) => t.clone(),
            None => String::new(),
        };

        s.push(c);
        self.search_request_build = Some(s).take();

        if DEBUG {
            println!("{:?}", self.search_request_build);
        }
    }

    fn delete_chars_search_request(&mut self) {
        if let Some(mut sr) = self.search_request_build.take().clone() {
            sr.pop();

            self.search_request_build = Some(sr).take();
        }

        if DEBUG {
            println!("Deleted {:?}", self.search_request_build);
        }
    }

    pub fn process_events_or_exit(&mut self, key_press_event: KeyboardEvent) -> bool {
        match key_press_event {
            KeyboardEvent::KeyPress(keyboard_actions) => {
                match keyboard_actions.state {
                    States::SEARCHOFF => {
                        self.search_completed = self.search_request_build.take();
                    }
                    States::EXIT => return true,
                    States::PAUSE => todo!(),
                    States::SEARCH => {
                        if keyboard_actions.key_event.modifiers != KeyModifiers::CONTROL {
                            if let KeyCode::Char(c) = keyboard_actions.key_event.code {
                                let _ = &self.build_search_request(c);
                            } else if KeyCode::Backspace == keyboard_actions.key_event.code {
                                self.delete_chars_search_request();
                            }
                        }

                        self.keyboard_actions = Some(keyboard_actions);
                    }
                    States::START => {
                        // First start get a random song lyric.
                    }
                    States::TYPE => {
                        // if current character is null this means the song is over.
                        let c = match &self.get_current_char() {
                            Some(c) => c.clone(),
                            None => return false,
                        };

                        self.total_hits += 1;
                        if c.is_whitespace()
                            && keyboard_actions
                                .key_event
                                .eq(&KeyEvent::from(KeyCode::Char(c)))
                        {
                            self.correct_hits += 1;
                            self.correct_hit = true;
                            self.update_text_color = true;

                            if self.song.is_some() {
                                self.song.as_mut().unwrap().update_sentence();
                            }
                            self.keyboard_actions = Some(keyboard_actions);
                        } else if c.is_uppercase()
                            && keyboard_actions
                                .key_event
                                .eq(&KeyEvent::from(KeyCode::Char(c)))
                        {
                            self.correct_hits += 1;
                            self.correct_hit = true;
                            self.update_text_color = true;

                            if self.song.is_some() {
                                self.song.as_mut().unwrap().update_sentence();
                            }
                            self.keyboard_actions = Some(keyboard_actions);
                        }
                        // if the c is lowercase and the keyevent happens to be a small one we have
                        // to compare that.
                        else if c.is_lowercase()
                            && keyboard_actions
                                .key_event
                                .eq(&KeyEvent::from(KeyCode::Char(c)))
                        {
                            self.correct_hits += 1;
                            self.correct_hit = true;
                            self.update_text_color = true;

                            if self.song.is_some() {
                                self.song.as_mut().unwrap().update_sentence();
                            }

                            let updated_keyboard_action =
                                KeyboardActions::from_char(c.to_ascii_uppercase());
                            self.keyboard_actions = Some(updated_keyboard_action);
                        } else {
                            if let KeyCode::Char(in_c) = keyboard_actions.key_event.code {
                                if in_c.is_whitespace() {
                                    let updated_keyboard_action = KeyboardActions::from_char(' ');
                                    self.keyboard_actions = Some(updated_keyboard_action);
                                } else {
                                    let updated_keyboard_action =
                                        KeyboardActions::from_char(in_c.to_ascii_uppercase());
                                    self.keyboard_actions = Some(updated_keyboard_action);
                                }
                            }

                            self.correct_hit = false;
                            self.update_text_color = true;
                        }
                    }
                }
            }
            KeyboardEvent::NoPress => {
                self.correct_hit = false;
                self.update_text_color = false;
                self.keyboard_actions = None;
            }
        }

        return false;
    }
}
