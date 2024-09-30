use crate::DEBUG;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::crossterm::event::KeyCode;

use crate::keyboard_event::{Actions, KeyboardActions, KeyboardEvent};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct TypingState {
    pub sentence: String,
    pub index: usize,
    pub update_text_color: bool,
    pub keyboard_event: Option<KeyboardActions>,
    pub correct_hit: bool,
    pub search_flag: bool,
    pub search_request: Option<String>,
    pub start_typing: bool,
    pub intro: bool,
}

impl TypingState {
    pub fn get_current_char(&mut self) -> char {
        self.sentence.chars().nth(self.index).unwrap()
    }

    pub fn process_events_or_exit(&mut self, key_press_event: KeyboardEvent) -> bool {
        match key_press_event {
            KeyboardEvent::KeyPress(keyboard_actions) => {
                if keyboard_actions.action == Actions::EXIT {
                    return true;
                }

                if self.search_flag == true {
                    if keyboard_actions.key_event.code == KeyCode::Enter {
                        self.search_flag = false;
                    } else if let KeyCode::Char(x) = keyboard_actions.key_event.code {
                        if let Some(ref mut s) = self.search_request.as_ref() {
                            let mut t = s.clone();
                            t.push(x);
                            s = String::from(t);
                        }
                    }
                } else if keyboard_actions.action == Actions::TYPE && self.search_flag == false {
                    let c = &self.get_current_char();

                    if DEBUG {
                        println!("Type pressed {:?}", keyboard_actions.key_event.code);
                    }

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
                        // change the keyboard keyboard action to capital because the keycode is
                        // for the capital characters.

                        let updated_keyboard_action =
                            KeyboardActions::from_char(c.to_ascii_uppercase());
                        self.keyboard_event = Some(updated_keyboard_action);
                    } else {
                        if let KeyEvent {
                            code: KeyCode::Char(incorrect_key_char),
                            modifiers: _,
                            kind: _,
                            state: _,
                        } = keyboard_actions.key_event
                        {
                            if incorrect_key_char.is_whitespace() {
                                let updated_keyboard_action = KeyboardActions::from_char(' ');
                                self.keyboard_event = Some(updated_keyboard_action);
                            } else {
                                let updated_keyboard_action = KeyboardActions::from_char(
                                    incorrect_key_char.to_ascii_uppercase(),
                                );
                                self.keyboard_event = Some(updated_keyboard_action);
                            }
                        }

                        self.correct_hit = false;
                        self.update_text_color = true;
                    }
                } else if keyboard_actions.action == Actions::SEARCH {
                    self.search_flag = true;
                } else if keyboard_actions.action == Actions::START {
                    // the typing starts here.
                } else {
                    self.correct_hit = false;
                    self.update_text_color = false;
                    self.keyboard_event = Some(keyboard_actions);
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
