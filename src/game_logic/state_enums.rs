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

#[derive(PartialEq, Eq)]
pub enum GameMode {
    SelectionWhite,
    SelectionBlack,
    MovementWhite,
    MovementBlack,
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
