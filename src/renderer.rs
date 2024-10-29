use crate::keyboard_event::States;
use crate::{constants::*, TypingState};
use core::f32;
use crossterm::event::KeyCode;
use libreq::response::SongStatus;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::{
    layout::{self, Constraint, Direction, Layout, Rect},
    prelude::*,
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};
use std::collections::HashMap;
use std::ops::{Div, Mul};
use std::rc::Rc;
use std::usize;

pub struct AppLayout {
    text_box: Rect,
    key_layers: Vec<Rc<[Rect]>>,
    total_hits: Rect,
    correct_hits_display: Rect,
    search_box: Rect,
}

pub fn generate_box(
    area: Rect,
    horizontal: Constraint,
    vertical: Constraint,
    vertical_flex: layout::Flex,
    horizontal_flex: layout::Flex,
) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(horizontal_flex)
        .areas(area);

    let [area] = Layout::vertical([vertical]).flex(vertical_flex).areas(area);

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
        Constraint::from_percentages([25, 25, 25, 25, 25]),
    )
    .split(area);

    generate_key_layout(Rc::clone(&rects), keys)
}

pub fn generate_key_layout(key_layers: Rc<[Rect]>, keys: &Vec<Vec<Key>>) -> Vec<Rc<[Rect]>> {
    let mut key_layout: Vec<Rc<[Rect]>> = Vec::new();

    let mut current_block: u32 = 0;
    for key_sub_vec in keys.into_iter() {
        let rat_vec: Vec<(u32, u32)> = key_sub_vec
            .iter()
            .map(|element| match element.key_length {
                KeyLength::SHORT => (1_u32, 10_u32),
                KeyLength::MEDIUM => (2_u32, 10_u32),
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
    let key_layers = generate_keyboard(
        frame.size(),
        Constraint::Percentage(KEYBOARD_PERCENTAGE),
        Constraint::Length(frame.size().height / 3_u16),
        keys,
    );

    let text_box = generate_box(
        frame.size(),
        Constraint::Percentage(TEXT_BOX_PERCENTAGE),
        Constraint::Length(4),
        layout::Flex::Center,
        layout::Flex::Center,
    );

    let search_box = generate_box(
        frame.size(),
        Constraint::Percentage(SEARCH_BOX_PERCENTAGE),
        Constraint::Length(4),
        layout::Flex::Start,
        layout::Flex::Center,
    );

    let correct_hits_display = generate_box(
        frame.size(),
        Constraint::Percentage(TIMER_BOX_PERCENTAGE),
        Constraint::Length(4),
        layout::Flex::Start,
        layout::Flex::Start,
    );

    let total_hits = generate_box(
        frame.size(),
        Constraint::Percentage(SCORE_BOX_PERCENTAGE),
        Constraint::Length(4),
        layout::Flex::Start,
        layout::Flex::End,
    );

    AppLayout {
        text_box,
        key_layers,
        search_box,
        total_hits,
        correct_hits_display,
    }
}

pub fn render_app_layout(frame: &mut Frame, key_board_layout: &AppLayout, keys: &[Vec<Key>]) {
    frame.render_widget(
        Block::new().borders(Borders::all()),
        key_board_layout.text_box,
    );

    frame.render_widget(
        Block::new().borders(Borders::ALL),
        key_board_layout.correct_hits_display,
    );

    frame.render_widget(
        Block::new().borders(Borders::ALL),
        key_board_layout.total_hits,
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
            let key_char: String = key.key_code.to_string();
            frame.render_widget(
                Paragraph::new(key_char)
                    .block(Block::new().padding(Padding::top(key_rect.height / 2)))
                    .centered(),
                *key_rect,
            );

            key.sec_key_code.map(|sc| {
                frame.render_widget(
                    Paragraph::new(sc.to_string()).block(Block::new().padding(Padding::new(
                        2,
                        0,
                        key_rect.height / 3,
                        0,
                    ))),
                    *key_rect,
                );
            });
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
            States::SEARCHTERMINATED => {
                // Do nothing, this will clear the search box render.
            }
            States::SEARCHOFF => {}
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
                    None => (),
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

                    if state_struct.correct_hit {
                        frame.render_widget(
                            Block::bordered()
                                .border_type(BorderType::QuadrantInside)
                                .style(Style::new())
                                .fg(Color::Green),
                            *r,
                        );
                    } else {
                        frame.render_widget(
                            Block::bordered()
                                .border_type(BorderType::QuadrantInside)
                                .style(Style::new())
                                .fg(Color::Red),
                            *r,
                        );
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
            None => (),
        }
    }
}

pub fn render_text(frame: &mut Frame, state_struct: &TypingState, app_layout: &AppLayout) {
    if let Some(err) = state_struct.error_string.as_ref() {
        frame.render_widget(
            Paragraph::new(Text::from(err.to_string()).red())
                .block(Block::new().padding(Padding::top(app_layout.text_box.height / 2)))
                .centered(),
            app_layout.text_box,
        );

        return;
    }

    match state_struct.get_current_status() {
        Some(status) => match status {
            SongStatus::Continuing => match state_struct.get_sentence() {
                Some(sen) => {
                    let index: i32 = match state_struct.get_current_cursor_location() {
                        Some(cur_location) => cur_location.try_into().unwrap(),
                        None => return,
                    };

                    if index == 0 {
                    frame.render_widget(
                    Paragraph::new(sen)
                        .block(Block::new().padding(Padding::top(app_layout.text_box.height / 2)))
                        .centered(),
                    app_layout.text_box,
                )} else {
                        let (correct_sen, remaining_sen)= sen.split_at(index as usize);
                        frame.render_widget(
                            Paragraph::new(Line::from(vec![
                                Span::styled(correct_sen,Style::default().fg(Color::Green)),
                                Span::styled(remaining_sen,Style::default().fg(Color::White)),
                            ]))
                                .block(Block::new().padding(Padding::top(app_layout.text_box.height / 2)))
                                .centered(),
                            app_layout.text_box,
                        )
                    }
                },
                None => frame.render_widget(
                    Paragraph::new("Song completed, search for a new song.")
                        .block(Block::new().padding(Padding::top(app_layout.text_box.height / 2)))
                        .centered(),
                    app_layout.text_box,
                ),
            },
            SongStatus::Completed => frame.render_widget(
                Paragraph::new(format!("Song completed, the score is the product of Ratio of correct hits to total hits times 10 = {}", 
                    (state_struct.correct_hits as f32).div(state_struct.total_hits as f32).mul(10.0 as f32)))
                    .block(Block::new().padding(Padding::top(app_layout.text_box.height / 2)))
                    .centered(),
                app_layout.text_box,
            ),
        },
        None => frame.render_widget(
                    Paragraph::new("Search and start a song, use CTRL-S to search a song, use this format t: <Song title name>, a: <Song artist name>")
                        .block(Block::new().padding(Padding::top(app_layout.text_box.height / 2)))
                        .centered(),
                    app_layout.text_box,
        ),
    }

    frame.render_widget(
        Paragraph::new(format!("{}", state_struct.correct_hits))
            .block(Block::new().title("Correct Hits"))
            .centered(),
        app_layout.correct_hits_display,
    );

    frame.render_widget(
        Paragraph::new(format!("{}", state_struct.total_hits))
            .block(Block::new().title("Total hits"))
            .centered(),
        app_layout.total_hits,
    );
}
