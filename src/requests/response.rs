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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SongStatus {
    Completed,
    Continuing,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Song {
    pub song: Option<Root>,
    pub sentence: String,
    pub line_index: u32,

    cur_loc: i32,
    song_status: SongStatus,
}

impl Song {
    pub fn get_sentence_ref(&self) -> &str {
        self.sentence.as_str()
    }

    pub fn new(s: Root) -> Self {
        Song {
            song: Some(s.to_owned()),
            sentence: s.first().unwrap().text.to_owned(),
            line_index: 0,
            cur_loc: 0,
            song_status: SongStatus::Continuing,
        }
    }

    pub fn get_current_char(&self) -> Option<char> {
        self.sentence.chars().nth(self.cur_loc as usize)
    }

    pub fn get_current_status(&self) -> Option<SongStatus> {
        Some(self.song_status.to_owned())
    }

    pub fn update_sentence(&mut self) {
        if self.song == None {
            return;
        }

        let local_song = self.song.as_ref().unwrap();

        self.cur_loc += 1;

        if (self.cur_loc as usize) >= self.sentence.len() {
            self.line_index += 1;

            loop {
                match local_song.get(self.line_index as usize) {
                    Some(lyric) => {
                        if lyric.text.len() != 0 {
                            self.cur_loc = 0;
                            self.sentence = lyric.text.to_owned();
                            break;
                        } else {
                            self.line_index += 1;
                        }
                        self.song_status = SongStatus::Continuing;
                    }
                    None => {
                        self.song_status = SongStatus::Completed;
                        break;
                    }
                }
            }
        }
    }
}
