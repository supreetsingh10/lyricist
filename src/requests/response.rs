use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root2 {
    pub text: String,
    pub time: Time,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Time {
    pub total: f64,
    pub minutes: i64,
    pub seconds: i64,
    pub hundredths: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Song {
    pub song: Root,
    pub sentence: String,

    pub line_index: u32,
    cur_loc: i32,
}

pub enum SongStatus {
    Completed,
    NextLine,
    Continuing,
}

impl Song {
    pub fn get_sentence_ref(&self) -> &str {
        self.sentence.as_str()
    }

    pub fn new(s: Root) -> Self {
        Song {
            song: s.clone(),
            sentence: s.first().unwrap().text.to_owned(),
            line_index: 0,
            cur_loc: 0,
        }
    }

    pub fn get_current_char(&self) -> Option<char> {
        self.sentence.chars().nth(self.cur_loc as usize)
    }

    pub fn update_sentence(&mut self) -> SongStatus {
        self.cur_loc += 1;

        if (self.cur_loc as usize) >= self.sentence.len() {
            if (self.line_index as usize) >= self.song.len() {
                return SongStatus::Completed;
            } else {
                self.line_index += 1;
                self.cur_loc = 0;
                self.sentence = self
                    .song
                    .get(self.line_index as usize)
                    .unwrap()
                    .text
                    .to_owned();

                return SongStatus::NextLine;
            }
        }

        SongStatus::Continuing
    }
}
