use crate::core::model::Color;
use crate::core::search::SearchOptions;

impl SearchOptions {
    pub fn new() -> SearchOptions {
        SearchOptions {
            white_time: None,
            white_increment: None,
            black_time: None,
            black_increment: None,
            moves_to_go: None
        }
    }

    pub fn search_time(&self, color: Color) -> u32 {
        let (time, moves_to_go) = match color {
            Color::White => (self.white_time.unwrap(), self.moves_to_go.unwrap()),
            Color::Black => (self.black_time.unwrap(), self.moves_to_go.unwrap())
        };

        time / (moves_to_go + 1)
    }
}
