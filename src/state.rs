use crate::keyboard_event::{KeyboardActions, KeyboardEvent, States};
use crate::DEBUG;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct TypingState {
    pub sentence: String,
    pub index: usize,
    pub update_text_color: bool,
    pub keyboard_event: Option<KeyboardActions>,
    pub correct_hit: bool,
    pub search_request: Option<String>,
    pub start_typing: bool,
    pub intro: bool,
}

impl TypingState {
    pub fn get_current_char(&mut self) -> char {
        self.sentence.chars().nth(self.index).unwrap()
    }

    pub fn build_search_request(&mut self, c: char) {
        let mut s = match &self.search_request {
            Some(t) => t.clone(),
            None => String::new(),
        };

        s.push(c);
        self.search_request = Some(s).take();

        if DEBUG {
            println!("{:?}", self.search_request);
        }
    }

    pub fn delete_chars_search_request(&mut self) {
        if let Some(mut sr) = self.search_request.take().clone() {
            sr.pop();

            self.search_request = Some(sr).take();
        }

        if DEBUG {
            println!("Deleted {:?}", self.search_request);
        }
    }

    pub fn process_events_or_exit(&mut self, key_press_event: KeyboardEvent) -> bool {
        match key_press_event {
            KeyboardEvent::KeyPress(keyboard_actions) => {
                match keyboard_actions.state {
                    States::SEARCHOFF => {}
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
                    }
                    States::START => {
                        // First start get a random song lyric.
                    }
                    States::TYPE => {
                        let c = &self.get_current_char();

                        if c.is_whitespace()
                            && keyboard_actions
                                .key_event
                                .eq(&KeyEvent::from(KeyCode::Char(*c)))
                        {
                            self.correct_hit = true;
                            self.update_text_color = true;
                            self.index += 1;
                            self.keyboard_event = Some(keyboard_actions);
                        } else if c.is_uppercase()
                            && keyboard_actions
                                .key_event
                                .eq(&KeyEvent::from(KeyCode::Char(*c)))
                        {
                            self.correct_hit = true;
                            self.update_text_color = true;
                            self.index += 1;
                            self.keyboard_event = Some(keyboard_actions);
                        }
                        // if the c is lowercase and the keyevent happens to be a small one we have
                        // to compare that.
                        else if c.is_lowercase()
                            && keyboard_actions
                                .key_event
                                .eq(&KeyEvent::from(KeyCode::Char(*c)))
                        {
                            self.correct_hit = true;
                            self.update_text_color = true;
                            self.index += 1;
                            // change the keyboard keyboard state to capital because the keycode is
                            // for the capital characters.

                            let updated_keyboard_action =
                                KeyboardActions::from_char(c.to_ascii_uppercase());
                            self.keyboard_event = Some(updated_keyboard_action);
                        } else {
                            if let KeyCode::Char(in_c) = keyboard_actions.key_event.code {
                                if in_c.is_whitespace() {
                                    let updated_keyboard_action = KeyboardActions::from_char(' ');
                                    self.keyboard_event = Some(updated_keyboard_action);
                                } else {
                                    let updated_keyboard_action =
                                        KeyboardActions::from_char(in_c.to_ascii_uppercase());
                                    self.keyboard_event = Some(updated_keyboard_action);
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
                self.keyboard_event = None;
            }
        }

        return false;
    }
}
