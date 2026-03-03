// handles how the range is handled e.g. 1..10
#[derive(Debug)]
pub enum InclusiveRange {
    Both,      // 1 inclusive, 10 inclusive
    None,      // 1 exclusive, 10 exclusive
    FirstOnly, // 1 inclusive, 10 exclusive
    LastOnly,  // 1 exclusive, 10 inclusive
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameResult {
    Going,
    WhiteWins,
    BlackWins,
    Draw,
}
