use regex::Regex;

use crate::converters::pgn_converter::{MAX_TAGS, PGNString};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum PGNTag {
    Event,
    Site,
    Date,
    Round,
    White,
    Black,
    WhiteElo,
    BlackElo,
    Opening,
    Result,
    Position,
    IgnoreType,
}

impl PGNTag {
    fn from_string(string_tag: &str) -> Self {
        return match string_tag {
            "Event" => PGNTag::Event,
            "Site" => PGNTag::Site,
            "Date" => PGNTag::Date,
            "Round" => PGNTag::Round,
            "White" => PGNTag::White,
            "Black" => PGNTag::Black,
            "WhiteElo" => PGNTag::WhiteElo,
            "BlackckElo" => PGNTag::BlackElo,
            "Opening" => PGNTag::Opening,
            "Result" => PGNTag::Result,
            "Position" => PGNTag::Position,
            _ => PGNTag::IgnoreType,
        };
    }
}

const PGN_TAGS: [&'static str; MAX_TAGS] = [
    "Event", "Site", "Date", "Round", "White", "Black", "WhiteElo", "BlackElo", "Opening",
    "Result", "Position",
];

impl PGNString {
    pub fn decode_tags(&mut self, original_pgn: &str) -> () {
        for tag in PGN_TAGS.iter() {
            let pattern: Regex = Regex::new(tag).unwrap();
            if pattern.find(original_pgn).is_some() {
                self.tags.insert(PGNTag::from_string(*tag), tag.to_string());
            }
        }
    }

    pub fn apply_tags(&mut self) -> () {
        for (enum_tag, string_tag) in self.tags.iter() {
            match enum_tag {
                PGNTag::Event
                | PGNTag::Site
                | PGNTag::Date
                | PGNTag::Round
                | PGNTag::Opening
                | PGNTag::Result
                | PGNTag::IgnoreType => continue, // skip these for now
                PGNTag::Black => self.black_player = string_tag.clone(),
                PGNTag::White => self.white_player = string_tag.clone(),
                PGNTag::BlackElo => self.black_elo = string_tag.parse().unwrap(),
                PGNTag::WhiteElo => self.white_elo = string_tag.parse().unwrap(),
                PGNTag::Position => self.start_pos = string_tag.clone(),
            };
        }
    }
}
