use crate::keyboard_event::States;
use crate::{constants::*, TypingState};
use crossterm::event::KeyCode;
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::{
    layout::{self, Constraint, Direction, Layout, Rect},
    prelude::Stylize,
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};
use std::collections::HashMap;
use std::rc::Rc;

#[allow(dead_code)]
pub struct AppLayout {
    text_box: Rect,
    key_layers: Vec<Rc<[Rect]>>,
    search_box: Rect,
}

pub fn center_rect(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(layout::Flex::Center)
        .areas(area);

    let [area] = Layout::vertical([vertical])
        .flex(layout::Flex::Center)
        .areas(area);

    area
}

pub fn generate_box(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(layout::Flex::Center)
        .areas(area);

    let [area] = Layout::vertical([vertical])
        .flex(layout::Flex::Start)
        .areas(area);

    area
}

pub fn generate_keyboard(
    area: Rect,
    horizontal: Constraint,
    vertical: Constraint,
    keys: &Vec<Vec<Key>>,
) -> Vec<Rc<[Rect]>> {
    let [area] = Layout::horizontal([horizontal])
        .flex(layout::Flex::Center)
        .areas(area);

    let [area] = Layout::vertical([vertical])
        .flex(layout::Flex::End)
        .areas(area);

    let rects = Layout::new(
        Direction::Vertical,
        Constraint::from_percentages([25, 25, 25, 25]),
    )
    .split(area);

    generate_key_layout(Rc::clone(&rects), keys)
}

pub fn generate_key_layout(key_layers: Rc<[Rect]>, keys: &Vec<Vec<Key>>) -> Vec<Rc<[Rect]>> {
    let mut key_layout: Vec<Rc<[Rect]>> = Vec::new();

    let mut current_block: u32 = 0;
    for key_sub_vec in keys.into_iter() {
        let rat_vec: Vec<(u32, u32)> = key_sub_vec
            .into_iter()
            .map(|element| match element.key_length {
                KeyLength::SHORT => (1_u32, 10_u32),
                KeyLength::LONG => (3_u32, 10_u32),
            })
            .collect();

        key_layout.push(
            Layout::new(Direction::Horizontal, Constraint::from_ratios(rat_vec))
                .split(key_layers[current_block as usize]),
        );

        current_block += 1;
    }

    key_layout
}

pub fn generate_app_layout(frame: &mut Frame, keys: &Vec<Vec<Key>>) -> AppLayout {
    let text_box = center_rect(
        frame.size(),
        Constraint::Percentage(70),
        Constraint::Length(4),
    );

    let key_layers = generate_keyboard(
        frame.size(),
        Constraint::Percentage(75),
        Constraint::Length(frame.size().height / 3_u16),
        keys,
    );

    let search_box = generate_box(
        frame.size(),
        Constraint::Percentage(30),
        Constraint::Length(4),
    );

    AppLayout {
        text_box,
        key_layers,
        search_box,
    }
}

pub fn render_app_layout(frame: &mut Frame, key_board_layout: &AppLayout, keys: &Vec<Vec<Key>>) {
    frame.render_widget(
        Block::new().borders(Borders::all()),
        key_board_layout.text_box,
    );

    frame.render_widget(
        Paragraph::new("Rock and roll")
            .block(Block::new().padding(Padding::top(key_board_layout.text_box.height / 2)))
            .centered(),
        key_board_layout.text_box,
    );

    for key_layer in key_board_layout.key_layers.iter() {
        for key_rect in key_layer.iter() {
            frame.render_widget(Block::new().borders(Borders::all()), *key_rect);
        }
    }

    for (i, key_sub_vec) in keys.iter().enumerate() {
        let key_sub_rect = match key_board_layout.key_layers.get(i) {
            Some(r) => r,
            None => panic!(
                "Failed to get the required rect, it should exist, please check what is wrong"
            ),
        };

        for (key_index, key) in key_sub_vec.iter().enumerate() {
            let key_rect = key_sub_rect.get(key_index).unwrap();
            let key_char: String = String::from(key.key_code.to_string());

            frame.render_widget(
                Paragraph::new(key_char)
                    .block(Block::new().padding(Padding::top(key_rect.height / 2)))
                    .centered(),
                *key_rect,
            );
        }
    }
}

pub fn render_events(
    frame: &mut Frame,
    state_struct: &TypingState,
    app_layout: &AppLayout,
    key_map: &HashMap<KeyCode, Coord>,
) {
    if let Some(l_key_event) = state_struct.keyboard_actions {
        match l_key_event.state {
            States::SEARCHOFF => {
                // remove the user input
            }
            States::EXIT => todo!(),
            States::PAUSE => todo!(),
            States::SEARCH => {
                match state_struct.search_request_build.clone() {
                    Some(s) => {
                        frame.render_widget(
                            Block::new().borders(Borders::ALL).title("Search"),
                            app_layout.search_box,
                        );
                        frame.render_widget(
                            Paragraph::new(Text::from(s)).block(Block::new().padding(
                                Padding::new(5, 5, app_layout.search_box.height / 2 - 1, 0),
                            )),
                            app_layout.search_box,
                        );
                    }
                    None => return,
                }
            }
            States::START => todo!(),
            States::TYPE => {
                if let Some(l_coord) = key_map.get(&l_key_event.key_event.code) {
                    let r = match app_layout.key_layers.get(l_coord.0 as usize) {
                        Some(vec_rc) => match vec_rc.get(l_coord.1 as usize) {
                            Some(r) => r,
                            None => panic!("Failed to get the key rect the parent vector exists, please check the index"),
                        }
                        None => panic!("The Key map is existing but the rect vector layout does not, please check the code"),
                    };

                    let b = Block::bordered().border_type(BorderType::QuadrantInside);

                    if state_struct.correct_hit {
                        frame.render_widget(b.style(Style::new()).fg(Color::Green), *r);
                    } else {
                        frame.render_widget(b.style(Style::new()).fg(Color::Red), *r);
                    }
                } else {
                    if DEBUG {
                        println!("Not found");
                    }
                }
            }
        };
    } else {
        match state_struct.search_request_build.clone() {
            Some(s) => {
                frame.render_widget(
                    Block::new().borders(Borders::ALL).title("Search"),
                    app_layout.search_box,
                );
                frame.render_widget(
                    Paragraph::new(Text::from(s)).block(Block::new().padding(Padding::new(
                        5,
                        5,
                        app_layout.search_box.height / 2 - 1,
                        0,
                    ))),
                    app_layout.search_box,
                );
            }
            None => return,
        }
    }
}

#[allow(dead_code)]
fn render_text() {}
