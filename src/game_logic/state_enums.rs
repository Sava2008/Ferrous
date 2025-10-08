#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PieceVariant {
    P,
    N,
    B,
    R,
    Q,
    K,
    None,
}

#[derive(PartialEq, Eq, Debug)]
pub enum GameMode {
    SelectionWhite,
    SelectionBlack,
    MovementWhite,
    MovementBlack,
    Draw,
    WhiteWin,
    BlackWin,
}

#[derive(PartialEq, Eq)]
pub enum LinearDirection {
    RankRight,
    RankLeft,
    FileUp,
    FileDown,
}

#[derive(PartialEq, Eq)]
pub enum DiagonalDirection {
    UpRight,
    DownRight,
    UpLeft,
    DownLeft,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KingChecked {
    White,
    Black,
    None,
}
